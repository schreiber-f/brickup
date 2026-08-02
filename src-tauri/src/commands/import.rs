use tauri::State;

use crate::state::AppState;

#[tauri::command]
pub async fn import_set(
    state: State<'_, AppState>,
    set_num: String,
) -> Result<(), String> {

    state
        .import_service
        .import_set_complete(&set_num)
        .await
        .map_err(|e| e.to_string())
}