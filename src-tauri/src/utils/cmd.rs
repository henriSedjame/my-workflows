use crate::commands::{execute_and_get, ENV_PREFIX, VARS_PREFIX};
use crate::models::errors::AppErrors;
use crate::models::state::AppState;
use tauri::{AppHandle, Manager};

pub fn evaluate_cmd_value(app: &AppHandle, cmd: String) -> Result<String, AppErrors> {
    let mut result = String::new();

    for cmd in cmd.trim().split(' ') {
        if cmd.starts_with(ENV_PREFIX) {
            let env_name = cmd.replace(ENV_PREFIX, "");
            let cmd = format!("echo ${}", env_name);
            if let Some(s) = execute_and_get(cmd.as_str(), || {}) {
                println!("ENV => {}", s);
                result = result + " " + &s;
            } else {
                return Err(AppErrors::EnvNotFound(env_name));
            }
        } else if cmd.starts_with(VARS_PREFIX) {
            let var_name = cmd.replace(VARS_PREFIX, "");
            if let Some(s) = get_var(app, var_name.as_str()) {
                result = result + " " + &s;
            } else {
                return Err(AppErrors::VarNotFound(var_name));
            }
        } else {
            result = result + " " + cmd;
        }
    }

    Ok(result)
}

pub fn get_var(app: &AppHandle, var_name: &str) -> Option<String> {
    let state = app.state::<AppState>();
    let vars = &state.clone().config.variables;
    vars.get(var_name).cloned()
}
