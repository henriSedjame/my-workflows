
use tauri::menu::{Menu, PredefinedMenuItem};
use tauri::{AppHandle, Wry};

use crate::tray::menu::menu_items::*;


pub mod menu_items {
    use std::fmt::Display;
    use tauri::{AppHandle, Wry};
    use tauri::image::Image;
    use tauri::menu::{IconMenuItem, MenuId, MenuItem, Submenu, SubmenuBuilder};
    use crate::tray::menu::accelerators::{Accelerator, Keys};

    mod id_values {
        pub const QUIT: &str = "Quit";
        pub const NAVIGATIONS: &str = "Navigations";
        
        pub const OPEN_JIRA: &str = "OpenJira";
        
        pub const OPEN_AWS: &str = "OpenAWS";

        pub const OPEN_GITLAB: &str = "OpenGitlab";
    }
    
    mod texts {
        pub const QUIT: &str = " Close ";
        pub const NAVIGATIONS: &str = " Navigations ";
        
        pub const OPEN_JIRA: &str = " Open Jira ";
        
        pub const OPEN_AWS: &str = " Open AWS ";

        pub const OPEN_GITLAB: &str = " Open Gitlab ";
    }

    mod urls {
        pub const JIRA: &str = "https://jira.vsct.fr/secure/RapidBoard.jspa?rapidView=4463&quickFilter=23849#";
        pub const AWS: &str = "http://aws_login";
        pub const GITLAB: &str = "https://gitlab.socrate.vsct.fr/invictus/invictus-root";
    }

    pub enum MenuItemIds {
        Quit,
        Navigations,
        Open {
            id: &'static str,
            url: &'static str,
        }
    }

    impl Display for MenuItemIds {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let label = match self {
                MenuItemIds::Quit => id_values::QUIT,
                MenuItemIds::Navigations => id_values::NAVIGATIONS,
                MenuItemIds::Open{ id, url: _} => id
            };
            write!(f, "{}", label)?;
            Ok(())
        }
    }

    impl From<MenuId> for MenuItemIds {
        fn from(id: MenuId) -> Self {
            match id.0.as_str() {
                id_values::QUIT => MenuItemIds::Quit,
                id_values::NAVIGATIONS => MenuItemIds::Navigations,
                id_values::OPEN_JIRA => MenuItemIds::from(id_values::OPEN_JIRA),
                id_values::OPEN_AWS =>  MenuItemIds::from(id_values::OPEN_AWS),
                id_values::OPEN_GITLAB => MenuItemIds::from(id_values::OPEN_GITLAB),
                _ => panic!("MenuItemIds::Quit is not a valid menu id"),
            }
        }
    }

    impl From<&str> for MenuItemIds {
        fn from(value: &str) -> Self {
            match value {
                id_values::OPEN_JIRA => MenuItemIds::Open {
                    id : id_values::OPEN_JIRA,
                    url : urls::JIRA,
                },
                id_values::OPEN_AWS => MenuItemIds::Open {
                    id : id_values::OPEN_AWS,
                    url : urls::AWS,
                },
                id_values::OPEN_GITLAB => MenuItemIds::Open {
                    id : id_values::OPEN_GITLAB,
                    url : urls::GITLAB,
                },
                _ => panic!("")
            }
        }
    }
    
    pub fn quit(app: &AppHandle) -> tauri::Result<MenuItem<Wry>> {
        MenuItem::with_id(app, MenuItemIds::Quit, texts::QUIT, true, None::<&str>)
    }

    pub fn workflows(app: &AppHandle) ->  tauri::Result<Submenu<Wry>> {
        let sb = SubmenuBuilder::with_id(app, MenuItemIds::Navigations, texts::NAVIGATIONS).build()?;
        sb.append_items(&[
            &workflow_item(app, id_values::OPEN_JIRA, texts::OPEN_JIRA, include_bytes!("../../icons/jira.png"), vec![Keys::CMD, Keys::J])?,
            &workflow_item(app, id_values::OPEN_AWS, texts::OPEN_AWS, include_bytes!("../../icons/aws.png"), vec![Keys::CMD, Keys::A])?,
            &workflow_item(app, id_values::OPEN_GITLAB, texts::OPEN_GITLAB, include_bytes!("../../icons/gitlab.png"), vec![Keys::CMD, Keys::G])?,
        ])?;
        
        Ok(sb)
    }

    fn workflow_item(app: &AppHandle, id: &'static str, text: &'static str, img: &[u8], accelerator_keys: Vec<Keys>) -> tauri::Result<IconMenuItem<Wry>> {
        let icon = Image::from_bytes(img).ok();
        IconMenuItem::with_id(app,
                              MenuItemIds::from(id),
                              text,
                              true,
                              icon,
                              Accelerator::of(accelerator_keys).build())
    }

}

pub mod accelerators {

    pub enum Keys {
        CMD,
        CTRL,
        J,
        A,
        G
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
            let v : Vec<String> = value.0.into_iter().map( move |k| String::from(k)).collect();
            v.join("+")
        }
    }
}

pub fn create_menu(app: &AppHandle) -> tauri::Result<Menu<Wry>> {

    Menu::with_items(app, &[
        &workflows(app)?,
        &PredefinedMenuItem::separator(app)?,
        &quit(app)?,
    ])
}

