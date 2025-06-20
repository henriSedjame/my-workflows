use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct AppConfig {
    pub variables: HashMap<String, String>,
    pub secrets: HashMap<String, String>,
    pub path: String,
    pub navigations: Vec<Navigation>,
    pub commands: Vec<Command>,
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

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CmdGrp {
    pub name: String,
    pub commands: Vec<Command>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum Command {
    Simple(Cmd),
    Group(CmdGrp),
}