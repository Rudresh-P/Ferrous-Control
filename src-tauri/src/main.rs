// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::process::Command;
use std::thread;

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
        const statusDiv = document.getElementById('status');
        const modalOverlay = document.getElementById('modalOverlay');
        const modalTitle = document.getElementById('modalTitle');
        const modalMessage = document.getElementById('modalMessage');
        const modalCancel = document.getElementById('modalCancel');
        const modalConfirm = document.getElementById('modalConfirm');

        let modalResolve = null;

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

        shutdownBtn.addEventListener('click', () => {
            executeCommand('/api/shutdown', 'shutdown');
        });

        sleepBtn.addEventListener('click', () => {
            executeCommand('/api/sleep', 'sleep');
        });

        cancelBtn.addEventListener('click', () => {
            cancelShutdown();
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
        Command::new("shutdown").args(["-h", "+1"]).spawn()
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
        Command::new("killall").args(["shutdown"]).spawn()
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
