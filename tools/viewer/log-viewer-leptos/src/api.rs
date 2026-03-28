/// HTTP API layer, mirroring tools/viewer/log-viewer/frontend/src/api/live.ts.
///
/// All fetches hit the local Rust HTTP server at `/api`.
use gloo_net::http::Request;

use crate::types::{LogContentResponse, LogFile};

const BASE: &str = "/api";

async fn fetch_json<T: serde::de::DeserializeOwned>(url: &str) -> Result<T, String> {
    let resp = Request::get(url)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if !resp.ok() {
        return Err(format!("HTTP {}", resp.status()));
    }
    resp.json::<T>().await.map_err(|e| e.to_string())
}

/// `GET /api/logs` — list available log files.
pub async fn list_log_files() -> Result<Vec<LogFile>, String> {
    fetch_json(&format!("{BASE}/logs")).await
}

/// `GET /api/logs/:name` — fetch all entries for a log file.
pub async fn load_log_file(name: &str) -> Result<Vec<crate::types::LogEntry>, String> {
    let resp: LogContentResponse =
        fetch_json(&format!("{BASE}/logs/{}", urlencoding::encode(name))).await?;
    Ok(resp.entries)
}
