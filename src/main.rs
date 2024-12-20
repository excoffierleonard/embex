use embex::{AppError, ImageProcessor, VisionApiClient};

struct App {
    api_client: VisionApiClient,
}

impl App {
    fn new() -> Self {
        Self {
            api_client: VisionApiClient::new(),
        }
    }

    async fn process_image(&self, image_path: &str) -> Result<String, AppError> {
        let base64_image = ImageProcessor::to_base64(image_path)?;
        self.api_client.analyze_image(base64_image).await
    }
}

#[tokio::main]
async fn main() {
    let app = App::new();

    match app.process_image("image.png").await {
        Ok(response) => println!("Analysis result: {response}"),
        Err(e) => eprintln!("Error: {e}"),
    }
}
