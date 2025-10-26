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
fn get_volume() -> Result<i32, String> {
    println!("Get volume request received via Tauri");

    if cfg!(target_os = "windows") {
        #[cfg(target_os = "windows")]
        {
            use std::os::windows::process::CommandExt;
            const CREATE_NO_WINDOW: u32 = 0x08000000;

            // Use PowerShell to get volume using COM audio endpoint
            let output = std::process::Command::new("powershell")
                .creation_flags(CREATE_NO_WINDOW)
                .args([
                    "-Command",
                    "$code = @'\nusing System;\nusing System.Runtime.InteropServices;\n\n[Guid(\"5CDF2C82-841E-4546-9722-0CF74078229A\"), InterfaceType(ComInterfaceType.InterfaceIsIUnknown)]\ninterface IAudioEndpointVolume {\n    int NotImpl1(); int NotImpl2();\n    int GetChannelCount(out int channelCount);\n    int SetMasterVolumeLevel(float level, Guid eventContext);\n    int SetMasterVolumeLevelScalar(float level, Guid eventContext);\n    int GetMasterVolumeLevel(out float level);\n    int GetMasterVolumeLevelScalar(out float level);\n}\n\n[Guid(\"D666063F-1587-4E43-81F1-B948E807363F\"), InterfaceType(ComInterfaceType.InterfaceIsIUnknown)]\ninterface IMMDevice {\n    int Activate(ref Guid id, int clsCtx, int activationParams, out IAudioEndpointVolume aev);\n}\n\n[Guid(\"A95664D2-9614-4F35-A746-DE8DB63617E6\"), InterfaceType(ComInterfaceType.InterfaceIsIUnknown)]\ninterface IMMDeviceEnumerator {\n    int NotImpl1();\n    int GetDefaultAudioEndpoint(int dataFlow, int role, out IMMDevice endpoint);\n}\n\n[ComImport, Guid(\"BCDE0395-E52F-467C-8E3D-C4579291692E\")]\nclass MMDeviceEnumeratorComObject { }\n\npublic class Audio {\n    public static float GetMasterVolume() {\n        IMMDeviceEnumerator deviceEnumerator = (IMMDeviceEnumerator)(new MMDeviceEnumeratorComObject());\n        IMMDevice device = null;\n        deviceEnumerator.GetDefaultAudioEndpoint(0, 0, out device);\n        Guid iidIAudioEndpointVolume = typeof(IAudioEndpointVolume).GUID;\n        IAudioEndpointVolume aev = null;\n        device.Activate(ref iidIAudioEndpointVolume, 0, 0, out aev);\n        float volume = 0;\n        aev.GetMasterVolumeLevelScalar(out volume);\n        return volume;\n    }\n}\n'@; Add-Type -TypeDefinition $code; $vol = [Audio]::GetMasterVolume(); [Math]::Round($vol * 100)"
                ])
                .output();

            match output {
                Ok(result) => {
                    let stdout = String::from_utf8_lossy(&result.stdout);
                    let stderr = String::from_utf8_lossy(&result.stderr);

                    println!("PowerShell stdout: {}", stdout);
                    println!("PowerShell stderr: {}", stderr);
                    println!("PowerShell exit code: {:?}", result.status.code());

                    let volume_str = stdout.trim();
                    match volume_str.parse::<i32>() {
                        Ok(vol) => {
                            let clamped_vol = vol.min(100).max(0);
                            println!("Successfully retrieved volume: {}%", clamped_vol);
                            Ok(clamped_vol)
                        },
                        Err(e) => {
                            println!("Failed to parse volume from: '{}', error: {}", volume_str, e);
                            Err("Failed to parse volume".to_string())
                        }
                    }
                }
                Err(e) => {
                    println!("Failed to execute PowerShell command: {}", e);
                    Err(format!("Failed to get volume: {}", e))
                }
            }
        }
        #[cfg(not(target_os = "windows"))]
        {
            Err("Windows-only code path".to_string())
        }
    } else if cfg!(target_os = "linux") {
        // Try pactl (PulseAudio) first
        let output = std::process::Command::new("pactl")
            .args(["get-sink-volume", "@DEFAULT_SINK@"])
            .output();

        match output {
            Ok(result) => {
                let stdout = String::from_utf8_lossy(&result.stdout);
                // Parse output like: "Volume: front-left: 65536 / 100% / 0.00 dB"
                if let Some(percent_pos) = stdout.find('%') {
                    let before_percent = &stdout[..percent_pos];
                    if let Some(last_space) = before_percent.rfind(|c: char| c.is_whitespace()) {
                        let volume_str = &before_percent[last_space + 1..];
                        match volume_str.parse::<i32>() {
                            Ok(vol) => return Ok(vol.min(100).max(0)),
                            Err(_) => {}
                        }
                    }
                }
                Err("Failed to parse volume".to_string())
            }
            Err(_) => {
                // Fallback to amixer (ALSA)
                let output = std::process::Command::new("amixer")
                    .args(["get", "Master"])
                    .output();

                match output {
                    Ok(result) => {
                        let stdout = String::from_utf8_lossy(&result.stdout);
                        if let Some(percent_start) = stdout.find('[') {
                            if let Some(percent_end) = stdout[percent_start..].find('%') {
                                let volume_str = &stdout[percent_start + 1..percent_start + percent_end];
                                match volume_str.parse::<i32>() {
                                    Ok(vol) => Ok(vol.min(100).max(0)),
                                    Err(_) => Err("Failed to parse volume".to_string())
                                }
                            } else {
                                Err("Failed to parse volume".to_string())
                            }
                        } else {
                            Err("Failed to parse volume".to_string())
                        }
                    }
                    Err(e) => Err(format!("Failed to get volume: {}", e))
                }
            }
        }
    } else if cfg!(target_os = "macos") {
        let output = std::process::Command::new("osascript")
            .args(["-e", "output volume of (get volume settings)"])
            .output();

        match output {
            Ok(result) => {
                let stdout = String::from_utf8_lossy(&result.stdout);
                let volume_str = stdout.trim();
                match volume_str.parse::<i32>() {
                    Ok(vol) => Ok(vol.min(100).max(0)),
                    Err(_) => Err("Failed to parse volume".to_string())
                }
            }
            Err(e) => Err(format!("Failed to get volume: {}", e))
        }
    } else {
        Err("Unsupported operating system".to_string())
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
            get_volume,
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
