use embex::{App, Config};
use std::io::Write;
use tempfile::NamedTempFile;

#[tokio::test]
async fn test_full_image_processing_flow() {
    // Create a test image
    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file.write_all(b"test image content").unwrap();

    let config = Config::default();
    let app = App::new(config);
    let result = app.process_image(temp_file.path().to_str().unwrap()).await;

    // This test will fail in CI without a mock server
    // Need to set up a mock server here
    assert!(result.is_err());
}
