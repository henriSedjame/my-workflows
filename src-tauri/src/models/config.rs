use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct AppConfig {
    pub variables: HashMap<String, String>,
    pub navigations: Vec<Navigation>,
    pub commands: Vec<Cmd>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Navigation {
    pub name: String,
    pub url: String,
    pub icon: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Cmd {
    pub name: String,
    pub cmd: String,
}
