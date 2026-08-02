use reqwest::Client;
use anyhow::Result;
use crate::api::models::{SearchResponse, SearchSetsRequest, SetSummary, SetPartsResponse, SetPart};

#[derive(Clone)]
pub struct RebrickableClient {
    client: Client,
    base_url: String,
    api_key: String,
}

impl RebrickableClient {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            base_url: "https://rebrickable.com/api/v3".into(),
            api_key,
        }
    }

    pub async fn test_connection(&self) -> Result<()> {

        let url = format!("{}/swagger/", self.base_url);

        let response = self.client
            .get("https://rebrickable.com/api/v3/swagger/")
            .send()
            .await?;

        println!("{}", response.status());

        Ok(())
    }

    pub async fn search_sets(&self, request: &SearchSetsRequest) -> Result<SearchResponse> {

        let url = format!("{}/lego/sets/", self.base_url);

        let mut query = Vec::new();
        query.push(("page", request.page.to_string()));
        query.push(("page_size", request.page_size.to_string()));
        if let Some(search) = &request.search {
            query.push(("search", search.clone()));
        }
        if let Some(theme) = request.theme_id {
            query.push(("theme_id", theme.to_string()));
        }
        if let Some(year) = request.min_year {
            query.push(("min_year", year.to_string()));
        }
        if let Some(year) = request.max_year {
            query.push(("max_year", year.to_string()));
        }
        if let Some(parts) = request.min_parts {
            query.push(("min_parts", parts.to_string()));
        }
        if let Some(parts) = request.max_parts {
            query.push(("max_parts", parts.to_string()));
        }
        if let Some(ordering) = &request.ordering {
            query.push(("ordering", ordering.to_string()));
        }

        let response = self
            .client
            .get(url)
            .header("Authorization", format!("key {}", self.api_key))
            .query(&query)
            .send()
            .await?;

        let result = response.json::<SearchResponse>().await?;

        Ok(result)
    }

    pub async fn get_set(
        &self,
        set_num: &str,
    ) -> Result<SetSummary> {
        let url = format!(
            "{}/lego/sets/{}/",
            self.base_url,
            set_num
        );

        let response = self.client
            .get(url)
            .header("Authorization", format!("key {}", self.api_key))
            .send()
            .await?;

        let result = response
            .json::<SetSummary>()
            .await?;

        Ok(result)
    }

    pub async fn get_set_parts(
        &self,
        set_num: &str,
    ) -> Result<Vec<SetPart>> {

        let mut all_parts = Vec::new();
        let mut page = 1;

        loop {
            let url = format!(
                "{}/lego/sets/{}/parts/",
                self.base_url,
                set_num
            );

            let response = self.client
                .get(&url)
                .header(
                    "Authorization",
                    format!("key {}", self.api_key)
                )
                .query(&[
                    ("page", page.to_string()),
                    ("page_size", "100000".to_string()),
                    ("inc_part_details", "1".to_string()),
                ])
                .send()
                .await?
                .error_for_status()?;

            let result = response
                .json::<SetPartsResponse>()
                .await?;

            all_parts.extend(result.results);

            match result.next {
                Some(_) => {
                    page += 1;
                }
                None => {
                    break;
                }
            }
        }

        Ok(all_parts)
    }
}