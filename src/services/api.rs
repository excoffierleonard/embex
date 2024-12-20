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

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;

    #[tokio::test]
    async fn test_analyze_image_success() {
        let mut server = Server::new_async().await;

        let mock = server
            .mock("POST", "/")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"response": "analysis result"}"#)
            .create();

        let config = Config {
            api_url: server.url(),
            model_name: "test_model".to_string(),
            prompt: "test_prompt".to_string(),
        };

        let client = VisionApiClient::new(config);
        let result = client.analyze_image("test_image_base64".to_string()).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "analysis result");
        mock.assert();
    }

    #[tokio::test]
    async fn test_analyze_image_api_error() {
        let mut server = Server::new_async().await;

        let mock = server
            .mock("POST", "/")
            .with_status(400)
            .with_header("content-type", "application/json")
            .with_body(r#"{"error": "invalid request"}"#)
            .create();

        let config = Config {
            api_url: server.url(),
            model_name: "test_model".to_string(),
            prompt: "test_prompt".to_string(),
        };

        let client = VisionApiClient::new(config);
        let result = client.analyze_image("test_image_base64".to_string()).await;

        assert!(result.is_err());
        assert!(matches!(result, Err(AppError::Api(msg)) if msg == "invalid request"));
        mock.assert();
    }

    #[tokio::test]
    async fn test_analyze_image_request_error() {
        let config = Config {
            api_url: "http://invalid_url".to_string(),
            model_name: "test_model".to_string(),
            prompt: "test_prompt".to_string(),
        };

        let client = VisionApiClient::new(config);
        let result = client.analyze_image("test_image_base64".to_string()).await;

        assert!(result.is_err());
        assert!(matches!(result, Err(AppError::Request(_))));
    }
}
