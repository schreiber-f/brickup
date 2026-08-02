use crate::api::models::*;
use crate::database::models::*;
use std::collections::HashMap;
pub fn map_set(set: &SetSummary) -> DbSet {
    DbSet {
        set_num: set.set_num.clone(),
        name: set.name.clone(),
        year: set.year,
        theme_id: set.theme_id,
        num_parts: set.num_parts,

        remote_image_url: set.set_img_url.clone(),
        local_image_path: None,

        set_url: set.set_url.clone(),
        last_modified: set.last_modified_dt.clone(),
    }
}

pub fn map_part(part: &SetPart) -> DbPart {
    DbPart {
        part_num: part.part.part_num.clone(),
        name: part.part.name.clone(),
        category_id: part.part.part_cat_id,
    }
}

pub fn map_parts(parts: &[SetPart]) -> Vec<DbPart> {
    let mut unique = HashMap::new();

    for part in parts {
        unique.entry(part.part.part_num.clone())
            .or_insert(DbPart {
                part_num: part.part.part_num.clone(),
                name: part.part.name.clone(),
                category_id: part.part.part_cat_id,
            });
    }

    unique.into_values().collect()
}

pub fn map_color(part: &SetPart) -> DbColor {
    DbColor {
        id: part.color.id,
        name: part.color.name.clone(),
        rgb: part.color.rgb.clone(),
        is_trans: part.color.is_trans,
    }
}

pub fn map_colors(parts: &[SetPart]) -> Vec<DbColor> {
    let mut unique = HashMap::new();

    for part in parts {
        unique.entry(part.color.id)
            .or_insert(DbColor {
                id: part.color.id,
                name: part.color.name.clone(),
                rgb: part.color.rgb.clone(),
                is_trans: part.color.is_trans,
            });
    }

    unique.into_values().collect()
}

pub fn map_part_variant(
    part: &SetPart,
) -> Option<DbPartVariant> {
    let element_id = part.element_id.clone()?;

    Some(DbPartVariant {
        element_id,

        part_num: part.part.part_num.clone(),

        color_id: part.color.id,

        remote_image_url: part.part.part_img_url.clone(),
        local_image_path: None,
    })
}

pub fn map_part_variants(parts: &[SetPart]) -> Vec<DbPartVariant> {
    let mut unique = HashMap::new();

    for part in parts {
        if let Some(element_id) = &part.element_id {
            unique.entry(element_id.clone())
                .or_insert(DbPartVariant {
                    element_id: element_id.clone(),
                    part_num: part.part.part_num.clone(),
                    color_id: part.color.id,
                    remote_image_url: part.part.part_img_url.clone(),
                    local_image_path: None,
                });
        }
    }

    unique.into_values().collect()
}

pub fn map_user_part_progress(
    part: &SetPart,
) -> Option<DbUserPartProgress> {
    let element_id = part.element_id.clone()?;

    Some(DbUserPartProgress {
        set_num: part.set_num.clone(),

        element_id,

        built_quantity: 0,

        is_spare: part.is_spare,
    })
}

pub fn map_user_progress(
    parts: &[SetPart],
) -> Vec<DbUserPartProgress> {
    let mut unique: HashMap<(String, bool), DbUserPartProgress> = HashMap::new();

    for part in parts {
        let Some(element_id) = &part.element_id else {
            continue;
        };

        let key = (
            element_id.clone(),
            part.is_spare,
        );

        unique
            .entry(key)
            .and_modify(|existing| {
                existing.built_quantity += part.quantity;
            })
            .or_insert(DbUserPartProgress {
                set_num: part.set_num.clone(),
                element_id: element_id.clone(),
                built_quantity: 0,
                is_spare: part.is_spare,
            });
    }

    unique.into_values().collect()
}

pub fn map_user_set(
    set: &SetSummary,
) -> DbUserSet {
    DbUserSet {
        set_num: set.set_num.clone(),

        status: BuildStatus::Planned,

        added_at: chrono::Utc::now().to_rfc3339(),

        started_at: None,

        completed_at: None,
    }
}

pub fn map_set_part(
    part: &SetPart,
) -> Option<DbSetPart> {
    let element_id = part.element_id.clone()?;

    Some(DbSetPart {
        set_num: part.set_num.clone(),

        element_id,

        quantity: part.quantity,

        is_spare: part.is_spare,
    })
}

pub fn map_set_parts(parts: &[SetPart]) -> Vec<DbSetPart> {
    let mut unique: HashMap<(String, bool), DbSetPart> = HashMap::new();

    for part in parts {
        let Some(element_id) = &part.element_id else {
            continue;
        };

        let key = (
            element_id.clone(),
            part.is_spare,
        );

        unique
            .entry(key)
            .and_modify(|existing| {
                existing.quantity += part.quantity;
            })
            .or_insert(DbSetPart {
                set_num: part.set_num.clone(),
                element_id: element_id.clone(),
                quantity: part.quantity,
                is_spare: part.is_spare,
            });
    }

    unique.into_values().collect()
}

