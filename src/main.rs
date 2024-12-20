use base64::{engine::general_purpose::STANDARD, Engine};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fs;
use thiserror::Error;

// Types
#[derive(Debug, Deserialize)]
struct ApiResponse {
    response: String,
}

#[derive(Debug, Deserialize)]
struct ApiErrorResponse {
    error: String,
}

#[derive(Debug, Serialize)]
struct ApiRequest {
    model: String,
    prompt: String,
    stream: bool,
    images: Vec<String>,
}

// Error handling
#[derive(Debug, Error)]
enum AppError {
    #[error("API error: {0}")]
    Api(String),
    #[error("Request failed: {0}")]
    Request(#[from] reqwest::Error),
    #[error("File error: {0}")]
    File(#[from] std::io::Error),
    #[error("Base64 encoding error: {0}")]
    Encoding(#[from] base64::DecodeError),
}

// Configuration
const API_URL: &str = "http://ollama.local/api/generate";
const MODEL_NAME: &str = "llama3.2-vision";
const DEFAULT_PROMPT: &str = "What is in this picture?";

// Image processing
struct ImageProcessor;

impl ImageProcessor {
    fn to_base64(path: &str) -> Result<String, AppError> {
        let bytes = fs::read(path)?;
        Ok(STANDARD.encode(bytes))
    }
}

// API client
struct VisionApiClient {
    client: Client,
}

impl VisionApiClient {
    fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    async fn analyze_image(&self, image_base64: String) -> Result<String, AppError> {
        let request = ApiRequest {
            model: MODEL_NAME.to_string(),
            prompt: DEFAULT_PROMPT.to_string(),
            stream: false,
            images: vec![image_base64],
        };

        let response = self.client.post(API_URL).json(&request).send().await?;

        if response.status().is_success() {
            let success: ApiResponse = response.json().await?;
            Ok(success.response)
        } else {
            let error: ApiErrorResponse = response.json().await?;
            Err(AppError::Api(error.error))
        }
    }
}

// Application
struct App {
    api_client: VisionApiClient,
}

impl App {
    fn new() -> Self {
        Self {
            api_client: VisionApiClient::new(),
        }
    }

    async fn process_image(&self, image_path: &str) -> Result<String, AppError> {
        let base64_image = ImageProcessor::to_base64(image_path)?;
        self.api_client.analyze_image(base64_image).await
    }
}

#[tokio::main]
async fn main() {
    let app = App::new();

    match app.process_image("image.png").await {
        Ok(response) => println!("Analysis result: {response}"),
        Err(e) => eprintln!("Error: {e}"),
    }
}
