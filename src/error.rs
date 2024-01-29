#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Generic {0}")]
    Generic(String),

    #[error("ElasticsearchError {0}")]
    ElasticsearchError(#[from] elasticsearch::Error),

    #[error("SerdeJsonError {0}")]
    SerdeJsonError(#[from] serde_json::Error),

    #[error("ReqwestError {0}")]
    ReqwestError(#[from] reqwest::Error),

    #[error("IOError {0}")]
    IOError(#[from] std::io::Error),
}
