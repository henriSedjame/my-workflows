use std::process::Command;
use crate::utils::cmd::PROGRAM_NAME;

pub fn get_command_process_ids(command: &str) -> tauri::Result<Vec<u32>> {
    
    let p1 = "ps -fu $USER";
    let p2 = "| grep '";
    let p3 = "' | grep -v 'grep' | awk '{print $2}'";
    let cmd = format!("{p1}{p2}{command}{p3}");

    let output = Command::new(PROGRAM_NAME)
        .arg("-c")
        .arg(cmd)
        .output()?;
    
    let ids =  String::from_utf8_lossy(&*output.stdout)
        .trim()
        .split("\n")
        .map(|s| s.trim().to_string().parse().unwrap())
        .collect();
    
    Ok(ids)
}