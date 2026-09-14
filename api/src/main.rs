use witness_core::types::*;
use witness_core::storage::Storage;
use witness_core::Result;
use async_graphql::{Context, Object, Schema, SimpleObject, ID, InputObject};
use async_graphql_axum::GraphQL;
use axum::{routing::get, Router, Extension};
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub type WitnessSchema = Schema<QueryRoot, MutationRoot, SubscriptionRoot>;

#[derive(Clone)]
pub struct AppState {
    pub storage: Arc<Storage>,
}

#[derive(SimpleObject)]
#[graphql(name = "ProvenanceNode")]
pub struct GQLProvenanceNode {
    id: ID,
    cid: String,
    epistemic_type: EpistemicType,
    payload: String,  // JSON string
    author: String,   // JSON string
    timestamp: String,
    parents: Vec<ID>,
    labels: Vec<String>,
    signature: Option<String>,  // JSON string
    domain: Option<String>,
    source_uri: Option<String>,
}

impl From<ProvenanceNode> for GQLProvenanceNode {
    fn from(node: ProvenanceNode) -> Self {
        Self {
            id: node.id.to_string().into(),
            cid: node.cid.0,
            epistemic_type: node.epistemic_type,
            payload: serde_json::to_string(&node.payload).unwrap_or_default(),
            author: serde_json::to_string(&node.author).unwrap_or_default(),
            timestamp: node.timestamp.to_rfc3339(),
            parents: node.parents.iter().map(|p| p.to_string().into()).collect(),
            labels: node.labels,
            signature: node.signature.map(|s| serde_json::to_string(&s).unwrap_or_default()),
            domain: node.domain,
            source_uri: node.source_uri,
        }
    }
}

#[derive(SimpleObject)]
#[graphql(name = "NodeConnection")]
pub struct GQLNodeConnection {
    nodes: Vec<GQLProvenanceNode>,
    page_info: GQLPageInfo,
    total_count: i64,
}

#[derive(SimpleObject)]
#[graphql(name = "PageInfo")]
pub struct GQLPageInfo {
    has_next_page: bool,
    has_previous_page: bool,
    start_cursor: Option<String>,
    end_cursor: Option<String>,
}

#[derive(InputObject)]
#[graphql(name = "QueryFilter")]
pub struct GQLQueryFilter {
    epistemic_types: Option<Vec<EpistemicType>>,
    domains: Option<Vec<String>>,
    authors: Option<Vec<String>>,
    date_from: Option<String>,
    date_to: Option<String>,
    labels: Option<Vec<String>>,
    has_falsifiers: Option<bool>,
    parent_of: Option<ID>,
    child_of: Option<ID>,
}

#[derive(InputObject)]
#[graphql(name = "ObservationInput")]
pub struct GQLObservationInput {
    quantity: String,
    value_numeric: Option<f64>,
    value_text: Option<String>,
    unit: Option<String>,
    categorical: Option<String>,
    latitude: Option<f64>,
    longitude: Option<f64>,
    station_id: Option<String>,
    location_desc: Option<String>,
    altitude_m: Option<f64>,
    measured_at: String,
    instrument_id: String,
    instrument_name: Option<String>,
    instrument_model: Option<String>,
    calibration_ref: Option<String>,
    uncertainty_value: Option<f64>,
    uncertainty_unit: Option<String>,
    confidence_level: Option<f64>,
    uncertainty_method: Option<String>,
    author_id: String,
    author_name: Option<String>,
    author_type: String,
    labels: Option<Vec<String>>,
    domain: Option<String>,
}

#[derive(SimpleObject)]
#[graphql(name = "NarrativeDiff")]
pub struct GQLNarrativeDiff {
    inference_id: ID,
    old_version: String,
    new_version: String,
    changed_fields: Vec<String>,
    timestamp: String,
    editor: String,
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn node(&self, ctx: &Context<'_>, id: ID) -> async_graphql::Result<Option<GQLProvenanceNode>> {
        let state = ctx.data::<AppState>()?;
        let uuid = uuid::Uuid::parse_str(&id)?;
        if let Some(node) = state.storage.get_node(uuid).await? {
            Ok(Some(node.into()))
        } else {
            Ok(None)
        }
    }

    async fn nodes(
        &self,
        ctx: &Context<'_>,
        filter: Option<GQLQueryFilter>,
        first: Option<i32>,
        after: Option<String>,
    ) -> async_graphql::Result<GQLNodeConnection> {
        let state = ctx.data::<AppState>()?;
        let limit = first.unwrap_or(50) as i64;
        let offset = after.as_ref().and_then(|s| s.parse().ok()).unwrap_or(0);

        let filter = filter.map(convert_filter).unwrap_or_default();
        let nodes = state.storage.query_nodes(&filter, limit + 1, offset).await?;
        let total = state.storage.count_nodes(&filter).await?;

        let has_next = nodes.len() > limit as usize;
        let nodes: Vec<_> = nodes.into_iter().take(limit as usize).map(Into::into).collect();

        Ok(GQLNodeConnection {
            page_info: GQLPageInfo {
                has_next_page: has_next,
                has_previous_page: offset > 0,
                start_cursor: nodes.first().map(|n| n.id.to_string()),
                end_cursor: nodes.last().map(|n| n.id.to_string()),
            },
            total_count: total,
            nodes,
        })
    }

    async fn observations(
        &self,
        ctx: &Context<'_>,
        filter: Option<GQLQueryFilter>,
        first: Option<i32>,
        after: Option<String>,
    ) -> async_graphql::Result<GQLNodeConnection> {
        let mut f = filter.map(convert_filter).unwrap_or_default();
        f.epistemic_types = Some(vec![EpistemicType::Observed]);
        self.nodes(ctx, Some(f.into()), first, after).await
    }

    async fn inferences(
        &self,
        ctx: &Context<'_>,
        filter: Option<GQLQueryFilter>,
        first: Option<i32>,
        after: Option<String>,
    ) -> async_graphql::Result<GQLNodeConnection> {
        let mut f = filter.map(convert_filter).unwrap_or_default();
        f.epistemic_types = Some(vec![EpistemicType::Inferred]);
        self.nodes(ctx, Some(f.into()), first, after).await
    }

    async fn generations(
        &self,
        ctx: &Context<'_>,
        filter: Option<GQLQueryFilter>,
        first: Option<i32>,
        after: Option<String>,
    ) -> async_graphql::Result<GQLNodeConnection> {
        let mut f = filter.map(convert_filter).unwrap_or_default();
        f.epistemic_types = Some(vec![EpistemicType::Generated]);
        self.nodes(ctx, Some(f.into()), first, after).await
    }

    async fn children(&self, ctx: &Context<'_>, parent_id: ID) -> async_graphql::Result<Vec<GQLProvenanceNode>> {
        let state = ctx.data::<AppState>()?;
        let uuid = uuid::Uuid::parse_str(&parent_id)?;
        let children = state.storage.get_children(uuid).await?;
        Ok(children.into_iter().map(Into::into).collect())
    }

    async fn parents(&self, ctx: &Context<'_>, child_id: ID) -> async_graphql::Result<Vec<GQLProvenanceNode>> {
        let state = ctx.data::<AppState>()?;
        let uuid = uuid::Uuid::parse_str(&child_id)?;
        let parents = state.storage.get_parents(uuid).await?;
        Ok(parents.into_iter().map(Into::into).collect())
    }

    async fn diffs(&self, ctx: &Context<'_>, inference_id: ID) -> async_graphql::Result<Vec<GQLNarrativeDiff>> {
        let state = ctx.data::<AppState>()?;
        let uuid = uuid::Uuid::parse_str(&inference_id)?;
        let diffs = state.storage.get_diffs(uuid).await?;
        Ok(diffs.into_iter().map(|d| GQLNarrativeDiff {
            inference_id: d.inference_id.to_string().into(),
            old_version: serde_json::to_string(&d.old_version).unwrap_or_default(),
            new_version: serde_json::to_string(&d.new_version).unwrap_or_default(),
            changed_fields: d.changed_fields,
            timestamp: d.timestamp.to_rfc3339(),
            editor: serde_json::to_string(&d.editor).unwrap_or_default(),
        }).collect())
    }

    async fn falsifiers(&self, ctx: &Context<'_>, inference_id: ID) -> async_graphql::Result<Vec<String>> {
        let state = ctx.data::<AppState>()?;
        let uuid = uuid::Uuid::parse_str(&inference_id)?;
        if let Some(node) = state.storage.get_node(uuid).await? {
            if let Some(inference) = node.payload.get("derivation") {
                if let Some(falsifiers) = inference.get("falsifiers").and_then(|f| f.as_array()) {
                    return Ok(falsifiers.iter().map(|f| f.get("description").and_then(|d| d.as_str()).unwrap_or("").to_string()).collect());
                }
            }
        }
        Ok(Vec::new())
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn ingest_observation(
        &self,
        ctx: &Context<'_>,
        input: GQLObservationInput,
    ) -> async_graphql::Result<GQLProvenanceNode> {
        let state = ctx.data::<AppState>()?;
        // Simplified - would use ingestion service in real impl
        Ok(GQLProvenanceNode {
            id: uuid::Uuid::new_v4().to_string().into(),
            cid: "pending".to_string(),
            epistemic_type: EpistemicType::Observed,
            payload: "{}".to_string(),
            author: "{}".to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            parents: vec![],
            labels: input.labels.unwrap_or_default(),
            signature: None,
            domain: input.domain,
            source_uri: None,
        })
    }
}

pub struct SubscriptionRoot;

#[Object]
impl SubscriptionRoot {
    async fn node_created(&self, ctx: &Context<'_>) -> async_graphql::Result<async_graphql::futures_util::stream::Stream<GQLProvenanceNode>> {
        // Would implement with async-graphql subscriptions
        Err(async_graphql::Error::new("Not implemented"))
    }
}

fn convert_filter(f: GQLQueryFilter) -> witness_core::types::QueryFilter {
    witness_core::types::QueryFilter {
        epistemic_types: f.epistemic_types,
        domains: f.domains,
        authors: f.authors,
        date_range: f.date_from.zip(f.date_to).map(|(from, to)| witness_core::types::DateRange {
            from: from.parse().unwrap_or_else(|_| chrono::Utc::now()),
            to: to.parse().unwrap_or_else(|_| chrono::Utc::now()),
        }),
        labels: f.labels,
        has_falsifiers: f.has_falsifiers,
        parent_of: f.parent_of.and_then(|id| uuid::Uuid::parse_str(&id).ok()),
        child_of: f.child_of.and_then(|id| uuid::Uuid::parse_str(&id).ok()),
    }
}

impl From<witness_core::types::QueryFilter> for GQLQueryFilter {
    fn from(f: witness_core::types::QueryFilter) -> Self {
        Self {
            epistemic_types: f.epistemic_types,
            domains: f.domains,
            authors: f.authors,
            date_from: f.date_range.as_ref().map(|r| r.from.to_rfc3339()),
            date_to: f.date_range.as_ref().map(|r| r.to.to_rfc3339()),
            labels: f.labels,
            has_falsifiers: f.has_falsifiers,
            parent_of: f.parent_of.map(|id| id.to_string().into()),
            child_of: f.child_of.map(|id| id.to_string().into()),
        }
    }
}

pub async fn run_server(database_url: &str, port: u16) -> Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new("info,witness=debug"))
        .with(tracing_subscriber::fmt::layer().json())
        .init();

    let storage = Arc::new(Storage::new(database_url).await?);
    let state = AppState { storage };

    let schema = Schema::build(QueryRoot, MutationRoot, SubscriptionRoot)
        .data(state)
        .finish();

    let app = Router::new()
        .route("/graphql", get(graphql_playground).post(graphql_handler))
        .route("/health", get(health_check))
        .layer(Extension(schema));

    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    tracing::info!("Witness API listening on http://0.0.0.0:{}", port);
    axum::serve(listener, app).await?;

    Ok(())
}

async fn graphql_playground() -> axum::response::Html<String> {
    axum::response::Html(async_graphql::http::playground_source(async_graphql::http::GraphQLPlaygroundConfig::new("/graphql")))
}

async fn graphql_handler(
    Extension(schema): Extension<WitnessSchema>,
    req: async_graphql_axum::GraphQLRequest,
) -> async_graphql_axum::GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn health_check() -> &'static str {
    "OK"
}