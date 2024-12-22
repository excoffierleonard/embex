use embex::{App, Config};

#[tokio::test]
async fn test_full_image_processing_flow() {
    // Source test images
    let test_image_path_1 = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/resources/test_image_1.png"
    );

    let test_image_path_2 = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/resources/test_image_2.png"
    );

    let config = Config::build().expect("Failed to load configuration");
    let app = App::new(config).await.expect("Failed to initialize app");
    let result = app
        .process_image(vec![test_image_path_1, test_image_path_2])
        .await;

    // This test will fail in CI without a mock server
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_full_image_retreival_flow() {
    // Lookup Prompt
    let prompt = "Green Forest";

    let config = Config::build().expect("Failed to load configuration");
    let app = App::new(config).await.expect("Failed to initialize app");
    let result = app.find_images(prompt).await;

    // This test will fail in CI without a mock server
    assert!(result.is_ok());
}
