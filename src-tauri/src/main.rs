use actix_files::Files;
use actix_web::{post, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Serialize, Deserialize)]
struct ApiResponse {
    success: bool,
    message: String,
}

#[post("/api/shutdown")]
async fn shutdown() -> impl Responder {
    println!("Shutdown request received");

    let result = if cfg!(target_os = "windows") {
        Command::new("shutdown").args(["/s", "/t", "0"]).spawn()
    } else if cfg!(target_os = "linux") {
        Command::new("shutdown").args(["-h", "now"]).spawn()
    } else if cfg!(target_os = "macos") {
        Command::new("shutdown").args(["-h", "now"]).spawn()
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
    println!("Restart request received");

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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let host = "0.0.0.0";
    let port = 7777;

    println!("Starting Ferrous Control web server");
    println!("Local access: http://127.0.0.1:{}", port);
    println!("Network access: http://<your-ip>:{}", port);
    println!("Server listening on {}:{}", host, port);

    HttpServer::new(|| {
        App::new()
            .service(shutdown)
            .service(restart)
            .service(Files::new("/", "../web-ui").index_file("index.html"))
    })
    .bind((host, port))?
    .run()
    .await
}
