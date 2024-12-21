use crate::{AppError, Config, DbClient, ImageProcessor, VisionApiClient};

pub struct App {
    api_client: VisionApiClient,
    db_client: DbClient,
}

impl App {
    pub async fn new(config: Config) -> Result<Self, AppError> {
        let db_client = DbClient::new(&config.database_url).await?;
        let api_client = VisionApiClient::new(config);

        Ok(Self {
            api_client,
            db_client,
        })
    }

    pub async fn process_image(&self, image_path: &str) -> Result<(), AppError> {
        let base64_image = ImageProcessor::to_base64(image_path)?;
        let description = self.api_client.analyze_image(base64_image.clone()).await?;
        let embedding = self
            .api_client
            .embed_description(description.clone())
            .await?;
        self.db_client
            .store_image_result(
                base64_image,
                description,
                embedding.into_iter().flatten().collect(),
            )
            .await?;
        Ok(())
    }
}
