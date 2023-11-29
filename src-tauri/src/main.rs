// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use net_process::{group_connections_by_pid, parse_connections, parse_processes, parse_stats};

#[tauri::command]
fn net_stat() -> String {
    // TODO: use cmd spawned from tauri instead of creating a new one
    //       each time this function is called
    let mut cmd = std::process::Command::new("netstat");
    cmd.arg("-e");
    cmd.arg("-s");
    cmd.stdout(std::process::Stdio::piped());
    cmd.stderr(std::process::Stdio::piped());

    let child = cmd.spawn().expect("failed to execute process");
    let result = child.wait_with_output().expect("failed to wait on child");
    if result.status.success() {
        let input = String::from_utf8(result.stdout).unwrap();
        let stats = parse_stats(input.as_str());
        serde_json::to_string(&stats).unwrap()
    } else {
        String::from_utf8(result.stderr).unwrap()
    }
}

#[tauri::command]
fn net_connections() -> String {
    // TODO: use cmd spawned from tauri instead of creating a new one
    //       each time this function is called
    let mut cmd = std::process::Command::new("netstat");
    cmd.arg("-ano");
    cmd.stdout(std::process::Stdio::piped());
    cmd.stderr(std::process::Stdio::piped());

    let child = cmd.spawn().expect("failed to execute process");
    let result = child.wait_with_output().expect("failed to wait on child");
    if result.status.success() {
        let input = String::from_utf8(result.stdout).unwrap();
        let connections = parse_connections(input.as_str());
        let grouped = group_connections_by_pid(connections);
        String::from_utf8(serde_json::to_string(&grouped).unwrap().into_bytes()).unwrap()
    } else {
        String::from_utf8(result.stderr).unwrap()
    }
}

#[tauri::command]
fn processes() -> String {
    // TODO: use cmd spawned from tauri instead of creating a new one
    //       each time this function is called
    let mut cmd = std::process::Command::new("tasklist");
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
