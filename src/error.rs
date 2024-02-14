#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Generic {0}")]
    Generic(String),

    #[error("ElasticsearchError {0}")]
    Elasticsearch(#[from] elasticsearch::Error),

    #[error("SerdeJsonError {0}")]
    SerdeJson(#[from] serde_json::Error),

    #[error("ReqwestError {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("IOError {0}")]
    IO(#[from] std::io::Error),
}
