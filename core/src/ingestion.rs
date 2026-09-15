use crate::Result;
use crate::signing::{SigningKeypair, compute_cid, sign_node, verify_cid, verify_node};
use crate::storage::Storage;
use crate::types::*;
use chrono::Utc;
use std::sync::Arc;
use uuid::Uuid;

pub struct IngestionService {
    storage: Arc<Storage>,
    default_keypair: Option<SigningKeypair>,
}

impl IngestionService {
    pub fn new(storage: Arc<Storage>) -> Self {
        Self {
            storage,
            default_keypair: None,
        }
    }

    pub fn with_keypair(mut self, keypair: SigningKeypair) -> Self {
        self.default_keypair = Some(keypair);
        self
    }

    /// Ingest a raw observation (measurement, sensor reading, human witness)
    pub async fn ingest_observation(
        &self,
        measurement: Measurement,
        author: Author,
        labels: Vec<String>,
        domain: Option<String>,
        source_uri: Option<String>,
        keypair: Option<&SigningKeypair>,
    ) -> Result<ProvenanceNode> {
        let node = self.build_observation_node(measurement, author, labels, domain, source_uri)?;
        let keypair = keypair.or(self.default_keypair.as_ref());
        let signed_node = self.sign_and_store(node, keypair).await?;
        Ok(signed_node)
    }

    /// Ingest an inference (derivation from observations to claim)
    pub async fn ingest_inference(
        &self,
        derivation: Derivation,
        author: Author,
        labels: Vec<String>,
        domain: Option<String>,
        source_uri: Option<String>,
        keypair: Option<&SigningKeypair>,
    ) -> Result<ProvenanceNode> {
        for premise in &derivation.premises {
            if self.storage.get_node(*premise).await?.is_none() {
                return Err(crate::WitnessError::Validation(format!(
                    "inference premise does not exist: {premise}"
                )));
            }
        }
        let node = self.build_inference_node(derivation, author, labels, domain, source_uri)?;
        let keypair = keypair.or(self.default_keypair.as_ref());
        let signed_node = self.sign_and_store(node, keypair).await?;
        Ok(signed_node)
    }

    /// Ingest a generation (LLM output, synthetic data)
    pub async fn ingest_generation(
        &self,
        generation: Generation,
        author: Author,
        labels: Vec<String>,
        domain: Option<String>,
        source_uri: Option<String>,
        keypair: Option<&SigningKeypair>,
    ) -> Result<ProvenanceNode> {
        let node = self.build_generation_node(generation, author, labels, domain, source_uri)?;
        let keypair = keypair.or(self.default_keypair.as_ref());
        let signed_node = self.sign_and_store(node, keypair).await?;
        Ok(signed_node)
    }

    /// Batch ingest from JSONL file (one node per line)
    pub async fn ingest_jsonl(
        &self,
        path: &str,
        keypair: Option<&SigningKeypair>,
    ) -> Result<Vec<ProvenanceNode>> {
        use tokio::fs::File;
        use tokio::io::{AsyncBufReadExt, BufReader};

        let file = File::open(path).await?;
        let reader = BufReader::new(file);
        let mut lines = reader.lines();
        let mut nodes = Vec::new();

        while let Some(line) = lines.next_line().await? {
            if line.trim().is_empty() {
                continue;
            }
            let node: ProvenanceNode = serde_json::from_str(&line)?;
            if !verify_cid(&node.payload, &node.cid)? {
                return Err(crate::WitnessError::Validation(format!(
                    "JSONL line has a CID that does not match its payload: {}",
                    node.id
                )));
            }
            if node.signature.is_some() && !verify_node(&node)? {
                return Err(crate::WitnessError::Validation(format!(
                    "JSONL line has an invalid signature: {}",
                    node.id
                )));
            }
            let signed = self.sign_and_store(node, keypair).await?;
            nodes.push(signed);
        }

        Ok(nodes)
    }

    /// Ingest from CSV (observations only for now)
    pub async fn ingest_csv_observations(
        &self,
        path: &str,
        author: Author,
        domain: Option<String>,
        column_mapping: CsvColumnMapping,
        keypair: Option<&SigningKeypair>,
    ) -> Result<Vec<ProvenanceNode>> {
        use csv::ReaderBuilder;
        use std::fs::File;

        let file = File::open(path)?;
        let mut reader = ReaderBuilder::new().from_reader(file);
        let mut nodes = Vec::new();

        for result in reader.deserialize::<serde_json::Value>() {
            let record = result?;
            let measurement = self.csv_record_to_measurement(&record, &column_mapping)?;
            let labels = vec!["csv-import".to_string()];
            let source_uri = Some(format!("file://{}", path));
            let node = self
                .ingest_observation(
                    measurement,
                    author.clone(),
                    labels,
                    domain.clone(),
                    source_uri,
                    keypair,
                )
                .await?;
            nodes.push(node);
        }

        Ok(nodes)
    }

    fn build_observation_node(
        &self,
        measurement: Measurement,
        author: Author,
        labels: Vec<String>,
        domain: Option<String>,
        source_uri: Option<String>,
    ) -> Result<ProvenanceNode> {
        let observation = Observation {
            node: ProvenanceNode::default(),
            measurement,
        };
        let payload = serde_json::to_value(&observation)?;
        let cid = compute_cid(&payload)?;
        let id = Uuid::new_v4();

        Ok(ProvenanceNode {
            id,
            cid,
            epistemic_type: EpistemicType::Observed,
            payload,
            author,
            timestamp: Utc::now(),
            parents: Vec::new(),
            labels,
            signature: None,
            domain,
            source_uri,
        })
    }

    fn build_inference_node(
        &self,
        derivation: Derivation,
        author: Author,
        labels: Vec<String>,
        domain: Option<String>,
        source_uri: Option<String>,
    ) -> Result<ProvenanceNode> {
        let inference = Inference {
            node: ProvenanceNode::default(),
            derivation,
        };
        let payload = serde_json::to_value(&inference)?;
        let cid = compute_cid(&payload)?;
        let id = Uuid::new_v4();
        let parents = inference.derivation.premises.clone();

        Ok(ProvenanceNode {
            id,
            cid,
            epistemic_type: EpistemicType::Inferred,
            payload,
            author,
            timestamp: Utc::now(),
            parents,
            labels,
            signature: None,
            domain,
            source_uri,
        })
    }

    fn build_generation_node(
        &self,
        generation: Generation,
        author: Author,
        labels: Vec<String>,
        domain: Option<String>,
        source_uri: Option<String>,
    ) -> Result<ProvenanceNode> {
        let payload = serde_json::to_value(&generation)?;
        let cid = compute_cid(&payload)?;
        let id = Uuid::new_v4();

        Ok(ProvenanceNode {
            id,
            cid,
            epistemic_type: EpistemicType::Generated,
            payload,
            author,
            timestamp: Utc::now(),
            parents: Vec::new(),
            labels,
            signature: None,
            domain,
            source_uri,
        })
    }

    async fn sign_and_store(
        &self,
        mut node: ProvenanceNode,
        keypair: Option<&SigningKeypair>,
    ) -> Result<ProvenanceNode> {
        if let Some(kp) = keypair {
            let sig = sign_node(&node, kp)?;
            node.signature = Some(sig);
        }
        self.storage.store_node(&node).await?;
        Ok(node)
    }

    fn csv_record_to_measurement(
        &self,
        record: &serde_json::Value,
        mapping: &CsvColumnMapping,
    ) -> Result<Measurement> {
        let get = |key: &str| record.get(key).and_then(|v| v.as_str()).unwrap_or("");

        Ok(Measurement {
            quantity: get(&mapping.quantity).to_string(),
            value: MeasuredValue {
                numeric: get(&mapping.value_numeric).parse().ok(),
                text: if mapping.value_text.is_empty() {
                    None
                } else {
                    Some(get(&mapping.value_text).to_string())
                },
                unit: if mapping.unit.is_empty() {
                    None
                } else {
                    Some(get(&mapping.unit).to_string())
                },
                categorical: if mapping.categorical.is_empty() {
                    None
                } else {
                    Some(get(&mapping.categorical).to_string())
                },
            },
            location: Location {
                latitude: get(&mapping.latitude).parse().ok(),
                longitude: get(&mapping.longitude).parse().ok(),
                station_id: if mapping.station_id.is_empty() {
                    None
                } else {
                    Some(get(&mapping.station_id).to_string())
                },
                description: if mapping.location_desc.is_empty() {
                    None
                } else {
                    Some(get(&mapping.location_desc).to_string())
                },
                altitude_m: get(&mapping.altitude).parse().ok(),
            },
            measured_at: get(&mapping.timestamp)
                .parse()
                .unwrap_or_else(|_| Utc::now()),
            instrument: InstrumentRef {
                id: get(&mapping.instrument_id).to_string(),
                name: if mapping.instrument_name.is_empty() {
                    None
                } else {
                    Some(get(&mapping.instrument_name).to_string())
                },
                model: if mapping.instrument_model.is_empty() {
                    None
                } else {
                    Some(get(&mapping.instrument_model).to_string())
                },
                calibration_ref: if mapping.calibration_ref.is_empty() {
                    None
                } else {
                    Some(get(&mapping.calibration_ref).to_string())
                },
            },
            uncertainty: if mapping.uncertainty_value.is_empty() {
                None
            } else {
                Some(Uncertainty {
                    value: get(&mapping.uncertainty_value).parse().unwrap_or(0.0),
                    unit: get(&mapping.uncertainty_unit).to_string(),
                    confidence_level: get(&mapping.confidence_level).parse().unwrap_or(0.95),
                    method: get(&mapping.uncertainty_method).to_string(),
                })
            },
            chain_of_custody: None,
        })
    }
}

#[derive(Debug, Clone, Default)]
pub struct CsvColumnMapping {
    pub quantity: String,
    pub value_numeric: String,
    pub value_text: String,
    pub unit: String,
    pub categorical: String,
    pub latitude: String,
    pub longitude: String,
    pub station_id: String,
    pub location_desc: String,
    pub altitude: String,
    pub timestamp: String,
    pub instrument_id: String,
    pub instrument_name: String,
    pub instrument_model: String,
    pub calibration_ref: String,
    pub uncertainty_value: String,
    pub uncertainty_unit: String,
    pub confidence_level: String,
    pub uncertainty_method: String,
}

impl Default for ProvenanceNode {
    fn default() -> Self {
        Self {
            id: Uuid::nil(),
            cid: CID(String::new()),
            epistemic_type: EpistemicType::Observed,
            payload: serde_json::Value::Null,
            author: Author {
                id: String::new(),
                name: None,
                author_type: AuthorType::Human,
                metadata: std::collections::HashMap::new(),
            },
            timestamp: Utc::now(),
            parents: Vec::new(),
            labels: Vec::new(),
            signature: None,
            domain: None,
            source_uri: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    async fn test_service() -> (IngestionService, std::path::PathBuf) {
        let path = std::env::temp_dir().join(format!("witness-test-{}.db", Uuid::new_v4()));
        std::fs::File::create(&path).unwrap();
        let storage = Arc::new(
            Storage::new(&format!("sqlite://{}", path.display()))
                .await
                .unwrap(),
        );
        (IngestionService::new(storage), path)
    }

    fn author() -> Author {
        Author {
            id: "instrument:test".to_string(),
            name: None,
            author_type: AuthorType::Instrument,
            metadata: HashMap::new(),
        }
    }

    fn measurement() -> Measurement {
        Measurement {
            quantity: "temperature".to_string(),
            value: MeasuredValue {
                numeric: Some(18.4),
                text: None,
                unit: Some("degC".to_string()),
                categorical: None,
            },
            location: Location {
                latitude: None,
                longitude: None,
                station_id: Some("test-station".to_string()),
                description: None,
                altitude_m: None,
            },
            measured_at: Utc::now(),
            instrument: InstrumentRef {
                id: "instrument:test".to_string(),
                name: None,
                model: None,
                calibration_ref: None,
            },
            uncertainty: None,
            chain_of_custody: None,
        }
    }

    #[tokio::test]
    async fn stored_node_cannot_be_overwritten() {
        let (service, path) = test_service().await;
        let node = service
            .ingest_observation(measurement(), author(), vec![], None, None, None)
            .await
            .unwrap();

        assert!(service.storage.store_node(&node).await.is_err());
        drop(service);
        std::fs::remove_file(path).unwrap();
    }

    #[tokio::test]
    async fn inference_rejects_missing_premise() {
        let (service, path) = test_service().await;
        let derivation = Derivation {
            premises: vec![Uuid::new_v4()],
            methodology: "test".to_string(),
            model_ref: None,
            parameters: HashMap::new(),
            claim: Claim {
                statement: "test claim".to_string(),
                claim_type: ClaimType::Descriptive,
                scope: ClaimScope::Specific,
            },
            falsifiers: vec![],
            inference_uncertainty: None,
        };

        let result = service
            .ingest_inference(derivation, author(), vec![], None, None, None)
            .await;

        assert!(matches!(result, Err(crate::WitnessError::Validation(_))));
        drop(service);
        std::fs::remove_file(path).unwrap();
    }
}
