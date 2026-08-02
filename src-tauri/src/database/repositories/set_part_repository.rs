use crate::database::models::{DbCompletePart, DbSetPart};
use anyhow::Result;
use sqlx::{Sqlite, SqlitePool, Transaction};
#[derive(Clone)]
pub struct SetPartRepository {
    pool: SqlitePool,
}

impl SetPartRepository {
    pub fn new(pool: &SqlitePool) -> Self {
        Self { pool: pool.clone() }
    }

    const COMPLETE_PART_SELECT: &str = r#"SELECT
    sp.set_num,
    sp.element_id,

    sp.quantity,
    sp.is_spare,

    pv.part_num,
    pv.color_id,

    pv.remote_image_url,
    pv.local_image_path,

    p.name AS part_name,
    p.category_id,

    c.name AS color_name,
    c.rgb AS color_rgb,
    c.is_trans AS color_is_trans,

    COALESCE(upp.built_quantity,0) AS built_quantity

FROM set_parts sp

JOIN part_variants pv
    ON pv.element_id = sp.element_id

JOIN parts p
    ON p.part_num = pv.part_num

JOIN colors c
    ON c.id = pv.color_id

LEFT JOIN user_part_progress upp
    ON upp.set_num = sp.set_num
   AND upp.element_id = sp.element_id
   AND upp.is_spare = sp.is_spare"#;

    pub async fn replace_for_set(&self, set_num: &str, parts: &[DbSetPart], tx: &mut Transaction<'_, Sqlite>) -> Result<()> {

        sqlx::query(
            r#"
            DELETE FROM set_parts
            WHERE set_num = ?;
            "#
        )
            .bind(set_num)
            .execute(&mut **tx)
            .await?;

        for part in parts {
            sqlx::query(
                r#"
                INSERT INTO set_parts (
                    set_num,
                    element_id,
                    quantity,
                    is_spare
                )
                VALUES (?, ?, ?, ?)

                ON CONFLICT(set_num, element_id, is_spare)
                DO UPDATE SET
                    quantity = quantity + excluded.quantity;
                "#,
            )
            .bind(set_num)
            .bind(&part.element_id)
            .bind(&part.quantity)
            .bind(&part.is_spare)
            .execute(&mut **tx)
            .await?;
        }

        Ok(())
    }

    pub async fn get_for_set(&self, set_num: &str) -> Result<Vec<DbSetPart>> {
        let parts = sqlx::query_as::<_, DbSetPart>(
            r#"
            SELECT * FROM set_parts
            WHERE set_num = ?;
            "#,
        )
        .bind(set_num)
        .fetch_all(&self.pool)
        .await?;

        Ok(parts)
    }

    pub async fn delete_for_set(&self, set_num: &str, tx: &mut Transaction<'_, Sqlite>) -> Result<()> {
        sqlx::query(
            r#"
            DELETE FROM set_parts
            WHERE set_num = ?;
            "#,
        )
        .bind(set_num)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    pub async fn count(&self, set_num: &str) -> Result<u32> {
        let count: i64 = sqlx::query_scalar(
            r#"
            SELECT COUNT(*)
            FROM set_parts
            WHERE set_num = ?;
            "#,
        )
        .bind(set_num)
        .fetch_one(&self.pool)
        .await?;

        Ok(count as u32)
    }

    pub async fn count_parts_quantity(&self, set_num: &str) -> Result<u32> {
        let count: Option<i64> = sqlx::query_scalar(
            r#"
            SELECT SUM(quantity)
            FROM set_parts
            WHERE set_num = ?;
            "#,
        )
        .bind(set_num)
        .fetch_one(&self.pool)
        .await?;

        Ok(count.unwrap_or(0) as u32)
    }

    pub async fn find_complete_by_element_id(
        &self,
        element_id: &str,
    ) -> Result<Option<DbCompletePart>> {
        let sql = format!(
            r#"
            {}
            WHERE sp.element_id = ?
            LIMIT 1;
            "#,
            Self::COMPLETE_PART_SELECT
        );

        let part = sqlx::query_as::<_, DbCompletePart>(&sql)
            .bind(element_id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(part)
    }

    pub async fn find_complete_for_set(
        &self,
        set_num: &str,
    ) -> Result<Vec<DbCompletePart>> {
        let sql = format!(
            r#"
            {}
            WHERE sp.set_num = ?
            ORDER BY p.name;
            "#,
            Self::COMPLETE_PART_SELECT
        );

        let parts = sqlx::query_as::<_, DbCompletePart>(&sql)
            .bind(set_num)
            .fetch_all(&self.pool)
            .await?;

        Ok(parts)
    }

    pub async fn search_complete_for_set(
        &self,
        set_num: &str,
        search: &str,
    ) -> Result<Vec<DbCompletePart>> {

        let search = format!("%{}%", search.to_lowercase());

        let sql = format!(
            r#"
            {}
            WHERE sp.set_num = ?
              AND (
                    LOWER(p.name) LIKE ?
                 OR LOWER(p.part_num) LIKE ?
                 OR LOWER(sp.element_id) LIKE ?
              )
            ORDER BY p.name;
    "#,
            Self::COMPLETE_PART_SELECT
        );

        let parts = sqlx::query_as::<_, DbCompletePart>(&sql)
            .bind(set_num)
            .bind(&search)
            .bind(&search)
            .bind(&search)
            .fetch_all(&self.pool)
            .await?;

        Ok(parts)
    }

    pub async fn find_missing_for_set(
        &self,
        set_num: &str,
    ) -> Result<Vec<DbCompletePart>> {

        let sql = format!(
            r#"
            {}
            WHERE sp.set_num = ?
              AND COALESCE(up.collected_quantity, 0) < sp.quantity
            ORDER BY p.name;
            "#,
            Self::COMPLETE_PART_SELECT
        );

        let parts = sqlx::query_as::<_, DbCompletePart>(&sql)
            .bind(set_num)
            .fetch_all(&self.pool)
            .await?;

        Ok(parts)
    }

    pub async fn find_completed_for_set(
        &self,
        set_num: &str,
    ) -> Result<Vec<DbCompletePart>> {

        let sql = format!(
            r#"
            {}
            WHERE sp.set_num = ?
            AND COALESCE(up.collected_quantity,0) >= sp.quantity
            ORDER BY p.name;
            "#, Self::COMPLETE_PART_SELECT
        );

        let parts = sqlx::query_as::<_, DbCompletePart>(&sql)
            .bind(set_num)
            .fetch_all(&self.pool)
            .await?;

        Ok(parts)
    }
}
