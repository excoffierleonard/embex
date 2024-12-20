use crate::{AppError, ImageProcessor, VisionApiClient};

#[derive(Default)]
pub struct App {
    api_client: VisionApiClient,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn process_image(&self, image_path: &str) -> Result<String, AppError> {
        let base64_image = ImageProcessor::to_base64(image_path)?;
        self.api_client.analyze_image(base64_image).await
    }
}
