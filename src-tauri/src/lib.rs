pub mod db;
pub mod encoding;
pub mod import;
pub mod parsers;

use std::sync::{Arc, Mutex};

use rusqlite::Connection;
use tauri::Manager;

pub(crate) struct AppState {
    pub(crate) db: Arc<Mutex<Connection>>,
}

fn init_database(
    app_handle: &tauri::AppHandle,
) -> Result<Arc<Mutex<Connection>>, Box<dyn std::error::Error>> {
    let app_dir = app_handle.path().app_data_dir()?;
    std::fs::create_dir_all(&app_dir)?;
    let db_path = app_dir.join("fm_valuescout.db");

    let conn = Connection::open(&db_path)?;
    // WAL mode pragmas
    conn.pragma_update(None, "journal_mode", "WAL")?;
    conn.pragma_update(None, "synchronous", "NORMAL")?;
    conn.pragma_update(None, "foreign_keys", "ON")?;
    conn.pragma_update(None, "cache_size", -32000)?; // 32MB page cache
    conn.pragma_update(None, "mmap_size", 268435456)?; // 256MB mmap
    conn.pragma_update(None, "busy_timeout", 5000)?; // 5s busy timeout

    db::create_tables(&conn)?;

    Ok(Arc::new(Mutex::new(conn)))
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

/// Set the user's managed club name. Persisted in the settings table.
#[tauri::command]
fn set_managed_club(state: tauri::State<'_, AppState>, club_name: String) -> Result<(), String> {
    let conn = state
        .db
        .lock()
        .map_err(|e| format!("Failed to acquire DB lock: {}", e))?;
    db::set_setting(&conn, "managed_club", &club_name).map_err(|e| format!("DB error: {}", e))
}

/// Get the user's managed club name. Returns None if not yet set.
#[tauri::command]
fn get_managed_club(state: tauri::State<'_, AppState>) -> Result<Option<String>, String> {
    let conn = state
        .db
        .lock()
        .map_err(|e| format!("Failed to acquire DB lock: {}", e))?;
    db::get_setting(&conn, "managed_club").map_err(|e| format!("DB error: {}", e))
}

/// Set a user preference by key/value pair. Persisted in the settings table.
#[tauri::command]
fn set_preference(
    state: tauri::State<'_, AppState>,
    key: String,
    value: String,
) -> Result<(), String> {
    let conn = state
        .db
        .lock()
        .map_err(|e| format!("Failed to acquire DB lock: {}", e))?;
    db::set_setting(&conn, &key, &value).map_err(|e| format!("DB error: {}", e))
}

/// Get a user preference by key. Returns None if the key is not set.
#[tauri::command]
fn get_preference(
    state: tauri::State<'_, AppState>,
    key: String,
) -> Result<Option<String>, String> {
    let conn = state
        .db
        .lock()
        .map_err(|e| format!("Failed to acquire DB lock: {}", e))?;
    db::get_setting(&conn, &key).map_err(|e| format!("DB error: {}", e))
}

/// Get all imported seasons, most recent first.
#[tauri::command]
fn get_seasons(state: tauri::State<'_, AppState>) -> Result<Vec<db::SeasonInfo>, String> {
    let conn = state
        .db
        .lock()
        .map_err(|e| format!("Failed to acquire DB lock: {}", e))?;
    db::get_seasons(&conn).map_err(|e| format!("DB error: {}", e))
}

/// Import a CSV file into the database.
/// file_path: absolute path to the CSV file
/// in_game_date: the in-game date (e.g. "15.6.2029")
#[tauri::command]
fn import_csv(
    state: tauri::State<'_, AppState>,
    file_path: String,
    in_game_date: String,
) -> Result<crate::import::ImportResult, String> {
    let mut conn = state
        .db
        .lock()
        .map_err(|e| format!("Failed to acquire DB lock: {}", e))?;
    crate::import::run_import(&mut *conn, &file_path, &in_game_date)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let db = init_database(&app.handle())?;
            app.manage(AppState { db });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            set_managed_club,
            get_managed_club,
            set_preference,
            get_preference,
            get_seasons,
            import_csv,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
