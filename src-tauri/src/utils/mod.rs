
use crate::models::state::AppState;
use crate::tray::menu::menu_items::{commands, id_values, navigations, remove_command_items, remove_navigation_items, remove_show_hide_view_item, show_hide_view_item};
use crate::tray::TRAY_ID;
use tauri::menu::{Menu, PredefinedMenuItem, Submenu};
use tauri::{AppHandle, Manager, Wry};
use crate::models::config::AppConfig;
use crate::models::errors::AppErrors;
use crate::utils::config::get_config;

pub(crate) mod cmd;
pub(crate) mod config;
pub(crate) mod constants;
pub(crate) mod process;

pub fn show_main_view(app: &AppHandle) -> Result<(), AppErrors> {
    let window = app.get_webview_window("main").unwrap();
    window.show().map_err(|e| AppErrors::FailedToShowApp(e))?;
    window.set_focus().map_err(|e| AppErrors::FailedToFocusOnAppView(e))
}

pub fn hide_main_view(app: &AppHandle) -> Result<(), AppErrors> {
    let window = app.get_webview_window("main").unwrap();
    window.hide().map_err(|e| AppErrors::FailedToShowApp(e))
}

pub fn close_main_view(app: &AppHandle)  -> Result<(), AppErrors>{
    let window = app.get_webview_window("main").unwrap();
    window.close().map_err(|e| AppErrors::FailedToCloseApp(e))
}

pub fn main_view_visible(app: &AppHandle) -> bool {
    let window = app.get_webview_window("main").unwrap();
    window.is_visible().unwrap()
}

pub fn update_tray_menu(app: &AppHandle) -> Result<(), AppErrors>{
    let state = app.state::<AppState>();
    let state_lock = state.lock().unwrap();
    if let Some(menu) = state_lock.menu.clone() {

        let visible = main_view_visible(&app);
        let (item_to_insert, item_to_remove) = if visible {
            (id_values::HIDE_VIEW, id_values::SHOW_VIEW)
        } else {   
            (id_values::SHOW_VIEW, id_values::HIDE_VIEW)
        };
        
        remove_show_hide_view_item(&menu)
            .map_err(|_| AppErrors::FailedToRemoveTrayMenuItem(item_to_remove.to_string()))?;

        let enabled = state_lock.view_visible;

        let position = menu.items().unwrap().len() - 1;

        menu.insert(&show_hide_view_item(app, !visible, enabled).unwrap(), position)
            .map_err(|_| AppErrors::FailedToInsertMenuItem(item_to_insert.to_string()))?;
        
        menu.insert(&PredefinedMenuItem::separator(app).unwrap(), position + 1)
            .map_err(|_| AppErrors::FailedToInsertMenuItem("SEPARATOR".to_string()))?;

        app.tray_by_id(TRAY_ID)
            .unwrap()
            .set_menu(Some(menu))
            .map_err(|_| AppErrors::FailedToSetTrayMenu)
    } else { 
        Ok(())
    }
}

fn update_config_items(
    app: &AppHandle,
    config: AppConfig,
    item_id: &str,
    menu: &Menu<Wry>,
    remove_items: impl Fn(&Menu<Wry>) -> Result<usize, AppErrors>,
    create_item: impl Fn(&AppHandle, Option<AppConfig>) -> tauri::Result<Submenu<Wry>>,
) -> Result<(), AppErrors> {
    let position = remove_items(&menu)?;
    let items = create_item(app, Some(config)).map_err(|_| AppErrors::FailedToCreateItem(item_id.to_string()));
    menu.insert(&items?, position).map_err(|_| AppErrors::FailedToInsertMenuItem(item_id.to_string()))
       
}

pub fn update_config_menu(app: &AppHandle) -> Result<(), AppErrors>{
    let state = app.state::<AppState>();
    let mut state_lock = state.lock().unwrap();

    let new_config = get_config()?;
    
    state_lock.config = new_config.clone();
    
    if let Some(menu) = state_lock.menu.clone() {
        update_config_items(app, new_config.clone(), id_values::NAVIGATIONS, &menu, remove_navigation_items, navigations)?;
        update_config_items(app, new_config.clone(), id_values::COMMANDS, &menu, remove_command_items, commands)?;

        app.tray_by_id(TRAY_ID)
            .unwrap()
            .set_menu(Some(menu))
            .map_err(
                |_| AppErrors::FailedToSetTrayMenu
            )
    } else {
        Ok(())
    }
    
}
