use witness_core::{
    types::*,
    storage::Storage,
    ingestion::IngestionService,
    signing::{SigningKeypair, compute_cid},
    Result,
};
use clap::{Parser, Subcommand};
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;

#[derive(Parser)]
#[command(name = "witness-ingest", version, about = "Witness ingestion CLI")]
struct Cli {
    #[arg(long, env = "WITNESS_DB", default_value = "sqlite://witness.db")]
    database: String,

    #[arg(long, env = "WITNESS_KEY")]
    key_file: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Ingest a single observation from JSON
    Observe {
        #[arg(long)] quantity: String,
        #[arg(long)] value: String,
        #[arg(long)] unit: Option<String>,
        #[arg(long)] latitude: Option<f64>,
        #[arg(long)] longitude: Option<f64>,
        #[arg(long)] station_id: Option<String>,
        #[arg(long)] instrument_id: String,
        #[arg(long)] author_id: String,
        #[arg(long)] author_name: Option<String>,
        #[arg(long)] domain: Option<String>,
        #[arg(long)] labels: Vec<String>,
    },
    /// Ingest observations from CSV
    Csv {
        #[arg(long)] file: String,
        #[arg(long)] author_id: String,
        #[arg(long)] author_name: Option<String>,
        #[arg(long)] domain: Option<String>,
        #[arg(long)] quantity_col: String,
        #[arg(long)] value_col: String,
        #[arg(long)] unit_col: Option<String>,
        #[arg(long)] lat_col: Option<String>,
        #[arg(long)] lon_col: Option<String>,
        #[arg(long)] station_col: Option<String>,
        #[arg(long)] instrument_col: String,
        #[arg(long)] timestamp_col: String,
    },
    /// Ingest from JSONL (one node per line)
    Jsonl {
        #[arg(long)] file: String,
    },
    /// Ingest an inference
    Infer {
        #[arg(long)] claim: String,
        #[arg(long)] methodology: String,
        #[arg(long)] premise: Vec<String>,
        #[arg(long)] author_id: String,
        #[arg(long)] author_name: Option<String>,
        #[arg(long)] domain: Option<String>,
        #[arg(long)] labels: Vec<String>,
    },
    /// Generate a new signing keypair
    GenKey {
        #[arg(long)] output: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let storage = Arc::new(Storage::new(&cli.database).await?);
    let keypair = if let Some(path) = cli.key_file {
        let bytes = tokio::fs::read(&path).await?;
        let arr: [u8; 32] = bytes.as_slice().try_into().map_err(|_| witness_core::WitnessError::Crypto("Key must be 32 bytes".to_string()))?;
        Some(SigningKeypair::from_bytes(&arr)?)
    } else {
        None
    };

    let service = IngestionService::new(storage.clone());
    if let Some(kp) = keypair.clone() {
        service = service.with_keypair(kp);
    }

    match cli.command {
        Commands::Observe { quantity, value, unit, latitude, longitude, station_id, instrument_id, author_id, author_name, domain, labels } => {
            let author = Author {
                id: author_id,
                name: author_name,
                author_type: AuthorType::Instrument,
                metadata: std::collections::HashMap::new(),
            };

            let measurement = Measurement {
                quantity,
                value: MeasuredValue {
                    numeric: value.parse().ok(),
                    text: if value.parse::<f64>().is_err() { Some(value) } else { None },
                    unit,
                    categorical: None,
                },
                location: Location {
                    latitude,
                    longitude,
                    station_id,
                    description: None,
                    altitude_m: None,
                },
                measured_at: Utc::now(),
                instrument: InstrumentRef {
                    id: instrument_id,
                    name: None,
                    model: None,
                    calibration_ref: None,
                },
                uncertainty: None,
                chain_of_custody: None,
            };

            let node = service.ingest_observation(measurement, author, labels, domain, None, keypair.as_ref()).await?;
            println!("Ingested observation: {}", node.id);
        }
        Commands::Csv { file, author_id, author_name, domain, quantity_col, value_col, unit_col, lat_col, lon_col, station_col, instrument_col, timestamp_col } => {
            let author = Author {
                id: author_id,
                name: author_name,
                author_type: AuthorType::Instrument,
                metadata: std::collections::HashMap::new(),
            };

            let mut mapping = witness_core::ingestion::CsvColumnMapping::default();
            mapping.quantity = quantity_col;
            mapping.value_numeric = value_col.clone();
            mapping.value_text = value_col;
            mapping.unit = unit_col.unwrap_or_default();
            mapping.latitude = lat_col.unwrap_or_default();
            mapping.longitude = lon_col.unwrap_or_default();
            mapping.station_id = station_col.unwrap_or_default();
            mapping.instrument_id = instrument_col;
            mapping.timestamp = timestamp_col;

            let nodes = service.ingest_csv_observations(&file, author, domain, mapping, keypair.as_ref()).await?;
            println!("Ingested {} observations from CSV", nodes.len());
        }
        Commands::Jsonl { file } => {
            let nodes = service.ingest_jsonl(&file, keypair.as_ref()).await?;
            println!("Ingested {} nodes from JSONL", nodes.len());
        }
        Commands::Infer { claim, methodology, premise, author_id, author_name, domain, labels } => {
            let author = Author {
                id: author_id,
                name: author_name,
                author_type: AuthorType::Human,
                metadata: std::collections::HashMap::new(),
            };

            let premises: Vec<Uuid> = premise.iter().filter_map(|p| Uuid::parse_str(p).ok()).collect();

            let derivation = Derivation {
                premises,
                methodology,
                model_ref: None,
                parameters: std::collections::HashMap::new(),
                claim: Claim {
                    statement: claim,
                    claim_type: ClaimType::Descriptive,
                    scope: ClaimScope::Specific,
                },
                falsifiers: Vec::new(),
                inference_uncertainty: None,
            };

            let node = service.ingest_inference(derivation, author, labels, domain, None, keypair.as_ref()).await?;
            println!("Ingested inference: {}", node.id);
        }
        Commands::GenKey { output } => {
            let kp = SigningKeypair::generate();
            let private_hex = kp.private_key_hex();
            tokio::fs::write(&output, private_hex).await?;
            println!("Generated keypair saved to {}", output);
            println!("Public key: {}", kp.public_key_hex());
        }
    }

    Ok(())
}