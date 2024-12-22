use crate::error::AppError;
use base64::{engine::general_purpose::STANDARD, Engine};
use std::fs;

pub struct ImageProcessor;

impl ImageProcessor {
    pub fn to_base64(path: &str) -> Result<String, AppError> {
        let bytes = fs::read(path)?;
        Ok(STANDARD.encode(bytes))
    }

    pub fn to_file(b64: &str) -> Result<(), AppError> {
        let bytes = STANDARD.decode(b64)?;
        fs::write("test_output.png", bytes)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_image_to_base64() {
        // Create a temporary test file
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(b"test image content").unwrap();

        let result = ImageProcessor::to_base64(temp_file.path().to_str().unwrap());
        assert!(result.is_ok());

        // Known base64 of "test image content"
        let expected = STANDARD.encode(b"test image content");
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn test_image_to_base64_nonexistent_file() {
        let result = ImageProcessor::to_base64("nonexistent.png");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::File(_)));
    }
}
