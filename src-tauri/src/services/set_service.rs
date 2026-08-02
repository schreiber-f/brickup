use crate::api::client::RebrickableClient;
use crate::api::models::{SearchResponse, SearchSetsRequest, SetPart, SetSummary, ImportedSet};
use anyhow::Result;
use crate::database::database::Database;
use crate::database::models::{DbSet, DbSetPart};
use crate::database::repositories_bundle::Repositories;

#[derive(Clone)]
pub struct SetService {
    client: RebrickableClient,
    repos: Repositories,
    database: Database
}

impl SetService {

    pub fn new(client: RebrickableClient, repos: Repositories, database: Database) -> Self {
        Self {
            client,
            repos,
            database
        }
    }

    // -------------------------
    // API
    // -------------------------

    pub async fn search(
        &self,
        request: &SearchSetsRequest,
    ) -> Result<SearchResponse> {

        self.client.search_sets(request).await

    }

    pub async fn get_details(
        &self,
        set_num: &str,
    ) -> Result<SetSummary> {

        self.client.get_set(set_num).await

    }

    pub async fn get_parts(
        &self,
        set_num: &str,
    ) -> Result<Vec<SetPart>> {

        self.client.get_set_parts(set_num).await

    }

    // -------------------------
    // Database
    // -------------------------


    pub async fn get_imported_set(
        &self,
        set_num: &str,
    ) -> Result<Option<DbSet>> {

        let set = self.repos
            .sets
            .get(set_num)
            .await?;

        Ok(set)
    }


    pub async fn is_imported(
        &self,
        set_num: &str,
    ) -> Result<bool> {

        let exists = self.repos
            .sets
            .get(set_num)
            .await?;

        if exists.is_none() {
            Ok(false)
        } else {
            Ok(true)
        }
    }

    
    pub async fn delete_set(
        &self,
        set_num: &str,
    ) -> Result<()> {

        let mut tx = self.database.pool().begin().await?;

        self.repos
            .sets
            .delete(set_num, &mut tx)
            .await?;

        tx.commit().await?;

        Ok(())
    }

}