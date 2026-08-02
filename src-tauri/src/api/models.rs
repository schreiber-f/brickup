use serde::Deserialize;
use serde::Serialize;
use strum::Display;

#[derive(Debug, Deserialize)]
pub struct SearchResponse {
    pub count: u32,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<SetSummary>,
}

#[derive(Debug, Deserialize)]
pub struct SetSummary {
    pub set_num: String,
    pub name: String,
    pub year: u16,
    pub theme_id: u32,
    pub num_parts: u32,

    pub set_img_url: Option<String>,

    pub set_url: String,

    pub last_modified_dt: String,
}


#[derive(Debug, Serialize, Display)]
pub enum Ordering {
    #[serde(rename = "year")]
    Year,

    #[serde(rename = "-year")]
    YearDesc,

    #[serde(rename = "num_parts")]
    Parts,

    #[serde(rename = "-num_parts")]
    PartsDesc,

    #[serde(rename = "name")]
    Name,

    #[serde(rename = "-name")]
    NameDesc,
}

#[derive(Debug, Serialize)]
pub struct SearchSetsRequest {
    pub search: Option<String>,
    pub page: u32,
    pub page_size: u32,

    pub theme_id: Option<u32>,

    pub min_year: Option<u16>,
    pub max_year: Option<u16>,

    pub min_parts: Option<u32>,
    pub max_parts: Option<u32>,

    pub ordering: Option<Ordering>,
}

#[derive(Debug, Deserialize)]
pub struct SetPartsResponse {
    pub count: u32,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<SetPart>,
}

#[derive(Debug, Deserialize)]
pub struct SetPart {
    pub set_num: String,

    pub quantity: u32,

    pub is_spare: bool,

    pub element_id: Option<String>,

    pub part: Part,

    pub color: Color,
}

#[derive(Debug, Deserialize)]
pub struct Part {
    pub part_num: String,

    pub name: String,

    pub part_cat_id: u32,

    pub part_img_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Color {
    pub id: u32,

    pub name: String,

    pub rgb: String,

    pub is_trans: bool,
}

pub struct ImportedSet {
    pub summary: SetSummary,
    pub parts: Vec<SetPart>,
}

pub struct ImportedImageData {
    pub set_num: String,
    pub set_image: Option<String>,
    pub part_images: Vec<(String,String)>,
}