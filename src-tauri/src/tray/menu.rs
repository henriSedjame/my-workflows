use tauri::menu::{Menu, PredefinedMenuItem};
use tauri::{AppHandle, Wry};

use crate::tray::menu::menu_items::*;

pub mod menu_items {
    use crate::models::state::AppState;
    use std::fmt::Display;
    use tauri::image::Image;
    use tauri::menu::{IconMenuItem, MenuId, MenuItem, Submenu, SubmenuBuilder};
    use tauri::{AppHandle, Error, Manager, Wry};
    use crate::utils::config::get_config_icons_path;

    mod id_values {
        pub const QUIT: &str = "Quit";

        pub const CONFIG: &str = "Config";

        pub const RELOAD: &str = "Reload";

        pub const NAVIGATIONS: &str = "Navigations";
        
        pub const COMMANDS: &str = "Commands";

        pub const OPEN: &str = "Open_";
        pub const CMD: &str = "Cmd_";

        pub const SEPARATOR: &str = "||";
    }

    mod texts {
        pub const QUIT: &str = " Close ";

        pub const CONFIG: &str = "Open configuration";

        pub const RELOAD: &str = "Reload app";

        pub const NAVIGATIONS: &str = "Navigate to ... ";
        
        pub const COMMANDS: &str = "Execute cmd ...";
       
    }

    #[derive(PartialOrd, PartialEq)]
    pub enum MenuItemIds {
        Quit,
        Config,
        Reload,
        Navigations,
        Commands,
        Open { id: String, url: String },
        Cmd { id: String, cmd: String },
    }

    impl Display for MenuItemIds {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                MenuItemIds::Quit => write!(f, "{}", id_values::QUIT)?,
                MenuItemIds::Config => write!(f, "{}", id_values::CONFIG)?,
                MenuItemIds::Reload => write!(f, "{}", id_values::RELOAD)?,
                MenuItemIds::Navigations => write!(f, "{}", id_values::NAVIGATIONS)?,
                MenuItemIds::Commands => write!(f, "{}", id_values::COMMANDS)?,
                MenuItemIds::Open { id, url } => {
                    write!(f, "{}{}{}", id, id_values::SEPARATOR, url)?
                },
                MenuItemIds::Cmd { id, cmd } => {
                    write!(f, "{}{}{}", id, id_values::SEPARATOR, cmd)?
                }
            };

            Ok(())
        }
    }

    impl From<MenuId> for MenuItemIds {
        fn from(id: MenuId) -> Self {
            match id.0.as_str() {
                id_values::QUIT => MenuItemIds::Quit,
                id_values::CONFIG => MenuItemIds::Config,
                id_values::RELOAD => MenuItemIds::Reload,
                id_values::NAVIGATIONS => MenuItemIds::Navigations,
                id_values::COMMANDS => MenuItemIds::Commands,
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
                            cmd
                        }
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

    pub fn config(app: &AppHandle) -> tauri::Result<IconMenuItem<Wry>> {
        let icon = Image::from_bytes(include_bytes!("../../icons/config.png")).ok();
        IconMenuItem::with_id(
            app,
            MenuItemIds::Config,
            texts::CONFIG,
            true,
            icon,
            None::<&str>,
        )
    }

    pub fn reload(app: &AppHandle) -> tauri::Result<IconMenuItem<Wry>> {
        let icon = Image::from_bytes(include_bytes!("../../icons/reload.png")).ok();
        IconMenuItem::with_id(
            app,
            MenuItemIds::Reload,
            texts::RELOAD,
            true,
            icon,
            None::<&str>,
        )
    }

    pub fn navigations(app: &AppHandle) -> tauri::Result<Submenu<Wry>> {
        let sb =
            SubmenuBuilder::with_id(app, MenuItemIds::Navigations, texts::NAVIGATIONS).build()?;
        let state = app.state::<AppState>();

        state
            .config
            .navigations
            .clone()
            .into_iter()
            .for_each(|nav| {
                let name = nav.name;
                let url = nav.url;
                match nav.icon {
                    None => {
                        if let Ok(item) = navigation_item(app, name, url) {
                            sb.append(&item).expect("");
                        }
                    }
                    Some(icon) => {
                        
                        let path = get_config_icons_path(icon).unwrap();
                        if let Ok(item) = navigation_icon_item(app, name, url, path.clone()) {
                            sb.append(&item).expect("Failed to add");
                        }
                    }
                }
            });

        Ok(sb)
    }

    fn navigation_item(app: &AppHandle, name: String, url: String) -> tauri::Result<MenuItem<Wry>> {
        MenuItem::with_id(
            app,
            MenuItemIds::Open {
                id: format!("{}{}", id_values::OPEN, name.clone()),
                url,
            },
            name.clone().as_str().to_uppercase(),
            true,
            None::<&str>,
        )
    }

    fn navigation_icon_item(
        app: &AppHandle,
        name: String,
        url: String,
        icon_path: String,
    ) -> tauri::Result<IconMenuItem<Wry>> {
        match std::fs::read(icon_path.clone()) {
            Ok(icon) => IconMenuItem::with_id(
                app,
                MenuItemIds::Open {
                    id: format!("{}{}", id_values::OPEN, name.clone()),
                    url,
                },
                name.clone().as_str().to_uppercase(),
                true,
                Some(Image::from_bytes(&icon)?),
                None::<&str>,
            ),
            Err(e) => {
                todo!("show message");
                Err(Error::UnknownPath)
            }
        }
    }

    pub fn commands(app: &AppHandle) -> tauri::Result<Submenu<Wry>> {
        let sb =
            SubmenuBuilder::with_id(app, MenuItemIds::Commands, texts::COMMANDS).build()?;
        let state = app.state::<AppState>();

        state
            .config
            .commands
            .clone()
            .into_iter()
            .for_each(|command| {
                let name = command.name;
                let cmd = command.cmd;
                if let Ok(item) = command_item(app, name, cmd) {
                        sb.append(&item).expect("");
                }
                
            });

        Ok(sb)
    }
    fn command_item(app: &AppHandle, name: String, cmd: String) -> tauri::Result<IconMenuItem<Wry>> {
        let icon = Image::from_bytes(include_bytes!("../../icons/cmd.png")).ok();
        IconMenuItem::with_id(
            app,
            MenuItemIds::Cmd {
                id: format!("{}{}", id_values::CMD, name.clone()),
                cmd,
            },
            format!("{}",  name.clone()),
            true,
            icon,
            None::<&str>,
        )
    }
}

pub mod accelerators {

    pub enum Keys {
        CMD,
        CTRL,
        J,
        A,
        G,
    }

    impl From<Keys> for String {
        fn from(value: Keys) -> Self {
            match value {
                Keys::CMD => String::from("CmdOrCtrl"),
                Keys::CTRL => String::from("CmdOrCtrl"),
                Keys::J => String::from("J"),
                Keys::A => String::from("A"),
                Keys::G => String::from("G"),
            }
        }
    }

    pub struct Accelerator(Vec<Keys>);

    impl Accelerator {
        pub(crate) fn of(keys: Vec<Keys>) -> Self {
            Accelerator(keys)
        }

        pub(crate) fn build(self) -> Option<String> {
            Some(self.into())
        }
    }

    impl From<Accelerator> for String {
        fn from(value: Accelerator) -> String {
            let v: Vec<String> = value.0.into_iter().map(move |k| String::from(k)).collect();
            v.join("+")
        }
    }
}

pub fn create_menu(app: &AppHandle) -> tauri::Result<Menu<Wry>> {
    Menu::with_items(
        app,
        &[
            &navigations(app)?,
            &PredefinedMenuItem::separator(app)?,
            &commands(app)?,
            &PredefinedMenuItem::separator(app)?,
            &config(app)?,
            &PredefinedMenuItem::separator(app)?,
            &reload(app)?,
            &PredefinedMenuItem::separator(app)?,
            &quit(app)?,
        ],
    )
}
