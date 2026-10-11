use async_graphql::{Context, EmptySubscription, ID, InputObject, Object, Schema, SimpleObject};
use async_graphql_axum::GraphQL;
use axum::Json;
use axum::{
    Router,
    extract::{Path, Query, State},
    routing::get,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;
use witness_core::Result as CoreResult;
use witness_core::ingestion::IngestionService;
use witness_core::storage::Storage;
use witness_core::types::*;

pub type WitnessSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

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
    payload: String, // JSON string
    author: String,  // JSON string
    timestamp: String,
    parents: Vec<ID>,
    labels: Vec<String>,
    signature: Option<String>, // JSON string
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
            signature: node
                .signature
                .map(|s| serde_json::to_string(&s).unwrap_or_default()),
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
    async fn node(
        &self,
        ctx: &Context<'_>,
        id: ID,
    ) -> async_graphql::Result<Option<GQLProvenanceNode>> {
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
        let limit = first.unwrap_or(50).clamp(1, 100) as i64;
        let offset = after
            .as_ref()
            .map(|cursor| {
                cursor.parse::<i64>().map_err(|_| {
                    async_graphql::Error::new("after must be an offset cursor returned by pageInfo")
                })
            })
            .transpose()?
            .unwrap_or(0);
        if offset < 0 {
            return Err("after must not be negative".into());
        }

        let filter = filter.map(convert_filter).transpose()?.unwrap_or_default();
        let nodes = state
            .storage
            .query_nodes(&filter, limit + 1, offset)
            .await?;
        let total = state.storage.count_nodes(&filter).await?;

        let has_next = nodes.len() > limit as usize;
        let nodes: Vec<_> = nodes
            .into_iter()
            .take(limit as usize)
            .map(Into::into)
            .collect();

        Ok(GQLNodeConnection {
            page_info: GQLPageInfo {
                has_next_page: has_next,
                has_previous_page: offset > 0,
                start_cursor: (!nodes.is_empty()).then(|| offset.to_string()),
                end_cursor: (!nodes.is_empty())
                    .then(|| offset.saturating_add(nodes.len() as i64).to_string()),
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
        let mut f = filter.map(convert_filter).transpose()?.unwrap_or_default();
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
        let mut f = filter.map(convert_filter).transpose()?.unwrap_or_default();
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
        let mut f = filter.map(convert_filter).transpose()?.unwrap_or_default();
        f.epistemic_types = Some(vec![EpistemicType::Generated]);
        self.nodes(ctx, Some(f.into()), first, after).await
    }

    async fn children(
        &self,
        ctx: &Context<'_>,
        parent_id: ID,
    ) -> async_graphql::Result<Vec<GQLProvenanceNode>> {
        let state = ctx.data::<AppState>()?;
        let uuid = uuid::Uuid::parse_str(&parent_id)?;
        let children = state.storage.get_children(uuid).await?;
        Ok(children.into_iter().map(Into::into).collect())
    }

    async fn parents(
        &self,
        ctx: &Context<'_>,
        child_id: ID,
    ) -> async_graphql::Result<Vec<GQLProvenanceNode>> {
        let state = ctx.data::<AppState>()?;
        let uuid = uuid::Uuid::parse_str(&child_id)?;
        let parents = state.storage.get_parents(uuid).await?;
        Ok(parents.into_iter().map(Into::into).collect())
    }

    async fn diffs(
        &self,
        ctx: &Context<'_>,
        inference_id: ID,
    ) -> async_graphql::Result<Vec<GQLNarrativeDiff>> {
        let state = ctx.data::<AppState>()?;
        let uuid = uuid::Uuid::parse_str(&inference_id)?;
        let diffs = state.storage.get_diffs(uuid).await?;
        Ok(diffs
            .into_iter()
            .map(|d| GQLNarrativeDiff {
                inference_id: d.inference_id.to_string().into(),
                old_version: serde_json::to_string(&d.old_version).unwrap_or_default(),
                new_version: serde_json::to_string(&d.new_version).unwrap_or_default(),
                changed_fields: d.changed_fields,
                timestamp: d.timestamp.to_rfc3339(),
                editor: serde_json::to_string(&d.editor).unwrap_or_default(),
            })
            .collect())
    }

    async fn falsifiers(
        &self,
        ctx: &Context<'_>,
        inference_id: ID,
    ) -> async_graphql::Result<Vec<String>> {
        let state = ctx.data::<AppState>()?;
        let uuid = uuid::Uuid::parse_str(&inference_id)?;
        if let Some(node) = state.storage.get_node(uuid).await?
            && let Some(inference) = node.payload.get("derivation")
            && let Some(falsifiers) = inference.get("falsifiers").and_then(|f| f.as_array())
        {
            return Ok(falsifiers
                .iter()
                .map(|f| {
                    f.get("description")
                        .and_then(|d| d.as_str())
                        .unwrap_or("")
                        .to_string()
                })
                .collect());
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
        if input.value_numeric.is_none()
            && input.value_text.is_none()
            && input.categorical.is_none()
        {
            return Err("an observation must contain a numeric, text, or categorical value".into());
        }

        let measured_at = input
            .measured_at
            .parse::<chrono::DateTime<chrono::Utc>>()
            .map_err(|_| async_graphql::Error::new("measuredAt must be an RFC 3339 timestamp"))?;
        let author_type = match input.author_type.to_ascii_lowercase().as_str() {
            "human" => AuthorType::Human,
            "instrument" => AuthorType::Instrument,
            "model" => AuthorType::Model,
            "institution" => AuthorType::Institution,
            "software" => AuthorType::Software,
            _ => {
                return Err(
                    "authorType must be human, instrument, model, institution, or software".into(),
                );
            }
        };
        let uncertainty = match input.uncertainty_value {
            Some(value) => {
                let confidence_level = input.confidence_level.ok_or_else(|| {
                    async_graphql::Error::new(
                        "confidenceLevel is required when uncertaintyValue is supplied",
                    )
                })?;
                if !(0.0..=1.0).contains(&confidence_level) {
                    return Err("confidenceLevel must be between 0 and 1".into());
                }
                Some(Uncertainty {
                    value,
                    unit: input.uncertainty_unit.ok_or_else(|| {
                        async_graphql::Error::new(
                            "uncertaintyUnit is required when uncertaintyValue is supplied",
                        )
                    })?,
                    confidence_level,
                    method: input.uncertainty_method.ok_or_else(|| {
                        async_graphql::Error::new(
                            "uncertaintyMethod is required when uncertaintyValue is supplied",
                        )
                    })?,
                })
            }
            None => None,
        };

        let measurement = Measurement {
            quantity: input.quantity,
            value: MeasuredValue {
                numeric: input.value_numeric,
                text: input.value_text,
                unit: input.unit,
                categorical: input.categorical,
            },
            location: Location {
                latitude: input.latitude,
                longitude: input.longitude,
                station_id: input.station_id,
                description: input.location_desc,
                altitude_m: input.altitude_m,
            },
            measured_at,
            instrument: InstrumentRef {
                id: input.instrument_id,
                name: input.instrument_name,
                model: input.instrument_model,
                calibration_ref: input.calibration_ref,
            },
            uncertainty,
            chain_of_custody: None,
        };
        let author = Author {
            id: input.author_id,
            name: input.author_name,
            author_type,
            metadata: std::collections::HashMap::new(),
        };
        let service = IngestionService::new(state.storage.clone());
        let node = service
            .ingest_observation(
                measurement,
                author,
                input.labels.unwrap_or_default(),
                input.domain,
                None,
                None,
            )
            .await?;

        Ok(node.into())
    }
}

pub struct SubscriptionRoot;

// Subscription disabled - would implement with async-graphql subscriptions
// #[async_graphql::Subscription]
// impl SubscriptionRoot {
//     async fn node_created(&self, _ctx: &Context<'_>) -> async_graphql::Result<async_graphql::futures_util::Stream<Result<GQLProvenanceNode, async_graphql::Error>>> {
//         Err(async_graphql::Error::new("Not implemented"))
//     }
// }

fn convert_filter(f: GQLQueryFilter) -> async_graphql::Result<witness_core::types::QueryFilter> {
    let date_range = match (f.date_from, f.date_to) {
        (None, None) => None,
        (Some(from), Some(to)) => {
            let from = from
                .parse()
                .map_err(|_| async_graphql::Error::new("dateFrom must be RFC 3339"))?;
            let to = to
                .parse()
                .map_err(|_| async_graphql::Error::new("dateTo must be RFC 3339"))?;
            if from > to {
                return Err("dateFrom must not be after dateTo".into());
            }
            Some(witness_core::types::DateRange { from, to })
        }
        _ => return Err("dateFrom and dateTo must be supplied together".into()),
    };
    Ok(witness_core::types::QueryFilter {
        epistemic_types: f.epistemic_types,
        domains: f.domains,
        authors: f.authors,
        date_range,
        labels: f.labels,
        has_falsifiers: f.has_falsifiers,
        parent_of: f.parent_of.map(|id| Uuid::parse_str(&id)).transpose()?,
        child_of: f.child_of.map(|id| Uuid::parse_str(&id)).transpose()?,
    })
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

pub async fn run_server(database_url: &str, host: &str, port: u16) -> CoreResult<()> {
    let address = if host == "localhost" {
        std::net::IpAddr::V4(std::net::Ipv4Addr::LOCALHOST)
    } else {
        host.parse::<std::net::IpAddr>().map_err(|_| {
            witness_core::WitnessError::Validation(
                "HOST must be localhost or a loopback IP address".into(),
            )
        })?
    };
    if !address.is_loopback() {
        return Err(witness_core::WitnessError::Validation(
            "Witness has unauthenticated APIs and only permits a loopback bind".into(),
        ));
    }
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new("info,witness=debug"))
        .with(tracing_subscriber::fmt::layer().json())
        .try_init()
        .ok();

    let listener = TcpListener::bind((address, port)).await?;
    let local_port = listener.local_addr()?.port();

    let storage = Arc::new(Storage::new(database_url).await?);
    let state = Arc::new(AppState { storage });

    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(state.as_ref().clone())
        .finish();

    let app = Router::new()
        .route(
            "/graphql",
            get(graphql_playground).post(axum::routing::on_service(
                axum::routing::MethodFilter::POST,
                GraphQL::new(schema.clone()),
            )),
        )
        .route("/health", get(health_check))
        // REST API endpoints
        .route("/api/nodes", get(get_nodes))
        .route("/api/nodes/{id}", get(get_node))
        .route("/api/domains", get(get_domains))
        .route("/api/nodes/{id}/children", get(get_children))
        .route("/api/nodes/{id}/falsifiers", get(get_falsifiers))
        .route("/api/nodes/{id}/diffs", get(get_diffs))
        // REST ingest endpoints (write) — one per epistemic type so a
        // stdlib-only client (e.g. the Humanity Grid bridge) can deposit records
        .route(
            "/api/ingest/observation",
            axum::routing::post(ingest_observation_rest),
        )
        .route(
            "/api/ingest/inference",
            axum::routing::post(ingest_inference_rest),
        )
        .route(
            "/api/ingest/generation",
            axum::routing::post(ingest_generation_rest),
        )
        // Dashboard
        .route("/", get(serve_dashboard))
        .route("/experiments/EXP-001", get(serve_exp001))
        .route(
            "/assets/witness.css",
            get(|| async {
                (
                    [(axum::http::header::CONTENT_TYPE, "text/css; charset=utf-8")],
                    include_str!("../../dashboard/assets/witness.css"),
                )
            }),
        )
        .route(
            "/assets/alpine.js",
            get(|| async {
                (
                    [(
                        axum::http::header::CONTENT_TYPE,
                        "application/javascript; charset=utf-8",
                    )],
                    include_str!("../../dashboard/assets/alpine.js"),
                )
            }),
        )
        .with_state(state)
        .layer(axum::middleware::from_fn_with_state(
            local_port,
            local_request_guard,
        ));
    tracing::info!("Witness API listening on http://{}:{}", host, port);
    axum::serve(listener, app).await?;

    Ok(())
}

fn local_authority(authority: &str, port: u16) -> bool {
    let Ok(authority) = authority.parse::<axum::http::uri::Authority>() else {
        return false;
    };
    let hostname = authority.host().trim_matches(['[', ']']);
    let loopback = hostname.eq_ignore_ascii_case("localhost")
        || hostname
            .parse::<std::net::IpAddr>()
            .is_ok_and(|ip| ip.is_loopback());
    loopback && authority.port_u16().unwrap_or(80) == port
}

async fn local_request_guard(
    State(port): State<u16>,
    request: axum::extract::Request,
    next: axum::middleware::Next,
) -> axum::response::Response {
    use axum::response::IntoResponse;
    let authority = request
        .headers()
        .get(axum::http::header::HOST)
        .and_then(|value| value.to_str().ok());
    if !authority.is_some_and(|host| local_authority(host, port)) {
        return axum::http::StatusCode::FORBIDDEN.into_response();
    }
    if let Some(origin) = request.headers().get(axum::http::header::ORIGIN) {
        let allowed = origin
            .to_str()
            .ok()
            .and_then(|value| value.parse::<axum::http::Uri>().ok())
            .is_some_and(|origin| {
                origin.scheme_str() == Some("http")
                    && origin
                        .authority()
                        .is_some_and(|authority| local_authority(authority.as_str(), port))
                    && origin
                        .path_and_query()
                        .is_none_or(|path| path.as_str() == "/")
            });
        if !allowed {
            return axum::http::StatusCode::FORBIDDEN.into_response();
        }
    }
    next.run(request).await
}

async fn graphql_playground() -> axum::response::Html<String> {
    axum::response::Html(
        async_graphql::http::GraphiQLSource::build()
            .endpoint("/graphql")
            .finish(),
    )
}

async fn health_check() -> &'static str {
    "OK"
}

async fn serve_dashboard() -> axum::response::Html<String> {
    axum::response::Html(include_str!("../../dashboard/templates/index.html").to_owned())
}

async fn serve_exp001() -> axum::response::Html<String> {
    axum::response::Html(include_str!("../../dashboard/templates/exp001.html").to_owned())
}

// REST API types
#[derive(Debug, Serialize, Deserialize)]
pub struct NodeResponse {
    pub id: String,
    pub cid: String,
    pub epistemic_type: String,
    pub payload: serde_json::Value,
    pub author: serde_json::Value,
    pub timestamp: String,
    pub parents: Vec<String>,
    pub labels: Vec<String>,
    pub signature: Option<serde_json::Value>,
    pub domain: Option<String>,
    pub source_uri: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NodesResponse {
    pub nodes: Vec<NodeResponse>,
    pub total_count: i64,
    pub page: i64,
    pub page_size: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DomainsResponse {
    pub domains: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChildrenResponse {
    pub children: Vec<NodeResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FalsifierResponse {
    pub description: String,
    pub measurement_type: String,
    pub location: Option<String>,
    pub timeframe: Option<String>,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FalsifiersResponse {
    pub falsifiers: Vec<FalsifierResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiffResponse {
    pub inference_id: String,
    pub old_version: serde_json::Value,
    pub new_version: serde_json::Value,
    pub changed_fields: Vec<String>,
    pub timestamp: String,
    pub editor: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiffsResponse {
    pub diffs: Vec<DiffResponse>,
}

impl From<ProvenanceNode> for NodeResponse {
    fn from(n: ProvenanceNode) -> Self {
        NodeResponse {
            id: n.id.to_string(),
            cid: n.cid.0,
            epistemic_type: format!("{:?}", n.epistemic_type),
            payload: n.payload,
            author: serde_json::to_value(n.author).unwrap_or_default(),
            timestamp: n.timestamp.to_rfc3339(),
            parents: n.parents.iter().map(|p| p.to_string()).collect(),
            labels: n.labels,
            signature: n
                .signature
                .map(|s| serde_json::to_value(s).unwrap_or_default()),
            domain: n.domain,
            source_uri: n.source_uri,
        }
    }
}

async fn get_node(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<NodeResponse>, axum::http::StatusCode> {
    let id = Uuid::parse_str(&id).map_err(|_| axum::http::StatusCode::BAD_REQUEST)?;
    let node = state
        .storage
        .get_node(id)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(axum::http::StatusCode::NOT_FOUND)?;
    Ok(Json(node.into()))
}

// REST API handlers
async fn get_nodes(
    State(state): State<Arc<AppState>>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<axum::Json<NodesResponse>, axum::http::StatusCode> {
    let node_type = params.get("type").map(|s| s.as_str()).unwrap_or("observed");
    let limit = params
        .get("limit")
        .and_then(|s| s.parse::<i64>().ok())
        .unwrap_or(20)
        .clamp(1, 100);
    let offset = params
        .get("offset")
        .and_then(|s| s.parse::<i64>().ok())
        .unwrap_or(0)
        .max(0);
    let page = (offset / limit).saturating_add(1);

    let epistemic_type = match node_type {
        "observed" => EpistemicType::Observed,
        "inferred" => EpistemicType::Inferred,
        "generated" => EpistemicType::Generated,
        _ => return Err(axum::http::StatusCode::BAD_REQUEST),
    };

    let mut filter = witness_core::types::QueryFilter {
        epistemic_types: Some(vec![epistemic_type]),
        ..Default::default()
    };

    if let Some(domain) = params.get("domain") {
        filter.domains = Some(vec![domain.clone()]);
    }
    if let Some(author) = params.get("author") {
        filter.authors = Some(vec![author.clone()]);
    }
    if let Some(label) = params.get("label") {
        filter.labels = Some(vec![label.clone()]);
    }

    let nodes = state
        .storage
        .query_nodes(&filter, limit, offset)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    let total = state
        .storage
        .count_nodes(&filter)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let node_responses: Vec<NodeResponse> = nodes
        .into_iter()
        .map(|n| NodeResponse {
            id: n.id.to_string(),
            cid: n.cid.0,
            epistemic_type: format!("{:?}", n.epistemic_type),
            payload: n.payload,
            author: serde_json::to_value(n.author).unwrap_or_default(),
            timestamp: n.timestamp.to_rfc3339(),
            parents: n.parents.iter().map(|p| p.to_string()).collect(),
            labels: n.labels,
            signature: n
                .signature
                .map(|s| serde_json::to_value(s).unwrap_or_default()),
            domain: n.domain,
            source_uri: n.source_uri,
        })
        .collect();

    Ok(Json(NodesResponse {
        nodes: node_responses,
        total_count: total,
        page,
        page_size: limit,
    }))
}

async fn get_domains(
    State(state): State<Arc<AppState>>,
) -> Result<Json<DomainsResponse>, axum::http::StatusCode> {
    // Get all unique domains from the database
    let rows = sqlx::query!("SELECT DISTINCT domain FROM nodes WHERE domain IS NOT NULL")
        .fetch_all(&*state.storage.pool)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let domains: Vec<String> = rows.into_iter().filter_map(|r| r.domain).collect();

    Ok(Json(DomainsResponse { domains }))
}

async fn get_children(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<ChildrenResponse>, axum::http::StatusCode> {
    let uuid = uuid::Uuid::parse_str(&id).map_err(|_| axum::http::StatusCode::BAD_REQUEST)?;
    let children = state
        .storage
        .get_children(uuid)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let child_responses: Vec<NodeResponse> = children
        .into_iter()
        .map(|n| NodeResponse {
            id: n.id.to_string(),
            cid: n.cid.0,
            epistemic_type: format!("{:?}", n.epistemic_type),
            payload: n.payload,
            author: serde_json::to_value(n.author).unwrap_or_default(),
            timestamp: n.timestamp.to_rfc3339(),
            parents: n.parents.iter().map(|p| p.to_string()).collect(),
            labels: n.labels,
            signature: n
                .signature
                .map(|s| serde_json::to_value(s).unwrap_or_default()),
            domain: n.domain,
            source_uri: n.source_uri,
        })
        .collect();

    Ok(Json(ChildrenResponse {
        children: child_responses,
    }))
}

async fn get_falsifiers(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<FalsifiersResponse>, axum::http::StatusCode> {
    let uuid = uuid::Uuid::parse_str(&id).map_err(|_| axum::http::StatusCode::BAD_REQUEST)?;

    let node = state
        .storage
        .get_node(uuid)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let falsifiers = if let Some(node) = node {
        if let Some(inference) = node.payload.get("derivation") {
            if let Some(falsifiers) = inference.get("falsifiers").and_then(|f| f.as_array()) {
                falsifiers
                    .iter()
                    .filter_map(|f| {
                        Some(FalsifierResponse {
                            description: f.get("description")?.as_str()?.to_string(),
                            measurement_type: f.get("measurement_type")?.as_str()?.to_string(),
                            location: f
                                .get("location")
                                .and_then(|l| l.as_str())
                                .map(|s| s.to_string()),
                            timeframe: f
                                .get("timeframe")
                                .and_then(|t| t.as_str())
                                .map(|s| s.to_string()),
                            status: f.get("status")?.as_str()?.to_string(),
                        })
                    })
                    .collect()
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    };

    Ok(Json(FalsifiersResponse { falsifiers }))
}

async fn get_diffs(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<DiffsResponse>, axum::http::StatusCode> {
    let uuid = uuid::Uuid::parse_str(&id).map_err(|_| axum::http::StatusCode::BAD_REQUEST)?;
    let diffs = state
        .storage
        .get_diffs(uuid)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let diff_responses: Vec<DiffResponse> = diffs
        .into_iter()
        .map(|d| DiffResponse {
            inference_id: d.inference_id.to_string(),
            old_version: serde_json::to_value(d.old_version).unwrap_or_default(),
            new_version: serde_json::to_value(d.new_version).unwrap_or_default(),
            changed_fields: d.changed_fields,
            timestamp: d.timestamp.to_rfc3339(),
            editor: serde_json::to_value(d.editor)
                .unwrap_or_default()
                .to_string(),
        })
        .collect();

    Ok(Json(DiffsResponse {
        diffs: diff_responses,
    }))
}

// REST ingest request / response types

#[derive(Debug, Serialize, Deserialize)]
pub struct RestIngestResponse {
    pub id: String,
    pub cid: String,
    pub epistemic_type: String,
    pub parents: Vec<String>,
}

impl From<ProvenanceNode> for RestIngestResponse {
    fn from(n: ProvenanceNode) -> Self {
        Self {
            id: n.id.to_string(),
            cid: n.cid.0,
            epistemic_type: format!("{:?}", n.epistemic_type).to_lowercase(),
            parents: n.parents.iter().map(|p| p.to_string()).collect(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestObservationRequest {
    pub quantity: String,
    pub value: String, // numeric or text
    pub unit: Option<String>,
    pub measured_at: Option<String>,
    pub instrument_id: String,
    pub instrument_name: Option<String>,
    pub author_id: String,
    pub author_name: Option<String>,
    pub author_type: Option<String>, // defaults to instrument
    pub domain: Option<String>,
    pub source_uri: Option<String>,
    pub labels: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestInferenceRequest {
    pub claim: String,
    pub methodology: String,
    pub premises: Vec<String>,
    pub claim_type: Option<String>,
    pub claim_scope: Option<String>,
    pub falsifiers: Option<Vec<RestFalsifier>>,
    pub inference_uncertainty: Option<f64>,
    pub author_id: String,
    pub author_name: Option<String>,
    pub author_type: Option<String>, // defaults to human
    pub domain: Option<String>,
    pub source_uri: Option<String>,
    pub labels: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestFalsifier {
    pub description: String,
    pub measurement_type: String,
    pub timeframe: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestGenerationRequest {
    pub content: String,
    pub generator: String, // model id
    pub model_version: Option<String>,
    pub prompt: Option<String>,
    pub human_reviewed: Option<bool>,
    pub author_id: String,
    pub author_name: Option<String>,
    pub domain: Option<String>,
    pub source_uri: Option<String>,
    pub labels: Option<Vec<String>>,
}

fn parse_rest_uuid_list(ids: &[String]) -> Result<Vec<Uuid>, axum::http::StatusCode> {
    ids.iter()
        .map(|s| Uuid::parse_str(s).map_err(|_| axum::http::StatusCode::BAD_REQUEST))
        .collect()
}

fn ingest_status(error: witness_core::WitnessError) -> axum::http::StatusCode {
    match error {
        witness_core::WitnessError::Validation(_) => axum::http::StatusCode::BAD_REQUEST,
        _ => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
    }
}

async fn ingest_observation_rest(
    State(state): State<Arc<AppState>>,
    Json(req): Json<RestObservationRequest>,
) -> Result<Json<RestIngestResponse>, axum::http::StatusCode> {
    // value: numeric if parseable, else text
    let value_numeric = req.value.parse::<f64>().ok();
    let measured_at = req
        .measured_at
        .as_deref()
        .map(str::parse::<chrono::DateTime<Utc>>)
        .transpose()
        .map_err(|_| axum::http::StatusCode::BAD_REQUEST)?
        .unwrap_or_else(Utc::now);
    let value_text = if value_numeric.is_some() {
        None
    } else {
        Some(req.value.clone())
    };
    if value_numeric.is_none() && value_text.is_none() {
        return Err(axum::http::StatusCode::BAD_REQUEST);
    }

    let author_type = match req
        .author_type
        .as_deref()
        .unwrap_or("instrument")
        .to_ascii_lowercase()
        .as_str()
    {
        "human" => AuthorType::Human,
        "instrument" => AuthorType::Instrument,
        "model" => AuthorType::Model,
        "institution" => AuthorType::Institution,
        "software" => AuthorType::Software,
        _ => return Err(axum::http::StatusCode::BAD_REQUEST),
    };

    let measurement = Measurement {
        quantity: req.quantity,
        value: MeasuredValue {
            numeric: value_numeric,
            text: value_text,
            unit: req.unit,
            categorical: None,
        },
        location: Location {
            latitude: None,
            longitude: None,
            station_id: None,
            description: None,
            altitude_m: None,
        },
        measured_at,
        instrument: InstrumentRef {
            id: req.instrument_id,
            name: req.instrument_name,
            model: None,
            calibration_ref: None,
        },
        uncertainty: None,
        chain_of_custody: None,
    };
    let author = Author {
        id: req.author_id,
        name: req.author_name,
        author_type,
        metadata: std::collections::HashMap::new(),
    };
    let service = IngestionService::new(state.storage.clone());
    let node = service
        .ingest_observation(
            measurement,
            author,
            req.labels.unwrap_or_default(),
            req.domain,
            req.source_uri,
            None,
        )
        .await
        .map_err(ingest_status)?;
    Ok(Json(node.into()))
}

async fn ingest_inference_rest(
    State(state): State<Arc<AppState>>,
    Json(req): Json<RestInferenceRequest>,
) -> Result<Json<RestIngestResponse>, axum::http::StatusCode> {
    let premises = parse_rest_uuid_list(&req.premises)?;
    let author_type = match req
        .author_type
        .as_deref()
        .unwrap_or("human")
        .to_ascii_lowercase()
        .as_str()
    {
        "human" => AuthorType::Human,
        "instrument" => AuthorType::Instrument,
        "model" => AuthorType::Model,
        "institution" => AuthorType::Institution,
        "software" => AuthorType::Software,
        _ => return Err(axum::http::StatusCode::BAD_REQUEST),
    };
    let claim_type = match req
        .claim_type
        .as_deref()
        .unwrap_or("descriptive")
        .to_ascii_lowercase()
        .as_str()
    {
        "causal" => ClaimType::Causal,
        "correlative" => ClaimType::Correlative,
        "predictive" => ClaimType::Predictive,
        "descriptive" => ClaimType::Descriptive,
        "counterfactual" => ClaimType::Counterfactual,
        _ => return Err(axum::http::StatusCode::BAD_REQUEST),
    };
    let claim_scope = match req
        .claim_scope
        .as_deref()
        .unwrap_or("specific")
        .to_ascii_lowercase()
        .as_str()
    {
        "specific" => ClaimScope::Specific,
        "general" => ClaimScope::General,
        "universal" => ClaimScope::Universal,
        _ => return Err(axum::http::StatusCode::BAD_REQUEST),
    };

    let falsifiers = req
        .falsifiers
        .unwrap_or_default()
        .into_iter()
        .map(|f| {
            Ok(Falsifier {
                description: f.description,
                measurement_type: f.measurement_type,
                location: None,
                timeframe: f.timeframe,
                status: match f
                    .status
                    .as_deref()
                    .unwrap_or("pending")
                    .to_ascii_lowercase()
                    .as_str()
                {
                    "pending" => FalsifierStatus::Pending,
                    "in-progress" => FalsifierStatus::InProgress,
                    "completed-falsified" => FalsifierStatus::CompletedFalsified,
                    "completed-confirmed" => FalsifierStatus::CompletedConfirmed,
                    _ => return Err(axum::http::StatusCode::BAD_REQUEST),
                },
            })
        })
        .collect::<Result<Vec<_>, axum::http::StatusCode>>()?;

    let derivation = Derivation {
        premises,
        methodology: req.methodology,
        model_ref: None,
        parameters: std::collections::HashMap::new(),
        claim: Claim {
            statement: req.claim,
            claim_type,
            scope: claim_scope,
        },
        falsifiers,
        inference_uncertainty: req.inference_uncertainty,
    };
    let author = Author {
        id: req.author_id,
        name: req.author_name,
        author_type,
        metadata: std::collections::HashMap::new(),
    };
    let service = IngestionService::new(state.storage.clone());
    let node = service
        .ingest_inference(
            derivation,
            author,
            req.labels.unwrap_or_default(),
            req.domain,
            req.source_uri,
            None,
        )
        .await
        .map_err(ingest_status)?;
    Ok(Json(node.into()))
}

async fn ingest_generation_rest(
    State(state): State<Arc<AppState>>,
    Json(req): Json<RestGenerationRequest>,
) -> Result<Json<RestIngestResponse>, axum::http::StatusCode> {
    let author = Author {
        id: req.author_id,
        name: req.author_name,
        author_type: AuthorType::Model,
        metadata: std::collections::HashMap::new(),
    };
    let generation = Generation {
        node: ProvenanceNode {
            id: Uuid::new_v4(),
            cid: CID::new(req.content.as_bytes()),
            epistemic_type: EpistemicType::Generated,
            payload: serde_json::json!({ "content": req.content }),
            author: author.clone(),
            timestamp: Utc::now(),
            parents: vec![],
            labels: req.labels.clone().unwrap_or_default(),
            domain: req.domain.clone(),
            source_uri: req.source_uri.clone(),
            signature: None,
        },
        model: ModelRef {
            id: req.generator,
            version: req.model_version.unwrap_or_else(|| "unknown".to_string()),
            hash: None,
            training_data_ref: None,
        },
        prompt: req.prompt,
        parameters: std::collections::HashMap::new(),
        human_reviewed: req.human_reviewed.unwrap_or(false),
    };
    let service = IngestionService::new(state.storage.clone());
    let node = service
        .ingest_generation(
            generation,
            author,
            req.labels.unwrap_or_default(),
            req.domain,
            req.source_uri,
            None,
        )
        .await
        .map_err(ingest_status)?;
    Ok(Json(node.into()))
}

// Main entry point
#[tokio::main]
async fn main() -> witness_core::Result<()> {
    let database_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://./witness.db".to_string());
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .unwrap_or(8080);
    let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());

    run_server(&database_url, &host, port).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    async fn state() -> Arc<AppState> {
        Arc::new(AppState {
            storage: Arc::new(Storage::new("sqlite::memory:").await.unwrap()),
        })
    }
    fn observation() -> serde_json::Value {
        serde_json::json!({"quantity":"temperature","value":"20","unit":"celsius","measured_at":"2026-10-01T09:00:00Z","instrument_id":"fixture","author_id":"test"})
    }
    async fn observe(state: &Arc<AppState>) -> RestIngestResponse {
        ingest_observation_rest(
            State(state.clone()),
            Json(serde_json::from_value(observation()).unwrap()),
        )
        .await
        .unwrap()
        .0
    }
    fn schema(state: &Arc<AppState>) -> WitnessSchema {
        Schema::build(QueryRoot, MutationRoot, EmptySubscription)
            .data(state.as_ref().clone())
            .finish()
    }

    #[tokio::test]
    async fn direct_record_lookup_preserves_type_and_reports_missing_or_invalid_ids() {
        let state = state().await;
        let record = observe(&state).await;
        let Json(found) = get_node(State(state.clone()), Path(record.id.clone()))
            .await
            .unwrap();
        assert_eq!(found.id, record.id);
        assert_eq!(found.epistemic_type, "Observed");
        assert_eq!(
            get_node(State(state.clone()), Path("bad".to_owned()))
                .await
                .unwrap_err(),
            axum::http::StatusCode::BAD_REQUEST
        );
        assert_eq!(
            get_node(State(state), Path(Uuid::new_v4().to_string()))
                .await
                .unwrap_err(),
            axum::http::StatusCode::NOT_FOUND
        );
    }

    #[tokio::test]
    async fn local_server_rejects_remote_hosts_and_cross_origin_requests() {
        use tower::ServiceExt;
        let port = 18080;
        let app = Router::new().route("/health", get(health_check)).layer(
            axum::middleware::from_fn_with_state(port, local_request_guard),
        );
        for (host, origin, expected) in [
            ("127.0.0.1:18080", None, 200),
            ("localhost:18080", Some("http://localhost:18080"), 200),
            ("[::1]:18080", Some("http://[::1]:18080"), 200),
            ("example.com:18080", None, 403),
            ("localhost.example.com:18080", None, 403),
            ("127.0.0.1:18081", None, 403),
            ("127.0.0.1:18080", Some("http://example.com"), 403),
            ("127.0.0.1:18080", Some("null"), 403),
            ("127.0.0.1:18080", Some("http://localhost:18080/extra"), 403),
        ] {
            let mut request = axum::http::Request::builder()
                .uri("/health")
                .header("Host", host);
            if let Some(origin) = origin {
                request = request.header("Origin", origin);
            }
            let response = app
                .clone()
                .oneshot(request.body(axum::body::Body::empty()).unwrap())
                .await
                .unwrap();
            assert_eq!(
                response.status().as_u16(),
                expected,
                "host={host} origin={origin:?}"
            );
        }
    }

    #[tokio::test]
    async fn remote_bind_is_rejected_before_creating_a_database() {
        let path = std::env::temp_dir().join(format!("witness-remote-{}.db", Uuid::new_v4()));
        let result = run_server(&format!("sqlite://{}", path.display()), "0.0.0.0", 0).await;
        assert!(matches!(
            result,
            Err(witness_core::WitnessError::Validation(_))
        ));
        assert!(!path.exists());
    }

    #[tokio::test]
    async fn rest_rejects_bad_measurements_without_storing_them() {
        let state = state().await;
        for (field, value) in [
            ("measured_at", "not-a-date"),
            ("value", "NaN"),
            ("value", "inf"),
            ("value", " "),
            ("quantity", ""),
            ("instrument_id", ""),
        ] {
            let mut body = observation();
            body[field] = value.into();
            let result = ingest_observation_rest(
                State(state.clone()),
                Json(serde_json::from_value(body).unwrap()),
            )
            .await;
            assert!(
                matches!(result, Err(axum::http::StatusCode::BAD_REQUEST)),
                "{field}={value} was not rejected"
            );
        }
        assert_eq!(
            state
                .storage
                .count_nodes(&QueryFilter::default())
                .await
                .unwrap(),
            0
        );
        let record = observe(&state).await;
        let stored = state
            .storage
            .get_node(Uuid::parse_str(&record.id).unwrap())
            .await
            .unwrap()
            .unwrap();
        assert_eq!(
            stored.payload["measurement"]["measured_at"],
            "2026-10-01T09:00:00Z"
        );
    }

    #[tokio::test]
    async fn inference_rejects_unknown_falsifier_status() {
        let state = state().await;
        let body = serde_json::json!({"claim":"fixture","methodology":"fixture","premises":[],"author_id":"test","falsifiers":[{"description":"review","measurement_type":"review","status":"typo-status"}]});
        let result = ingest_inference_rest(
            State(state.clone()),
            Json(serde_json::from_value(body).unwrap()),
        )
        .await;
        assert!(matches!(result, Err(axum::http::StatusCode::BAD_REQUEST)));
        assert_eq!(
            state
                .storage
                .count_nodes(&QueryFilter::default())
                .await
                .unwrap(),
            0
        );
    }

    #[tokio::test]
    async fn graphql_cursor_advances_and_bad_cursors_or_filters_fail() {
        let state = state().await;
        for _ in 0..3 {
            observe(&state).await;
        }
        let schema = schema(&state);
        let response = schema.execute("{ observations(first: 2) { totalCount nodes { id } pageInfo { endCursor hasNextPage } } }").await;
        assert!(response.errors.is_empty(), "{:?}", response.errors);
        let first = response.data.into_json().unwrap()["observations"].clone();
        let cursor = first["pageInfo"]["endCursor"].as_str().unwrap();
        assert_eq!(first["nodes"].as_array().unwrap().len(), 2);
        assert_eq!(first["pageInfo"]["hasNextPage"], true);
        let response = schema.execute(format!("{{ observations(first: 2, after: \"{cursor}\") {{ totalCount nodes {{ id }} pageInfo {{ hasNextPage hasPreviousPage }} }} }}")).await;
        assert!(response.errors.is_empty(), "{:?}", response.errors);
        let next = response.data.into_json().unwrap()["observations"].clone();
        assert_eq!(next["nodes"].as_array().unwrap().len(), 1);
        assert_eq!(next["totalCount"], 3);
        assert_eq!(next["pageInfo"]["hasNextPage"], false);
        assert_eq!(next["pageInfo"]["hasPreviousPage"], true);
        assert!(
            !first["nodes"]
                .as_array()
                .unwrap()
                .contains(&next["nodes"][0])
        );
        for query in [
            r#"{nodes(after:"garbage"){totalCount}}"#,
            r#"{nodes(after:"-1"){totalCount}}"#,
            r#"{nodes(filter:{parentOf:"not-a-uuid"}){totalCount}}"#,
            r#"{nodes(filter:{dateFrom:"bad",dateTo:"2026-10-01T00:00:00Z"}){totalCount}}"#,
            r#"{nodes(filter:{dateFrom:"2026-10-01T00:00:00Z"}){totalCount}}"#,
        ] {
            assert!(
                !schema.execute(query).await.errors.is_empty(),
                "invalid query succeeded: {query}"
            );
        }
    }

    #[tokio::test]
    async fn relationship_and_falsifier_filters_match_counts() {
        let state = state().await;
        let first = observe(&state).await;
        observe(&state).await;
        let body = serde_json::json!({"claim":"fixture","methodology":"fixture","premises":[first.id],"author_id":"test","falsifiers":[{"description":"review","measurement_type":"review"}]});
        let inference = ingest_inference_rest(
            State(state.clone()),
            Json(serde_json::from_value(body).unwrap()),
        )
        .await
        .unwrap()
        .0;
        let schema = schema(&state);
        for (filter, id, expected) in [
            (
                format!("parentOf:\"{}\"", inference.id),
                first.id.as_str(),
                1,
            ),
            (
                format!("childOf:\"{}\"", first.id),
                inference.id.as_str(),
                1,
            ),
            ("hasFalsifiers:true".into(), inference.id.as_str(), 1),
            ("hasFalsifiers:false".into(), "", 2),
            (
                format!("childOf:\"{}\",hasFalsifiers:true", first.id),
                inference.id.as_str(),
                1,
            ),
        ] {
            let response = schema
                .execute(format!(
                    "{{nodes(filter:{{{filter}}}){{totalCount nodes{{id}}}}}}"
                ))
                .await;
            assert!(response.errors.is_empty(), "{:?}", response.errors);
            let data = response.data.into_json().unwrap()["nodes"].clone();
            assert_eq!(data["totalCount"], expected);
            assert_eq!(data["nodes"].as_array().unwrap().len(), expected as usize);
            if !id.is_empty() {
                assert_eq!(data["nodes"][0]["id"], id);
            }
        }
    }

    #[tokio::test]
    async fn extreme_rest_offset_does_not_overflow() {
        let state = state().await;
        let params = HashMap::from([
            ("limit".into(), "1".into()),
            ("offset".into(), i64::MAX.to_string()),
        ]);
        let response = get_nodes(State(state), Query(params)).await.unwrap().0;
        assert!(response.nodes.is_empty());
        assert_eq!(response.page, i64::MAX);
    }
}
