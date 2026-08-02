use anyhow::Result;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;

#[derive(Clone)]
pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn new(
        url: &str
    ) -> Result<Self> {

        let pool =
            SqlitePoolOptions::new()
                .max_connections(5)
                .connect(url)
                .await?;


        Ok(Self {
            pool
        })
    }


    pub fn pool(
        &self
    ) -> &SqlitePool {
        &self.pool
    }
}