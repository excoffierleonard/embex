use crate::error::AppError;
use base64::{engine::general_purpose::STANDARD, Engine};
use std::fs;

pub struct ImageProcessor;

impl ImageProcessor {
    pub fn to_base64(path: &str) -> Result<String, AppError> {
        let bytes = fs::read(path)?;
        Ok(STANDARD.encode(bytes))
    }
}
