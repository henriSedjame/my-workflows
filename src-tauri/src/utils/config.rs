use crate::models::config::AppConfig;
use crate::models::errors::AppErrors;
use crate::models::{CONFIG_FILE_NAME, ICONS_DIR, WF_APP_DIR};
use std::fs;
use std::process::Command;

/// Get value of $HOME environment variable
pub fn get_home_dir() -> Result<String, AppErrors> {
    let pwd = Command::new("sh").arg("-c").arg("echo $HOME").output()?;
    Ok(String::from_utf8(pwd.stdout)?.trim().to_string())
}

/// Get configuration directory path
pub fn get_config_dir_path() -> Result<String, AppErrors> {
    let home_path = get_home_dir()?;
    Ok(format!("{home_path}/{WF_APP_DIR}"))
}

/// Get configuration file path
pub fn get_config_path() -> Result<String, AppErrors> {
    let home_path = get_home_dir()?;
    Ok(format!("{home_path}/{WF_APP_DIR}/{CONFIG_FILE_NAME}"))
}

/// Create __.wfapp__ directory and __config.json__ file if they do not exist yet
///
/// Returns the __config.json__ file path
pub fn create_config_dir() -> Result<String, AppErrors> {
    let home_path = get_home_dir()?;
    let wfapp_dir = format!("{home_path}/{WF_APP_DIR}");

    let config_path = format!("{wfapp_dir}/{CONFIG_FILE_NAME}");
    let icons_dir = format!("{wfapp_dir}/{ICONS_DIR}");
    
    if let Ok(false) = fs::exists(&wfapp_dir) {
        fs::create_dir(wfapp_dir)?;
    }

    if let Ok(false) = fs::exists(&icons_dir) {
        fs::create_dir(icons_dir)?;
    }
    
    if let Ok(false) = fs::exists(&config_path) {
        
        let mut config_file = fs::File::create(&config_path)?;

        let config = AppConfig::default();

        serde_json::to_writer_pretty(&mut config_file, &config)?;
        
    }
    
    Ok(config_path)
}

/// Get the path to the icons directory
pub fn get_config_icons_path(icon: String) -> Result<String, AppErrors> {
    let home_dir = get_home_dir()?;
    let path = format!("{home_dir}/{WF_APP_DIR}/{ICONS_DIR}/{icon}");
    Ok(path)
}
