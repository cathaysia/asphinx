use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("File not found: {0}")]
    NotFound(String),
}
