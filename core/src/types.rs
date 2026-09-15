use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// The three epistemic types - the fundamental classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, async_graphql::Enum)]
#[serde(rename_all = "lowercase")]
pub enum EpistemicType {
    /// Something was measured, recorded, dated, located, and can still be pointed at
    Observed,
    /// A model, person, or committee moved from observations to a claim, and the move is written down
    Inferred,
    /// Fluency with no parent in the world. Useful, sometimes beautiful, and not evidence.
    Generated,
}

/// Cryptographic signature for tamper-evidence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signature {
    pub algorithm: String,  // "ed25519"
    pub public_key: String, // hex-encoded
    pub signature: String,  // hex-encoded
}

/// Content-addressed identifier (SHA-256 of canonical JSON)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CID(pub String);

impl CID {
    pub fn new(data: &[u8]) -> Self {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(data);
        let hash = hasher.finalize();
        Self(hex::encode(hash))
    }
}

/// A node in the provenance graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvenanceNode {
    /// Unique identifier for this node
    pub id: Uuid,
    /// Content-addressed hash of the payload
    pub cid: CID,
    /// The epistemic classification
    pub epistemic_type: EpistemicType,
    /// The actual content (observation data, inference text, generation output)
    pub payload: serde_json::Value,
    /// Author/signer of this node
    pub author: Author,
    /// When this node was created
    pub timestamp: DateTime<Utc>,
    /// Parent node IDs (empty for Observations, non-empty for Inferred)
    pub parents: Vec<Uuid>,
    /// Cryptographic signature over the canonicalized node (excluding signature field)
    pub signature: Option<Signature>,
    /// Human-readable labels/tags
    pub labels: Vec<String>,
    /// Domain/context (e.g., "climate", "clinical-trials", "astronomy")
    pub domain: Option<String>,
    /// Source URI if ingested from external system
    pub source_uri: Option<String>,
}

/// Author identity (person, instrument, model, institution)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Author {
    pub id: String, // DID, ORCID, instrument ID, model ID, etc.
    pub name: Option<String>,
    pub author_type: AuthorType,
    pub metadata: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AuthorType {
    Human,
    Instrument,
    Model,
    Institution,
    Software,
}

/// An observation - the leaf nodes of the graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Observation {
    pub node: ProvenanceNode,
    /// Measurement metadata
    pub measurement: Measurement,
}

/// What was actually measured
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Measurement {
    /// What quantity was measured
    pub quantity: String,
    /// Value with units
    pub value: MeasuredValue,
    /// Where it was measured (lat/lon, station ID, etc.)
    pub location: Location,
    /// When it was measured
    pub measured_at: DateTime<Utc>,
    /// Instrument/method used
    pub instrument: InstrumentRef,
    /// Uncertainty/error bounds
    pub uncertainty: Option<Uncertainty>,
    /// Chain of custody / collection metadata
    pub chain_of_custody: Option<ChainOfCustody>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeasuredValue {
    pub numeric: Option<f64>,
    pub text: Option<String>,
    pub unit: Option<String>,
    pub categorical: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub station_id: Option<String>,
    pub description: Option<String>,
    pub altitude_m: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstrumentRef {
    pub id: String,
    pub name: Option<String>,
    pub model: Option<String>,
    pub calibration_ref: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Uncertainty {
    pub value: f64,
    pub unit: String,
    pub confidence_level: f64, // e.g., 0.95 for 95%
    pub method: String,        // "gaussian", "bootstrap", "instrument-spec"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainOfCustody {
    pub collectors: Vec<String>,
    pub handlers: Vec<String>,
    pub seals: Vec<String>,
    pub timestamps: Vec<DateTime<Utc>>,
}

/// An inference - internal nodes connecting observations to claims
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inference {
    pub node: ProvenanceNode,
    /// Explicit premises -> conclusion structure
    pub derivation: Derivation,
}

/// The logical move from observations to claim
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Derivation {
    /// Premise observation IDs
    pub premises: Vec<Uuid>,
    /// Methodology description (or reference to methodology doc)
    pub methodology: String,
    /// Model/code used (with version/hash)
    pub model_ref: Option<ModelRef>,
    /// Parameters used
    pub parameters: HashMap<String, serde_json::Value>,
    /// The claim being made
    pub claim: Claim,
    /// What would falsify this claim (critical tests)
    pub falsifiers: Vec<Falsifier>,
    /// Confidence/uncertainty in the inference itself
    pub inference_uncertainty: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelRef {
    pub id: String,
    pub version: String,
    pub hash: Option<String>, // model weights hash
    pub training_data_ref: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claim {
    pub statement: String,
    pub claim_type: ClaimType,
    pub scope: ClaimScope,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ClaimType {
    Causal,
    Correlative,
    Predictive,
    Descriptive,
    Counterfactual,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ClaimScope {
    Specific,  // This river gauge at this time
    General,   // All rivers of this type
    Universal, // Physical law
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Falsifier {
    /// Description of the observation that would force an update
    pub description: String,
    /// Type of measurement needed
    pub measurement_type: String,
    /// Where/when it would need to occur
    pub location: Option<Location>,
    pub timeframe: Option<String>,
    /// Current status: "pending", "in-progress", "completed-falsified", "completed-confirmed"
    pub status: FalsifierStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FalsifierStatus {
    Pending,
    InProgress,
    CompletedFalsified,
    CompletedConfirmed,
}

/// A generation - LLM outputs, synthetic data, creative works
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Generation {
    pub node: ProvenanceNode,
    /// Model that generated this
    pub model: ModelRef,
    /// Prompt/input that produced this
    pub prompt: Option<String>,
    /// Generation parameters
    pub parameters: HashMap<String, serde_json::Value>,
    /// Whether this was human-reviewed/edited
    pub human_reviewed: bool,
}

/// Graph edges for traversal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvenanceEdge {
    pub from: Uuid,
    pub to: Uuid,
    pub edge_type: EdgeType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum EdgeType {
    Supports,    // observation -> inference
    Derives,     // inference -> inference
    Generates,   // inference -> generation (rare)
    References,  // any -> any (soft link)
    Contradicts, // observation/inference -> inference
    Updates,     // new version of same node
}

/// Query filters for the API
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QueryFilter {
    pub epistemic_types: Option<Vec<EpistemicType>>,
    pub domains: Option<Vec<String>>,
    pub authors: Option<Vec<String>>,
    pub date_range: Option<DateRange>,
    pub labels: Option<Vec<String>>,
    pub has_falsifiers: Option<bool>,
    pub parent_of: Option<Uuid>,
    pub child_of: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateRange {
    pub from: DateTime<Utc>,
    pub to: DateTime<Utc>,
}

/// Response types for GraphQL
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeConnection {
    pub nodes: Vec<ProvenanceNode>,
    pub page_info: PageInfo,
    pub total_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageInfo {
    pub has_next_page: bool,
    pub has_previous_page: bool,
    pub start_cursor: Option<String>,
    pub end_cursor: Option<String>,
}

/// Diff for tracking narrative edits
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarrativeDiff {
    pub inference_id: Uuid,
    pub old_version: ProvenanceNode,
    pub new_version: ProvenanceNode,
    pub changed_fields: Vec<String>,
    pub timestamp: DateTime<Utc>,
    pub editor: Author,
}
