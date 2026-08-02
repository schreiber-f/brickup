use anyhow::Result;
use directories::ProjectDirs;

use crate::{
    api::client::RebrickableClient,
    database::{
        database::Database,
        schema,
        repositories_bundle::Repositories,
    },
    services::{
        image_service::ImageService,
        set_service::SetService,
        import_service::ImportService,
        part_service::PartService,
        user_set_service::UserSetService,
        user_part_progress_service::UserPartProgressService,
    },
    state::AppState,
};


pub async fn initialize() -> Result<AppState> {
    dotenvy::dotenv().ok();

    let dirs = ProjectDirs::from(
        "com",
        "BrickUp",
        "BrickUp",
    )
        .expect("Could not determine app directory");


    let data_dir = dirs.data_dir();

    tokio::fs::create_dir_all(data_dir)
        .await?;


    let db_path = data_dir.join("brickup.db");


    let database_url = format!(
        "sqlite://{}",
        db_path.display()
    );


    println!(
        "Database: {}",
        database_url
    );


    let database = Database::new(
        &database_url
    )
        .await?;


    schema::initialize(
        database.pool()
    )
        .await?;


    let repos = Repositories::new(
        database.pool()
    );


    let api_key = std::env::var(
        "REBRICKABLE_API_KEY"
    )
        .expect("Missing REBRICKABLE_API_KEY");


    let client = RebrickableClient::new(
        api_key
    );


    let set_service = SetService::new(
        client.clone(),
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
        image_service.clone(),
    );


    let part_service = PartService::new(
        repos.clone()
    );


    let user_set_service = UserSetService::new(
        repos.clone(),
    );


    let progress_service =
        UserPartProgressService::new(
            repos.clone(),
            database.clone(),
        );


    Ok(AppState {
        set_service,
        import_service,
        image_service,
        part_service,
        user_set_service,
        progress_service,
    })
}