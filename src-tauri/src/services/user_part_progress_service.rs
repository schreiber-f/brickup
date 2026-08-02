use anyhow::Result;

use crate::database::database::Database;
use crate::database::models::DbUserPartProgress;
use crate::database::repositories_bundle::Repositories;


#[derive(Clone)]
pub struct UserPartProgressService {
    repos: Repositories,
    database: Database,
}


impl UserPartProgressService {

    pub fn new(
        repos: Repositories,
        database: Database,
    ) -> Self {
        Self {
            repos,
            database,
        }
    }


    pub async fn get(
        &self,
        set_num: &str,
        element_id: &str,
    ) -> Result<Vec<DbUserPartProgress>> {

        self.repos.progress
            .get(set_num, element_id)
            .await
    }


    pub async fn get_one(
        &self,
        set_num: &str,
        element_id: &str,
        is_spare: bool,
    ) -> Result<Option<DbUserPartProgress>> {

        self.repos.progress
            .get_one(
                set_num,
                element_id,
                is_spare,
            )
            .await
    }


    pub async fn get_for_set(
        &self,
        set_num: &str,
    ) -> Result<Vec<DbUserPartProgress>> {

        self.repos.progress
            .get_for_set(set_num)
            .await
    }


    pub async fn get_for_element(
        &self,
        element_id: &str,
    ) -> Result<Vec<DbUserPartProgress>> {

        self.repos.progress
            .get_for_element(element_id)
            .await
    }


    pub async fn get_progress(
        &self,
        set_num: &str,
    ) -> Result<(i64, i64)> {

        self.repos.progress
            .get_set_progress(set_num)
            .await
    }


    pub async fn count_missing(
        &self,
        set_num: &str,
    ) -> Result<i64> {

        self.repos.progress
            .count_missing_for_set(set_num)
            .await
    }


    pub async fn set_quantity(
        &self,
        set_num: &str,
        element_id: &str,
        is_spare: bool,
        quantity: u32,
    ) -> Result<()> {

        let mut tx = self.database
            .pool()
            .begin()
            .await?;


        let progress = DbUserPartProgress {
            set_num: set_num.to_string(),
            element_id: element_id.to_string(),
            built_quantity: quantity,
            is_spare,
        };


        self.repos.progress
            .upsert(
                &progress,
                &mut tx,
            )
            .await?;


        tx.commit()
            .await?;

        Ok(())
    }


    pub async fn increase(
        &self,
        set_num: &str,
        element_id: &str,
        is_spare: bool,
    ) -> Result<()> {

        self.change_quantity(
            set_num,
            element_id,
            is_spare,
            1,
        )
            .await
    }


    pub async fn decrease(
        &self,
        set_num: &str,
        element_id: &str,
        is_spare: bool,
    ) -> Result<()> {

        self.change_quantity(
            set_num,
            element_id,
            is_spare,
            -1,
        )
            .await
    }


    pub async fn change_quantity(
        &self,
        set_num: &str,
        element_id: &str,
        is_spare: bool,
        amount: i32,
    ) -> Result<()> {

        let mut tx = self.database
            .pool()
            .begin()
            .await?;


        self.repos.progress
            .change_quantity(
                set_num,
                element_id,
                is_spare,
                amount,
                &mut tx,
            )
            .await?;


        tx.commit()
            .await?;

        Ok(())
    }


    pub async fn reset_set(
        &self,
        set_num: &str,
    ) -> Result<()> {

        let mut tx = self.database
            .pool()
            .begin()
            .await?;


        self.repos.progress
            .reset_for_set(
                set_num,
                &mut tx,
            )
            .await?;


        tx.commit()
            .await?;

        Ok(())
    }


    pub async fn delete(
        &self,
        set_num: &str,
        element_id: &str,
        is_spare: bool,
    ) -> Result<()> {

        let mut tx = self.database
            .pool()
            .begin()
            .await?;


        self.repos.progress
            .delete(
                set_num,
                element_id,
                is_spare,
                &mut tx,
            )
            .await?;


        tx.commit()
            .await?;

        Ok(())
    }


    pub async fn delete_for_set(
        &self,
        set_num: &str,
    ) -> Result<()> {

        let mut tx = self.database
            .pool()
            .begin()
            .await?;


        self.repos.progress
            .delete_for_set(
                set_num,
                &mut tx,
            )
            .await?;


        tx.commit()
            .await?;

        Ok(())
    }
}