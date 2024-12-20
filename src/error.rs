use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("API error: {0}")]
    Api(String),
    #[error("Request failed: {0}")]
    Request(#[from] reqwest::Error),
    #[error("File error: {0}")]
    File(#[from] std::io::Error),
    #[error("Base64 encoding error: {0}")]
    Encoding(#[from] base64::DecodeError),
}
