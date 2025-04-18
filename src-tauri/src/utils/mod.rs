use crate::models::state::AppState;
use crate::tray::menu::menu_items::{remove_show_hide_view_item, show_hide_view_item};
use crate::tray::TRAY_ID;
use tauri::{AppHandle, Manager};

pub(crate) mod cmd;
pub(crate) mod config;


pub fn show_main_view(app: &AppHandle) {
    let window = app.get_webview_window("main").unwrap();
    window.show().unwrap();
}

pub fn hide_main_view(app: &AppHandle) {
    let window = app.get_webview_window("main").unwrap();
    window.hide().unwrap();
}

pub fn main_view_visible(app: &AppHandle) -> bool {
    let window = app.get_webview_window("main").unwrap();
    window.is_visible().unwrap()
}

pub fn update_tray_menu(app: &AppHandle) {
    let state = app.state::<AppState>();
    let state_lock = state.lock().unwrap();
    if let Some(menu) =  state_lock.menu.clone() {
        
        remove_show_hide_view_item(&menu).unwrap();
        
        let enabled = state_lock.open_tabs;
        
        if main_view_visible(&app) {
            menu.append(&show_hide_view_item(app, false, enabled).unwrap()).unwrap();
        } else {
            menu.append(&show_hide_view_item(app, true, enabled).unwrap()).unwrap();
        }
        app.tray_by_id(TRAY_ID).unwrap().set_menu(Some(menu)).unwrap();
    }
    
}