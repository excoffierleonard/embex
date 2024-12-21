use crate::error::AppError;
use sqlx::{postgres::PgPool, Pool, Postgres};

pub struct DbClient {
    pool: Pool<Postgres>,
}

impl DbClient {
    pub async fn new(database_url: &str) -> Result<Self, AppError> {
        let pool = PgPool::connect(database_url)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(Self { pool })
    }

    pub async fn store_image_result(
        &self,
        b64: String,
        description: String,
        embedding: Vec<f32>,
    ) -> Result<(), AppError> {
        sqlx::query("INSERT INTO images (b64, description, embedding) VALUES ($1, $2, $3)")
            .bind(b64)
            .bind(description)
            .bind(&embedding[..])
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(())
    }
}
