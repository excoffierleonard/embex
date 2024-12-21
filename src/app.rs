use crate::{AppError, Config, ImageProcessor, VisionApiClient};

#[derive(Default)]
pub struct App {
    api_client: VisionApiClient,
}

impl App {
    pub fn new(config: Config) -> Self {
        Self {
            api_client: VisionApiClient::new(config),
        }
    }

    pub async fn process_image(&self, image_path: &str) -> Result<Vec<Vec<f32>>, AppError> {
        let base64_image = ImageProcessor::to_base64(image_path)?;
        let description = self.api_client.analyze_image(base64_image).await?;
        let embedding = self
            .api_client
            .embed_description(description.clone())
            .await?;
        Ok(embedding)
    }
}
