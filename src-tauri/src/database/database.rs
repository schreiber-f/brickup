use anyhow::Result;
use sqlx::SqlitePool;

#[derive(Clone)]
pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn new(path: &str) -> Result<Self> {
        println!("Opening database at {}", path);

        let pool = SqlitePool::connect(path).await?;

        sqlx::query("PRAGMA foreign_keys = ON;")
            .execute(&pool)
            .await?;

        Ok(Self { pool })
    }

    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }
}