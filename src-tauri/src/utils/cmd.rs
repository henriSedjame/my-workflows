use std::process::{Command, Output};

use crate::models::errors::AppErrors;
use crate::models::state::AppState;
use tauri::{AppHandle, Manager};

pub const PROGRAM_NAME: &str = "sh";

pub const ENV_PREFIX: &str = "$env.";
pub const VARS_PREFIX: &str = "$vars.";
pub const SECRETS_PREFIX: &str = "$secrets.";

pub struct EvaluatedCmd {
    pub original_cmd: String,
    pub modified_cmd: String,
}
pub fn evaluate_cmd_value(app: &AppHandle, cmd: String) -> Result<EvaluatedCmd, AppErrors> {
    let mut result = String::new();
    let mut modified = String::new();

    for cmd in cmd.trim().split(' ') {
        if cmd.starts_with(ENV_PREFIX) {
            let env_name = cmd.replace(ENV_PREFIX, "");
            let cmd = format!("echo ${}", env_name);
            if let Some(s) = execute_and_get(cmd.as_str(), || {}) {
                result = result + " " + &s;
                modified = modified + " " + &s;
            } else {
                return Err(AppErrors::EnvNotFound(env_name));
            }
        } else if cmd.starts_with(VARS_PREFIX) {
            let var_part = *cmd.split("/").collect::<Vec<&str>>().first().unwrap();
            let remaining_part = cmd.replace(var_part, "");
            let var_name = var_part.replace(VARS_PREFIX, "");
            if let Some(s) = get_var(app, var_name.as_str()) {
                result = result + " " + &s + &remaining_part;
                modified = modified + " " + &s + &remaining_part;
            } else {
                return Err(AppErrors::VarNotFound(var_name));
            }
            
        }  else if cmd.starts_with(SECRETS_PREFIX) {
            let var_name = cmd.replace(SECRETS_PREFIX, "");
            if let Some(s) = get_secret(app, var_name.as_str()) {
                result = result + " " + &s;
                modified = modified + " " + "🔐{hidden}";
            } else {
                return Err(AppErrors::SecretNotFound(var_name));
            }
        } else {
            result = result + " " + cmd;
            modified = modified + " " + cmd;
        }
    }

    Ok(EvaluatedCmd {
        original_cmd: result,
        modified_cmd: modified,
    })
}

pub fn get_var(app: &AppHandle, var_name: &str) -> Option<String> {
    let state = app.state::<AppState>();
    let vars = &state.lock().unwrap().config.variables;
    vars.get(var_name).cloned()
}

pub fn get_secret(app: &AppHandle, var_name: &str) -> Option<String> {
    let state = app.state::<AppState>();
    let secrets = &state.lock().unwrap().config.secrets;
    secrets.get(var_name).cloned()
}

pub fn execute_cmd(cmd: &str, handler: impl Fn(Output), on_error: impl Fn()) {
    let output = tauri::async_runtime::block_on(async move {
        println!("Executing command: {}", cmd);
        Command::new(PROGRAM_NAME)
            .arg("-c")
            .arg(cmd)
            .output()
            .unwrap()
    });

    if output.status.success()  {
        handler(output);
    } else {
        on_error();
    }
}

pub fn execute_and_get(cmd: &str, on_error: impl Fn()) -> Option<String> {
    let output = tauri::async_runtime::block_on(async move {
        Command::new(PROGRAM_NAME)
            .arg("-c")
            .arg(cmd)
            .output()
            .unwrap()
    });

    if output.status.success() {
        let value = String::from_utf8(output.stdout).unwrap();
        if value.is_empty() {
            None
        } else {
            Some(value)
        }
    } else {
        on_error();
        None
    }
}