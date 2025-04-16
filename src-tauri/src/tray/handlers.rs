use tauri::tray::{MouseButton, MouseButtonState, TrayIcon, TrayIconEvent};
use tauri::Manager;

pub fn handle_icon_click(tray: &TrayIcon, event: TrayIconEvent) {
  
    let main_view = tray.app_handle().get_webview_window("main").unwrap();
    println!("EVENT => {:?}", event);
    if let TrayIconEvent::Click { button, button_state, .. } = event {
        println!("Clicked on {:?}", event);
        if button == MouseButton::Left {
            if main_view.is_visible().unwrap() {
                main_view.hide().unwrap()
            } else {
                main_view.show().unwrap()
            }
        }
    }
}
