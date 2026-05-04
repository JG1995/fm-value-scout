// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::Manager;

mod commands;
mod data;
mod models;
mod parser;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            commands::parse_csv,
            commands::archetypes::list_archetypes,
            commands::archetypes::list_available_metrics,
            commands::archetypes::save_user_archetype,
            commands::archetypes::delete_user_archetype,
        ])
        .setup(|app| {
            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("failed to resolve app data dir");
            let store = commands::archetypes::ArchetypeStore::new(app_data_dir);
            app.manage(store);
            Ok(())
        });

    #[cfg(debug_assertions)]
    {
        builder = builder.plugin(tauri_plugin_mcp_bridge::init());
    }

    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
