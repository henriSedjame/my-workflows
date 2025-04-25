use std::process::{Command, Output};
use crate::models::errors::AppErrors;
use crate::models::state::AppState;
use regex::Captures;
use tauri::{AppHandle, Manager};
pub(crate) use crate::utils::constants::{ENVS, ENV_PREFIX, VARS, VARS_PREFIX, SECRETS, SECRETS_PREFIX, PARAMS, PARAM_PREFIX, SUFFIX, MODIFIED_SECRET, PROGRAM_NAME};



#[derive(Debug)]
pub struct EvaluatedCmd {
    pub original_cmd: String,
    pub modified_cmd: String,
    pub parameters: Option<Vec<String>>,
}


pub fn evaluate_cmd_value(app: &AppHandle, cmd: String) -> Result<EvaluatedCmd, AppErrors> {
    let mut original_cmd = cmd.clone();
    let mut modified_cmd = cmd.clone();
    let mut parameters : Vec<String> = vec![];

    let regex = regex::Regex::new(
        r"(?<envs>(\$\{env.\w+}))|(?<variables>(\$\{vars.\w+}))|(?<secrets>(\$\{secrets.\w+}))|(?<params>(\$\{param.\w+}))",
    )
    .unwrap();

    for cap in regex.captures_iter(cmd.as_ref()) {
        handle_capture(
            &cap,
            ENVS,
            ENV_PREFIX,
            |s| {
                let cmd = format!("echo ${s}");
                execute_and_get(cmd.as_str(), || {})
            },
            |match_str, s| {
                original_cmd = original_cmd.replace(match_str, s.as_str());
                modified_cmd = modified_cmd.replace(match_str, s.as_str());
            },
            |s| Some(AppErrors::EnvNotFound(s)),
        )?;

        handle_capture(
            &cap,
            VARS,
            VARS_PREFIX,
            |s| get_var(app, s.as_str()) ,
            |match_str, s| {
                original_cmd = original_cmd.replace(match_str, s.as_str());
                modified_cmd = modified_cmd.replace(match_str, s.as_str());
            },
            |s| Some(AppErrors::VarNotFound(s)),
        )?;

        handle_capture(
            &cap,
            SECRETS,
            SECRETS_PREFIX,
            |s| get_secret(app, s.as_str()) ,
            |match_str, s| {
                original_cmd = original_cmd.replace(match_str, s.as_str());
                modified_cmd = modified_cmd.replace(match_str, MODIFIED_SECRET);
            },
            |s| Some(AppErrors::SecretNotFound(s)),
        )?;

        handle_capture(
            &cap,
            PARAMS,
            PARAM_PREFIX,
            |s| Some(s) ,
            |_match_str, s| parameters.push(s),
            |_s| None,
        )?;
    }

    Ok(EvaluatedCmd {
        original_cmd,
        modified_cmd,
        parameters: if parameters.is_empty() { None } else { Some(parameters) },
    })
}

pub fn handle_capture(
    cap: &Captures,
    name: &str,
    prefix: &str,
    getter: impl Fn(String) -> Option<String>,
    mut transformer: impl FnMut(&str, String) -> (),
    err: impl Fn(String) -> Option<AppErrors>,
) -> Result<(), AppErrors> {
    if let Some(m) = cap.name(name) {
        let match_str = m.as_str().trim();
        let var_name = match_str.replace(prefix, "");
        if let Some(s) = getter(var_name.to_string().replace(SUFFIX, "")) {
            transformer(match_str, s);
        } else {
            if let Some(e) = err(var_name){
                return Err(e);
            }
        }
    }

    Ok(())
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
        Command::new(PROGRAM_NAME)
            .arg("-c")
            .arg(cmd)
            .output()
            .unwrap()
    });

    if output.status.success() {
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
        let value = String::from_utf8(output.stdout).unwrap().trim().to_string();
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
