use axum::{Json, Router};
use axum::routing::get;
use std::env;
use anyhow::Result;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use tokio::process::Command;
use crate::config::get_settings;
use crate::data_classes::iagon_node::{IagonNodeInfo, IagonNodeResponse, IagonNodeStatus};

pub fn router() -> Router {
    Router::new()
        .route("/status", get(iagon_node_response))
}

pub async fn iagon_node_response() -> Result<Json<IagonNodeResponse>, (StatusCode, String)> {
    let settings = get_settings();
    let command = match cli_path().await {
        Ok(command) => command,
        Err((status, msg)) => return Err((status, msg.to_string())),
    };

    let get_node_status = settings.iagon.get_node_status.to_ascii_lowercase() == "true";
    let get_info = settings.iagon.get_info.to_ascii_lowercase() == "true";
    let status = if get_node_status {
        match iagon_node_status(command.clone()).await {
            Ok(status) => Some(status),
            Err((_, msg)) => {
                eprintln!("Error getting node status: {}", msg);
                None
            }
        }
    } else {
        None
    };
    let info = if get_info {
        match iagon_node_info(command.clone()).await {
            Ok(i) => Some(i),
            Err((_, msg)) => {
                eprintln!("Error getting node info: {}", msg);
                None
            }
        }
    } else {
        None
    };

    Ok(Json(IagonNodeResponse {
        status,
        info,
    }))

}

async fn cli_path() -> Result<String, (StatusCode, String)>{
    let cli_path = match env::var("IAGON_CLI_PATH") {
        Ok(path) => path,
        Err(_) => return Err((StatusCode::INTERNAL_SERVER_ERROR, "IAGON_CLI_PATH env var is not set".into())),
    };
    Ok(cli_path)
}

async fn iagon_node_status(command: String) -> Result<IagonNodeStatus, (StatusCode, String)> {
    let output = match Command::new("sh")
    .arg("-c")
    .arg(format!("{} {}", command, "get:status"))
    .output()
    .await {
        Ok(output) => output,
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    };

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        if stdout.contains("Running") {
            Ok(IagonNodeStatus::NodeStatusUp)
        } else {
            Ok(IagonNodeStatus::NodeStatusDown)
        }
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        Err((StatusCode::INTERNAL_SERVER_ERROR, stderr))
    }
}

async fn iagon_node_info(command: String) -> Result<IagonNodeInfo, (StatusCode, String)> {
    let output = match Command::new("sh")
    .arg("-c")
    .arg(format!("{} {}", command, "get:info"))
    .output()
    .await {
        Ok(output) => output,
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    };

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        return Err((StatusCode::INTERNAL_SERVER_ERROR, stderr));
    }
        let stdout = String::from_utf8_lossy(&output.stdout);
    let mut info = IagonNodeInfo {
        node_id: String::new(),
        port: String::new(),
        device: String::new(),
        cpu: String::new(),
        os: String::new(),
        path: String::new(),
        storage: String::new(),
        country: String::new(),
    };

    for line in stdout.lines() {
        if let Some((key, value)) = line.strip_prefix("=> ").and_then(|l| l.split_once(":")) {
            let key = key.trim();
            let value = value.trim().to_string();
            match key {
                "Node Id" => info.node_id = value,
                "Port" => info.port = value,
                "Device" => info.device = value,
                "CPU" => info.cpu = value,
                "OS" => info.os = value,
                "Path" => info.path = value,
                "Committed Storage" => info.storage = value,
                "Country" => info.country = value,
                _ => {} // ignore unknown keys
            }
        }
    };
    Ok(info)
}