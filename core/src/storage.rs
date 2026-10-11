use crate::Result;
use crate::types::*;
use chrono::{DateTime, Utc};
use sqlx_core::{query::query as sql_query, row::Row};
use sqlx_sqlite::{SqliteConnectOptions, SqlitePool};
use std::borrow::Cow;
use std::future::Future;
use std::pin::Pin;
use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;

pub struct Storage {
    pub pool: Arc<SqlitePool>,
}

// Keep migrations embedded so installed binaries work outside the source tree.
// A custom source avoids SQLx's macro dependency on unused MySQL/Postgres stacks.
// Register new migrations here; the inventory regression checks the directory.
#[derive(Debug)]
struct EmbeddedMigrations;

impl sqlx_core::migrate::MigrationSource<'static> for EmbeddedMigrations {
    fn resolve(
        self,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = std::result::Result<
                        Vec<sqlx_core::migrate::Migration>,
                        sqlx_core::error::BoxDynError,
                    >,
                > + Send,
        >,
    > {
        Box::pin(async {
            Ok(vec![sqlx_core::migrate::Migration::new(
                1,
                Cow::Borrowed("initial schema"),
                sqlx_core::migrate::MigrationType::Simple,
                Cow::Borrowed(include_str!("../migrations/001_initial_schema.sql")),
                false,
            )])
        })
    }
}

impl Storage {
    pub async fn new(database_url: &str) -> Result<Self> {
        // Let SQLx parse paths and URI parameters; stripping leading slashes
        // changes absolute paths into unrelated files under the working directory.
        let options = SqliteConnectOptions::from_str(database_url)?.create_if_missing(true);
        let filename = options.get_filename();
        if let Some(parent) = filename
            .parent()
            .filter(|path| !path.as_os_str().is_empty())
        {
            std::fs::create_dir_all(parent)?;
        }
        let pool = SqlitePool::connect_with(options).await?;
        let migrator = sqlx_core::migrate::Migrator::new(EmbeddedMigrations).await?;
        if let Err(error) = migrator.run(&pool).await {
            pool.close().await;
            return Err(error.into());
        }
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
        sql_query(
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
            sql_query(
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
        let mut q = sql_query(query);
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

        append_node_filters(&mut query, &mut args, filter);

        query.push_str(" ORDER BY timestamp DESC, id DESC LIMIT ? OFFSET ?");
        args.push(limit.to_string());
        args.push(offset.to_string());

        let mut q = sql_query(&query);
        for arg in args {
            q = q.bind(arg);
        }

        let rows = q.fetch_all(&*self.pool).await?;
        rows.into_iter().map(Self::row_to_node).collect()
    }

    pub async fn count_nodes(&self, filter: &QueryFilter) -> Result<i64> {
        let mut query = String::from("SELECT COUNT(*) as count FROM nodes WHERE 1=1");
        let mut args: Vec<String> = Vec::new();

        append_node_filters(&mut query, &mut args, filter);

        let mut q = sql_query(&query);
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
        let mut q = sql_query(query);
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

        sql_query(
            r#"
            INSERT INTO narrative_diffs (inference_id, old_version, new_version, changed_fields, timestamp, editor)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(inference_id_str)
        .bind(old_version)
        .bind(new_version)
        .bind(changed_fields)
        .bind(timestamp_str)
        .bind(editor)
        .execute(&*self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_diffs(&self, inference_id: Uuid) -> Result<Vec<NarrativeDiff>> {
        let inference_str = inference_id.to_string();
        let rows = sql_query(
            "SELECT inference_id, old_version, new_version, changed_fields, timestamp, editor FROM narrative_diffs WHERE inference_id = ? ORDER BY timestamp DESC",
        )
        .bind(inference_str)
        .fetch_all(&*self.pool)
        .await?;

        rows.into_iter()
            .map(|r| {
                Ok(NarrativeDiff {
                    inference_id: parse_uuid(
                        r.try_get("inference_id")?,
                        "narrative diff inference_id",
                    )?,
                    old_version: serde_json::from_str(r.try_get("old_version")?)?,
                    new_version: serde_json::from_str(r.try_get("new_version")?)?,
                    changed_fields: serde_json::from_str(r.try_get("changed_fields")?)?,
                    timestamp: parse_timestamp(
                        r.try_get("timestamp")?,
                        "narrative diff timestamp",
                    )?,
                    editor: serde_json::from_str(r.try_get("editor")?)?,
                })
            })
            .collect()
    }

    fn row_to_node(row: sqlx_sqlite::SqliteRow) -> Result<ProvenanceNode> {
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

fn append_node_filters(query: &mut String, args: &mut Vec<String>, filter: &QueryFilter) {
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

    if let Some(authors) = &filter.authors {
        for a in authors {
            query.push_str(" AND author LIKE '%' || ? || '%'");
            args.push(a.clone());
        }
    }

    if let Some(range) = &filter.date_range {
        query.push_str(" AND timestamp BETWEEN ? AND ?");
        args.push(range.from.to_rfc3339());
        args.push(range.to.to_rfc3339());
    }

    if let Some(labels) = &filter.labels {
        for l in labels {
            query.push_str(" AND EXISTS (SELECT 1 FROM json_each(nodes.labels) AS label WHERE label.value = ?)");
            args.push(l.clone());
        }
    }

    if let Some(has_falsifiers) = filter.has_falsifiers {
        query.push_str(if has_falsifiers {
            " AND COALESCE(json_array_length(payload, '$.derivation.falsifiers'), 0) > 0"
        } else {
            " AND COALESCE(json_array_length(payload, '$.derivation.falsifiers'), 0) = 0"
        });
    }
    if let Some(child) = filter.parent_of {
        query.push_str(
            " AND id IN (SELECT from_id FROM edges WHERE to_id = ? AND edge_type = 'supports')",
        );
        args.push(child.to_string());
    }
    if let Some(parent) = filter.child_of {
        query.push_str(
            " AND id IN (SELECT to_id FROM edges WHERE from_id = ? AND edge_type = 'supports')",
        );
        args.push(parent.to_string());
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

    #[tokio::test]
    async fn embedded_migrations_match_files_and_preserve_checksums_on_reopen() {
        let embedded = sqlx_core::migrate::Migrator::new(EmbeddedMigrations)
            .await
            .unwrap();
        let files = sqlx_core::migrate::Migrator::new(
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("migrations"),
        )
        .await
        .unwrap();
        assert_eq!(embedded.iter().count(), files.iter().count());
        for (a, b) in embedded.iter().zip(files.iter()) {
            assert_eq!(a.version, b.version);
            assert_eq!(a.description, b.description);
            assert_eq!(a.sql, b.sql);
            assert_eq!(a.checksum, b.checksum);
        }
        let (storage, path) = test_storage().await;
        storage.pool.close().await;
        drop(storage);
        let reopened = Storage::new(&format!("sqlite://{}", path.display()))
            .await
            .unwrap();
        let count: i64 =
            sqlx_core::query_scalar::query_scalar("SELECT COUNT(*) FROM _sqlx_migrations")
                .fetch_one(&*reopened.pool)
                .await
                .unwrap();
        assert_eq!(count, 1);
        sql_query("UPDATE _sqlx_migrations SET checksum = X'00'")
            .execute(&*reopened.pool)
            .await
            .unwrap();
        reopened.pool.close().await;
        drop(reopened);
        assert!(matches!(
            Storage::new(&format!("sqlite://{}", path.display())).await,
            Err(crate::WitnessError::Migration(_))
        ));
        crate::remove_test_path(&path, false).await;
    }

    #[tokio::test]
    async fn narrative_diff_round_trip_preserves_all_fields() {
        let (storage, path) = test_storage().await;
        let author = Author {
            id: "human:test".into(),
            name: Some("Test editor".into()),
            author_type: AuthorType::Human,
            metadata: Default::default(),
        };
        let payload = serde_json::json!({"claim": "original fixture"});
        let node = ProvenanceNode {
            id: Uuid::new_v4(),
            cid: CID::new(&serde_json::to_vec(&payload).unwrap()),
            epistemic_type: EpistemicType::Inferred,
            payload,
            author: author.clone(),
            timestamp: Utc::now(),
            parents: vec![],
            signature: None,
            labels: vec!["fixture".into()],
            domain: None,
            source_uri: None,
        };
        storage.store_node(&node).await.unwrap();
        let mut edited = node.clone();
        edited.payload = serde_json::json!({"claim": "edited fixture"});
        edited.cid = CID::new(&serde_json::to_vec(&edited.payload).unwrap());
        let diff = NarrativeDiff {
            inference_id: node.id,
            old_version: node.clone(),
            new_version: edited,
            changed_fields: vec!["payload.claim".into()],
            timestamp: Utc::now(),
            editor: author,
        };
        storage.store_diff(&diff).await.unwrap();
        let restored = storage.get_diffs(node.id).await.unwrap();
        assert_eq!(restored.len(), 1);
        assert_eq!(
            serde_json::to_value(&restored[0]).unwrap(),
            serde_json::to_value(&diff).unwrap()
        );
        storage.pool.close().await;
        drop(storage);
        crate::remove_test_path(&path, false).await;
    }

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
        sql_query(
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

        storage.pool.close().await;
        drop(storage);
        crate::remove_test_path(&path, false).await;
    }

    #[tokio::test]
    async fn unknown_stored_type_is_not_reclassified_as_observed() {
        let (storage, path) = test_storage().await;
        let id = Uuid::new_v4();
        let mut connection = storage.pool.acquire().await.unwrap();
        sql_query("PRAGMA ignore_check_constraints = ON")
            .execute(&mut *connection)
            .await
            .unwrap();
        sql_query(
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

        storage.pool.close().await;
        drop(storage);
        crate::remove_test_path(&path, false).await;
    }

    #[tokio::test]
    async fn filtered_count_matches_filtered_sql_query() {
        let (storage, path) = test_storage().await;

        for (author, labels) in [
            ("US Geological Survey", r#"["demo-fixture","hydrology"]"#),
            ("National Weather Service", r#"["demo-fixture","weather"]"#),
        ] {
            sql_query(
                "INSERT INTO nodes (id, cid, epistemic_type, payload, author, timestamp, parents, labels, domain) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            )
            .bind(Uuid::new_v4().to_string())
            .bind("test-cid")
            .bind("observed")
            .bind("{}")
            .bind(format!(r#"{{"id":"test","name":"{author}","author_type":"instrument","metadata":{{}}}}"#))
            .bind(Utc::now().to_rfc3339())
            .bind("[]")
            .bind(labels)
            .bind(if author == "US Geological Survey" { "hydrology" } else { "weather" })
            .execute(&*storage.pool)
            .await
            .unwrap();
        }

        let filter = QueryFilter {
            epistemic_types: Some(vec![EpistemicType::Observed]),
            authors: Some(vec!["US Geological Survey".to_string()]),
            labels: Some(vec!["demo-fixture".to_string()]),
            ..Default::default()
        };

        let nodes = storage.query_nodes(&filter, 20, 0).await.unwrap();
        let count = storage.count_nodes(&filter).await.unwrap();
        assert_eq!(nodes.len(), 1);
        assert_eq!(count, nodes.len() as i64);

        storage.pool.close().await;
        drop(storage);
        crate::remove_test_path(&path, false).await;
    }

    #[tokio::test]
    async fn label_filters_match_complete_literal_tags_and_counts() {
        let (storage, path) = test_storage().await;
        for labels in [
            vec!["EXP-001"],
            vec!["EXP-001-backup"],
            vec!["literal%_tag"],
        ] {
            let payload = serde_json::json!({});
            let node = ProvenanceNode {
                id: Uuid::new_v4(),
                cid: CID::new(&serde_json::to_vec(&payload).unwrap()),
                epistemic_type: EpistemicType::Observed,
                payload,
                author: Author {
                    id: "test".into(),
                    name: None,
                    author_type: AuthorType::Software,
                    metadata: Default::default(),
                },
                timestamp: Utc::now(),
                parents: vec![],
                signature: None,
                labels: labels.into_iter().map(str::to_owned).collect(),
                domain: None,
                source_uri: None,
            };
            storage.store_node(&node).await.unwrap();
        }
        for (labels, count) in [
            (vec!["EXP-001"], 1),
            (vec!["EXP-001-backup"], 1),
            (vec!["%"], 0),
            (vec!["_tag"], 0),
            (vec!["literal%_tag"], 1),
            (vec!["exp-001"], 0),
            (vec!["EXP-001", "missing"], 0),
        ] {
            let filter = QueryFilter {
                labels: Some(labels.into_iter().map(str::to_owned).collect()),
                ..Default::default()
            };
            assert_eq!(
                storage.query_nodes(&filter, 100, 0).await.unwrap().len(),
                count,
                "{filter:?}"
            );
            assert_eq!(storage.count_nodes(&filter).await.unwrap(), count as i64);
        }
        storage.pool.close().await;
        drop(storage);
        crate::remove_test_path(&path, false).await;
    }

    #[tokio::test]
    async fn fresh_absolute_database_uses_the_requested_path() {
        let directory = std::env::temp_dir().join(format!("witness-absolute-{}", Uuid::new_v4()));
        let path = directory.join("nested").join("fresh.db");
        let storage = Storage::new(&format!("sqlite://{}?mode=rwc", path.display()))
            .await
            .unwrap();
        assert!(path.is_file());
        #[cfg(unix)]
        {
            let relative_shadow =
                std::path::PathBuf::from(path.to_string_lossy().trim_start_matches('/'));
            assert!(!relative_shadow.exists());
        }
        assert_eq!(
            storage.count_nodes(&QueryFilter::default()).await.unwrap(),
            0
        );
        storage.pool.close().await;
        drop(storage);
        crate::remove_test_path(&directory, true).await;
    }

    #[tokio::test]
    async fn memory_database_migrates_without_a_disk_file() {
        let storage = Storage::new("sqlite::memory:").await.unwrap();
        assert_eq!(
            storage.count_nodes(&QueryFilter::default()).await.unwrap(),
            0
        );
        storage.pool.close().await;
    }
}
