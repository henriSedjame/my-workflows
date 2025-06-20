use crate::models::state::AppState;
use crate::tray::menu::menu_items::*;
use tauri::menu::{Menu, PredefinedMenuItem};
use tauri::{AppHandle, Manager, Wry};

pub mod menu_items {
    use crate::models::config::AppConfig;
    use crate::models::errors::AppErrors;
    use crate::models::state::AppState;
    use crate::utils::config::get_config_icons_path;
    use std::fmt::Display;
    use tauri::image::Image;
    use tauri::menu::{IconMenuItem, Menu, MenuId, MenuItem, Submenu, SubmenuBuilder};
    use tauri::Error::UnknownPath;
    use tauri::{AppHandle, Manager, Wry};
    use crate::models::config::Command::{Group, Simple};

    pub mod id_values {
        pub const QUIT: &str = "Quit";

        pub const CONFIG: &str = "Config";

        pub const RELOAD: &str = "ReloadConfig";

        pub const NAVIGATIONS: &str = "Navigations";

        pub const COMMANDS: &str = "Commands";
        
        pub const COMMAND_GROUP: &str = "CommandGroup_";

        pub const OPEN: &str = "Open_";
        pub const CMD: &str = "Cmd_";

        pub const SEPARATOR: &str = "||";

        pub const SHOW_VIEW: &str = "show_view";
        pub const HIDE_VIEW: &str = "hide_view";
    }

    mod texts {
        pub const QUIT: &str = "📴 Close app";

        pub const CONFIG: &str = "⚙️ Open configuration";

        pub const RELOAD_CONFIG: &str = "🔄 Reload configuration";

        pub const NAVIGATIONS: &str = "📂 Navigations ";

        pub const COMMANDS: &str = "📂 Commands ";
        
        pub const SHOW_VIEW: &str = "👀 Show the view";

        pub const HIDE_VIEW: &str = "🙈 Hide the view";
    }

    #[derive(PartialEq, PartialOrd)]
    pub struct PrefixedId(String);

    #[derive(PartialOrd, PartialEq)]
    pub enum MenuItemIds {
        Quit,
        Config,
        ReloadConfig,
        Navigations,
        Commands,
        CommandGroup(String),
        ShowView,
        HideView,
        Open { id: String, url: String },
        Cmd { id: String, cmd: String },
    }

    impl Display for MenuItemIds {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                MenuItemIds::Quit => write!(f, "{}", id_values::QUIT)?,
                MenuItemIds::Config => write!(f, "{}", id_values::CONFIG)?,
                MenuItemIds::ReloadConfig => write!(f, "{}", id_values::RELOAD)?,
                MenuItemIds::Navigations => write!(f, "{}", id_values::NAVIGATIONS)?,
                MenuItemIds::Commands => write!(f, "{}", id_values::COMMANDS)?,
                MenuItemIds::CommandGroup(name) => write!(f, "{}{}" , id_values::COMMAND_GROUP, name)?,
                MenuItemIds::ShowView => write!(f, "{}", id_values::SHOW_VIEW)?,
                MenuItemIds::HideView => write!(f, "{}", id_values::HIDE_VIEW)?,
                MenuItemIds::Open { id, url } => {
                    write!(f, "{}{}{}", id, id_values::SEPARATOR, url)?
                }
                MenuItemIds::Cmd { id, cmd } => write!(f, "{}{}{}", id, id_values::SEPARATOR, cmd)?,
            };

            Ok(())
        }
    }

    impl From<MenuId> for MenuItemIds {
        fn from(id: MenuId) -> Self {
            match id.0.as_str() {
                id_values::QUIT => MenuItemIds::Quit,
                id_values::CONFIG => MenuItemIds::Config,
                id_values::RELOAD => MenuItemIds::ReloadConfig,
                id_values::NAVIGATIONS => MenuItemIds::Navigations,
                id_values::COMMANDS => MenuItemIds::Commands,
                id_values::SHOW_VIEW => MenuItemIds::ShowView,
                id_values::HIDE_VIEW => MenuItemIds::HideView,
                id => {
                    if id.starts_with(id_values::OPEN) {
                        let parts: Vec<&str> = id.split(id_values::SEPARATOR).collect();
                        let open_id = parts.get(0).unwrap().to_string();
                        let open_url = parts.get(1).unwrap().to_string();
                        MenuItemIds::Open {
                            id: open_id.replace(id_values::OPEN, ""),
                            url: open_url,
                        }
                    } else if id.starts_with(id_values::CMD) {
                        let parts: Vec<&str> = id.split(id_values::SEPARATOR).collect();
                        let cmd_id = parts.get(0).unwrap().to_string();
                        let cmd = parts.get(1).unwrap().to_string();
                        MenuItemIds::Cmd {
                            id: cmd_id.replace(id_values::CMD, ""),
                            cmd,
                        }
                    } else if id.starts_with(id_values::COMMAND_GROUP) {
                        let group_name = id.replace(id_values::COMMAND_GROUP, "");
                        MenuItemIds::CommandGroup(group_name)
                    }
                    else {
                        panic!("Invalid menu item id")
                    }
                }
            }
        }
    }

    pub fn quit(app: &AppHandle) -> tauri::Result<MenuItem<Wry>> {
        MenuItem::with_id(app, MenuItemIds::Quit, texts::QUIT, true, None::<&str>)
    }

    pub fn config(app: &AppHandle) -> tauri::Result<MenuItem<Wry>> {
        MenuItem::with_id(
            app,
            MenuItemIds::Config,
            texts::CONFIG,
            true,
           
            None::<&str>,
        )
    }

    pub fn reload(app: &AppHandle) -> tauri::Result<MenuItem<Wry>> {
        MenuItem::with_id(
            app,
            MenuItemIds::ReloadConfig,
            texts::RELOAD_CONFIG,
            true,
            None::<&str>,
        )
    }

    pub fn navigations(app: &AppHandle, config: Option<AppConfig>) -> tauri::Result<Submenu<Wry>> {
        let sb =
            SubmenuBuilder::with_id(app, MenuItemIds::Navigations, texts::NAVIGATIONS).build()?;

        let navigations = if let Some(config) = config {
            config.navigations
        } else {
            let state = app.state::<AppState>();
            let state_lock = state.lock().unwrap();
            state_lock.config.navigations.clone()
        };

        for nav in navigations.into_iter() {
            let name = nav.name;
            let url = nav.url;
            let icon = match nav.icon {
                None => Image::from_bytes(include_bytes!("../../icons/default-web.png")).ok(),
                Some(icon) => {
                    let path = get_config_icons_path(icon).unwrap();
                    match std::fs::read(path) {
                        Ok(icon) => Image::from_bytes(&icon).ok(),
                        Err(_) => return Err(UnknownPath),
                    }
                }
            };

            if let Ok(item) = navigation_icon_item(app, name, url, icon) {
                sb.append(&item)?;
            }
        }

        Ok(sb)
    }

    fn navigation_icon_item(
        app: &AppHandle,
        name: String,
        url: String,
        icon: Option<Image>,
    ) -> tauri::Result<IconMenuItem<Wry>> {
        IconMenuItem::with_id(
            app,
            MenuItemIds::Open {
                id: format!("{}{}", id_values::OPEN, name.clone()),
                url,
            },
            name.clone().as_str().to_uppercase(),
            true,
            icon,
            None::<&str>,
        )
    }

    pub fn commands(app: &AppHandle, config: Option<AppConfig>) -> tauri::Result<Submenu<Wry>> {
        let sb = SubmenuBuilder::with_id(app, MenuItemIds::Commands, texts::COMMANDS)
            .build()?;

        let commands = if let Some(config) = config {
            config.commands
        } else {
            let state = app.state::<AppState>();
            let state_lock = state.lock().unwrap();
            state_lock.config.commands.clone()
        };

        let mut simple_commands = Vec::new();
        let mut group_commands = Vec::new();
        
        for cmd in commands.into_iter() {
            
            match cmd {
                Simple(cmd) => {
                    if let Ok(item) = command_item(app, cmd.name, cmd.cmd) {
                        simple_commands.push(item);
                    }
                },
                Group(group) => {
                    let sbg = SubmenuBuilder::with_id(app, MenuItemIds::CommandGroup(group.name.clone()), group.name)
                        .build()?;
                    
                    for cmd in group.commands.into_iter() {
                        if let Ok(item) = command_item(app, cmd.name, cmd.cmd) {
                            sbg.append(&item)?;
                        }
                    }
                    
                    group_commands.push(sbg);
                }
            }
            
        }

        for sbg in group_commands.into_iter() {
            sb.append(&sbg)?;
        }
        
        for item in simple_commands.into_iter() {
            sb.append(&item)?;
        }
        
        Ok(sb)
    }

    fn command_item(
        app: &AppHandle,
        name: String,
        cmd: String,
    ) -> tauri::Result<IconMenuItem<Wry>> {
        let icon = Image::from_bytes(include_bytes!("../../icons/cmd.png")).ok();
        IconMenuItem::with_id(
            app,
            MenuItemIds::Cmd {
                id: format!("{}{}", id_values::CMD, name.clone()),
                cmd,
            },
            format!("{}", name.clone()),
            true,
            icon,
            None::<&str>,
        )
    }

    pub fn show_hide_view_item(
        app: &AppHandle,
        open: bool,
        enabled: bool,
    ) -> tauri::Result<MenuItem<Wry>> {
        MenuItem::with_id(
            app,
            if open {
                MenuItemIds::ShowView
            } else {
                MenuItemIds::HideView
            },
            if open {
                texts::SHOW_VIEW
            } else {
                texts::HIDE_VIEW
            },
            enabled,
            None::<&str>,
        )
    }

    pub fn remove_show_hide_view_item(menu: &Menu<Wry>) -> tauri::Result<()> {
        let items = menu.items()?;

        if let Some(p) = items.iter().position(|item| {
            let menu_id = item.id();
            menu_id.0.as_str() == id_values::SHOW_VIEW || menu_id.0.as_str() == id_values::HIDE_VIEW
        }) {
            let item = items.get(p).unwrap();
            let sep_item = items.get(p + 1).unwrap();
            menu.remove(item)?;
            menu.remove(sep_item)?;
        };

        Ok(())
    }

    fn remove_items(menu: &Menu<Wry>, id_value: &str) -> Result<usize, AppErrors> {
        let items = menu
            .items()
            .map_err(|_| AppErrors::FailedToRetrieveMenuItems)?;
        if let Some(p) = items.iter().position(|item| {
            let menu_id = item.id();
            menu_id.0.as_str() == id_value
        }) {
            let item = items.get(p).unwrap();
            menu.remove(item)
                .map_err(|_| AppErrors::FailedToRemoveTrayMenuItem(id_value.to_string()))?;
            Ok(p)
        } else {
            Ok(0)
        }
    }

    pub fn remove_navigation_items(menu: &Menu<Wry>) -> Result<usize, AppErrors> {
        remove_items(menu, id_values::NAVIGATIONS)
    }

    pub fn remove_command_items(menu: &Menu<Wry>) -> Result<usize, AppErrors> {
        remove_items(menu, id_values::COMMANDS)
    }
}

pub fn create_menu(app: &AppHandle) -> tauri::Result<Menu<Wry>> {
    let menu = Menu::with_items(
        app,
        &[
            &navigations(app, None)?,
            &PredefinedMenuItem::separator(app)?,
            &commands(app, None)?,
            &PredefinedMenuItem::separator(app)?,
            &config(app)?,
            &PredefinedMenuItem::separator(app)?,
            &reload(app)?,
            &PredefinedMenuItem::separator(app)?,
            &quit(app)?,
        ],
    )?;

    app.state::<AppState>().lock().unwrap().menu = Some(menu.clone());

    Ok(menu)
}
