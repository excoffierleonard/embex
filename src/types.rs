use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ApiResponse {
    pub response: String,
}

#[derive(Debug, Deserialize)]
pub struct ApiErrorResponse {
    pub error: String,
}

#[derive(Debug, Serialize)]
pub struct ApiRequest {
    pub model: String,
    pub prompt: String,
    pub stream: bool,
    pub images: Vec<String>,
}
