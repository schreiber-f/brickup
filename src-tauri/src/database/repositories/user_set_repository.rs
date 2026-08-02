use anyhow::Result;
use sqlx::{Sqlite, SqlitePool, Transaction};
use crate::database::models::{DbUserSet, BuildStatus, DbCompleteUserSet};
#[derive(Clone)]
pub struct UserSetRepository {
    pool: SqlitePool,
}

impl UserSetRepository {
    pub fn new(pool: &SqlitePool) -> Self {
        Self {
            pool: pool.clone(),
        }
    }

    const COMPLETE_USER_SET_SELECT: &str = r#"SELECT
    us.set_num,

    s.name,
    s.year,
    s.theme_id,
    s.num_parts,

    s.remote_image_url,
    s.local_image_path,

    us.status,
    us.added_at,
    us.started_at,
    us.completed_at,

    COALESCE(SUM(upp.built_quantity),0) AS built_quantity,

    COALESCE(SUM(sp.quantity),0) AS total_quantity

FROM user_sets us

JOIN sets s
    ON s.set_num = us.set_num

LEFT JOIN set_parts sp
    ON sp.set_num = us.set_num

LEFT JOIN user_part_progress upp
    ON upp.set_num = sp.set_num
   AND upp.element_id = sp.element_id
   AND upp.is_spare = sp.is_spare
    "#;

    pub async fn upsert(
        &self,
        user_set: &DbUserSet,
        tx: &mut Transaction<'_, Sqlite>
    ) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO user_sets (
                set_num,
                status,
                added_at,
                started_at,
                completed_at
            )
            VALUES (?, ?, ?, ?, ?)

            ON CONFLICT(set_num)
            DO UPDATE SET
                status = excluded.status,
                added_at = excluded.added_at,
                started_at = excluded.started_at,
                completed_at = excluded.completed_at;

            "#
        )
        .bind(&user_set.set_num)
        .bind(user_set.status.as_str())
        .bind(&user_set.added_at)
        .bind(&user_set.started_at)
        .bind(&user_set.completed_at)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    pub async fn update_status(
        &self,
        set_num: &str,
        status: BuildStatus,
        tx: &mut Transaction<'_, Sqlite>
    ) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE user_sets
            SET status = ?
            WHERE set_num = ?;
            "#
        )
        .bind(status.as_str())
        .bind(set_num)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    pub async fn get(&self, set_num: &str) -> Result<Option<DbUserSet>> {
        let user_set = sqlx::query_as::<_, DbUserSet>(
            r#"
            SELECT * FROM user_sets
            WHERE set_num = ?;
            "#
        ).bind(set_num)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user_set)
    }

    pub async fn get_all(&self) -> Result<Vec<DbUserSet>> {
        let user_sets = sqlx::query_as::<_, DbUserSet>(
            r#"
            SELECT * FROM user_sets
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(user_sets)
    }

    pub async fn get_by_status(
        &self,
        status: &BuildStatus,
    ) -> Result<Vec<DbUserSet>> {
        let user_sets = sqlx::query_as::<_, DbUserSet>(
            r#"
            SELECT * FROM user_sets
            WHERE status = ?
            "#
        )
        .bind(status.as_str())
        .fetch_all(&self.pool)
        .await?;

        Ok(user_sets)
    }

    pub async fn delete(&self, set_num: &str, tx: &mut Transaction<'_, Sqlite>) -> Result<()> {
        sqlx::query(
            r#"
            DELETE FROM user_sets
            WHERE set_num = ?;
            "#
        )
        .bind(set_num)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    pub async fn get_complete(
        &self,
        set_num: &str,
    ) -> Result<Option<DbCompleteUserSet>> {

        let sql = format!(
            r#"
        {}
        WHERE us.set_num = ?
        GROUP BY us.set_num;
        "#,
            Self::COMPLETE_USER_SET_SELECT
        );

        let result = sqlx::query_as::<_, DbCompleteUserSet>(&sql)
            .bind(set_num)
            .fetch_optional(&self.pool)
            .await?;

        Ok(result)
    }

    pub async fn get_all_complete(
        &self,
    ) -> Result<Vec<DbCompleteUserSet>> {

        let sql = format!(
            r#"
        {}
        GROUP BY us.set_num
        ORDER BY us.added_at DESC;
        "#,
            Self::COMPLETE_USER_SET_SELECT
        );


        let sets = sqlx::query_as::<_, DbCompleteUserSet>(&sql)
            .fetch_all(&self.pool)
            .await?;

        Ok(sets)
    }

    pub async fn get_complete_by_status(
        &self,
        status: &BuildStatus,
    ) -> Result<Vec<DbCompleteUserSet>> {

        let sql = format!(
            r#"
        {}
        WHERE us.status = ?
        GROUP BY us.set_num
        ORDER BY us.added_at DESC;
        "#,
            Self::COMPLETE_USER_SET_SELECT
        );


        let sets = sqlx::query_as::<_, DbCompleteUserSet>(&sql)
            .bind(status.as_str())
            .fetch_all(&self.pool)
            .await?;

        Ok(sets)
    }

    pub async fn search_complete(
        &self,
        search: &str,
    ) -> Result<Vec<DbCompleteUserSet>> {

        let search = format!("%{}%", search.to_lowercase());


        let sql = format!(
            r#"
        {}
        WHERE
            LOWER(us.set_num) LIKE ?
            OR LOWER(s.name) LIKE ?
        GROUP BY us.set_num
        ORDER BY s.name;
        "#,
            Self::COMPLETE_USER_SET_SELECT
        );


        let sets = sqlx::query_as::<_, DbCompleteUserSet>(&sql)
            .bind(&search)
            .bind(&search)
            .fetch_all(&self.pool)
            .await?;


        Ok(sets)
    }

    pub async fn exists(
        &self,
        set_num: &str,
    ) -> Result<bool> {

        let exists: i64 = sqlx::query_scalar(
            r#"
        SELECT EXISTS(
            SELECT 1
            FROM user_sets
            WHERE set_num = ?
        );
        "#
        )
            .bind(set_num)
            .fetch_one(&self.pool)
            .await?;

        Ok(exists == 1)
    }

    pub async fn count_by_status(
        &self,
    ) -> Result<Vec<(String,i64)>> {

        let result = sqlx::query_as::<_, (String,i64)>(
            r#"
        SELECT
            status,
            COUNT(*)
        FROM user_sets
        GROUP BY status;
        "#
        )
            .fetch_all(&self.pool)
            .await?;

        Ok(result)
    }

    pub async fn update_status_with_dates(
        &self,
        set_num: &str,
        status: BuildStatus,
        started_at: Option<String>,
        completed_at: Option<String>,
        tx: &mut Transaction<'_, Sqlite>
    ) -> Result<()> {

        sqlx::query(
            r#"
        UPDATE user_sets
        SET
            status = ?,
            started_at = COALESCE(?, started_at),
            completed_at = COALESCE(?, completed_at)
        WHERE set_num = ?;
        "#
        )
            .bind(status.as_str())
            .bind(started_at)
            .bind(completed_at)
            .bind(set_num)
            .execute(&mut **tx)
            .await?;

        Ok(())
    }

    pub async fn delete_complete(
        &self,
        set_num: &str,
        tx: &mut Transaction<'_, Sqlite>
    ) -> Result<()> {

        sqlx::query(
            r#"
        DELETE FROM user_part_progress
        WHERE set_num = ?;
        "#
        )
            .bind(set_num)
            .execute(&mut **tx)
            .await?;


        sqlx::query(
            r#"
        DELETE FROM user_sets
        WHERE set_num = ?;
        "#
        )
            .bind(set_num)
            .execute(&mut **tx)
            .await?;

        Ok(())
    }
}