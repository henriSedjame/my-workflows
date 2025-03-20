use tauri::AppHandle;
use tauri::image::Image;
use tauri::tray::{TrayIcon, TrayIconBuilder};
use crate::tray::menu::{create_menu, menu_items::MenuItemIds};

mod menu;
mod handlers;

pub fn create(app: &AppHandle) -> tauri::Result<TrayIcon> {
    let icon = Image::from_bytes(include_bytes!("../../icons/icon.ico"))?;
    TrayIconBuilder::new()
        .icon(icon)
        .icon_as_template(true)
        .menu(&create_menu(app)?)
        .show_menu_on_left_click(true)
        //.on_tray_icon_event(handle_icon_click)
        .on_menu_event(|app, event| {
            match event.id.into() {
                MenuItemIds::Quit => { app.hide_menu().unwrap() }
                MenuItemIds::Navigations => {}
                MenuItemIds::Open {id: _ , url} => { open::that(url).unwrap() }
                
            }
        })
        .build(app)
}
