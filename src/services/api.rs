use crate::{
    config,
    error::AppError,
    types::{ApiErrorResponse, ApiRequest, ApiResponse},
};
use reqwest::Client;
use std::io::{Error, ErrorKind};

#[derive(Default)]
pub struct VisionApiClient {
    client: Client,
}

impl VisionApiClient {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn analyze_image(&self, image_base64: String) -> Result<String, AppError> {
        let config = config::Config::build().map_err(|e| {
            eprintln!("Configuration error: {}", e);
            Error::new(ErrorKind::Other, e)
        })?;

        let request = ApiRequest {
            model: config.model_name,
            prompt: config.prompt,
            stream: false,
            images: vec![image_base64],
        };

        let response = self
            .client
            .post(config.api_url)
            .json(&request)
            .send()
            .await?;

        if response.status().is_success() {
            let success: ApiResponse = response.json().await?;
            Ok(success.response)
        } else {
            let error: ApiErrorResponse = response.json().await?;
            Err(AppError::Api(error.error))
        }
    }
}

// Need to write Unit tests
