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
fn sleep() -> CommandResponse {
    println!("Sleep request received via Tauri");

    let result = if cfg!(target_os = "windows") {
        Command::new("rundll32.exe")
            .args(["powrprof.dll,SetSuspendState", "0,1,0"])
            .spawn()
    } else if cfg!(target_os = "linux") {
        Command::new("systemctl").args(["suspend"]).spawn()
    } else if cfg!(target_os = "macos") {
        Command::new("pmset").args(["sleepnow"]).spawn()
    } else {
        return CommandResponse {
            success: false,
            message: "Unsupported operating system".to_string(),
        };
    };

    match result {
        Ok(_) => CommandResponse {
            success: true,
            message: "Sleep command executed".to_string(),
        },
        Err(e) => CommandResponse {
            success: false,
            message: format!("Failed to execute sleep: {}", e),
        },
    }
}

#[tauri::command]
fn increase_volume(amount: Option<i32>) -> CommandResponse {
    let volume_change = amount.unwrap_or(2);
    println!("Increase volume request received via Tauri (amount: {})", volume_change);

    let result = if cfg!(target_os = "windows") {
        #[cfg(target_os = "windows")]
        {
            use std::os::windows::process::CommandExt;
            const CREATE_NO_WINDOW: u32 = 0x08000000;

            Command::new("powershell")
                .creation_flags(CREATE_NO_WINDOW)
                .args([
                    "-Command",
                    &format!(
                        "$obj = New-Object -ComObject WScript.Shell; for($i=0; $i -lt {}; $i++) {{ $obj.SendKeys([char]175) }}",
                        volume_change / 2
                    )
                ])
                .spawn()
        }
        #[cfg(not(target_os = "windows"))]
        {
            return CommandResponse {
                success: false,
                message: "Windows-only code path".to_string(),
            };
        }
    } else if cfg!(target_os = "linux") {
        // Try pactl (PulseAudio) first, fallback to amixer (ALSA)
        let pactl_result = Command::new("pactl")
            .args(["set-sink-volume", "@DEFAULT_SINK@", &format!("+{}%", volume_change)])
            .spawn();

        if pactl_result.is_ok() {
            pactl_result
        } else {
            Command::new("amixer")
                .args(["set", "Master", &format!("{}%+", volume_change)])
                .spawn()
        }
    } else if cfg!(target_os = "macos") {
        Command::new("osascript")
            .args([
                "-e",
                &format!(
                    "set volume output volume (output volume of (get volume settings) + {})",
                    volume_change
                )
            ])
            .spawn()
    } else {
        return CommandResponse {
            success: false,
            message: "Unsupported operating system".to_string(),
        };
    };

    match result {
        Ok(_) => CommandResponse {
            success: true,
            message: format!("Volume increased by {}", volume_change),
        },
        Err(e) => CommandResponse {
            success: false,
            message: format!("Failed to increase volume: {}", e),
        },
    }
}

#[tauri::command]
fn decrease_volume(amount: Option<i32>) -> CommandResponse {
    let volume_change = amount.unwrap_or(2);
    println!("Decrease volume request received via Tauri (amount: {})", volume_change);

    let result = if cfg!(target_os = "windows") {
        #[cfg(target_os = "windows")]
        {
            use std::os::windows::process::CommandExt;
            const CREATE_NO_WINDOW: u32 = 0x08000000;

            Command::new("powershell")
                .creation_flags(CREATE_NO_WINDOW)
                .args([
                    "-Command",
                    &format!(
                        "$obj = New-Object -ComObject WScript.Shell; for($i=0; $i -lt {}; $i++) {{ $obj.SendKeys([char]174) }}",
                        volume_change / 2
                    )
                ])
                .spawn()
        }
        #[cfg(not(target_os = "windows"))]
        {
            return CommandResponse {
                success: false,
                message: "Windows-only code path".to_string(),
            };
        }
    } else if cfg!(target_os = "linux") {
        // Try pactl (PulseAudio) first, fallback to amixer (ALSA)
        let pactl_result = Command::new("pactl")
            .args(["set-sink-volume", "@DEFAULT_SINK@", &format!("-{}%", volume_change)])
            .spawn();

        if pactl_result.is_ok() {
            pactl_result
        } else {
            Command::new("amixer")
                .args(["set", "Master", &format!("{}%-", volume_change)])
                .spawn()
        }
    } else if cfg!(target_os = "macos") {
        Command::new("osascript")
            .args([
                "-e",
                &format!(
                    "set volume output volume (output volume of (get volume settings) - {})",
                    volume_change
                )
            ])
            .spawn()
    } else {
        return CommandResponse {
            success: false,
            message: "Unsupported operating system".to_string(),
        };
    };

    match result {
        Ok(_) => CommandResponse {
            success: true,
            message: format!("Volume decreased by {}", volume_change),
        },
        Err(e) => CommandResponse {
            success: false,
            message: format!("Failed to decrease volume: {}", e),
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
            sleep,
            increase_volume,
            decrease_volume,
            get_local_ip
        ])
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                window.hide().unwrap();
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
