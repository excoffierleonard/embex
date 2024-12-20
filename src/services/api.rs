use crate::{
    config::Config,
    error::AppError,
    types::{ApiErrorResponse, ApiRequest, ApiResponse},
};
use reqwest::Client;

#[derive(Default)]
pub struct VisionApiClient {
    client: Client,
    config: Config,
}

impl VisionApiClient {
    pub fn new(config: Config) -> Self {
        Self {
            client: Client::new(),
            config,
        }
    }

    pub async fn analyze_image(&self, image_base64: String) -> Result<String, AppError> {
        let request = ApiRequest {
            model: self.config.model_name.clone(),
            prompt: self.config.prompt.clone(),
            stream: false,
            images: vec![image_base64],
        };

        let response = self
            .client
            .post(&self.config.api_url)
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
