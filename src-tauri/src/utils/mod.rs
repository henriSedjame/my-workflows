use tauri::{AppHandle, Manager, PhysicalSize};

pub(crate) mod config;
pub(crate) mod cmd;

pub fn reset_app_size(app: &AppHandle) {
    app.get_webview_window("main")
        .unwrap()
        .set_size(PhysicalSize {
            width: 800,
            height: 600,
        })
        .expect("");

    app.show().unwrap();
}