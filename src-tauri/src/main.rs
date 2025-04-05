// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::io::{BufRead, BufReader, BufWriter, Write};
use std::process::{Command, Stdio};

fn main() {
    tauri_angular_app_lib::run()

    /*let mut cmd =
        Command::new("zsh")
            .arg("-c")
            .arg("awssec -u henri_sedjame -p 478859035381 -r ADFS-EVOYAGEURS_DEV")
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .stdin(Stdio::piped())
            .spawn()
            
            .unwrap();

    {
        let stdout = cmd.stdout.as_mut().unwrap();
        let stdout_reader = BufReader::new(stdout);
        let stdout_lines = stdout_reader.lines();

        for line in stdout_lines {
            println!("Read: {:?}", line);
        }
    }
    {
        let stdin = cmd.stdin.as_mut().unwrap();
        let mut stdout_reader = BufWriter::new(stdin);
        let stdout_lines = stdout_reader.write("\r\n".as_bytes()).unwrap();
        
    }
    // It's streaming here

    let status = cmd.wait();
    println!("Exited with status {:?}", status);*/
}
