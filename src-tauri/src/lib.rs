pub mod api;
pub mod services;
pub mod database;
pub mod mapper;
pub mod state;
pub mod init;
mod commands;

use init::initialize;
use state::AppState;


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {

    tauri::async_runtime::block_on(async {

        let state: AppState = initialize()
            .await
            .expect("Failed initializing application");


        tauri::Builder::default()
            .plugin(
                tauri_plugin_opener::init()
            )
            .manage(state)
            .invoke_handler(
                tauri::generate_handler![
                    commands::import::import_set,
                ]
            )
            .run(
                tauri::generate_context!()
            )
            .expect(
                "error while running tauri application"
            );

    });
}