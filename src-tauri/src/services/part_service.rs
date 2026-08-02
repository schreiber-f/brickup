use anyhow::Result;

use crate::database::models::DbCompletePart;
use crate::database::repositories_bundle::Repositories;

#[derive(Clone)]
pub struct PartService {
    repos: Repositories,
}

impl PartService {
    pub fn new(repos: Repositories) -> Self {
        Self { repos }
    }

    // ---------- Single ----------

    pub async fn get(
        &self,
        element_id: &str,
    ) -> Result<Option<DbCompletePart>> {
        self.repos
            .set_parts
            .find_complete_by_element_id(element_id)
            .await
    }

    // ---------- All ----------

    pub async fn get_for_set(
        &self,
        set_num: &str,
    ) -> Result<Vec<DbCompletePart>> {
        self.repos
            .set_parts
            .find_complete_for_set(set_num)
            .await
    }

    // ---------- Search ----------

    pub async fn search(
        &self,
        set_num: &str,
        search: &str,
    ) -> Result<Vec<DbCompletePart>> {

        if search.trim().is_empty() {
            return self.get_for_set(set_num).await;
        }

        self.repos
            .set_parts
            .search_complete_for_set(set_num, search)
            .await
    }

    // ---------- Progress ----------

    pub async fn get_missing(
        &self,
        set_num: &str,
    ) -> Result<Vec<DbCompletePart>> {
        self.repos
            .set_parts
            .find_missing_for_set(set_num)
            .await
    }

    pub async fn get_completed(
        &self,
        set_num: &str,
    ) -> Result<Vec<DbCompletePart>> {
        self.repos
            .set_parts
            .find_completed_for_set(set_num)
            .await
    }

    // ---------- Counts ----------

    pub async fn count(
        &self,
        set_num: &str,
    ) -> Result<u32> {
        self.repos
            .set_parts
            .count(set_num)
            .await
    }

    pub async fn count_quantity(
        &self,
        set_num: &str,
    ) -> Result<u32> {
        self.repos
            .set_parts
            .count_parts_quantity(set_num)
            .await
    }
}