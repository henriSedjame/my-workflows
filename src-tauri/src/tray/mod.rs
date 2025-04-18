use crate::models::events::commands::CommandRequested;
use crate::models::events::emit_event;
use crate::tray::menu::{create_menu, menu_items::{id_values, MenuItemIds}};
use crate::utils::cmd::{evaluate_cmd_value, execute_cmd};
use crate::utils::config::get_config_path;

use tauri::image::Image;
use tauri::menu::MenuEvent;
use tauri::tray::{TrayIcon, TrayIconBuilder};
use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_dialog::{DialogExt, MessageDialogKind};
use uuid::Uuid;

use crate::utils::{hide_main_view, show_main_view, update_tray_menu};

pub(crate) mod menu;

pub const TRAY_ID: &str = "tray-id";

pub fn create(app: &AppHandle) -> tauri::Result<TrayIcon> {
    let icon = Image::from_bytes(include_bytes!("../../icons/icon.ico"))?;
    TrayIconBuilder::with_id(TRAY_ID)
        .icon(icon)
        .icon_as_template(true)
        .menu(&create_menu(app)?)
        .show_menu_on_left_click(true)
        .on_menu_event(|app, event: MenuEvent| {
            let id : MenuItemIds = event.id.into();
            match id {
                MenuItemIds::Quit => app.hide_menu().unwrap(),
                MenuItemIds::Config => {
                    if let Ok(path) = get_config_path() {
                        let cmd = format!("open {}", path);
                        let cmd = cmd.as_str();

                        execute_cmd(cmd, |_|{}, || {
                            app.dialog()
                                .message("Failed to open configuration file")
                                .title("⚙️OPEN CONFIGURATION")
                                .kind(MessageDialogKind::Error)
                                .blocking_show();
                        });
                    };
                }
                MenuItemIds::Reload => app.restart(),
                MenuItemIds::ShowView => {
                    show_main_view(app);
                    update_tray_menu(&app);
                },
                MenuItemIds::HideView => {
                    hide_main_view(app);
                    update_tray_menu(&app);
                },
                MenuItemIds::Navigations => {}
                MenuItemIds::Open { id: _, url } => open::that(url).unwrap(),
                MenuItemIds::Commands => {}
                MenuItemIds::Cmd { id, cmd } => match evaluate_cmd_value(app, cmd) {
                    Ok(cmd) => {
                        
                        show_main_view(app);
                        
                        emit_event(app, CommandRequested {
                            command_id: Uuid::new_v4(),
                            command_label: id.replace(id_values::CMD, "").as_str(),
                            command_value: cmd.modified_cmd.as_str(),
                            command_to_execute: cmd.original_cmd.as_str()
                        }).unwrap();
                    }
                    Err(e) => {
                        app.dialog()
                            .message(format!("{}", e))
                            .title(format!("# {}", id).as_str().to_uppercase())
                            .kind(MessageDialogKind::Error)
                            .blocking_show();
                    }
                },
            }
        })
        .build(app)
}
