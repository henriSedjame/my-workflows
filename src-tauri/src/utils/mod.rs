use std::fs;
use crate::models::state::AppState;
use crate::tray::menu::menu_items::{
    commands, navigations, remove_command_items, remove_navigation_items,
    remove_show_hide_view_item, show_hide_view_item,
};
use crate::tray::TRAY_ID;
use tauri::menu::{Menu, PredefinedMenuItem, Submenu};
use tauri::{AppHandle, Manager, Wry};
use crate::models::config::AppConfig;
use crate::utils::config::{create_config_dir, get_config};

pub(crate) mod cmd;
pub(crate) mod config;
pub(crate) mod constants;
pub(crate) mod process;

pub fn show_main_view(app: &AppHandle) {
    let window = app.get_webview_window("main").unwrap();
    window.show().unwrap();
    window.set_focus().unwrap();
}

pub fn hide_main_view(app: &AppHandle) {
    let window = app.get_webview_window("main").unwrap();
    window.hide().unwrap();
}

pub fn close_main_view(app: &AppHandle) {
    let window = app.get_webview_window("main").unwrap();
    window.close().unwrap();
}

pub fn main_view_visible(app: &AppHandle) -> bool {
    let window = app.get_webview_window("main").unwrap();
    window.is_visible().unwrap()
}

pub fn update_tray_menu(app: &AppHandle) {
    let state = app.state::<AppState>();
    let state_lock = state.lock().unwrap();
    if let Some(menu) = state_lock.menu.clone() {
        remove_show_hide_view_item(&menu).unwrap();

        let enabled = state_lock.view_visible;

        let position = menu.items().unwrap().len() - 1;

        if main_view_visible(&app) {
            menu.insert(&show_hide_view_item(app, false, enabled).unwrap(), position)
                .unwrap();
        } else {
            menu.insert(&show_hide_view_item(app, true, enabled).unwrap(), position)
                .unwrap();
        }
        menu.insert(&PredefinedMenuItem::separator(app).unwrap(), position + 1)
            .unwrap();

        app.tray_by_id(TRAY_ID)
            .unwrap()
            .set_menu(Some(menu))
            .unwrap();
    }
}

fn update_config_items(
    app: &AppHandle,
    config: AppConfig,
    menu: &Menu<Wry>,
    remove_items: impl Fn(&Menu<Wry>) -> tauri::Result<usize>,
    create_item: impl Fn(&AppHandle, Option<AppConfig>) -> tauri::Result<Submenu<Wry>>,
) {
    let position = remove_items(&menu).unwrap();
    let items = create_item(app, Some(config));
    match menu.insert(&items.unwrap(), position) {
        Ok(_) => (),
        Err(e) => println!("Error: {}", e),
    }
}

pub fn update_config_menu(app: &AppHandle) {
    let state = app.state::<AppState>();
    let mut state_lock = state.lock().unwrap();

    let new_config = get_config().unwrap();
    
    state_lock.config = new_config.clone();
    
    if let Some(menu) = state_lock.menu.clone() {
        update_config_items(app, new_config.clone(),  &menu, remove_navigation_items, navigations);
        update_config_items(app, new_config.clone(), &menu, remove_command_items, commands);

        app.tray_by_id(TRAY_ID)
            .unwrap()
            .set_menu(Some(menu))
            .unwrap();
    }
}
