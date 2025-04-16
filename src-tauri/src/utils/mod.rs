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