pub mod types;
pub mod storage;
pub mod signing;
pub mod ingestion;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum WitnessError {
    #[error("Storage error: {0}")]
    Storage(#[from] sqlx::Error),
    #[error("Migration error: {0}")]
    Migration(#[from] sqlx::migrate::MigrateError),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("Signing error: {0}")]
    Signing(String),
    #[error("IPFS error: {0}")]
    Ipfs(String),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Crypto error: {0}")]
    Crypto(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("CSV error: {0}")]
    Csv(#[from] csv::Error),
}

pub type Result<T> = std::result::Result<T, WitnessError>;