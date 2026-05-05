mod db;
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let db = init_database(&app.handle())?;
            app.manage(AppState { db });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
