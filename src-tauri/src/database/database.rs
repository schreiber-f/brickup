use anyhow::Result;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;
use std::path::Path;

#[derive(Clone)]
pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn new(
        url: &str
    ) -> Result<Self> {
        // SQLite Datei erstellen falls sie nicht existiert
        if let Some(path) = url.strip_prefix("sqlite:") {
            let path = Path::new(path);

            if let Some(parent) = path.parent() {
                if !parent.as_os_str().is_empty() {
                    tokio::fs::create_dir_all(parent).await?;
                }
            }

            if !path.exists() {
                tokio::fs::File::create(path).await?;
            }
        }


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