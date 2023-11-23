// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use net_process::parse_stats;

#[tauri::command]
fn net_stat() -> String {
    let mut cmd = std::process::Command::new("netstat");
    cmd.arg("-e");
    cmd.arg("-s");
    cmd.stdout(std::process::Stdio::piped());
    cmd.stderr(std::process::Stdio::piped());

    let child = cmd.spawn().expect("failed to execute process");
    let result = child.wait_with_output().expect("failed to wait on child");
    if result.status.success() {
        let stats = parse_stats(String::from_utf8(result.stdout).unwrap().as_str());
        serde_json::to_string(&stats).unwrap()
    } else {
        String::from_utf8(result.stderr).unwrap()
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![net_stat])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
