use crate::error::AppError;
use sqlx::{postgres::PgPool, Pool, Postgres, Row};

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

    pub async fn initialize(&self) -> Result<(), AppError> {
        sqlx::query(
            "
            CREATE EXTENSION IF NOT EXISTS vector;
            ",
        )
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        sqlx::query(
            "
            CREATE TABLE IF NOT EXISTS images (
                id UUID DEFAULT gen_random_uuid(),
                data BYTEA NOT NULL,
                description TEXT,
                embedding vector(768),
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                PRIMARY KEY (id)
            );
            ",
        )
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(())
    }

    // TODO: Need to also input the title of the image and its metadata to ensure no duplicates for a well defined db
    pub async fn store_image_result(
        &self,
        b64: String,
        description: String,
        embedding: Vec<f32>,
    ) -> Result<(), AppError> {
        sqlx::query(
            "
            INSERT INTO images (data, description, embedding) 
            VALUES (decode($1, 'base64'), $2, $3)
            ",
        )
        .bind(b64)
        .bind(description)
        .bind(&embedding[..])
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(())
    }

    // TODO: Need to make the LIMIT a variable that can be set by the user
    // TODO: Need to define unit tests for all program
    pub async fn fetch_similar_images(&self, embedding: Vec<f32>) -> Result<Vec<String>, AppError> {
        let result = sqlx::query(
            "
            SELECT encode(data, 'base64') as data
            FROM images
            WHERE embedding IS NOT NULL
            ORDER BY embedding <=> $1::vector(768)
            LIMIT $2;
            ",
        )
        .bind(&embedding[..])
        .bind(3)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        let images = result
            .iter()
            .map(|row| {
                let data: String = row.get("data");
                data.trim().replace("\n", "").replace("\r", "")
            })
            .collect();

        Ok(images)
    }
}
