use embex::{App, Config};

#[tokio::test]
async fn test_full_image_processing_flow() {
    // Source test images
    let test_input_folder_path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/resources");

    let config = Config::build().expect("Failed to load configuration");
    let app = App::new(config).await.expect("Failed to initialize app");
    let result = app.process_images(test_input_folder_path).await;

    // This test will fail in CI without a mock server
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_full_image_retreival_flow() {
    // Lookup Prompt
    let prompt = "An Otter";
    let test_output_folder_path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/output");

    let config = Config::build().expect("Failed to load configuration");
    let app = App::new(config).await.expect("Failed to initialize app");
    let result = app.find_images(prompt, test_output_folder_path).await;

    // This test will fail in CI without a mock server
    assert!(result.is_ok());
}
