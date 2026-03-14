mod database;
mod entities;
mod scan;

use database::{connect, create_tables};
use sea_orm::DatabaseConnection;
use tauri::{Manager, State};

struct Db(DatabaseConnection);

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // ejecución async dentro de setup
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                let db = connect().await;
                create_tables(&db).await;
                handle.manage(Db(db));
            });

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            custom_msg,
            scan::scan_folder
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn custom_msg() {
    println!("Esta cosa parece que funciona")
}
