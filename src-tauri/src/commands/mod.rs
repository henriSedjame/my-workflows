use crate::models::events::commands::CommandExecutionEvent;

use crate::models::events::commands::CommandExecutionEvent::{
    CommandEnded, CommandFailed, CommandProgress, CommandStarted,
};
use crate::models::state::{AppState, RunningCommand};
use crate::utils::process::get_command_process_ids;
use crate::utils::{hide_main_view, update_tray_menu};
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::time::Instant;
use tauri::{AppHandle, State};

#[tauri::command]
pub async fn execute_command(
    app: AppHandle,
    command_id: String,
    command_value: String,
    channel: tauri::ipc::Channel<CommandExecutionEvent>,
    state: State<'_, AppState>,
) -> tauri::Result<bool> {
    channel.send(CommandStarted)?;

    let start = Instant::now();

    let mut state_lock = state.lock().unwrap();

    let path = state_lock.config.path.clone();

    let script = format!("export PATH={path} && {command_value}");

    let mut command = Command::new("sh")
        .arg("-c")
        .arg(script.as_str())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    state_lock.view_visible = true;

    state_lock.running_commands.push(RunningCommand {
        command_id: command_id.clone(),
        command_value: command_value.clone(),
    });

    tauri::async_runtime::spawn(async move {
        /* handle output */
        {
            let stdout = command.stdout.as_mut().unwrap();
            let stdout_reader = BufReader::new(stdout);
            let stdout_lines = stdout_reader.lines();

            for line in stdout_lines.into_iter().flatten() {
                channel
                    .send(CommandProgress {
                        progress_line: line,
                    })
                    .unwrap();
            }
        }

        let status = command.wait().unwrap();

        if status.success() {
            channel
                .send(CommandEnded {
                    duration: start.elapsed().as_millis(),
                    status_code: status.code().unwrap(),
                })
                .unwrap();
        } else {
            let stdout = command.stderr.as_mut().unwrap();
            let stdout_reader = BufReader::new(stdout);
            let stdout_lines = stdout_reader.lines();

            let mut lines = vec![];

            for line in stdout_lines.into_iter().flatten() {
                lines.push(line);
            }

            if !lines.is_empty() {
                channel
                    .send(CommandFailed {
                        errors_lines: lines,
                        duration: start.elapsed().as_millis(),
                        status_code: status.code().or_else(|| Some(-1)).unwrap(),
                    })
                    .unwrap();
            } else {
                channel
                    .send(CommandEnded {
                        duration: start.elapsed().as_millis(),
                        status_code: status.code().or_else(|| Some(-1)).unwrap(),
                    })
                    .unwrap();
            }
        }
    });

    tauri::async_runtime::spawn(async move {
        update_tray_menu(&app);
    });

    Ok(true)
}

#[tauri::command]
pub fn kill_command(
    command_id: String,
    state: State<'_, AppState>,
) -> tauri::Result<bool> {
   
    let mut state_lock = state.lock().unwrap();
    
    if let Some(index) = state_lock
        .running_commands
        .iter()
        .position(|cmd| cmd.command_id == command_id)
    {
        let command = state_lock.running_commands.get(index).unwrap();

        let mut all_killed = true;

        for pid in get_command_process_ids(command.command_value.as_str())? {
            if nix::sys::signal::kill(
                nix::unistd::Pid::from_raw(pid as i32),
                Some(nix::sys::signal::Signal::SIGINT),
            ).is_err() { all_killed = false; };
        }

        if all_killed {
            state_lock.running_commands.remove(index);
        }
    }

    Ok(true)
}

#[tauri::command]
pub fn hide_view(app: AppHandle, state: State<'_, AppState>, open_tabs: bool) -> tauri::Result<()> {
    let mut state_lock = state.lock().unwrap();
    if !open_tabs {
        state_lock.view_visible = false;
    }
    hide_main_view(&app);
    tauri::async_runtime::spawn(async move {
        update_tray_menu(&app);
    });
    Ok(())
}
