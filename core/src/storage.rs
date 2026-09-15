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

        let mut transaction = self.pool.begin().await?;
        sqlx::query(
            r#"
            INSERT INTO nodes (id, cid, epistemic_type, payload, author, timestamp, parents, labels, signature, domain, source_uri)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(id_str)
        .bind(cid_str)
        .bind(epistemic_type_str)
        .bind(payload)
        .bind(author)
        .bind(timestamp_str)
        .bind(parents)
        .bind(labels)
        .bind(signature)
        .bind(&node.domain)
        .bind(&node.source_uri)
        .execute(&mut *transaction)
        .await?;

        // Store edges for graph traversal
        for parent_id in &node.parents {
            let from_str = parent_id.to_string();
            let to_str = node.id.to_string();
            sqlx::query(
                "INSERT OR IGNORE INTO edges (from_id, to_id, edge_type) VALUES (?, ?, 'supports')",
            )
            .bind(from_str)
            .bind(to_str)
            .execute(&mut *transaction)
            .await?;
        }

        transaction.commit().await?;
        Ok(())
    }

    pub async fn get_node(&self, id: Uuid) -> Result<Option<ProvenanceNode>> {
        let id_str = id.to_string();
        let query = "SELECT id, cid, epistemic_type, payload, author, timestamp, parents, labels, signature, domain, source_uri FROM nodes WHERE id = ?";
        let mut q = sqlx::query(query);
        q = q.bind(id_str);

        let row = q.fetch_optional(&*self.pool).await?;

        row.map(Self::row_to_node).transpose()
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
        rows.into_iter().map(Self::row_to_node).collect()
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

        rows.into_iter().map(Self::row_to_node).collect()
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

        rows.into_iter()
            .map(|r| {
                Ok(NarrativeDiff {
                    inference_id: parse_uuid(&r.inference_id, "narrative diff inference_id")?,
                    old_version: serde_json::from_str(&r.old_version)?,
                    new_version: serde_json::from_str(&r.new_version)?,
                    changed_fields: serde_json::from_str(&r.changed_fields)?,
                    timestamp: parse_timestamp(&r.timestamp, "narrative diff timestamp")?,
                    editor: serde_json::from_str(&r.editor)?,
                })
            })
            .collect()
    }

    fn row_to_node(row: sqlx::sqlite::SqliteRow) -> Result<ProvenanceNode> {
        let id = row.get::<String, _>("id");
        let epistemic_type = row.get::<String, _>("epistemic_type");
        let timestamp = row.get::<String, _>("timestamp");

        Ok(ProvenanceNode {
            id: parse_uuid(&id, "node id")?,
            cid: CID(row.get::<String, _>("cid")),
            epistemic_type: match epistemic_type.as_str() {
                "observed" => EpistemicType::Observed,
                "inferred" => EpistemicType::Inferred,
                "generated" => EpistemicType::Generated,
                other => {
                    return Err(crate::WitnessError::Validation(format!(
                        "unsupported stored epistemic type: {other}"
                    )));
                }
            },
            payload: serde_json::from_str(&row.get::<String, _>("payload"))?,
            author: serde_json::from_str(&row.get::<String, _>("author"))?,
            timestamp: parse_timestamp(&timestamp, "node timestamp")?,
            parents: serde_json::from_str(&row.get::<String, _>("parents"))?,
            labels: serde_json::from_str(&row.get::<String, _>("labels"))?,
            signature: row
                .get::<Option<String>, _>("signature")
                .map(|value| serde_json::from_str(&value))
                .transpose()?,
            domain: row.get("domain"),
            source_uri: row.get("source_uri"),
        })
    }
}

fn parse_uuid(value: &str, field: &str) -> Result<Uuid> {
    Uuid::parse_str(value).map_err(|error| {
        crate::WitnessError::Validation(format!("invalid stored {field}: {error}"))
    })
}

fn parse_timestamp(value: &str, field: &str) -> Result<DateTime<Utc>> {
    DateTime::parse_from_rfc3339(value)
        .map(|timestamp| timestamp.with_timezone(&Utc))
        .map_err(|error| {
            crate::WitnessError::Validation(format!("invalid stored {field}: {error}"))
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn test_storage() -> (Storage, std::path::PathBuf) {
        let path = std::env::temp_dir().join(format!("witness-storage-{}.db", Uuid::new_v4()));
        std::fs::File::create(&path).unwrap();
        let storage = Storage::new(&format!("sqlite://{}", path.display()))
            .await
            .unwrap();
        (storage, path)
    }

    #[tokio::test]
    async fn malformed_stored_node_returns_error_instead_of_panicking() {
        let (storage, path) = test_storage().await;
        let id = Uuid::new_v4();
        sqlx::query(
            "INSERT INTO nodes (id, cid, epistemic_type, payload, author, timestamp, parents, labels) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(id.to_string())
        .bind("invalid-cid")
        .bind("observed")
        .bind("{not-json")
        .bind(r#"{"id":"test","name":null,"author_type":"human","metadata":{}}"#)
        .bind(Utc::now().to_rfc3339())
        .bind("[]")
        .bind("[]")
        .execute(&*storage.pool)
        .await
        .unwrap();

        let result = storage.get_node(id).await;
        assert!(matches!(result, Err(crate::WitnessError::Serialization(_))));

        drop(storage);
        std::fs::remove_file(path).unwrap();
    }

    #[tokio::test]
    async fn unknown_stored_type_is_not_reclassified_as_observed() {
        let (storage, path) = test_storage().await;
        let id = Uuid::new_v4();
        let mut connection = storage.pool.acquire().await.unwrap();
        sqlx::query("PRAGMA ignore_check_constraints = ON")
            .execute(&mut *connection)
            .await
            .unwrap();
        sqlx::query(
            "INSERT INTO nodes (id, cid, epistemic_type, payload, author, timestamp, parents, labels) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(id.to_string())
        .bind("invalid-cid")
        .bind("unclassified")
        .bind("{}")
        .bind(r#"{"id":"test","name":null,"author_type":"human","metadata":{}}"#)
        .bind(Utc::now().to_rfc3339())
        .bind("[]")
        .bind("[]")
        .execute(&mut *connection)
        .await
        .unwrap();
        drop(connection);

        let result = storage.get_node(id).await;
        assert!(matches!(result, Err(crate::WitnessError::Validation(_))));

        drop(storage);
        std::fs::remove_file(path).unwrap();
    }
}
