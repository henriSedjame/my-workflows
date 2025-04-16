
use crate::models::events::commands::{CommandExecutionEvent};

use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};
use std::time::Instant;
use tauri::{AppHandle, Manager, State};
use crate::models::events::commands::CommandExecutionEvent::{CommandStarted, CommandProgress, CommandFailed, CommandEnded};
use crate::models::state::{AppState, RunningCommand};
use sysinfo::{Pid, System};
use time::error::Format::StdIo;

#[tauri::command]
pub async fn execute_command(command_id: String, command_value: String, channel: tauri::ipc::Channel<CommandExecutionEvent>, state: State<'_, AppState>)  -> tauri::Result<bool> {
    
    channel.send(CommandStarted)?;
    
    let start = Instant::now();
    
    let mut command = Command::new("sh")
            .arg("-c")
            .arg(command_value.as_str())
            .stdout(Stdio::piped())
            
            .stderr(Stdio::piped())
            .spawn()?;


    let mut state_lock = state.lock().unwrap();

    state_lock.running_commands.push(RunningCommand {
        command_id,
        processs_id: command.id()
    });

    tauri::async_runtime::spawn(async move {
        /* handle output */
        {
            let stdout = command.stdout.as_mut().unwrap();
            let stdout_reader = BufReader::new(stdout);
            let stdout_lines = stdout_reader.lines();

            for line in stdout_lines.into_iter().flatten() {
                channel.send(CommandProgress {
                    progress_line: line,
                }).unwrap();
            }
        }

        /* handle input */
        {

        }

        /* handle errors */
        {
            let stdout = command.stderr.as_mut().unwrap();
            let stdout_reader = BufReader::new(stdout);
            let stdout_lines = stdout_reader.lines();

            let mut lines = vec![];

            for line in stdout_lines.into_iter().flatten() {
                lines.push(line);
            }

            if !lines.is_empty() {
                channel.send(CommandFailed { errors_lines: lines, }).unwrap();
            }
        }
        
        let status = command.wait().unwrap();
        
        println!("command status {status:?}");
        
        if status.success() {
            channel.send(CommandEnded { duration: start.elapsed().as_millis() }).unwrap();
        } else { 
            
        }

        
    });
    
    Ok(true)
}

#[tauri::command]
pub fn kill_command(command_id: String,app: AppHandle, state: State<'_, AppState>) -> tauri::Result<bool> {
    let webview = app.get_webview_window("main").unwrap();
    webview.eval("console.log('Start kill process')")?;
    
    let mut state_lock = state.lock().unwrap();
    if let Some(index) =  state_lock.running_commands.iter().position(|cmd| cmd.command_id == command_id) {
        webview.eval("console.log('Found index')")?;
        let system = System::new_all();
        let command = state_lock.running_commands.get(index).unwrap();
        webview.eval("console.log('found command')")?;
        if let Some(process) = system.process(Pid::from_u32(command.processs_id)) {
            if process.kill() {
                webview.eval("console.log('kill process')")?;
                state_lock.running_commands.remove(index);
            } else {
                
                webview.eval("console.log('fail to kill process')")?;
            }
        }
    }

    Ok(true)
}
