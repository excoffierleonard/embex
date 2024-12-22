use crate::{AppError, Config, DbClient, ImageProcessor, VisionApiClient};

pub struct App {
    api_client: VisionApiClient,
    db_client: DbClient,
}

impl App {
    pub async fn new(config: Config) -> Result<Self, AppError> {
        let db_client = DbClient::new(&config.database_url).await?;
        db_client.initialize().await?;
        let api_client = VisionApiClient::new(config);

        Ok(Self {
            api_client,
            db_client,
        })
    }

    pub async fn process_images(&self, folder_path: &str) -> Result<(), AppError> {
        // Need more efficient implementation of this loop maybe with iterators on all level since some apis can take vectors of strings

        let image_paths = std::fs::read_dir(folder_path)?
            .filter_map(Result::ok)
            .filter(|entry| entry.path().is_file())
            .map(|entry| entry.path().to_string_lossy().into_owned())
            .collect::<Vec<String>>();

        for path in image_paths {
            let base64_image = ImageProcessor::to_base64(&path)?;
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
        }

        Ok(())
    }

    pub async fn find_images(
        &self,
        prompt: &str,
        output_folder_path: &str,
    ) -> Result<(), AppError> {
        let embedding = self
            .api_client
            .embed_description(prompt.to_string())
            .await?;

        let base64_images = self
            .db_client
            .fetch_similar_images(embedding.into_iter().flatten().collect())
            .await?;

        for (i, base64_image) in base64_images.iter().enumerate() {
            let file_name = format!("{}/output_{}.png", output_folder_path, i + 1);
            ImageProcessor::to_file(base64_image, &file_name)?;
        }

        Ok(())
    }
}
