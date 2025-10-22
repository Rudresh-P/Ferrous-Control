use local_ip_address::local_ip;
use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Serialize, Deserialize)]
struct CommandResponse {
    success: bool,
    message: String,
}

#[tauri::command]
fn shutdown() -> CommandResponse {
    println!("Shutdown request received via Tauri");

    let result = if cfg!(target_os = "windows") {
        Command::new("shutdown").args(["/s", "/t", "60"]).spawn()
    } else if cfg!(target_os = "linux") {
        Command::new("shutdown").args(["-h", "+1"]).spawn()
    } else if cfg!(target_os = "macos") {
        Command::new("shutdown").args(["-h", "+1"]).spawn()
    } else {
        return CommandResponse {
            success: false,
            message: "Unsupported operating system".to_string(),
        };
    };

    match result {
        Ok(_) => CommandResponse {
            success: true,
            message: "Shutdown command executed".to_string(),
        },
        Err(e) => CommandResponse {
            success: false,
            message: format!("Failed to execute shutdown: {}", e),
        },
    }
}

#[tauri::command]
fn restart() -> CommandResponse {
    println!("Restart request received via Tauri");

    let result = if cfg!(target_os = "windows") {
        Command::new("shutdown").args(["/r", "/t", "0"]).spawn()
    } else if cfg!(target_os = "linux") {
        Command::new("shutdown").args(["-r", "now"]).spawn()
    } else if cfg!(target_os = "macos") {
        Command::new("shutdown").args(["-r", "now"]).spawn()
    } else {
        return CommandResponse {
            success: false,
            message: "Unsupported operating system".to_string(),
        };
    };

    match result {
        Ok(_) => CommandResponse {
            success: true,
            message: "Restart command executed".to_string(),
        },
        Err(e) => CommandResponse {
            success: false,
            message: format!("Failed to execute restart: {}", e),
        },
    }
}

#[tauri::command]
fn cancel_shutdown() -> CommandResponse {
    println!("Cancel shutdown request received via Tauri");

    let result = if cfg!(target_os = "windows") {
        Command::new("shutdown").args(["/a"]).spawn()
    } else if cfg!(target_os = "linux") {
        Command::new("shutdown").args(["-c"]).spawn()
    } else if cfg!(target_os = "macos") {
        Command::new("killall").args(["shutdown"]).spawn()
    } else {
        return CommandResponse {
            success: false,
            message: "Unsupported operating system".to_string(),
        };
    };

    match result {
        Ok(_) => CommandResponse {
            success: true,
            message: "Shutdown cancelled".to_string(),
        },
        Err(e) => CommandResponse {
            success: false,
            message: format!("Failed to cancel shutdown: {}", e),
        },
    }
}

#[tauri::command]
fn get_local_ip() -> String {
    match local_ip() {
        Ok(ip) => ip.to_string(),
        Err(_) => "Unable to get IP".to_string(),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::Builder::new().build())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            shutdown,
            restart,
            cancel_shutdown,
            get_local_ip
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
