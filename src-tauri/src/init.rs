use anyhow::Result;
use crate::{
    app_state::AppState,
    api::client::RebrickableClient,
    database::database::Database,
    database::repositories_bundle::Repositories,
    services::{
        set_service::SetService,
        import_service::ImportService,
        image_service::ImageService,
    },
};
use dotenvy;


pub fn load_api_key() -> String{
    dotenvy::dotenv().ok();

    let api_key =
        std::env::var("REBRICKABLE_API_KEY")
            .expect("API Key fehlt");

    api_key
}

pub async fn create_app_state() -> Result<AppState> {

    let database = Database::new(
        "sqlite:brickup.db"
    )
        .await?;


    let repos = Repositories::new(
        database.pool()
    );


    let client = RebrickableClient::new(
        load_api_key()
    );


    let set_service = SetService::new(
        client,
        repos.clone(),
        database.clone()
    );


    let image_service = ImageService::new(
        repos.clone()
    )
        .await?;


    let import_service = ImportService::new(
        set_service.clone(),
        repos.clone(),
        database.clone(),
        image_service.clone()
    );


    Ok(AppState {
        set_service,
        import_service,
        image_service,
    })
}