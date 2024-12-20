use embex::{App, Config};

#[tokio::test]
async fn test_full_image_processing_flow() {
    // Source test image
    let test_image_path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/resources/test_image.png");

    let config = Config::build().expect("Failed to load configuration");
    let app = App::new(config);
    let result = app.process_image(test_image_path).await;

    match &result {
        Ok(_) => println!("Processing succeeded"),
        Err(e) => println!("Processing failed with error: {:?}", e),
    }

    // This test will fail in CI without a mock server
    assert!(result.is_ok());
}