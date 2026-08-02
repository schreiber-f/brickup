use anyhow::Result;

use crate::database::models::{
    DbUserSet,
    DbCompleteUserSet,
    BuildStatus,
};

use crate::database::repositories_bundle::Repositories;


#[derive(Clone)]
pub struct UserSetService {
    repos: Repositories,
}


impl UserSetService {

    pub fn new(
        repos: Repositories,
    ) -> Self {
        Self {
            repos,
        }
    }


    // -------------------------
    // Complete reads
    // -------------------------

    pub async fn get(
        &self,
        set_num: &str,
    ) -> Result<Option<DbCompleteUserSet>> {

        self.repos.user_sets
            .get_complete(set_num)
            .await
    }


    pub async fn get_all(
        &self,
    ) -> Result<Vec<DbCompleteUserSet>> {

        self.repos.user_sets
            .get_all_complete()
            .await
    }


    pub async fn search(
        &self,
        search: &str,
    ) -> Result<Vec<DbCompleteUserSet>> {

        self.repos.user_sets
            .search_complete(search)
            .await
    }


    pub async fn get_by_status(
        &self,
        status: &BuildStatus,
    ) -> Result<Vec<DbCompleteUserSet>> {

        self.repos.user_sets
            .get_complete_by_status(status)
            .await
    }



    // -------------------------
    // Basic operations
    // -------------------------


    pub async fn exists(
        &self,
        set_num: &str,
    ) -> Result<bool> {

        self.repos.user_sets
            .exists(set_num)
            .await
    }



    pub async fn get_raw(
        &self,
        set_num: &str,
    ) -> Result<Option<DbUserSet>> {

        self.repos.user_sets
            .get(set_num)
            .await
    }



    // -------------------------
    // User actions
    // -------------------------


    pub async fn update_status(
        &self,
        set_num: &str,
        status: BuildStatus,
    ) -> Result<()> {

        let mut tx = self.repos.pool.begin().await?;

        self.repos.user_sets
            .update_status(
                set_num,
                status,
                &mut tx,
            )
            .await?;

        tx.commit().await?;

        Ok(())
    }



    pub async fn delete(
        &self,
        set_num: &str,
    ) -> Result<()> {

        let mut tx = self.repos.pool.begin().await?;

        self.repos.user_sets
            .delete_complete(
                set_num,
                &mut tx,
            )
            .await?;

        tx.commit().await?;

        Ok(())
    }



    // -------------------------
    // Dashboard helpers
    // -------------------------


    pub async fn count_by_status(
        &self,
    ) -> Result<Vec<(String, i64)>> {

        self.repos.user_sets
            .count_by_status()
            .await
    }

}