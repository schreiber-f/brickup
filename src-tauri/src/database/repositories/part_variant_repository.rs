use anyhow::Result;
use sqlx::{Sqlite, SqlitePool, Transaction};
use crate::database::models::{DbPartVariant};
#[derive(Clone)]
pub struct PartVariantRepository {
    pool: SqlitePool,
}

impl PartVariantRepository {
    pub fn new(pool: &SqlitePool) -> Self {
        Self {
            pool: pool.clone(),
        }
    }

    pub async fn upsert(
        &self,
        variant: &DbPartVariant,
        tx: &mut Transaction<'_, Sqlite>
    ) -> Result<()> {

        sqlx::query(
            r#"
            INSERT INTO part_variants (
                element_id,
                part_num,
                color_id,
                remote_image_url,
                local_image_path
            )
            VALUES (?, ?, ?, ?, ?)

            ON CONFLICT(element_id)
            DO UPDATE SET
                part_num = excluded.part_num,
                color_id = excluded.color_id,
                remote_image_url = excluded.remote_image_url,
                local_image_path = excluded.local_image_path;
            "#
        )
            .bind(&variant.element_id)
            .bind(&variant.part_num)
            .bind(variant.color_id)
            .bind(&variant.remote_image_url)
            .bind(&variant.local_image_path)
            .execute(&mut **tx)
            .await?;

        Ok(())
    }

    pub async fn get(
        &self,
        element_id: &str,
    ) -> Result<Option<DbPartVariant>> {
        let variant = sqlx::query_as::<_, DbPartVariant>(
            r#"
            SELECT *
            FROM part_variants
            WHERE element_id = ?;
            "#
        )
        .bind(element_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(variant)
    }

    pub async fn update_local_image_path(
        &self,
        element_id: &str,
        path: &str,
        tx: &mut Transaction<'_, Sqlite>,
    ) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE part_variants
            SET local_image_path = ?
            WHERE element_id = ?;
            "#
        )
        .bind(path)
        .bind(element_id)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    pub async fn update_image_path(
        &self,
        element_id: &str,
        path: &str,
    ) -> Result<()> {

        sqlx::query(
            r#"
        UPDATE part_variants
        SET local_image_path = ?
        WHERE element_id = ?
        AND local_image_path IS NULL;
        "#
        )
            .bind(path)
            .bind(element_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }


    pub async fn upsert_many(&self, variants: &[DbPartVariant], tx: &mut Transaction<'_, Sqlite>) -> Result<()> {

        for variant in variants {
            sqlx::query(
                r#"
                INSERT INTO part_variants (
                    element_id,
                    part_num,
                    color_id,
                    remote_image_url,
                    local_image_path
                )
                VALUES (?, ?, ?, ?, ?)

                ON CONFLICT(element_id)
                DO UPDATE SET
                    part_num = excluded.part_num,
                    color_id = excluded.color_id,
                    remote_image_url = excluded.remote_image_url,
                    local_image_path = excluded.local_image_path;
                "#
            )
            .bind(&variant.element_id)
            .bind(&variant.part_num)
            .bind(variant.color_id)
            .bind(&variant.remote_image_url)
            .bind(&variant.local_image_path)
            .execute(&mut **tx)
            .await?;
        }

        Ok(())
    }

    pub async fn delete(&self, element_id: &str, tx: &mut Transaction<'_, Sqlite>) -> Result<()> {
        sqlx::query(
            r#"
            DELETE FROM part_variants
            WHERE element_id = ?;
            "#
        )
        .bind(element_id)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }
}