use std::io;
use std::process::{Command, Output};
use crate::models::errors::AppErrors;
use crate::models::state::AppState;
use regex::Captures;
use tauri::{AppHandle, Manager};
use crate::models::errors::AppErrors::{CommandFailed, FailedToExecuteCommand};
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
                execute_and_get(cmd.as_str(), "Failed to get env variable", AppErrors::EnvNotFound(s.as_str().to_string()))
            },
            |match_str, s| {
                original_cmd = original_cmd.replace(match_str, s.as_str());
                modified_cmd = modified_cmd.replace(match_str, s.as_str());
            }
        )?;

        handle_capture(
            &cap,
            VARS,
            VARS_PREFIX,
            |s| get_var(app, s.as_str()),
            |match_str, s| {
                original_cmd = original_cmd.replace(match_str, s.as_str());
                modified_cmd = modified_cmd.replace(match_str, s.as_str());
            }
        )?;

        handle_capture(
            &cap,
            SECRETS,
            SECRETS_PREFIX,
            |s| get_secret(app, s.as_str()),
            |match_str, s| {
                original_cmd = original_cmd.replace(match_str, s.as_str());
                modified_cmd = modified_cmd.replace(match_str, MODIFIED_SECRET);
            }
        )?;

        handle_capture(
            &cap,
            PARAMS,
            PARAM_PREFIX,
            |s| Ok(s) ,
            |_match_str, s| parameters.push(s),
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
    getter: impl Fn(String) -> Result<String, AppErrors>,
    mut transformer: impl FnMut(&str, String) -> (),
) -> Result<(), AppErrors> {
    if let Some(m) = cap.name(name) {
        let match_str = m.as_str().trim();
        let var_name = match_str.replace(prefix, "");
        let s = getter(var_name.to_string().replace(SUFFIX, ""))?;
        transformer(match_str, s);
    }

    Ok(())
}

pub fn get_var(app: &AppHandle, var_name: &str) ->  Result<String, AppErrors> {
    let state = app.state::<AppState>();
    let vars = &state.lock().unwrap().config.variables;
    match vars.get(var_name) {
        Some(var) => Ok(var.clone()),
        None => Err(AppErrors::VarNotFound(var_name.to_string()))
    }
}

pub fn get_secret(app: &AppHandle, var_name: &str) -> Result<String, AppErrors> {
    let state = app.state::<AppState>();
    let secrets = &state.lock().unwrap().config.secrets;
    match secrets.get(var_name) {
        Some(secret) => Ok(secret.clone()),
        None => Err(AppErrors::SecretNotFound(var_name.to_string()))
    }

}

pub fn execute_cmd(cmd: &str, handler: impl Fn(Output), err_msg: &str) -> Result<(), AppErrors> {
    let output: io::Result<Output> = tauri::async_runtime::block_on(async move {
        Command::new(PROGRAM_NAME)
            .arg("-c")
            .arg(cmd)
            .output()
    });

    match output {
        Ok(output) => {
            let status = output.status;
            if status.success() {
                handler(output);
                Ok(())
            } else {
                Err(CommandFailed(err_msg.to_string()))
            }
        },
        Err(e) => {
            Err(FailedToExecuteCommand(format!(" Failed to execute command {} \n ({e})", cmd.to_string())))
        }
    }

}

pub fn execute_and_get(cmd: &str, err_msg: &str, empty_output_err: AppErrors) -> Result<String, AppErrors> {
    let output = tauri::async_runtime::block_on(async move {
        Command::new(PROGRAM_NAME)
            .arg("-c")
            .arg(cmd)
            .output()
    });

    match output {
        Ok(output) => {
            let status = output.status;
            if status.success() {
                let value = String::from_utf8(output.stdout)?.trim().to_string();
                if value.is_empty() {
                    return Err(empty_output_err);
                }
                Ok(value)
            } else {
                Err(CommandFailed(err_msg.to_string()))
            }
        },
        Err(e) => {
            Err(FailedToExecuteCommand(format!(" Failed to execute command {}, ({e})", cmd.to_string())))
        }
    }

}
