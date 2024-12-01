pub mod utils;

#[tauri::command(rename_all = "snake_case")]
async fn get_ign(game_id: &str, zone_id: &str) -> Result<String, String> {
    match utils::mlbb::get_ign(game_id, zone_id).await {
        Some(ign) => Ok(ign),
        None => Err("failed to get ign".to_owned()),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![get_ign])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
