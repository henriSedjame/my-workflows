use std::collections::HashSet;
use std::process::Command;
use tauri::Error;
use crate::utils::cmd::PROGRAM_NAME;

pub fn get_command_process_ids(command: &str) -> tauri::Result<Vec<u32>> {
    let p1 = "ps -fu $USER";
    let p2 = "| grep '";
    let p3 = "' | grep -v 'grep' | awk '{print $2}'";
    
    let ids = command.split("&&").map(|command|{
        let cmd = format!("{p1}{p2}{command}{p3}");

        let output = Command::new(PROGRAM_NAME)
            .arg("-c")
            .arg(cmd)
            .output().unwrap();

        let ids: Vec<u32> =  String::from_utf8_lossy(&*output.stdout)
            .trim()
            .split("\n")
            .map(|s| {
                let value = s.trim().to_string();
                if value.is_empty() {
                    return 0;
                }
                value.parse().or::<Error>(Ok(0)).unwrap()
            })
            .filter(|&x| x != 0)
            .collect();
        ids
    }).flatten()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    
    Ok(ids)
}