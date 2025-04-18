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
    EmitEventError(String)
}
