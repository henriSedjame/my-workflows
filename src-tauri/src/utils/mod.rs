use tauri::{AppHandle, Manager, PhysicalSize, TitleBarStyle, WebviewUrl, WebviewWindowBuilder};
use tauri::window::Color;

pub(crate) mod cmd;
pub(crate) mod config;

pub fn reset_app_size(app: &AppHandle) {
    app.get_webview_window("main")
        .unwrap()
        .set_size(PhysicalSize {
            width: 1200,
            height: 1200,
        })
        .expect("");

    app.show().unwrap();
}

