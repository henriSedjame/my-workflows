use crate::models::events::commands::CommandRequested;
use crate::models::events::emit_event;
use crate::tray::menu::{
    create_menu,
    menu_items::{id_values, MenuItemIds},
};
use crate::utils::cmd::{evaluate_cmd_value, execute_cmd};
use crate::utils::config::get_config_dir_path;

use crate::models::errors::AppErrors;
use crate::utils::{
    close_main_view, hide_main_view, show_main_view, update_config_menu, update_tray_menu,
};
use tauri::image::Image;
use tauri::menu::MenuEvent;
use tauri::tray::{TrayIcon, TrayIconBuilder};
use tauri::AppHandle;
use tauri_plugin_dialog::{DialogExt, MessageDialogKind};
use uuid::Uuid;

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
            let id: MenuItemIds = event.id.into();
            match id {
                MenuItemIds::Quit => handle(
                    app,
                    String::from("Close Application"),
                    || close_main_view(app)
                ),
                MenuItemIds::Config => handle(
                    app,
                    String::from("Open Configuration Directory"),
                    || {
                        let path = get_config_dir_path()?;
                        let cmd = format!("open {}", path);
                        let cmd = cmd.as_str();

                        execute_cmd(cmd, |_| {}, "Failed to open the configuration directory.")
                    }
                ),
                MenuItemIds::ReloadConfig => handle(
                    app,
                    String::from("Reload Configuration"),
                    || update_config_menu(app),
                ),
                MenuItemIds::ShowView => handle(
                    app,
                    String::from("Show View"),
                    || {
                        show_main_view(app)?;
                        update_tray_menu(&app)
                    }
                ),
                MenuItemIds::HideView => handle(
                    app,
                    String::from("Hide View"),
                    || {
                        hide_main_view(app)?;
                        update_tray_menu(&app)
                    }
                ),
                MenuItemIds::Navigations => {}
                MenuItemIds::Open { id: _, url } => handle(
                    app,
                   format!("Open {}", url),
                    || {
                        open::that(url.clone()).map_err(|_| AppErrors::FailedToOpenUrl(url.clone()))
                    }
                )
                ,
                MenuItemIds::Commands => {}
                MenuItemIds::Cmd { id, cmd } => handle(
                    app,
                    String::from("Execute Command"), 
                    || {
                        let cmd = evaluate_cmd_value(app, cmd.clone())?;
                        show_main_view(app)?;
                        emit_event(
                            app,
                            CommandRequested {
                                command_id: Uuid::new_v4(),
                                command_label: id.replace(id_values::CMD, "").as_str(),
                                command_value: cmd.modified_cmd.as_str(),
                                command_to_execute: cmd.original_cmd.as_str(),
                                command_params: cmd.parameters,
                            },
                        ).map_err(|_| AppErrors::FailedToExecuteCommand(cmd.original_cmd))
                    }
                ),
            }
        })
        .build(app)
}

fn handle(
    app: &AppHandle,
    title: String,
    block: impl Fn() -> Result<(), AppErrors>
) {
    if let Err(e) = block() {
        app.dialog()
            .message(format!("{}", e))
            .title(title.to_uppercase())
            .kind(MessageDialogKind::Error)
            .blocking_show();
    }
}
