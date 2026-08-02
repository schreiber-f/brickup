use anyhow::Result;
use sqlx::{Sqlite, SqlitePool, Transaction};
use crate::database::models::{DbUserPartProgress};
#[derive(Clone)]
pub struct UserPartProgressRepository {
    pool: SqlitePool,
}

impl UserPartProgressRepository {
    pub fn new(pool: &SqlitePool) -> Self {
        Self {
            pool: pool.clone(),
        }
    }

    pub async fn upsert(&self, user_part_progress: &DbUserPartProgress, tx: &mut Transaction<'_, Sqlite>) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO user_part_progress (
                set_num,
                element_id,
                built_quantity,
                is_spare
            )
            VALUES (?, ?, ?, ?)

            ON CONFLICT(set_num, element_id, is_spare)
            DO UPDATE SET
                built_quantity = excluded.built_quantity;
            "#
        )
            .bind(&user_part_progress.set_num)
            .bind(&user_part_progress.element_id)
            .bind(&user_part_progress.built_quantity)
            .bind(&user_part_progress.is_spare)
            .execute(&mut **tx)
            .await?;

        Ok(())
    }

    pub async fn upsert_many(&self, user_part_progresses: &[DbUserPartProgress], tx: &mut Transaction<'_, Sqlite>) -> Result<()> {

        for user_part_progress in user_part_progresses {
            sqlx::query(
                r#"
            INSERT INTO user_part_progress (
                set_num,
                element_id,
                built_quantity,
                is_spare
            )
            VALUES (?, ?, ?, ?)

            ON CONFLICT(set_num, element_id, is_spare)
            DO UPDATE SET
                built_quantity = excluded.built_quantity;
            "#
            )
                .bind(&user_part_progress.set_num)
                .bind(&user_part_progress.element_id)
                .bind(&user_part_progress.built_quantity)
                .bind(&user_part_progress.is_spare)
                .execute(&mut **tx)
                .await?;
        }

        Ok(())
    }

    pub async fn get(&self, set_num: &str, element_id: &str) -> Result<Vec<DbUserPartProgress>> {
        let user_part_progress = sqlx::query_as::<_, DbUserPartProgress>(
            r#"
            SELECT * FROM user_part_progress
            WHERE set_num = ?
            AND element_id = ?
            "#
        )
        .bind(set_num)
        .bind(element_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(user_part_progress)
    }

    pub async fn get_one(&self, set_num: &str, element_id: &str,
                         is_spare: bool) -> Result<Option<DbUserPartProgress>> {
        let user_part_progress = sqlx::query_as::<_, DbUserPartProgress>(
            r#"
            SELECT * FROM user_part_progress
            WHERE set_num = ?
            AND element_id = ?
            AND is_spare = ?
            "#
        )
            .bind(set_num)
            .bind(element_id)
            .bind(is_spare)
            .fetch_optional(&self.pool)
            .await?;

        Ok(user_part_progress)
    }

    pub async fn get_for_set(&self, set_num: &str) -> Result<Vec<DbUserPartProgress>> {
        let user_part_progresses = sqlx::query_as::<_, DbUserPartProgress>(
            r#"
            SELECT * FROM user_part_progress
            WHERE set_num = ?
            "#
        )
            .bind(set_num)
            .fetch_all(&self.pool)
            .await?;

        Ok(user_part_progresses)
    }

    pub async fn delete(&self, set_num: &str, element_id: &str, is_spare: bool, tx: &mut Transaction<'_, Sqlite>) -> Result<()> {
        sqlx::query(
            r#"
            DELETE FROM user_part_progress
            WHERE set_num = ?
            AND element_id = ?
            AND is_spare = ?;
           "#
        )
        .bind(set_num)
        .bind(element_id)
        .bind(is_spare)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    pub async fn delete_for_set(&self, set_num: &str, tx: &mut Transaction<'_, Sqlite>) -> Result<()> {
        sqlx::query(
            r#"
            DELETE FROM user_part_progress
            WHERE set_num = ?
            "#
        )
        .bind(set_num)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    pub async fn get_set_progress(
        &self,
        set_num: &str,
    ) -> Result<(i64, i64)> {

        let result = sqlx::query_as::<_, (i64, i64)>(
            r#"
        SELECT
            COALESCE(SUM(upp.built_quantity),0),
            COALESCE(SUM(sp.quantity),0)

        FROM set_parts sp

        LEFT JOIN user_part_progress upp
            ON upp.set_num = sp.set_num
           AND upp.element_id = sp.element_id
           AND upp.is_spare = sp.is_spare

        WHERE sp.set_num = ?
        "#
        )
            .bind(set_num)
            .fetch_one(&self.pool)
            .await?;

        Ok(result)
    }

    pub async fn count_missing_for_set(
        &self,
        set_num: &str,
    ) -> Result<i64> {

        let count = sqlx::query_scalar(
            r#"
        SELECT COUNT(*)

        FROM set_parts sp

        LEFT JOIN user_part_progress upp
            ON upp.set_num = sp.set_num
           AND upp.element_id = sp.element_id
           AND upp.is_spare = sp.is_spare

        WHERE sp.set_num = ?
        AND COALESCE(upp.built_quantity,0) < sp.quantity
        "#
        )
            .bind(set_num)
            .fetch_one(&self.pool)
            .await?;

        Ok(count)
    }

    pub async fn change_quantity(
        &self,
        set_num: &str,
        element_id: &str,
        is_spare: bool,
        amount: i32,
        tx: &mut Transaction<'_, Sqlite>,
    ) -> Result<()> {

        sqlx::query(
            r#"
        UPDATE user_part_progress

        SET built_quantity =
            MAX(0, built_quantity + ?)

        WHERE set_num = ?
        AND element_id = ?
        AND is_spare = ?
        "#
        )
            .bind(amount)
            .bind(set_num)
            .bind(element_id)
            .bind(is_spare)
            .execute(&mut **tx)
            .await?;

        Ok(())
    }

    pub async fn reset_for_set(
        &self,
        set_num: &str,
        tx: &mut Transaction<'_, Sqlite>,
    ) -> Result<()> {

        sqlx::query(
            r#"
        UPDATE user_part_progress

        SET built_quantity = 0

        WHERE set_num = ?
        "#
        )
            .bind(set_num)
            .execute(&mut **tx)
            .await?;

        Ok(())
    }

    pub async fn get_for_element(
        &self,
        element_id: &str,
    ) -> Result<Vec<DbUserPartProgress>> {

        let progress = sqlx::query_as::<_, DbUserPartProgress>(
            r#"
        SELECT *
        FROM user_part_progress
        WHERE element_id = ?
        "#
        )
            .bind(element_id)
            .fetch_all(&self.pool)
            .await?;

        Ok(progress)
    }
}