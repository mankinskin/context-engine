//! Transparent reverse-proxy handlers: forwards `/api/*` to the `ticket serve`
//! backend, passing through all headers (excluding host) and returning the
//! raw JSON response.
//!
//! This keeps the ticket-viewer decoupled from context-tasks — it is purely a
//! UI shell that delegates all data access to the running serve instance.

use axum::{
    body::Body,
    extract::{Path, Query, Request, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};
use std::collections::HashMap;

use super::AppState;

/// Forward a GET request to the backend.
pub async fn proxy_get(
    State(state): State<AppState>,
    Path(path): Path<String>,
    Query(query): Query<HashMap<String, String>>,
    headers: HeaderMap,
) -> Response {
    forward(&state.backend_url, "GET", &path, &query, headers, None).await
}

/// Forward a POST request (body passthrough) to the backend.
pub async fn proxy_post(
    State(state): State<AppState>,
    Path(path): Path<String>,
    Query(query): Query<HashMap<String, String>>,
    headers: HeaderMap,
    req: Request,
) -> Response {
    let body_bytes = axum::body::to_bytes(req.into_body(), 4 * 1024 * 1024)
        .await
        .unwrap_or_default();
    forward(
        &state.backend_url,
        "POST",
        &path,
        &query,
        headers,
        Some(body_bytes.to_vec()),
    )
    .await
}

async fn forward(
    backend_url: &str,
    method: &str,
    path: &str,
    query: &HashMap<String, String>,
    headers: HeaderMap,
    body: Option<Vec<u8>>,
) -> Response {
    let query_str = if query.is_empty() {
        String::new()
    } else {
        let pairs: Vec<String> = query
            .iter()
            .map(|(k, v)| format!("{}={}", urlencoding(k), urlencoding(v)))
            .collect();
        format!("?{}", pairs.join("&"))
    };

    let url = format!("{}/api/{}{}", backend_url, path, query_str);

    let client = reqwest::Client::new();
    let mut req = match method {
        "GET" => client.get(&url),
        "POST" => client.post(&url),
        _ => return StatusCode::METHOD_NOT_ALLOWED.into_response(),
    };

    // Forward relevant headers (auth, content-type) but not host/connection.
    for (name, value) in &headers {
        let n = name.as_str().to_lowercase();
        if n == "authorization" || n == "content-type" || n == "accept" {
            if let Ok(v) = value.to_str() {
                req = req.header(name.as_str(), v);
            }
        }
    }

    if let Some(b) = body {
        req = req.body(b);
    }

    match req.send().await {
        Ok(resp) => {
            let status = resp.status();
            let body_bytes = resp.bytes().await.unwrap_or_default();
            (
                StatusCode::from_u16(status.as_u16())
                    .unwrap_or(StatusCode::BAD_GATEWAY),
                Body::from(body_bytes),
            )
                .into_response()
        }
        Err(e) => {
            tracing::error!(error = %e, url, "Proxy request failed");
            (
                StatusCode::BAD_GATEWAY,
                format!("{{\"error\":\"proxy error: {e}\"}}"),
            )
                .into_response()
        }
    }
}

fn urlencoding(s: &str) -> String {
    s.chars()
        .flat_map(|c| {
            if c.is_alphanumeric() || "-._~".contains(c) {
                vec![c]
            } else {
                format!("%{:02X}", c as u32).chars().collect()
            }
        })
        .collect()
}
