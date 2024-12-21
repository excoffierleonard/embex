use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ApiErrorResponse {
    pub error: String,
}

#[derive(Debug, Serialize)]
pub struct CompletionApiRequest {
    pub model: String,
    pub prompt: String,
    pub stream: bool,
    pub images: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct CompletionApiResponse {
    pub response: String,
}

#[derive(Debug, Serialize)]
pub struct EmbeddingApiRequest {
    pub model: String,
    pub input: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct EmbeddingApiResponse {
    pub embeddings: Vec<Vec<f32>>,
}
