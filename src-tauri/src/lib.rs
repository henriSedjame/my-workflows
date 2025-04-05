mod commands;
mod models;
mod tray;
mod utils;

use crate::models::state::AppState;
use tauri::Manager;
use tauri_plugin_autostart::{MacosLauncher, ManagerExt};
use tauri_plugin_dialog::{DialogExt, MessageDialogKind};
use crate::utils::{ reset_app_size};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec!["--flag1", "--flag2"]),
        ))
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_process::init())
        .setup(|app| {

            /*Enable auto start*/{
                let autostart_manager = app.autolaunch();
                let _ = autostart_manager.enable();
            }

            /* Hide app and dock icon */{
                app.set_activation_policy(tauri::ActivationPolicy::Accessory);
                //app.hide().unwrap();
            }


            let app_handle = app.handle();
           
            reset_app_size(app_handle);

            /* Add initial state */ {
                match AppState::new() {
                    Ok(state) => {
                        app.manage(state);
                        tray::create(app_handle)?;
                        Ok(())
                    }
                    Err(e) => {
                        app.dialog()
                            .message(format!("👉 {}", e))
                            .title("Failed to retrieve configuration".to_uppercase())
                            .kind(MessageDialogKind::Error)
                            .blocking_show();
                        panic!("")
                    }
                }
            }

        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
