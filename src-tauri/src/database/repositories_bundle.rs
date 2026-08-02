use sqlx::SqlitePool;

use crate::database::repositories::{
    color_repository::ColorRepository,
    part_repository::PartRepository,
    part_variant_repository::PartVariantRepository,
    set_part_repository::SetPartRepository,
    set_repository::SetRepository,
    user_part_progress_repository::UserPartProgressRepository,
    user_set_repository::UserSetRepository,
};

#[derive(Clone)]
pub struct Repositories {
    pub pool: SqlitePool,
    
    pub parts: PartRepository,
    pub colors: ColorRepository,
    pub variants: PartVariantRepository,
    pub sets: SetRepository,
    pub set_parts: SetPartRepository,
    pub user_sets: UserSetRepository,
    pub progress: UserPartProgressRepository,
}

impl Repositories {
    pub fn new(pool: &SqlitePool) -> Self {
        Self {
            pool: pool.clone(),
            parts: PartRepository::new(pool),
            colors: ColorRepository::new(pool),
            variants: PartVariantRepository::new(pool),
            sets: SetRepository::new(pool),
            set_parts: SetPartRepository::new(pool),
            user_sets: UserSetRepository::new(pool),
            progress: UserPartProgressRepository::new(pool),
        }
    }
}