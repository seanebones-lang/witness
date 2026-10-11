pub mod ingestion;
pub mod signing;
pub mod storage;
pub mod types;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum WitnessError {
    #[error("Storage error: {0}")]
    Storage(#[from] sqlx_core::Error),
    #[error("Migration error: {0}")]
    Migration(#[from] sqlx_core::migrate::MigrateError),
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

// SQLite uses background connection workers. Windows may briefly retain a file
// handle after pool shutdown (including filesystem scanners). Retry only its
// sharing/lock violation while cleaning disposable test files; persistent locks
// and every other error still fail the test.
#[cfg(test)]
async fn remove_test_path(path: &std::path::Path, directory: bool) {
    for attempt in 0..80 {
        let result = if directory {
            std::fs::remove_dir_all(path)
        } else {
            std::fs::remove_file(path)
        };
        match result {
            Ok(()) => return,
            Err(error)
                if cfg!(windows)
                    && matches!(error.raw_os_error(), Some(32 | 33))
                    && attempt < 79 =>
            {
                tokio::time::sleep(std::time::Duration::from_millis(25)).await;
            }
            Err(error) => panic!("Failed to remove test path {}: {error}", path.display()),
        }
    }
}
