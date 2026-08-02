use anyhow::{Context, Result};
use directories::ProjectDirs;
use reqwest::Client;
use std::path::{Path, PathBuf};
use tokio::fs;
use futures::stream::{self, StreamExt};
use crate::database::repositories_bundle::Repositories;

#[derive(Clone)]
pub struct ImageService {
    client: Client,
    root: PathBuf,
    repos: Repositories,
}

impl ImageService {
    pub async fn new(repos: Repositories,) -> Result<Self> {
        let dirs = ProjectDirs::from(
            "com",
            "BrickUp",
            "BrickUp",
        )
            .context("Could not determine app data directory")?;

        let root = dirs.data_dir().join("images");

        fs::create_dir_all(root.join("sets")).await?;
        fs::create_dir_all(root.join("parts")).await?;

        println!("Image directory: {}", root.display());

        Ok(Self {
            client: Client::new(),
            repos,
            root,
        })
    }

    pub async fn cache_set_image(
        &self,
        set_num: &str,
        url: &str,
    ) -> Result<PathBuf> {

        let path = self
            .root
            .join("sets")
            .join(format!("{set_num}.jpg"));


        self.download_if_missing(url, &path)
            .await?;


        self.repos.sets
            .update_image_path(
                set_num,
                path.to_string_lossy().as_ref(),
            )
            .await?;


        Ok(path)
    }

    pub async fn cache_part_image(
        &self,
        element_id: &str,
        url: &str,
    ) -> Result<PathBuf> {

        let path = self
            .root
            .join("parts")
            .join(format!("{element_id}.jpg"));


        self.download_if_missing(url, &path)
            .await?;


        self.repos.variants
            .update_image_path(
                element_id,
                path.to_string_lossy().as_ref(),
            )
            .await?;


        Ok(path)
    }

    async fn download_if_missing(
        &self,
        url: &str,
        path: &Path,
    ) -> Result<()> {
        if fs::try_exists(path).await? {
            return Ok(());
        }

        let bytes = self.client
            .get(url)
            .send()
            .await?
            .error_for_status()?
            .bytes()
            .await?;

        fs::write(path, bytes).await?;

        Ok(())
    }

    pub async fn cache_images_for_set(
        &self,
        set_num: &str,
        set_image: Option<String>,
        parts: Vec<(String, String)>,
    ) -> Result<()> {

        println!("Starting image cache for {}", set_num);

        if let Some(url) = set_image {
            self.cache_set_image(set_num, &url)
                .await?;
        }

        stream::iter(parts)
            .map(|(element_id, url)| {
                let service = self.clone();

                async move {
                    if let Err(err) = service
                        .cache_part_image(
                            &element_id,
                            &url,
                        )
                        .await
                    {
                        eprintln!(
                            "Image download failed {}: {}",
                            element_id,
                            err
                        );
                    }
                }
            })
            .buffer_unordered(10)
            .collect::<Vec<_>>()
            .await;


        println!("Finished image cache for {}", set_num);

        Ok(())
    }
}