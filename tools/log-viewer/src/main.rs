//! Log Viewer HTTP Server
//!
//! Serves a web interface for viewing and querying tracing logs.
//! Endpoints:
//! - GET /api/logs - List available log files
//! - GET /api/logs/:name - Get log file content
//! - GET /api/logs/:name/search?q=query - Search within a log file
//! - Static files served from /static

mod log_parser;

use axum::{
    Router,
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::get,
};
use serde::{Deserialize, Serialize};
use std::{
    env,
    net::SocketAddr,
    path::PathBuf,
    sync::Arc,
};
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

use log_parser::{LogEntry, LogParser};

/// Application state shared across handlers
#[derive(Clone)]
struct AppState {
    log_dir: PathBuf,
    workspace_root: PathBuf,
    parser: Arc<LogParser>,
}

/// Response for listing log files
#[derive(Serialize)]
struct LogFileInfo {
    name: String,
    size: u64,
    modified: Option<String>,
}

/// Response for log content
#[derive(Serialize)]
struct LogContentResponse {
    name: String,
    entries: Vec<LogEntry>,
    total_lines: usize,
}

/// Query params for source
#[derive(Deserialize)]
struct SourceQuery {
    #[serde(default)]
    line: Option<usize>,
    #[serde(default = "default_context")]
    context: usize,
}

fn default_context() -> usize { 5 }

/// Search query parameters
#[derive(Deserialize)]
struct SearchQuery {
    q: String,
    #[serde(default)]
    level: Option<String>,
    #[serde(default)]
    limit: Option<usize>,
}

/// Search result response
#[derive(Serialize)]
struct SearchResponse {
    query: String,
    matches: Vec<LogEntry>,
    total_matches: usize,
}

/// Error response
#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    // Determine log directory from environment or default
    let log_dir = env::var("LOG_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            // Default to target/test-logs in workspace root
            let mut path = env::current_dir().expect("Failed to get current directory");
            // Try to find workspace root by looking for Cargo.toml
            while !path.join("Cargo.toml").exists() && path.parent().is_some() {
                path = path.parent().unwrap().to_path_buf();
            }
            path.join("target").join("test-logs")
        });

    // Find workspace root
    let workspace_root = env::var("WORKSPACE_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let mut path = env::current_dir().expect("Failed to get current directory");
            while !path.join("Cargo.toml").exists() && path.parent().is_some() {
                path = path.parent().unwrap().to_path_buf();
            }
            path
        });

    info!("Log directory: {}", log_dir.display());
    info!("Workspace root: {}", workspace_root.display());

    let state = AppState {
        log_dir: log_dir.clone(),
        workspace_root,
        parser: Arc::new(LogParser::new()),
    };

    // Static file serving for the frontend
    let static_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("static");
    info!("Static directory: {}", static_dir.display());

    // Build router
    let app = Router::new()
        .route("/api/logs", get(list_logs))
        .route("/api/logs/:name", get(get_log))
        .route("/api/logs/:name/search", get(search_log))
        .route("/api/source/*path", get(get_source))
        .nest_service("/", ServeDir::new(&static_dir))
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any))
        .with_state(state);

    // Bind to address
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("Starting server at http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

/// List all available log files
async fn list_logs(
    State(state): State<AppState>,
) -> Result<Json<Vec<LogFileInfo>>, (StatusCode, Json<ErrorResponse>)> {
    let entries = std::fs::read_dir(&state.log_dir).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: format!("Failed to read log directory: {}", e),
            }),
        )
    })?;

    let mut logs: Vec<LogFileInfo> = Vec::new();
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().map_or(false, |ext| ext == "log") {
            let metadata = entry.metadata().ok();
            logs.push(LogFileInfo {
                name: path.file_name().unwrap().to_string_lossy().to_string(),
                size: metadata.as_ref().map_or(0, |m| m.len()),
                modified: metadata.and_then(|m| {
                    m.modified().ok().map(|t| {
                        let datetime: chrono::DateTime<chrono::Utc> = t.into();
                        datetime.format("%Y-%m-%d %H:%M:%S").to_string()
                    })
                }),
            });
        }
    }

    // Sort by modified time (newest first)
    logs.sort_by(|a, b| b.modified.cmp(&a.modified));

    Ok(Json(logs))
}

/// Get contents of a specific log file
async fn get_log(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> Result<Json<LogContentResponse>, (StatusCode, Json<ErrorResponse>)> {
    // Validate filename (prevent path traversal)
    if name.contains("..") || name.contains('/') || name.contains('\\') {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Invalid filename".to_string(),
            }),
        ));
    }

    let path = state.log_dir.join(&name);
    let content = std::fs::read_to_string(&path).map_err(|e| {
        (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: format!("Failed to read log file: {}", e),
            }),
        )
    })?;

    let entries = state.parser.parse(&content);
    let total_lines = content.lines().count();

    Ok(Json(LogContentResponse {
        name,
        entries,
        total_lines,
    }))
}

/// Search within a log file
async fn search_log(
    State(state): State<AppState>,
    Path(name): Path<String>,
    Query(query): Query<SearchQuery>,
) -> Result<Json<SearchResponse>, (StatusCode, Json<ErrorResponse>)> {
    // Validate filename
    if name.contains("..") || name.contains('/') || name.contains('\\') {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Invalid filename".to_string(),
            }),
        ));
    }

    let path = state.log_dir.join(&name);
    let content = std::fs::read_to_string(&path).map_err(|e| {
        (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: format!("Failed to read log file: {}", e),
            }),
        )
    })?;

    let entries = state.parser.parse(&content);
    
    // Build regex for search
    let regex = regex::RegexBuilder::new(&query.q)
        .case_insensitive(true)
        .build()
        .map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: format!("Invalid regex: {}", e),
                }),
            )
        })?;

    // Filter entries
    let mut matches: Vec<LogEntry> = entries
        .into_iter()
        .filter(|entry| {
            // Check level filter
            if let Some(ref level_filter) = query.level {
                if !entry.level.eq_ignore_ascii_case(level_filter) {
                    return false;
                }
            }
            // Check query match against multiple fields
            regex.is_match(&entry.message) || 
                regex.is_match(&entry.raw) ||
                regex.is_match(&entry.event_type) ||
                regex.is_match(&entry.level) ||
                entry.span_name.as_ref().map(|s| regex.is_match(s)).unwrap_or(false) ||
                entry.file.as_ref().map(|f| regex.is_match(f)).unwrap_or(false) ||
                entry.fields.iter().any(|(k, v)| regex.is_match(k) || regex.is_match(v))
        })
        .collect();

    let total_matches = matches.len();

    // Apply limit
    if let Some(limit) = query.limit {
        matches.truncate(limit);
    }

    Ok(Json(SearchResponse {
        query: query.q,
        matches,
        total_matches,
    }))
}

/// Detect language from file extension
fn detect_language(path: &str) -> String {
    let ext = path.rsplit('.').next().unwrap_or("");
    match ext {
        "rs" => "rust",
        "ts" | "tsx" => "typescript",
        "js" | "jsx" => "javascript",
        "json" => "json",
        "toml" => "toml",
        "yaml" | "yml" => "yaml",
        "md" => "markdown",
        "html" => "html",
        "css" => "css",
        _ => "plaintext",
    }.to_string()
}

/// Sanitize and resolve source path
fn resolve_source_path(workspace_root: &PathBuf, path: &str) -> Result<PathBuf, String> {
    // Normalize path separators
    let normalized = path.replace('\\', "/");
    
    // Remove leading slashes
    let clean_path = normalized.trim_start_matches('/');
    
    // Check for path traversal
    if clean_path.contains("..") {
        return Err("Path traversal not allowed".to_string());
    }
    
    let full_path = workspace_root.join(clean_path);
    
    // Verify the path is within workspace
    if !full_path.starts_with(workspace_root) {
        return Err("Path outside workspace".to_string());
    }
    
    Ok(full_path)
}

/// Get full source file content or snippet around a line
async fn get_source(
    State(state): State<AppState>,
    Path(path): Path<String>,
    Query(query): Query<SourceQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    let full_path = resolve_source_path(&state.workspace_root, &path).map_err(|e| {
        (StatusCode::BAD_REQUEST, Json(ErrorResponse { error: e }))
    })?;
    
    let content = std::fs::read_to_string(&full_path).map_err(|e| {
        (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: format!("Failed to read source file: {}", e),
            }),
        )
    })?;
    
    let language = detect_language(&path);
    
    // If line is specified, return a snippet
    if let Some(line) = query.line {
        let lines: Vec<&str> = content.lines().collect();
        let total_lines = lines.len();
        
        let line = line.min(total_lines).max(1);
        let start_line = line.saturating_sub(query.context).max(1);
        let end_line = (line + query.context).min(total_lines);
        
        let snippet_lines: Vec<&str> = lines[(start_line - 1)..end_line].to_vec();
        let snippet_content = snippet_lines.join("\n");
        
        Ok(Json(serde_json::json!({
            "path": path,
            "content": snippet_content,
            "start_line": start_line,
            "end_line": end_line,
            "highlight_line": line,
            "language": language
        })))
    } else {
        // Return full file
        let total_lines = content.lines().count();
        Ok(Json(serde_json::json!({
            "path": path,
            "content": content,
            "language": language,
            "total_lines": total_lines
        })))
    }
}
