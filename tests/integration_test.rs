use embex::{App, Config};

#[tokio::test]
async fn test_full_image_processing_flow() {
    // Source test image
    let test_image_path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/resources/test_image.png"
    );

    let config = Config::build().expect("Failed to load configuration");
    let app = App::new(config).await.expect("Failed to initialize app");
    let result = app.process_image(test_image_path).await;

    // This test will fail in CI without a mock server
    assert!(result.is_ok());
}
