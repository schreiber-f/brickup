use anyhow::Result;
use sqlx::{Sqlite, SqlitePool, Transaction};
use crate::database::models::{DbSet};
#[derive(Clone)]
pub struct SetRepository {
    pool: SqlitePool,
}

impl SetRepository {

    pub fn new(pool: &SqlitePool) -> Self {
        Self { pool: pool.clone() }
    }

    pub async fn upsert(
        &self,
        set: &DbSet,
        tx: &mut Transaction<'_, Sqlite>
    ) -> Result<()>{
        
    sqlx::query(
        r#"
    INSERT INTO sets (
        set_num,
        name,
        year,
        theme_id,
        num_parts,
        remote_image_url,
        local_image_path,
        set_url,
        last_modified
    )
    VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)

    ON CONFLICT(set_num)
    DO UPDATE SET
        name = excluded.name,
        year = excluded.year,
        theme_id = excluded.theme_id,
        num_parts = excluded.num_parts,
        remote_image_url = excluded.remote_image_url,
        local_image_path = excluded.local_image_path,
        set_url = excluded.set_url,
        last_modified = excluded.last_modified;
    "#
    )
        .bind(&set.set_num)
        .bind(&set.name)
        .bind(set.year)
        .bind(set.theme_id)
        .bind(set.num_parts)
        .bind(&set.remote_image_url)
        .bind(&set.local_image_path)
        .bind(&set.set_url)
        .bind(&set.last_modified)
        .execute(&mut **tx)
        .await?;

    Ok(())
    }

    pub async fn update_image_path(
        &self,
        set_num: &str,
        path: &str,
    ) -> Result<()> {

        sqlx::query(
            r#"
        UPDATE sets
        SET local_image_path = ?
        WHERE set_num = ?
        AND local_image_path IS NULL;
        "#
        )
            .bind(path)
            .bind(set_num)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn get(&self, set_num: &str) -> Result<Option<DbSet>> {
        let set = sqlx::query_as::<_, DbSet>(
            r#"
            SELECT * FROM sets
            WHERE set_num = ?;
            "#
        )
        .bind(set_num)
        .fetch_optional(&self.pool)
        .await?;

        Ok(set)
    }

    pub async fn delete(&self, set_num: &str, tx: &mut Transaction<'_, Sqlite>) -> Result<()> {
        sqlx::query(
            r#"
            DELETE FROM sets
            WHERE set_num = ?;
            "#
        )
        .bind(set_num)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }
}