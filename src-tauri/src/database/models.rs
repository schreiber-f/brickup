use strum::Display;
use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
pub enum BuildStatus{
    Planned,
    Building,
    Completed,
    Paused,
}
impl BuildStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Planned => "planned",
            Self::Building => "building",
            Self::Paused => "paused",
            Self::Completed => "completed",
        }
    }
}
// 1. Erstelle die Konvertierung von String zu Enum
impl TryFrom<String> for BuildStatus {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "planned" => Ok(Self::Planned),
            "building" => Ok(Self::Building),
            "completed" => Ok(Self::Completed),
            "paused" => Ok(Self::Paused),
            _ => Err(format!("Unbekannter Status: {}", value)),
        }
    }
}


#[derive(Debug, Clone, sqlx::FromRow)]
pub struct DbSet {
    pub set_num: String,
    pub name: String,
    pub year: u16,
    pub theme_id: u32,
    pub num_parts: u32,

    pub remote_image_url: Option<String>,
    pub local_image_path: Option<String>,

    pub set_url: String,
    pub last_modified: String,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct DbPart {
    pub part_num: String,
    pub name: String,
    pub category_id: u32,
}

#[derive(Debug, Clone,  sqlx::FromRow)]
pub struct DbColor {
    pub id: u32,
    pub name: String,
    pub rgb: String,
    pub is_trans: bool,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct DbPartVariant {
    pub element_id: String,

    pub part_num: String,

    pub color_id: u32,

    pub remote_image_url: Option<String>,
    pub local_image_path: Option<String>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct DbSetPart {
    pub set_num: String,

    pub element_id: String,

    pub quantity: u32,

    pub is_spare: bool,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct DbUserSet {
    pub set_num: String,

    #[sqlx(try_from = "String")]
    pub status: BuildStatus,

    pub added_at: String,

    pub started_at: Option<String>,

    pub completed_at: Option<String>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct DbUserPartProgress {
    pub set_num: String,

    pub element_id: String,

    pub built_quantity: u32,

    pub is_spare: bool,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct DbCompletePart {
    pub set_num: String,

    pub element_id: String,

    // part variant
    pub part_num: String,
    pub color_id: i32,

    // part
    pub part_name: String,
    pub category_id: i32,

    // color
    pub color_name: String,
    pub color_rgb: String,
    pub color_is_trans: bool,

    // images
    pub remote_image_url: Option<String>,
    pub local_image_path: Option<String>,

    // set relation
    pub quantity: i32,
    pub is_spare: bool,

    // progress
    pub built_quantity: i32,
}
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct DbCompleteUserSet {
    pub set_num: String,

    pub name: String,
    pub year: i32,
    pub theme_id: i32,
    pub num_parts: i32,

    pub remote_image_url: Option<String>,
    pub local_image_path: Option<String>,

    // user state
    pub status: String,

    pub added_at: String,
    pub started_at: Option<String>,
    pub completed_at: Option<String>,

    // calculated
    pub built_quantity: i32,
    pub total_quantity: i32,
}