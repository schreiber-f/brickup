use anyhow::Result;
use sqlx::{Sqlite, SqlitePool, Transaction};
use crate::database::models::{DbPart};
#[derive(Clone)]
pub struct PartRepository {
    pool: SqlitePool,
}

impl PartRepository {
    pub fn new(pool: &SqlitePool) -> Self {
        Self {
            pool: pool.clone(),
        }
    }
    
    pub async fn upsert(&self, part: &DbPart, tx: &mut Transaction<'_, Sqlite>) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO parts (
                part_num,
                name,
                category_id
            )
            VALUES (?, ?, ?)

            ON CONFLICT(part_num)
            DO UPDATE SET
                name = excluded.name,
                category_id = excluded.category_id;
            "#
        )
            .bind(&part.part_num)
            .bind(&part.name)
            .bind(&part.category_id)
            .execute(&mut **tx)
            .await?;
        
        Ok(())
    }
    
    pub async fn upsert_many(&self, parts: &[DbPart], tx: &mut Transaction<'_, Sqlite>) -> Result<()> {

        for part in parts {
            sqlx::query(
                r#"
                INSERT INTO parts (
                    part_num,
                    name,
                    category_id
                )
                VALUES (?, ?, ?)
    
                ON CONFLICT(part_num)
                DO UPDATE SET
                    name = excluded.name,
                    category_id = excluded.category_id;
                "#
            )
                .bind(&part.part_num)
                .bind(&part.name)
                .bind(&part.category_id)
                .execute(&mut **tx)
                .await?;
        }

        Ok(())
    }
    
    pub async fn get(&self, part_num: &str) -> Result<Option<DbPart>> {
        let part = sqlx::query_as::<_, DbPart>(
            r#"
            SELECT *
            FROM parts
            WHERE part_num = ?;
            "#
        )
            .bind(part_num)
            .fetch_optional(&self.pool)
            .await?;

        Ok(part)
    }
    
    pub async fn delete(&self, part_num: &str, tx: &mut Transaction<'_, Sqlite>) -> Result<()> {
        sqlx::query(
            r#"
            DELETE FROM parts
            WHERE part_num = ?;
            "#
        )
            .bind(part_num)
            .execute(&mut **tx)
            .await?;
        
        Ok(())
    }
}