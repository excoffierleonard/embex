use crate::error::AppError;
use sqlx::{postgres::PgPool, Pool, Postgres, Row};

pub struct DbClient {
    pool: Pool<Postgres>,
}

#[derive(Debug)]
pub struct QueryResult {
    pub rows: Vec<TableRow>,
}

#[derive(Debug)]
pub struct TableRow {
    pub b64: String,
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

    pub async fn fetch_similar_images(&self, embedding: Vec<f32>) -> Result<QueryResult, AppError> {
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

        let rows = result
            .iter()
            .map(|row| TableRow {
                b64: row.get("data"),
            })
            .collect();

        Ok(QueryResult { rows })
    }
}
