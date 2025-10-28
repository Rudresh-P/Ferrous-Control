// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::process::Command;
use std::thread;

#[cfg(target_os = "windows")]
mod volume_control;

const HTML_CONTENT: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Ferrous Control Web Interface</title>
    <style>
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }

        body {
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
            display: flex;
            justify-content: center;
            align-items: center;
        }

        .container {
            background: white;
            padding: 3rem;
            border-radius: 20px;
            box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
            text-align: center;
            min-width: 400px;
            max-width: 90%;
            width: 100%;
        }

        h1 {
            text-align: center;
            color: #333;
            margin-bottom: 0.5rem;
            font-size: 2.5rem;
        }

        .subtitle {
            color: #666;
            margin-bottom: 2rem;
            font-size: 1.1rem;
        }

        .button-container {
            display: flex;
            gap: 1.5rem;
            justify-content: center;
            margin-bottom: 2rem;
            flex-wrap: wrap;
        }

        .control-btn {
            display: flex;
            flex-direction: column;
            align-items: center;
            gap: 0.5rem;
            padding: 2rem 2.5rem;
            border: none;
            border-radius: 15px;
            font-size: 1.2rem;
            font-weight: 600;
            cursor: pointer;
            transition: all 0.3s ease;
            color: white;
            min-width: 140px;
            flex: 1 1 auto;
        }

        .control-btn .icon {
            font-size: 3rem;
        }

        @media (max-width: 768px) {
            body {
                padding: 1rem;
            }

            .container {
                padding: 2rem 1.5rem;
                min-width: unset;
                max-width: 100%;
                border-radius: 15px;
            }

            h1 {
                font-size: 2rem;
            }

            .subtitle {
                font-size: 1rem;
                margin-bottom: 1.5rem;
            }

            .button-container {
                gap: 1rem;
                flex-direction: column;
            }

            .control-btn {
                width: 100%;
                min-width: unset;
                padding: 1.5rem 2rem;
                font-size: 1.1rem;
            }

            .control-btn .icon {
                font-size: 2.5rem;
            }

            .status {
                font-size: 0.9rem;
                padding: 0.6rem;
            }
        }

        @media (max-width: 480px) {
            .container {
                padding: 1.5rem 1rem;
            }

            h1 {
                font-size: 1.75rem;
            }

            .subtitle {
                font-size: 0.95rem;
            }

            .control-btn {
                padding: 1.25rem 1.5rem;
                font-size: 1rem;
            }

            .control-btn .icon {
                font-size: 2rem;
            }
        }

        .shutdown-btn {
            background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%);
        }

        .shutdown-btn:hover {
            transform: translateY(-5px);
            box-shadow: 0 10px 30px rgba(245, 87, 108, 0.4);
        }

        .sleep-btn {
            background: linear-gradient(135deg, #a8edea 0%, #fed6e3 100%);
        }

        .sleep-btn:hover {
            transform: translateY(-5px);
            box-shadow: 0 10px 30px rgba(168, 237, 234, 0.4);
        }

        .cancel-btn {
            background: linear-gradient(135deg, #ffa751 0%, #ffe259 100%);
        }

        .cancel-btn:hover {
            transform: translateY(-5px);
            box-shadow: 0 10px 30px rgba(255, 167, 81, 0.4);
        }

        .volume-up-btn {
            background: linear-gradient(135deg, #4facfe 0%, #00f2fe 100%);
        }

        .volume-up-btn:hover {
            transform: translateY(-5px);
            box-shadow: 0 10px 30px rgba(79, 172, 254, 0.4);
        }

        .volume-down-btn {
            background: linear-gradient(135deg, #43e97b 0%, #38f9d7 100%);
        }

        .volume-down-btn:hover {
            transform: translateY(-5px);
            box-shadow: 0 10px 30px rgba(67, 233, 123, 0.4);
        }

        .control-btn:active {
            transform: translateY(-2px);
        }

        .control-btn:disabled {
            opacity: 0.6;
            cursor: not-allowed;
            transform: none !important;
        }

        .status {
            min-height: 30px;
            padding: 0.75rem;
            border-radius: 8px;
            font-size: 0.95rem;
            font-weight: 500;
        }

        .status.success {
            background: #d4edda;
            color: #155724;
            border: 1px solid #c3e6cb;
        }

        .status.error {
            background: #f8d7da;
            color: #721c24;
            border: 1px solid #f5c6cb;
        }

        .status.info {
            background: #d1ecf1;
            color: #0c5460;
            border: 1px solid #bee5eb;
        }

        .modal-overlay {
            display: none;
            position: fixed;
            top: 0;
            left: 0;
            width: 100%;
            height: 100%;
            background: rgba(0, 0, 0, 0.5);
            justify-content: center;
            align-items: center;
            z-index: 1000;
        }

        .modal-overlay.active {
            display: flex;
        }

        .modal {
            background: white;
            padding: 2rem;
            border-radius: 15px;
            box-shadow: 0 10px 40px rgba(0, 0, 0, 0.3);
            max-width: 400px;
            width: 90%;
            text-align: center;
        }

        .modal h2 {
            margin-bottom: 1rem;
            color: #333;
            font-size: 1.5rem;
        }

        .modal p {
            margin-bottom: 1.5rem;
            color: #666;
            font-size: 1rem;
        }

        .modal-buttons {
            display: flex;
            gap: 1rem;
            justify-content: center;
            flex-wrap: wrap;
        }

        .modal-btn {
            padding: 0.75rem 1.5rem;
            border: none;
            border-radius: 8px;
            font-size: 1rem;
            font-weight: 600;
            cursor: pointer;
            transition: all 0.3s ease;
            min-width: 100px;
        }

        @media (max-width: 480px) {
            .modal {
                padding: 1.5rem;
                width: 95%;
            }

            .modal h2 {
                font-size: 1.25rem;
            }

            .modal p {
                font-size: 0.95rem;
                margin-bottom: 1.25rem;
            }

            .modal-buttons {
                flex-direction: column-reverse;
                gap: 0.75rem;
            }

            .modal-btn {
                width: 100%;
                padding: 0.875rem 1.5rem;
            }
        }

        .modal-btn-confirm {
            background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%);
            color: white;
        }

        .modal-btn-confirm:hover {
            transform: translateY(-2px);
            box-shadow: 0 5px 15px rgba(245, 87, 108, 0.4);
        }

        .modal-btn-cancel {
            background: #e0e0e0;
            color: #333;
        }

        .modal-btn-cancel:hover {
            background: #d0d0d0;
            transform: translateY(-2px);
        }

        .volume-display {
            margin: 1.5rem auto;
            max-width: 300px;
            padding: 1.5rem;
            background: linear-gradient(135deg, #f5f7fa 0%, #c3cfe2 100%);
            border-radius: 15px;
            box-shadow: 0 4px 15px rgba(0, 0, 0, 0.1);
        }

        .volume-level {
            display: flex;
            align-items: center;
            justify-content: center;
            gap: 0.75rem;
            margin-bottom: 1rem;
        }

        .volume-icon {
            font-size: 2rem;
        }

        .volume-percentage {
            font-size: 2rem;
            font-weight: 700;
            color: #4facfe;
            text-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
        }

        .volume-slider {
            width: 100%;
            -webkit-appearance: none;
            appearance: none;
            height: 12px;
            background: rgba(255, 255, 255, 0.6);
            border-radius: 10px;
            outline: none;
            box-shadow: inset 0 2px 4px rgba(0, 0, 0, 0.1);
            cursor: pointer;
            transition: background 0.3s ease;
        }

        .volume-slider:hover {
            background: rgba(255, 255, 255, 0.8);
        }

        .volume-slider::-webkit-slider-thumb {
            -webkit-appearance: none;
            appearance: none;
            width: 24px;
            height: 24px;
            background: linear-gradient(135deg, #4facfe 0%, #00f2fe 100%);
            border-radius: 50%;
            cursor: pointer;
            box-shadow: 0 2px 8px rgba(79, 172, 254, 0.6);
            transition: all 0.3s ease;
        }

        .volume-slider::-webkit-slider-thumb:hover {
            transform: scale(1.1);
            box-shadow: 0 4px 12px rgba(79, 172, 254, 0.8);
        }

        .volume-slider::-webkit-slider-thumb:active {
            transform: scale(0.95);
        }

        .volume-slider::-moz-range-thumb {
            width: 24px;
            height: 24px;
            background: linear-gradient(135deg, #4facfe 0%, #00f2fe 100%);
            border-radius: 50%;
            cursor: pointer;
            border: none;
            box-shadow: 0 2px 8px rgba(79, 172, 254, 0.6);
            transition: all 0.3s ease;
        }

        .volume-slider::-moz-range-thumb:hover {
            transform: scale(1.1);
            box-shadow: 0 4px 12px rgba(79, 172, 254, 0.8);
        }

        .volume-slider::-moz-range-thumb:active {
            transform: scale(0.95);
        }

        .volume-slider::-moz-range-track {
            background: transparent;
            border: none;
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>Ferrous Control</h1>
        <p class="subtitle">Remote PC Control Panel</p>

        <div class="button-container">
            <button id="shutdownBtn" class="control-btn shutdown-btn">
                <span class="icon">🔴</span>
                <span>Shutdown</span>
            </button>

            <button id="sleepBtn" class="control-btn sleep-btn">
                <span class="icon">😴</span>
                <span>Sleep</span>
            </button>

            <button id="cancelBtn" class="control-btn cancel-btn">
                <span class="icon">⛔</span>
                <span>Cancel</span>
            </button>
        </div>

        <div class="button-container">
            <button id="volumeUpBtn" class="control-btn volume-up-btn">
                <span class="icon">🔊</span>
                <span>Volume Up</span>
            </button>

            <button id="volumeDownBtn" class="control-btn volume-down-btn">
                <span class="icon">🔉</span>
                <span>Volume Down</span>
            </button>
        </div>

        <div id="volumeDisplay" class="volume-display" style="display: none;">
            <div class="volume-level">
                <span class="volume-icon">🔊</span>
                <span id="volumePercentage" class="volume-percentage">0%</span>
            </div>
            <input type="range" id="volumeSlider" class="volume-slider" min="0" max="100" value="0" step="1">
        </div>

        <div id="status" class="status"></div>
    </div>

    <div id="modalOverlay" class="modal-overlay">
        <div class="modal">
            <h2 id="modalTitle">Confirm Action</h2>
            <p id="modalMessage">Are you sure?</p>
            <div class="modal-buttons">
                <button id="modalCancel" class="modal-btn modal-btn-cancel">Cancel</button>
                <button id="modalConfirm" class="modal-btn modal-btn-confirm">Confirm</button>
            </div>
        </div>
    </div>

    <script>
        const shutdownBtn = document.getElementById('shutdownBtn');
        const sleepBtn = document.getElementById('sleepBtn');
        const cancelBtn = document.getElementById('cancelBtn');
        const volumeUpBtn = document.getElementById('volumeUpBtn');
        const volumeDownBtn = document.getElementById('volumeDownBtn');
        const statusDiv = document.getElementById('status');
        const modalOverlay = document.getElementById('modalOverlay');
        const modalTitle = document.getElementById('modalTitle');
        const modalMessage = document.getElementById('modalMessage');
        const modalCancel = document.getElementById('modalCancel');
        const modalConfirm = document.getElementById('modalConfirm');
        const volumeDisplay = document.getElementById('volumeDisplay');
        const volumePercentage = document.getElementById('volumePercentage');
        const volumeSlider = document.getElementById('volumeSlider');

        let modalResolve = null;
        let isUpdatingVolume = false;

        async function fetchVolume() {
            console.log('Fetching volume...');
            try {
                const response = await fetch('/api/volume/get');
                const data = await response.json();

                if (data.volume !== undefined) {
                    console.log('Volume received:', data.volume);
                    isUpdatingVolume = true;
                    volumePercentage.textContent = data.volume + '%';
                    volumeSlider.value = data.volume;
                    volumeDisplay.style.display = 'block';
                    isUpdatingVolume = false;
                } else {
                    console.error('No volume data in response');
                }
            } catch (error) {
                console.error('Failed to fetch volume:', error);
            }
        }

        async function setVolume(volume) {
            console.log('Setting volume to:', volume);
            try {
                const response = await fetch('/api/volume/set', {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json',
                    },
                    body: JSON.stringify({ volume: parseInt(volume) }),
                });

                const data = await response.json();
                if (!data.success) {
                    console.error('Failed to set volume:', data.message);
                }
            } catch (error) {
                console.error('Failed to set volume:', error);
            }
        }

        // Debounce function to avoid too many API calls
        let volumeTimeout = null;
        function debounceSetVolume(volume) {
            clearTimeout(volumeTimeout);
            volumeTimeout = setTimeout(() => {
                setVolume(volume);
            }, 150);
        }

        // Update volume display in real-time as user drags
        volumeSlider.addEventListener('input', (e) => {
            if (!isUpdatingVolume) {
                const volume = e.target.value;
                volumePercentage.textContent = volume + '%';
                debounceSetVolume(volume);
            }
        });

        // Also handle change event for final value
        volumeSlider.addEventListener('change', (e) => {
            if (!isUpdatingVolume) {
                const volume = e.target.value;
                setVolume(volume).then(() => {
                    // Refresh volume after a short delay to confirm
                    setTimeout(fetchVolume, 300);
                });
            }
        });

        // Fetch volume on page load
        fetchVolume();

        function showModal(title, message) {
            return new Promise((resolve) => {
                modalTitle.textContent = title;
                modalMessage.textContent = message;
                modalOverlay.classList.add('active');
                modalResolve = resolve;
            });
        }

        function closeModal(result) {
            modalOverlay.classList.remove('active');
            if (modalResolve) {
                modalResolve(result);
                modalResolve = null;
            }
        }

        modalCancel.addEventListener('click', () => closeModal(false));
        modalConfirm.addEventListener('click', () => closeModal(true));
        modalOverlay.addEventListener('click', (e) => {
            if (e.target === modalOverlay) {
                closeModal(false);
            }
        });

        function showStatus(message, type) {
            statusDiv.textContent = message;
            statusDiv.className = `status ${type}`;

            setTimeout(() => {
                statusDiv.textContent = '';
                statusDiv.className = 'status';
            }, 5000);
        }

        async function executeCommand(endpoint, action) {
            const confirmed = await showModal(
                'Confirm Action',
                `Are you sure you want to ${action} the PC?`
            );

            if (!confirmed) {
                return;
            }

            showStatus(`Executing ${action}...`, 'info');

            try {
                const response = await fetch(endpoint, {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json',
                    },
                });

                const data = await response.json();

                if (data.success) {
                    showStatus(`${action} command sent successfully!`, 'success');
                } else {
                    showStatus(`Error: ${data.message}`, 'error');
                }
            } catch (error) {
                showStatus(`Network error: ${error.message}`, 'error');
            }
        }

        async function cancelShutdown() {
            showStatus('Cancelling shutdown...', 'info');

            try {
                const response = await fetch('/api/cancel', {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json',
                    },
                });

                const data = await response.json();

                if (data.success) {
                    showStatus(data.message, 'success');
                } else {
                    showStatus(`Error: ${data.message}`, 'error');
                }
            } catch (error) {
                showStatus(`Network error: ${error.message}`, 'error');
            }
        }

        async function changeVolume(endpoint) {
            console.log('Changing volume via:', endpoint);
            try {
                await fetch(endpoint, {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json',
                    },
                });
                // Wait a bit for the volume change to take effect, then refresh
                setTimeout(fetchVolume, 200);
            } catch (error) {
                console.error('Failed to change volume:', error);
            }
        }

        shutdownBtn.addEventListener('click', () => {
            executeCommand('/api/shutdown', 'shutdown');
        });

        sleepBtn.addEventListener('click', () => {
            executeCommand('/api/sleep', 'sleep');
        });

        cancelBtn.addEventListener('click', () => {
            cancelShutdown();
        });

        volumeUpBtn.addEventListener('click', () => {
            changeVolume('/api/volume/increase');
        });

        volumeDownBtn.addEventListener('click', () => {
            changeVolume('/api/volume/decrease');
        });
    </script>
</body>
</html>"#;

#[derive(Serialize, Deserialize)]
struct ApiResponse {
    success: bool,
    message: String,
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(HTML_CONTENT)
}

#[post("/api/shutdown")]
async fn shutdown() -> impl Responder {
    println!("Shutdown request received via web API");

    let result = if cfg!(target_os = "windows") {
        Command::new("shutdown").args(["/s", "/t", "60"]).spawn()
    } else if cfg!(target_os = "linux") {
        Command::new("shutdown").args(["-h", "+1"]).spawn()
    } else if cfg!(target_os = "macos") {
        Command::new("sh")
            .args(["-c", "sleep 60 && osascript -e 'tell app \"System Events\" to shut down' &"])
            .spawn()
    } else {
        return HttpResponse::InternalServerError().json(ApiResponse {
            success: false,
            message: "Unsupported operating system".to_string(),
        });
    };

    match result {
        Ok(_) => HttpResponse::Ok().json(ApiResponse {
            success: true,
            message: "Shutdown command executed".to_string(),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse {
            success: false,
            message: format!("Failed to execute shutdown: {}", e),
        }),
    }
}

#[post("/api/restart")]
async fn restart() -> impl Responder {
    println!("Restart request received via web API");

    let result = if cfg!(target_os = "windows") {
        Command::new("shutdown").args(["/r", "/t", "0"]).spawn()
    } else if cfg!(target_os = "linux") {
        Command::new("shutdown").args(["-r", "now"]).spawn()
    } else if cfg!(target_os = "macos") {
        Command::new("shutdown").args(["-r", "now"]).spawn()
    } else {
        return HttpResponse::InternalServerError().json(ApiResponse {
            success: false,
            message: "Unsupported operating system".to_string(),
        });
    };

    match result {
        Ok(_) => HttpResponse::Ok().json(ApiResponse {
            success: true,
            message: "Restart command executed".to_string(),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse {
            success: false,
            message: format!("Failed to execute restart: {}", e),
        }),
    }
}

#[post("/api/cancel")]
async fn cancel_shutdown() -> impl Responder {
    println!("Cancel shutdown request received via web API");

    let result = if cfg!(target_os = "windows") {
        Command::new("shutdown").args(["/a"]).spawn()
    } else if cfg!(target_os = "linux") {
        Command::new("shutdown").args(["-c"]).spawn()
    } else if cfg!(target_os = "macos") {
        Command::new("pkill")
            .args(["-f", "sleep 60 && osascript"])
            .spawn()
    } else {
        return HttpResponse::InternalServerError().json(ApiResponse {
            success: false,
            message: "Unsupported operating system".to_string(),
        });
    };

    match result {
        Ok(_) => HttpResponse::Ok().json(ApiResponse {
            success: true,
            message: "Shutdown cancelled".to_string(),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse {
            success: false,
            message: format!("Failed to cancel shutdown: {}", e),
        }),
    }
}

#[post("/api/sleep")]
async fn sleep() -> impl Responder {
    println!("Sleep request received via web API");

    let result = if cfg!(target_os = "windows") {
        Command::new("rundll32.exe")
            .args(["powrprof.dll,SetSuspendState", "0,1,0"])
            .spawn()
    } else if cfg!(target_os = "linux") {
        Command::new("systemctl").args(["suspend"]).spawn()
    } else if cfg!(target_os = "macos") {
        Command::new("pmset").args(["sleepnow"]).spawn()
    } else {
        return HttpResponse::InternalServerError().json(ApiResponse {
            success: false,
            message: "Unsupported operating system".to_string(),
        });
    };

    match result {
        Ok(_) => HttpResponse::Ok().json(ApiResponse {
            success: true,
            message: "Sleep command executed".to_string(),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse {
            success: false,
            message: format!("Failed to execute sleep: {}", e),
        }),
    }
}

#[post("/api/volume/increase")]
async fn increase_volume() -> impl Responder {
    println!("Increase volume request received via web API");

    let volume_change = 2;

    if cfg!(target_os = "windows") {
        #[cfg(target_os = "windows")]
        {
            match volume_control::VolumeControl::increase_volume(volume_change) {
                Ok(new_volume) => HttpResponse::Ok().json(ApiResponse {
                    success: true,
                    message: format!("Volume increased to {}%", new_volume),
                }),
                Err(e) => HttpResponse::InternalServerError().json(ApiResponse {
                    success: false,
                    message: format!("Failed to increase volume: {}", e),
                }),
            }
        }
        #[cfg(not(target_os = "windows"))]
        {
            HttpResponse::InternalServerError().json(ApiResponse {
                success: false,
                message: "Windows-only code path".to_string(),
            })
        }
    } else if cfg!(target_os = "linux") {
        let pactl_result = Command::new("pactl")
            .args(["set-sink-volume", "@DEFAULT_SINK@", &format!("+{}%", volume_change)])
            .spawn();

        let result = if pactl_result.is_ok() {
            pactl_result
        } else {
            Command::new("amixer")
                .args(["set", "Master", &format!("{}%+", volume_change)])
                .spawn()
        };

        match result {
            Ok(_) => HttpResponse::Ok().json(ApiResponse {
                success: true,
                message: format!("Volume increased by {}", volume_change),
            }),
            Err(e) => HttpResponse::InternalServerError().json(ApiResponse {
                success: false,
                message: format!("Failed to increase volume: {}", e),
            }),
        }
    } else if cfg!(target_os = "macos") {
        let result = Command::new("osascript")
            .args([
                "-e",
                &format!(
                    "set volume output volume (output volume of (get volume settings) + {})",
                    volume_change
                )
            ])
            .spawn();

        match result {
            Ok(_) => HttpResponse::Ok().json(ApiResponse {
                success: true,
                message: format!("Volume increased by {}", volume_change),
            }),
            Err(e) => HttpResponse::InternalServerError().json(ApiResponse {
                success: false,
                message: format!("Failed to increase volume: {}", e),
            }),
        }
    } else {
        HttpResponse::InternalServerError().json(ApiResponse {
            success: false,
            message: "Unsupported operating system".to_string(),
        })
    }
}

#[post("/api/volume/decrease")]
async fn decrease_volume() -> impl Responder {
    println!("Decrease volume request received via web API");

    let volume_change = 2;

    if cfg!(target_os = "windows") {
        #[cfg(target_os = "windows")]
        {
            match volume_control::VolumeControl::decrease_volume(volume_change) {
                Ok(new_volume) => HttpResponse::Ok().json(ApiResponse {
                    success: true,
                    message: format!("Volume decreased to {}%", new_volume),
                }),
                Err(e) => HttpResponse::InternalServerError().json(ApiResponse {
                    success: false,
                    message: format!("Failed to decrease volume: {}", e),
                }),
            }
        }
        #[cfg(not(target_os = "windows"))]
        {
            HttpResponse::InternalServerError().json(ApiResponse {
                success: false,
                message: "Windows-only code path".to_string(),
            })
        }
    } else if cfg!(target_os = "linux") {
        let pactl_result = Command::new("pactl")
            .args(["set-sink-volume", "@DEFAULT_SINK@", &format!("-{}%", volume_change)])
            .spawn();

        let result = if pactl_result.is_ok() {
            pactl_result
        } else {
            Command::new("amixer")
                .args(["set", "Master", &format!("{}%-", volume_change)])
                .spawn()
        };

        match result {
            Ok(_) => HttpResponse::Ok().json(ApiResponse {
                success: true,
                message: format!("Volume decreased by {}", volume_change),
            }),
            Err(e) => HttpResponse::InternalServerError().json(ApiResponse {
                success: false,
                message: format!("Failed to decrease volume: {}", e),
            }),
        }
    } else if cfg!(target_os = "macos") {
        let result = Command::new("osascript")
            .args([
                "-e",
                &format!(
                    "set volume output volume (output volume of (get volume settings) - {})",
                    volume_change
                )
            ])
            .spawn();

        match result {
            Ok(_) => HttpResponse::Ok().json(ApiResponse {
                success: true,
                message: format!("Volume decreased by {}", volume_change),
            }),
            Err(e) => HttpResponse::InternalServerError().json(ApiResponse {
                success: false,
                message: format!("Failed to decrease volume: {}", e),
            }),
        }
    } else {
        HttpResponse::InternalServerError().json(ApiResponse {
            success: false,
            message: "Unsupported operating system".to_string(),
        })
    }
}

#[derive(Serialize)]
struct VolumeResponse {
    volume: i32,
}

#[get("/api/volume/get")]
async fn get_volume() -> impl Responder {
    println!("Get volume request received via web API");

    if cfg!(target_os = "windows") {
        #[cfg(target_os = "windows")]
        {
            match volume_control::VolumeControl::get_volume() {
                Ok(volume) => {
                    println!("Successfully retrieved volume: {}%", volume);
                    HttpResponse::Ok().json(VolumeResponse {
                        volume,
                    })
                },
                Err(e) => {
                    HttpResponse::InternalServerError().json(ApiResponse {
                        success: false,
                        message: format!("Failed to get volume: {}", e),
                    })
                }
            }
        }
        #[cfg(not(target_os = "windows"))]
        {
            HttpResponse::InternalServerError().json(ApiResponse {
                success: false,
                message: "Windows-only code path".to_string(),
            })
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
                    Ok(vol) => {
                        let volume = vol.min(100).max(0);
                        println!("Successfully retrieved volume: {}%", volume);
                        HttpResponse::Ok().json(VolumeResponse {
                            volume,
                        })
                    },
                    Err(_) => {
                        HttpResponse::InternalServerError().json(ApiResponse {
                            success: false,
                            message: "Failed to parse volume".to_string(),
                        })
                    }
                }
            }
            Err(e) => {
                HttpResponse::InternalServerError().json(ApiResponse {
                    success: false,
                    message: format!("Failed to get volume: {}", e),
                })
            }
        }
    } else {
        HttpResponse::InternalServerError().json(ApiResponse {
            success: false,
            message: "Unsupported operating system for volume detection".to_string(),
        })
    }
}

#[derive(Deserialize)]
struct SetVolumeRequest {
    volume: i32,
}

#[post("/api/volume/set")]
async fn set_volume(req: actix_web::web::Json<SetVolumeRequest>) -> impl Responder {
    println!("Set volume request received via web API: {}%", req.volume);

    let volume_level = req.volume.clamp(0, 100);

    if cfg!(target_os = "windows") {
        #[cfg(target_os = "windows")]
        {
            match volume_control::VolumeControl::set_volume(volume_level) {
                Ok(_) => {
                    println!("Successfully set volume to {}%", volume_level);
                    HttpResponse::Ok().json(ApiResponse {
                        success: true,
                        message: format!("Volume set to {}%", volume_level),
                    })
                },
                Err(e) => {
                    HttpResponse::InternalServerError().json(ApiResponse {
                        success: false,
                        message: format!("Failed to set volume: {}", e),
                    })
                }
            }
        }
        #[cfg(not(target_os = "windows"))]
        {
            HttpResponse::InternalServerError().json(ApiResponse {
                success: false,
                message: "Windows-only code path".to_string(),
            })
        }
    } else if cfg!(target_os = "linux") {
        // Try pactl (PulseAudio) first, fallback to amixer (ALSA)
        let pactl_result = Command::new("pactl")
            .args(["set-sink-volume", "@DEFAULT_SINK@", &format!("{}%", volume_level)])
            .spawn();

        let result = if pactl_result.is_ok() {
            pactl_result
        } else {
            Command::new("amixer")
                .args(["set", "Master", &format!("{}%", volume_level)])
                .spawn()
        };

        match result {
            Ok(_) => HttpResponse::Ok().json(ApiResponse {
                success: true,
                message: format!("Volume set to {}%", volume_level),
            }),
            Err(e) => HttpResponse::InternalServerError().json(ApiResponse {
                success: false,
                message: format!("Failed to set volume: {}", e),
            }),
        }
    } else if cfg!(target_os = "macos") {
        let result = Command::new("osascript")
            .args([
                "-e",
                &format!("set volume output volume {}", volume_level)
            ])
            .spawn();

        match result {
            Ok(_) => HttpResponse::Ok().json(ApiResponse {
                success: true,
                message: format!("Volume set to {}%", volume_level),
            }),
            Err(e) => HttpResponse::InternalServerError().json(ApiResponse {
                success: false,
                message: format!("Failed to set volume: {}", e),
            }),
        }
    } else {
        HttpResponse::InternalServerError().json(ApiResponse {
            success: false,
            message: "Unsupported operating system".to_string(),
        })
    }
}

#[actix_web::main]
async fn start_web_server() -> std::io::Result<()> {
    let host = "0.0.0.0";
    let port = 7777;

    println!("Starting Ferrous Control web server");
    println!("Local access: http://127.0.0.1:{}", port);
    println!("Network access: http://<your-ip>:{}", port);
    println!("Server listening on {}:{}", host, port);

    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(shutdown)
            .service(restart)
            .service(cancel_shutdown)
            .service(sleep)
            .service(increase_volume)
            .service(decrease_volume)
            .service(get_volume)
            .service(set_volume)
    })
    .bind((host, port))?
    .run()
    .await
}

fn main() {
    // Start the Actix-web server in a background thread
    thread::spawn(|| {
        if let Err(e) = start_web_server() {
            eprintln!("Failed to start web server: {}", e);
        }
    });

    // Start the Tauri app (this will block until the app is closed)
    ferrous_control_lib::run();
}
