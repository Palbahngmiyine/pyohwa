use thiserror::Error;

#[derive(Error, Debug)]
pub enum SearchError {
    #[error("search index serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}
