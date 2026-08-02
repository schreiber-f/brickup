use anyhow::Result;
use sqlx::{Sqlite, SqlitePool, Transaction};
use crate::database::models::{DbColor};
#[derive(Clone)]
pub struct ColorRepository {
    pool: SqlitePool,
}

impl ColorRepository {
    pub fn new(pool: &SqlitePool) -> Self {
        Self {
            pool: pool.clone(),
        }
    }

    pub async fn upsert(&self, color: &DbColor, tx: &mut Transaction<'_, Sqlite>) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO colors (
                id,
                name,
                rgb,
                is_trans
            )
            VALUES (?, ?, ?, ?)

            ON CONFLICT(id)
            DO UPDATE SET
                name = excluded.name,
                rgb = excluded.rgb,
                is_trans = excluded.is_trans;
            "#
        )
            .bind(&color.id)
            .bind(&color.name)
            .bind(&color.rgb)
            .bind(&color.is_trans)
            .execute(&mut **tx)
            .await?;

        Ok(())
    }

    pub async fn upsert_many(&self, colors: &[DbColor], tx: &mut Transaction<'_, Sqlite>) -> Result<()> {

        for color in colors {
            sqlx::query(
                r#"
            INSERT INTO colors (
                id,
                name,
                rgb,
                is_trans
            )
            VALUES (?, ?, ?, ?)

            ON CONFLICT(id)
            DO UPDATE SET
                name = excluded.name,
                rgb = excluded.rgb,
                is_trans = excluded.is_trans;
            "#
            )
                .bind(&color.id)
                .bind(&color.name)
                .bind(&color.rgb)
                .bind(&color.is_trans)
                .execute(&mut **tx)
                .await?;
        }

        Ok(())
    }

    pub async fn get(&self, id: u32) -> Result<Option<DbColor>> {
        let color = sqlx::query_as::<_, DbColor>(
            r#"
            SELECT *
            FROM colors
            WHERE id = ?;
            "#
        )
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(color)
    }

    pub async fn delete(&self, id: u32, tx: &mut Transaction<'_, Sqlite>) -> Result<()> {
        sqlx::query(
            r#"
            DELETE FROM colors
            WHERE id = ?;
            "#
        )
            .bind(id)
            .execute(&mut **tx)
            .await?;

        Ok(())
    }
}