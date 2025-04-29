use std::io;
use std::string::FromUtf8Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppErrors {
    #[error("Failed to read $HOME environment variable")]
    FailedToReadHome(#[from] FromUtf8Error),
    #[error("Failed to read config file")]
    FailedToReadConfigFile(#[from] io::Error),
    #[error("{0}")]
    FailedToParseConfig(#[from] serde_json::error::Error),
    #[error("Env {0} not found")]
    EnvNotFound(String),
    #[error("Variable {0} not found")]
    VarNotFound(String),
    #[error("Secret {0} not found")]
    SecretNotFound(String),
    #[error("{0}")]
    EmitEventError(String),
    #[error("Failed to close app ({0})")]   
    FailedToCloseApp(tauri::Error),
    #[error("Failed to show app ({0})")]
    FailedToShowApp(tauri::Error),
    #[error("Failed to hide app ({0})")]
    FailedToHideApp(tauri::Error),
    #[error("Failed to focus on app view ({0})")]
    FailedToFocusOnAppView(tauri::Error),
    #[error("Failed to execute command:\n{0}")]   
    FailedToExecuteCommand(String),
    #[error("{0}")]  
    CommandFailed(String),
    #[error("Failed to set tray menu")] 
    FailedToSetTrayMenu,
    #[error("Failed to remove tray menu item {0}")]
    FailedToRemoveTrayMenuItem(String),
    #[error("Failed to retrieve tray menu items")]
    FailedToRetrieveMenuItems,
    #[error("Failed to create tray menu item {0}")]
    FailedToCreateItem(String),
    #[error("Failed to insert tray menu item {0}")]
    FailedToInsertMenuItem(String),
    #[error("Failed to open url {0}")]
    FailedToOpenUrl(String),
}
