use crate::{
    config::Config,
    error::AppError,
    types::{
        ApiErrorResponse, CompletionApiRequest, CompletionApiResponse, EmbeddingApiRequest,
        EmbeddingApiResponse,
    },
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
        let request = CompletionApiRequest {
            model: self.config.completion_model.clone(),
            prompt: self.config.completion_prompt.clone(),
            stream: false,
            images: vec![image_base64],
        };

        let response = self
            .client
            .post(&self.config.completion_endpoint)
            .json(&request)
            .send()
            .await?;

        if response.status().is_success() {
            let success: CompletionApiResponse = response.json().await?;
            Ok(success.response)
        } else {
            let error: ApiErrorResponse = response.json().await?;
            Err(AppError::Api(error.error))
        }
    }

    pub async fn embed_description(&self, description: String) -> Result<Vec<Vec<f32>>, AppError> {
        let request = EmbeddingApiRequest {
            model: self.config.embedding_model.clone(),
            input: vec![description],
        };

        let response = self
            .client
            .post(&self.config.embedding_endpoint)
            .json(&request)
            .send()
            .await?;

        if response.status().is_success() {
            let success: EmbeddingApiResponse = response.json().await?;
            Ok(success.embeddings)
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
            completion_endpoint: server.url(),
            completion_model: "test_completion_model".to_string(),
            completion_prompt: "test_completion_prompt".to_string(),
            embedding_endpoint: "test_embedding_endpoint".to_string(),
            embedding_model: "test_embedding_model".to_string(),
            database_url: "test_db_url".to_string(),
        };

        let client = VisionApiClient::new(config);
        let result = client.analyze_image("test_image_base64".to_string()).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "analysis result");
        mock.assert();
    }

    #[tokio::test]
    async fn test_embed_description_success() {
        let mut server = Server::new_async().await;

        let mock = server
            .mock("POST", "/")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"embeddings": [[-0.0054740426,0.016664743,-0.18402556]]}"#)
            .create();

        let config = Config {
            completion_endpoint: "test_completion_endpoint".to_string(),
            completion_model: "test_completion_model".to_string(),
            completion_prompt: "test_completion_prompt".to_string(),
            embedding_endpoint: server.url(),
            embedding_model: "test_embedding_model".to_string(),
            database_url: "test_db_url".to_string(),
        };

        let client = VisionApiClient::new(config);
        let result = client
            .embed_description("test_description".to_string())
            .await;

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            vec![vec![-0.0054740426, 0.016664743, -0.18402556]]
        );
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
            completion_endpoint: server.url(),
            completion_model: "test_completion_model".to_string(),
            completion_prompt: "test_completion_prompt".to_string(),
            embedding_endpoint: "test_embedding_endpoint".to_string(),
            embedding_model: "test_embedding_model".to_string(),
            database_url: "test_db_url".to_string(),
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
            completion_endpoint: "http://invalidurl".to_string(),
            completion_model: "test_completion_model".to_string(),
            completion_prompt: "test_completion_prompt".to_string(),
            embedding_endpoint: "test_embedding_endpoint".to_string(),
            embedding_model: "test_embedding_model".to_string(),
            database_url: "test_db_url".to_string(),
        };

        let client = VisionApiClient::new(config);
        let result = client.analyze_image("test_image_base64".to_string()).await;

        assert!(result.is_err());
        assert!(matches!(result, Err(AppError::Request(_))));
    }
}
