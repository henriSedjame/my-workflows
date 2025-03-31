mod commands;
mod models;
mod tray;
mod utils;

use crate::models::state::AppState;
use tauri::Manager;
use tauri_plugin_dialog::{DialogExt, MessageDialogKind};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_process::init())
        .setup(|app| {

            app.hide().unwrap();
            app.get_webview_window("main").unwrap().hide_menu().unwrap();

            let app_handle = app.handle();

            match AppState::new() {
                Ok(state) => {
                    app.manage(state);
                    tray::create(app_handle)?;
                    Ok(())
                },
                Err(e) => {
                    app.dialog()
                        .message(format!("👉 {}", e))
                        .title("Failed to retrieve configuration".to_uppercase())
                        .kind(MessageDialogKind::Error)
                        .blocking_show();
                    panic!("")
                }
            }
            
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
