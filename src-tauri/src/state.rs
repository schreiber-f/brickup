use crate::services::{
    set_service::SetService,
    import_service::ImportService,
    image_service::ImageService,
    part_service::PartService,
    user_set_service::UserSetService,
    user_part_progress_service::UserPartProgressService,
};

#[derive(Clone)]
pub struct AppState {
    pub set_service: SetService,
    pub import_service: ImportService,
    pub image_service: ImageService,
    pub part_service: PartService,
    pub user_set_service: UserSetService,
    pub progress_service: UserPartProgressService,
}