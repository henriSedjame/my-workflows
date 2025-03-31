use crate::models::config::AppConfig;
use crate::models::errors::AppErrors;
use crate::utils::config::create_config_dir;
use std::fs;

pub struct AppState {
    pub config: AppConfig,
}

impl AppState {
    pub(crate) fn new() -> Result<Self, AppErrors> {
        let config_path = create_config_dir()?;

        let config_str = fs::read_to_string(config_path.as_str())?;

        match serde_json::from_str::<AppConfig>(config_str.as_str()) {
            Ok(config) => {
                Ok(AppState { config })
            }
            Err(e) => {
                println!("Config JSON => {}", e);
                Err(AppErrors::FailedToParseConfig(e))
            }
        }
    }
}
