use crate::commands::{execute, execute_and_handle};
use crate::tray::menu::{create_menu, menu_items::MenuItemIds};
use crate::utils::cmd::evaluate_cmd_value;
use crate::utils::config::get_config_path;
use std::fmt::format;
use tauri::image::Image;
use tauri::tray::{TrayIcon, TrayIconBuilder};
use tauri::{AppHandle, Manager};
use tauri_plugin_dialog::{DialogExt, MessageDialogKind};

mod handlers;
pub(crate) mod menu;

pub const TRAY_ID: &str = "tray-id";

pub fn create(app: &AppHandle) -> tauri::Result<TrayIcon> {
    let icon = Image::from_bytes(include_bytes!("../../icons/icon.ico"))?;
    TrayIconBuilder::with_id(TRAY_ID)
        .icon(icon)
        .icon_as_template(true)
        .menu(&create_menu(app)?)
        .show_menu_on_left_click(true)
        //.on_tray_icon_event(handle_icon_click)
        .on_menu_event(|app, event| match event.id.into() {
            MenuItemIds::Quit => app.hide_menu().unwrap(),
            MenuItemIds::Config => {
                if let Ok(path) = get_config_path() {
                    let cmd = format!("open {}", path);
                    let cmd = cmd.as_str();

                    execute(cmd, || {
                        app.dialog()
                            .message("Failed to open configuration file")
                            .title("⚙️OPEN CONFIGURATION")
                            .kind(MessageDialogKind::Error)
                            .blocking_show();
                    })
                    .expect("");
                };
            }
            MenuItemIds::Reload => app.restart(),
            MenuItemIds::Navigations => {}
            MenuItemIds::Open { id: _, url } => open::that(url).unwrap(),
            MenuItemIds::Commands => {}
            MenuItemIds::Cmd { id, cmd } => match evaluate_cmd_value(app, cmd) {
                Ok(cmd) => {
                    execute_and_handle(
                        cmd.as_str(),
                        |output| {
                            let result = String::from_utf8(output.stdout).unwrap();
                            app.dialog()
                                .message(result.as_str())
                                .title(format!("# {}", id).as_str().to_uppercase())
                                .kind(MessageDialogKind::Info)
                                .blocking_show();
                        },
                        || {},
                    )
                    .unwrap();
                }
                Err(e) => {
                    app.dialog()
                        .message(format!("{}", e))
                        .title(format!("# {}", id).as_str().to_uppercase())
                        .kind(MessageDialogKind::Error)
                        .blocking_show();
                }
            },
        })
        .build(app)
}
