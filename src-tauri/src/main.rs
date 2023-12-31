// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use net_process::{group_connections_by_pid, parse_connections, parse_processes, parse_stats};
use tauri::api::process::{Command, CommandEvent};

#[tauri::command]
async fn net_stat() -> String {
    // TODO: use cmd spawned from tauri instead of creating a new one
    //       each time this function is called
    let (mut rx, _child) = Command::new("netstat")
        .args(["-s", "-e"])
        .spawn()
        .expect("failed to execute process");

    let mut input = String::new();
    while let Some(event) = rx.recv().await {
        let line = match event {
            CommandEvent::Stdout(line) => line,
            CommandEvent::Stderr(line) => line,
            CommandEvent::Terminated(_) => break,
            _ => continue,
        };
        input.push_str(line.as_str());
    }
    let stats = parse_stats(input.as_str());
    serde_json::to_string(&stats).unwrap()
}

#[tauri::command]
async fn net_connections() -> String {
    // TODO: use cmd spawned from tauri instead of creating a new one
    //       each time this function is called
    let (mut rx, _child) = Command::new("netstat")
        .args(["-ano"])
        .spawn()
        .expect("failed to execute process");

    let mut input = String::new();
    while let Some(event) = rx.recv().await {
        let line = match event {
            CommandEvent::Stdout(line) => line,
            CommandEvent::Stderr(line) => line,
            CommandEvent::Terminated(_) => break,
            _ => continue,
        };
        input.push_str(line.as_str());
    }
    let connections = parse_connections(input.as_str());
    let grouped = group_connections_by_pid(connections);
    String::from_utf8(serde_json::to_string(&grouped).unwrap().into_bytes()).unwrap()
}

#[tauri::command]
fn processes() -> String {
    // TODO: use cmd spawned from tauri instead of creating a new one
    //       each time this function is called
    let mut cmd = std::process::Command::new("tasklist");
    cmd.args(["/fo", "csv"]);
    cmd.stdout(std::process::Stdio::piped());
    cmd.stderr(std::process::Stdio::piped());

    let child = cmd.spawn().expect("failed to execute process");
    let result = child.wait_with_output().expect("failed to wait on child");
    if result.status.success() {
        // FIXME: for some reason, the output of tasklist is not utf8,
        //        and I cannot figure out why. For now, we just convert
        //        the bytes to chars and then back to a string.
        let input = result
            .stdout
            .iter()
            .map(|x| char::from(*x))
            .collect::<String>();
        let processes = parse_processes(input.as_str());
        String::from_utf8(serde_json::to_string(&processes).unwrap().into_bytes()).unwrap()
    } else {
        String::from_utf8(result.stderr).unwrap()
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            net_stat,
            net_connections,
            processes
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
