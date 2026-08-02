use anyhow::Result;
use crate::api::models::ImportedImageData;
use crate::database::database::Database;
use crate::mapper::api_mapper::*;
use crate::services::set_service::SetService;
use crate::database::repositories_bundle::Repositories;
use crate::services::image_service::ImageService;

#[derive(Clone)]
pub struct ImportService {
    set_service: SetService,
    repos: Repositories,
    database: Database,
    image_service: ImageService,
}

impl ImportService {
    pub fn new(
        set_service: SetService,
        repos: Repositories,
        database: Database,
        image_service: ImageService,
    ) -> Self {
        Self {
            set_service,
            repos,
            database,
            image_service,
        }
    }

    pub async fn import_set(
        &self,
        set_num: &str,
    ) -> Result<ImportedImageData> {

        let summary = self.set_service
            .get_details(set_num)
            .await?;

        let parts = self.set_service
            .get_parts(set_num)
            .await?;

        let db_set = map_set(&summary);
        let db_parts = map_parts(&parts);
        let db_colors = map_colors(&parts);
        let db_variants = map_part_variants(&parts);
        let db_set_parts = map_set_parts(&parts);
        let db_progress = map_user_progress(&parts);
        let db_user_set = map_user_set(&summary);

        let mut tx = self.database.pool().begin().await?;

        self.repos.parts
            .upsert_many(&db_parts, &mut tx)
            .await?;
        self.repos.colors
            .upsert_many(&db_colors, &mut tx)
            .await?;
        self.repos.variants
            .upsert_many(&db_variants, &mut tx)
            .await?;
        self.repos.sets
            .upsert(&db_set, &mut tx)
            .await?;
        self.repos.set_parts
            .replace_for_set(&db_set.set_num, &db_set_parts, &mut tx)
            .await?;
        self.repos.user_sets
            .upsert(&db_user_set, &mut tx)
            .await?;
        self.repos.progress
            .upsert_many(&db_progress, &mut tx)
            .await?;

        tx.commit().await?;

        Ok(ImportedImageData {
            set_num: summary.set_num,
            set_image: summary.set_img_url,
            part_images: parts
                .into_iter()
                .filter_map(|p| {
                    match (
                        p.element_id,
                        p.part.part_img_url
                    ) {
                        (Some(id), Some(url)) =>
                            Some((id,url)),
                        _ => None
                    }
                })
                .collect(),
        })
    }

    pub async fn import_set_complete(
    &self,
    set_num: &str,
) -> Result<()> {

    let images = self
        .import_set(set_num)
        .await?;

    self.image_service
        .cache_images_for_set(
            &images.set_num,
            images.set_image,
            images.part_images,
        )
        .await?;

    Ok(())
}
}