use crate::models::config::AppConfig;
use crate::models::errors::AppErrors;
use crate::utils::config::create_config_dir;
use std::fs;
use std::sync::Mutex;
use tauri::menu::Menu;
use tauri::Wry;

pub struct RunningCommand {
    pub command_id: String,
    pub command_value: String,
}

pub struct AppStateInner {
    pub config: AppConfig,
    pub menu: Option<Menu<Wry>>,
    pub running_commands: Vec<RunningCommand>,
    pub view_visible: bool
}

impl AppStateInner {
    pub(crate) fn new() -> Result<Self, AppErrors> {
        let config_path = create_config_dir()?;

        let config_str = fs::read_to_string(config_path.as_str())?;

        match serde_json::from_str::<AppConfig>(config_str.as_str()) {
            Ok(config) => Ok(AppStateInner {
                config,
                menu: None,
                running_commands: vec![],
                view_visible: false
            }),
            Err(e) => {
                println!("Config JSON => {}", e);
                Err(AppErrors::FailedToParseConfig(e))
            }
        }
    }
}

pub(crate) type AppState = Mutex<AppStateInner>;
