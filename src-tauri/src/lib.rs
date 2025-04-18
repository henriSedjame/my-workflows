mod commands;
mod models;
mod tray;
mod utils;

use crate::models::state::AppStateInner;
use crate::utils::hide_main_view;
use commands::{execute_command, kill_command, no_running_command, hide_view};
use std::sync::Mutex;
use tauri::{Listener, Manager};
use tauri_plugin_autostart::{MacosLauncher, ManagerExt};
use tauri_plugin_dialog::{DialogExt, MessageDialogKind};


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
            }

            let app_handle = app.handle();

            hide_main_view(app_handle);

            /* Add initial state */ {
                match AppStateInner::new() {
                    Ok(inner) => {
                        app.manage(Mutex::new(inner));
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
        .invoke_handler(tauri::generate_handler![
            execute_command,
            kill_command,
            no_running_command,
            hide_view
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
