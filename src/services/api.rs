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

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server; // Changed from mockito::mock

    #[tokio::test]
    async fn test_analyze_image_success() {
        let mut server = Server::new(); // Create new server
        let mock = server
            .mock("POST", "/api/generate")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"response":"A beautiful landscape"}"#)
            .create();

        let client = VisionApiClient::new();
        let result = client.analyze_image("base64_string".to_string()).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "A beautiful landscape");
        mock.assert();
    }

    #[tokio::test]
    async fn test_analyze_image_error() {
        let mut server = Server::new();
        let mock = server
            .mock("POST", "/api/generate")
            .with_status(400)
            .with_header("content-type", "application/json")
            .with_body(r#"{"error":"Invalid image format"}"#)
            .create();

        let client = VisionApiClient::new();
        let result = client.analyze_image("invalid_base64".to_string()).await;

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::Api(_)));
        mock.assert();
    }
}
