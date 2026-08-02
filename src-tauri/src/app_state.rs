use crate::services::{
    set_service::SetService,
    import_service::ImportService,
    image_service::ImageService,
};

pub struct AppState {
    pub set_service: SetService,
    pub import_service: ImportService,
    pub image_service: ImageService,
}