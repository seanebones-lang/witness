use crate::Result;
use crate::types::*;
use chrono::{DateTime, Utc};
use sqlx::{Row, SqlitePool};
use std::sync::Arc;
use uuid::Uuid;

pub struct Storage {
    pub pool: Arc<SqlitePool>,
}

impl Storage {
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = SqlitePool::connect(database_url).await?;
        sqlx::migrate!("./migrations").run(&pool).await?;
        Ok(Self {
            pool: Arc::new(pool),
        })
    }

    pub async fn store_node(&self, node: &ProvenanceNode) -> Result<()> {
        let payload = serde_json::to_string(&node.payload)?;
        let parents = serde_json::to_string(&node.parents)?;
        let labels = serde_json::to_string(&node.labels)?;
        let signature = node
            .signature
            .as_ref()
            .map(serde_json::to_string)
            .transpose()?;
        let author = serde_json::to_string(&node.author)?;

        let id_str = node.id.to_string();
        let cid_str = node.cid.0.clone();
        let epistemic_type_str = format!("{:?}", node.epistemic_type).to_lowercase();
        let timestamp_str = node.timestamp.to_rfc3339();

        sqlx::query!(
            r#"
            INSERT INTO nodes (id, cid, epistemic_type, payload, author, timestamp, parents, labels, signature, domain, source_uri)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            ON CONFLICT(id) DO UPDATE SET
                cid = excluded.cid,
                epistemic_type = excluded.epistemic_type,
                payload = excluded.payload,
                author = excluded.author,
                timestamp = excluded.timestamp,
                parents = excluded.parents,
                labels = excluded.labels,
                signature = excluded.signature,
                domain = excluded.domain,
                source_uri = excluded.source_uri
            "#,
            id_str,
            cid_str,
            epistemic_type_str,
            payload,
            author,
            timestamp_str,
            parents,
            labels,
            signature,
            node.domain,
            node.source_uri
        )
        .execute(&*self.pool)
        .await?;

        // Store edges for graph traversal
        for parent_id in &node.parents {
            let from_str = parent_id.to_string();
            let to_str = node.id.to_string();
            sqlx::query!(
                "INSERT OR IGNORE INTO edges (from_id, to_id, edge_type) VALUES (?, ?, 'supports')",
                from_str,
                to_str
            )
            .execute(&*self.pool)
            .await?;
        }

        Ok(())
    }

    pub async fn get_node(&self, id: Uuid) -> Result<Option<ProvenanceNode>> {
        let id_str = id.to_string();
        let query = "SELECT id, cid, epistemic_type, payload, author, timestamp, parents, labels, signature, domain, source_uri FROM nodes WHERE id = ?";
        let mut q = sqlx::query(query);
        q = q.bind(id_str);

        let row = q.fetch_optional(&*self.pool).await?;

        Ok(row.map(|r| self.row_to_node(r)))
    }

    pub async fn query_nodes(
        &self,
        filter: &QueryFilter,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<ProvenanceNode>> {
        let mut query = String::from(
            "SELECT id, cid, epistemic_type, payload, author, timestamp, parents, labels, signature, domain, source_uri FROM nodes WHERE 1=1",
        );
        let mut args: Vec<String> = Vec::new();

        if let Some(types) = &filter.epistemic_types {
            let placeholders = types.iter().map(|_| "?").collect::<Vec<_>>().join(",");
            query.push_str(&format!(" AND epistemic_type IN ({})", placeholders));
            for t in types {
                args.push(format!("{:?}", t).to_lowercase());
            }
        }

        if let Some(domains) = &filter.domains {
            let _placeholders = domains.iter().map(|_| "?").collect::<Vec<_>>().join(",");
            query.push_str(&format!(" AND domain IN ({})", _placeholders));
            for d in domains {
                args.push(d.clone());
            }
        }

        if let Some(authors) = &filter.authors {
            let _placeholders = authors.iter().map(|_| "?").collect::<Vec<_>>().join(",");
            query.push_str(" AND author LIKE '%' || ? || '%'");
            for a in authors {
                args.push(a.clone());
            }
        }

        if let Some(range) = &filter.date_range {
            query.push_str(" AND timestamp BETWEEN ? AND ?");
            args.push(range.from.to_rfc3339());
            args.push(range.to.to_rfc3339());
        }

        if let Some(labels) = &filter.labels {
            let _placeholders = labels.iter().map(|_| "?").collect::<Vec<_>>().join(",");
            query.push_str(" AND labels LIKE '%' || ? || '%'");
            for l in labels {
                args.push(l.clone());
            }
        }

        query.push_str(" ORDER BY timestamp DESC LIMIT ? OFFSET ?");
        args.push(limit.to_string());
        args.push(offset.to_string());

        let mut q = sqlx::query(&query);
        for arg in args {
            q = q.bind(arg);
        }

        let rows = q.fetch_all(&*self.pool).await?;
        Ok(rows.into_iter().map(|r| self.row_to_node(r)).collect())
    }

    pub async fn count_nodes(&self, filter: &QueryFilter) -> Result<i64> {
        let mut query = String::from("SELECT COUNT(*) as count FROM nodes WHERE 1=1");
        let mut args: Vec<String> = Vec::new();

        if let Some(types) = &filter.epistemic_types {
            let placeholders = types.iter().map(|_| "?").collect::<Vec<_>>().join(",");
            query.push_str(&format!(" AND epistemic_type IN ({})", placeholders));
            for t in types {
                args.push(format!("{:?}", t).to_lowercase());
            }
        }

        if let Some(domains) = &filter.domains {
            let placeholders = domains.iter().map(|_| "?").collect::<Vec<_>>().join(",");
            query.push_str(&format!(" AND domain IN ({})", placeholders));
            for d in domains {
                args.push(d.clone());
            }
        }

        let mut q = sqlx::query(&query);
        for arg in args {
            q = q.bind(arg);
        }

        let row = q.fetch_one(&*self.pool).await?;
        Ok(row.get("count"))
    }

    pub async fn get_children(&self, parent_id: Uuid) -> Result<Vec<ProvenanceNode>> {
        let parent_str = parent_id.to_string();
        let query = "SELECT n.id, n.cid, n.epistemic_type, n.payload, n.author, n.timestamp, n.parents, n.labels, n.signature, n.domain, n.source_uri
             FROM nodes n
             JOIN edges e ON n.id = e.to_id
             WHERE e.from_id = ? AND e.edge_type = 'supports'";
        let mut q = sqlx::query(query);
        q = q.bind(parent_str);

        let rows = q.fetch_all(&*self.pool).await?;

        Ok(rows.into_iter().map(|r| self.row_to_node(r)).collect())
    }

    pub async fn get_parents(&self, child_id: Uuid) -> Result<Vec<ProvenanceNode>> {
        let child = self.get_node(child_id).await?;
        if let Some(child) = child {
            let mut parents = Vec::new();
            for parent_id in child.parents {
                if let Some(p) = self.get_node(parent_id).await? {
                    parents.push(p);
                }
            }
            Ok(parents)
        } else {
            Ok(Vec::new())
        }
    }

    pub async fn store_diff(&self, diff: &NarrativeDiff) -> Result<()> {
        let old_version = serde_json::to_string(&diff.old_version)?;
        let new_version = serde_json::to_string(&diff.new_version)?;
        let changed_fields = serde_json::to_string(&diff.changed_fields)?;
        let editor = serde_json::to_string(&diff.editor)?;

        let inference_id_str = diff.inference_id.to_string();
        let timestamp_str = diff.timestamp.to_rfc3339();

        sqlx::query!(
            r#"
            INSERT INTO narrative_diffs (inference_id, old_version, new_version, changed_fields, timestamp, editor)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
            inference_id_str,
            old_version,
            new_version,
            changed_fields,
            timestamp_str,
            editor
        )
        .execute(&*self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_diffs(&self, inference_id: Uuid) -> Result<Vec<NarrativeDiff>> {
        let inference_str = inference_id.to_string();
        let rows = sqlx::query!(
            "SELECT inference_id, old_version, new_version, changed_fields, timestamp, editor FROM narrative_diffs WHERE inference_id = ? ORDER BY timestamp DESC",
            inference_str
        )
        .fetch_all(&*self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| NarrativeDiff {
                inference_id: Uuid::parse_str(&r.inference_id).unwrap(),
                old_version: serde_json::from_str(&r.old_version).unwrap(),
                new_version: serde_json::from_str(&r.new_version).unwrap(),
                changed_fields: serde_json::from_str(&r.changed_fields).unwrap(),
                timestamp: DateTime::parse_from_rfc3339(&r.timestamp)
                    .unwrap()
                    .with_timezone(&Utc),
                editor: serde_json::from_str(&r.editor).unwrap(),
            })
            .collect())
    }

    fn row_to_node(&self, row: sqlx::sqlite::SqliteRow) -> ProvenanceNode {
        ProvenanceNode {
            id: Uuid::parse_str(&row.get::<String, _>("id")).unwrap(),
            cid: CID(row.get::<String, _>("cid")),
            epistemic_type: match row.get::<String, _>("epistemic_type").as_str() {
                "observed" => EpistemicType::Observed,
                "inferred" => EpistemicType::Inferred,
                "generated" => EpistemicType::Generated,
                _ => EpistemicType::Observed,
            },
            payload: serde_json::from_str(&row.get::<String, _>("payload")).unwrap(),
            author: serde_json::from_str(&row.get::<String, _>("author")).unwrap(),
            timestamp: DateTime::parse_from_rfc3339(&row.get::<String, _>("timestamp"))
                .unwrap()
                .with_timezone(&Utc),
            parents: serde_json::from_str(&row.get::<String, _>("parents")).unwrap(),
            labels: serde_json::from_str(&row.get::<String, _>("labels")).unwrap(),
            signature: row
                .get::<Option<String>, _>("signature")
                .and_then(|s| serde_json::from_str(&s).ok()),
            domain: row.get("domain"),
            source_uri: row.get("source_uri"),
        }
    }
}
