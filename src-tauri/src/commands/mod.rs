use crate::models::errors::AppErrors;
use std::io::{BufRead, BufReader};
use std::process::{Command, Output, Stdio};

pub const PROGRAM_NAME: &str = "sh";

pub const ENV_PREFIX: &str = "$env.";
pub const VARS_PREFIX: &str = "$vars.";

pub fn execute_and_handle(
    cmd: &str,
    handler: impl Fn(Output),
    on_error: impl Fn(),
) -> Result<(), AppErrors> {
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

    Ok(())
}

pub fn execute(cmd: &str, on_error: impl Fn()) -> Result<(), AppErrors> {
    execute_and_handle(cmd, |_| {}, on_error)
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

pub fn execute_and_stream_as_event(cmd: &str, on_error: impl Fn()) {
    let mut cmd_child = tauri::async_runtime::block_on(async move {
        Command::new(PROGRAM_NAME)
            .arg("-c")
            .arg(cmd)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap()
    });

    /* handle output */
    {
        let stdout = cmd_child.stdout.as_mut().unwrap();
        let stdout_reader = BufReader::new(stdout);
        let stdout_lines = stdout_reader.lines();

        for line in stdout_lines {
            println!("Read: {:?}", line);
        }
    }

    /* handle error */
    {
        let stdout = cmd_child.stderr.as_mut().unwrap();
        let stdout_reader = BufReader::new(stdout);
        let stdout_lines = stdout_reader.lines();

        for line in stdout_lines {
            println!("Read: {:?}", line);
        }
    }

    let status = cmd_child.wait().unwrap();
}
