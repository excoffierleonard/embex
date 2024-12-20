use crate::{
    config::{API_URL, DEFAULT_PROMPT, MODEL_NAME},
    error::AppError,
    types::{ApiErrorResponse, ApiRequest, ApiResponse},
};
use reqwest::Client;

#[derive(Default)]
pub struct VisionApiClient {
    client: Client,
}

impl VisionApiClient {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn analyze_image(&self, image_base64: String) -> Result<String, AppError> {
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
