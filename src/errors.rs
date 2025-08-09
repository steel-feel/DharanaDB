use thiserror::Error;

#[derive(Debug, Error)]
pub enum DharanaError {
    #[error("Failed to read file: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Failed to parse integer: {0}")]
    ParseError(#[from] std::num::ParseIntError),

    #[error("Serialization error: {0}")]
    SerialError(#[from] serde_json::Error  ),

    #[error("File format is invalid: {0}")]
    InvalidFormat(String),
}



/// Result type for kvs.
pub type SingleResult<T> = std::result::Result<T, DharanaError>;