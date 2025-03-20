use tauri::Manager;
use tauri::tray::{MouseButtonState, TrayIcon, TrayIconEvent};

fn handle_icon_click(tray: &TrayIcon, event: TrayIconEvent) {
    let main_view = tray.app_handle().get_webview_window("main").unwrap();
    if let TrayIconEvent::Click { button_state, .. } = event {
        if button_state == MouseButtonState::Up {
            if main_view.is_visible().unwrap() {
                main_view.hide().unwrap()
            } else {
                main_view.show().unwrap()
            }
        }
    }
}