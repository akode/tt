use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExtractionError {
    #[error("No method to extract from {0}")]
    UnknownDomain(String),
}
