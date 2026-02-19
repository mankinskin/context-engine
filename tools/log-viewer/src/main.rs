//! Log Viewer HTTP Server
//!
//! Serves a web interface for viewing and querying tracing logs.
//! Endpoints:
//! - GET /api/logs - List available log files
//! - GET /api/logs/:name - Get log file content
//! - GET /api/search/:name?q=query - Search within a log file
//! - GET /api/source/*path - Get source file content
//! - Static files served from /static
//!
//! # Configuration
//! 
//! Config file search order:
//! 1. Path in `LOG_VIEWER_CONFIG` environment variable
//! 2. `./log-viewer.toml` (current directory)
//! 3. `./config/log-viewer.toml` (config subdirectory)
//! 4. `~/.config/log-viewer/config.toml` (user config directory)
//!
//! # Environment Variables (override config file values)
//! - `LOG_DIR` - Directory containing log files (default: target/test-logs)
//! - `WORKSPACE_ROOT` - Workspace root for source file resolution
//! - `LOG_LEVEL` - Logging level: trace, debug, info, warn, error (default: info)
//! - `LOG_FILE` - Enable file logging to logs/log-viewer.log

mod config;
mod log_parser;

use config::Config;

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
use tracing::{debug, error, info, warn, instrument};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use log_parser::{LogEntry, LogParser};

/// Application state shared across handlers
#[derive(Clone)]
pub struct AppState {
    pub log_dir: PathBuf,
    pub workspace_root: PathBuf,
    pub parser: Arc<LogParser>,
}

/// Response for listing log files
#[derive(Serialize, Deserialize, Debug)]
pub struct LogFileInfo {
    pub name: String,
    pub size: u64,
    pub modified: Option<String>,
}

/// Response for log content
#[derive(Serialize, Deserialize, Debug)]
pub struct LogContentResponse {
    pub name: String,
    pub entries: Vec<LogEntry>,
    pub total_lines: usize,
}

/// Query params for source
#[derive(Deserialize, Debug)]
pub struct SourceQuery {
    #[serde(default)]
    pub line: Option<usize>,
    #[serde(default = "default_context")]
    pub context: usize,
}

fn default_context() -> usize { 5 }

/// Search query parameters
#[derive(Deserialize, Debug)]
pub struct SearchQuery {
    pub q: String,
    #[serde(default)]
    pub level: Option<String>,
    #[serde(default)]
    pub limit: Option<usize>,
}

/// Search result response
#[derive(Serialize, Deserialize, Debug)]
pub struct SearchResponse {
    pub query: String,
    pub matches: Vec<LogEntry>,
    pub total_matches: usize,
}

/// Error response
#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    pub error: String,
}

/// Initialize tracing with optional file output
fn init_tracing(config: &Config) {
    let filter = EnvFilter::try_new(&config.logging.level)
        .unwrap_or_else(|_| EnvFilter::new("info"));

    let fmt_layer = fmt::layer()
        .with_target(true)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true);

    // Check if file logging is enabled
    if config.logging.file_logging {
        let log_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("logs");
        std::fs::create_dir_all(&log_dir).ok();
        
        let file_appender = tracing_appender::rolling::daily(&log_dir, "log-viewer.log");
        let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
        
        // Store the guard to keep the appender alive
        // Note: In production, you'd want to store this guard somewhere
        std::mem::forget(_guard);
        
        let file_layer = fmt::layer()
            .with_writer(non_blocking)
            .with_ansi(false)
            .with_target(true)
            .with_thread_ids(true)
            .with_file(true)
            .with_line_number(true);
        
        tracing_subscriber::registry()
            .with(filter)
            .with(fmt_layer)
            .with(file_layer)
            .init();
        
        info!("File logging enabled to logs/log-viewer.log");
    } else {
        tracing_subscriber::registry()
            .with(filter)
            .with(fmt_layer)
            .init();
    }
}

/// Create the application state from config
pub fn create_app_state_from_config(config: &Config) -> AppState {
    AppState {
        log_dir: config.resolve_log_dir(),
        workspace_root: config.resolve_workspace_root(),
        parser: Arc::new(LogParser::new()),
    }
}

/// Create the application state (for backward compatibility and tests)
pub fn create_app_state(log_dir: Option<PathBuf>, workspace_root: Option<PathBuf>) -> AppState {
    let log_dir = log_dir.or_else(|| {
        env::var("LOG_DIR").map(PathBuf::from).ok()
    }).unwrap_or_else(|| {
        // Default to target/test-logs in workspace root
        let mut path = env::current_dir().expect("Failed to get current directory");
        // Try to find workspace root by looking for Cargo.toml
        while !path.join("Cargo.toml").exists() && path.parent().is_some() {
            path = path.parent().unwrap().to_path_buf();
        }
        path.join("target").join("test-logs")
    });

    let workspace_root = workspace_root.or_else(|| {
        env::var("WORKSPACE_ROOT").map(PathBuf::from).ok()
    }).unwrap_or_else(|| {
        let mut path = env::current_dir().expect("Failed to get current directory");
        while !path.join("Cargo.toml").exists() && path.parent().is_some() {
            path = path.parent().unwrap().to_path_buf();
        }
        path
    });

    AppState {
        log_dir,
        workspace_root,
        parser: Arc::new(LogParser::new()),
    }
}

/// Create the router with all routes
pub fn create_router(state: AppState, static_dir: Option<PathBuf>) -> Router {
    let mut router = Router::new()
        .route("/api/logs", get(list_logs))
        .route("/api/logs/:name", get(get_log))
        .route("/api/search/:name", get(search_log))
        .route("/api/source/*path", get(get_source))
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any))
        .with_state(state);
    
    // Only add static file serving if directory exists
    if let Some(dir) = static_dir {
        if dir.exists() {
            router = router.nest_service("/", ServeDir::new(&dir));
        }
    }
    
    router
}

#[tokio::main]
async fn main() {
    // Load configuration from file and environment
    let config = Config::load();
    
    init_tracing(&config);

    let state = create_app_state_from_config(&config);
    info!(log_dir = %state.log_dir.display(), exists = state.log_dir.exists(), "Log directory");
    info!(workspace_root = %state.workspace_root.display(), "Workspace root");

    // Static file serving for the frontend
    let static_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("static");
    info!(static_dir = %static_dir.display(), "Static directory");

    let app = create_router(state, Some(static_dir));

    // Bind to address from config
    let addr: SocketAddr = format!("{}:{}", config.server.host, config.server.port)
        .parse()
        .expect("Invalid server address in config");
    info!(%addr, "Starting server");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

/// List all available log files
#[instrument(skip(state), fields(log_dir = %state.log_dir.display()))]
async fn list_logs(
    State(state): State<AppState>,
) -> Result<Json<Vec<LogFileInfo>>, (StatusCode, Json<ErrorResponse>)> {
    debug!("Listing log files");
    
    // If directory doesn't exist, return empty list
    if !state.log_dir.exists() {
        info!("Log directory does not exist, returning empty list");
        return Ok(Json(Vec::new()));
    }
    
    let entries = std::fs::read_dir(&state.log_dir).map_err(|e| {
        error!(error = %e, "Failed to read log directory");
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
            let file_info = LogFileInfo {
                name: path.file_name().unwrap().to_string_lossy().to_string(),
                size: metadata.as_ref().map_or(0, |m| m.len()),
                modified: metadata.and_then(|m| {
                    m.modified().ok().map(|t| {
                        let datetime: chrono::DateTime<chrono::Utc> = t.into();
                        datetime.format("%Y-%m-%d %H:%M:%S").to_string()
                    })
                }),
            };
            debug!(file = %file_info.name, size = file_info.size, "Found log file");
            logs.push(file_info);
        }
    }

    // Sort by modified time (newest first)
    logs.sort_by(|a, b| b.modified.cmp(&a.modified));
    
    info!(count = logs.len(), "Listed log files");

    Ok(Json(logs))
}

/// Get contents of a specific log file
#[instrument(skip(state), fields(log_dir = %state.log_dir.display()))]
async fn get_log(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> Result<Json<LogContentResponse>, (StatusCode, Json<ErrorResponse>)> {
    debug!(file = %name, "Getting log file content");
    
    // Validate filename (prevent path traversal)
    if name.contains("..") || name.contains('/') || name.contains('\\') {
        warn!(file = %name, "Invalid filename - path traversal attempt");
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Invalid filename".to_string(),
            }),
        ));
    }

    let path = state.log_dir.join(&name);
    debug!(path = %path.display(), "Reading log file");
    
    let content = std::fs::read_to_string(&path).map_err(|e| {
        error!(error = %e, path = %path.display(), "Failed to read log file");
        (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: format!("Failed to read log file: {}", e),
            }),
        )
    })?;

    let content_len = content.len();
    let entries = state.parser.parse(&content);
    let total_lines = content.lines().count();
    
    info!(
        file = %name,
        entries = entries.len(),
        total_lines = total_lines,
        content_bytes = content_len,
        "Parsed log file"
    );

    Ok(Json(LogContentResponse {
        name,
        entries,
        total_lines,
    }))
}

/// Search within a log file
#[instrument(skip(state), fields(log_dir = %state.log_dir.display()))]
async fn search_log(
    State(state): State<AppState>,
    Path(name): Path<String>,
    Query(query): Query<SearchQuery>,
) -> Result<Json<SearchResponse>, (StatusCode, Json<ErrorResponse>)> {
    debug!(file = %name, query = %query.q, level = ?query.level, limit = ?query.limit, "Searching log file");
    
    // Validate filename
    if name.contains("..") || name.contains('/') || name.contains('\\') {
        warn!(file = %name, "Invalid filename - path traversal attempt");
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Invalid filename".to_string(),
            }),
        ));
    }

    let path = state.log_dir.join(&name);
    let content = std::fs::read_to_string(&path).map_err(|e| {
        error!(error = %e, path = %path.display(), "Failed to read log file for search");
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
            warn!(error = %e, query = %query.q, "Invalid regex in search query");
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
    
    info!(
        file = %name,
        query = %query.q,
        total_matches = total_matches,
        returned = matches.len(),
        "Search completed"
    );

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
#[instrument(skip(state), fields(workspace_root = %state.workspace_root.display()))]
async fn get_source(
    State(state): State<AppState>,
    Path(path): Path<String>,
    Query(query): Query<SourceQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    debug!(path = %path, line = ?query.line, context = query.context, "Getting source file");
    
    let full_path = resolve_source_path(&state.workspace_root, &path).map_err(|e| {
        warn!(error = %e, path = %path, "Invalid source path");
        (StatusCode::BAD_REQUEST, Json(ErrorResponse { error: e }))
    })?;
    
    debug!(full_path = %full_path.display(), "Resolved source path");
    
    let content = std::fs::read_to_string(&full_path).map_err(|e| {
        error!(error = %e, path = %full_path.display(), "Failed to read source file");
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
        
        info!(
            path = %path,
            line = line,
            start = start_line,
            end = end_line,
            language = %language,
            "Returning source snippet"
        );
        
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
        info!(
            path = %path,
            total_lines = total_lines,
            language = %language,
            "Returning full source file"
        );
        Ok(Json(serde_json::json!({
            "path": path,
            "content": content,
            "language": language,
            "total_lines": total_lines
        })))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum_test::TestServer;
    use tempfile::TempDir;
    use std::fs;

    /// Create a test app with a temporary log directory
    fn create_test_app() -> (TestServer, TempDir, TempDir) {
        let log_dir = TempDir::new().unwrap();
        let workspace_dir = TempDir::new().unwrap();
        
        let state = AppState {
            log_dir: log_dir.path().to_path_buf(),
            workspace_root: workspace_dir.path().to_path_buf(),
            parser: Arc::new(LogParser::new()),
        };
        
        let router = create_router(state, None);
        let server = TestServer::new(router).unwrap();
        
        (server, log_dir, workspace_dir)
    }
    
    /// Helper to create a sample log file
    fn create_log_file(dir: &TempDir, name: &str, content: &str) {
        let path = dir.path().join(name);
        fs::write(&path, content).unwrap();
    }
    
    /// Helper to create a sample source file
    fn create_source_file(dir: &TempDir, path: &str, content: &str) {
        let full_path = dir.path().join(path);
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(&full_path, content).unwrap();
    }

    #[tokio::test]
    async fn test_list_logs_empty() {
        let (server, _log_dir, _workspace_dir) = create_test_app();
        
        let response = server.get("/api/logs").await;
        response.assert_status_ok();
        
        let logs: Vec<LogFileInfo> = response.json();
        assert!(logs.is_empty());
    }

    #[tokio::test]
    async fn test_list_logs_with_files() {
        let (server, log_dir, _workspace_dir) = create_test_app();
        
        create_log_file(&log_dir, "test1.log", "INFO test message");
        create_log_file(&log_dir, "test2.log", "DEBUG another message");
        // Create a non-log file that should be ignored
        create_log_file(&log_dir, "readme.txt", "not a log");
        
        let response = server.get("/api/logs").await;
        response.assert_status_ok();
        
        let logs: Vec<LogFileInfo> = response.json();
        assert_eq!(logs.len(), 2);
        
        let names: Vec<&str> = logs.iter().map(|l| l.name.as_str()).collect();
        assert!(names.contains(&"test1.log"));
        assert!(names.contains(&"test2.log"));
        assert!(!names.contains(&"readme.txt"));
    }

    #[tokio::test]
    async fn test_get_log_file() {
        let (server, log_dir, _workspace_dir) = create_test_app();
        
        let log_content = r#"{"timestamp":"2024-01-01T00:00:00Z","level":"INFO","fields":{"message":"enter"},"span":{"name":"test_span"}}
{"timestamp":"2024-01-01T00:00:01Z","level":"DEBUG","fields":{"message":"some debug message"}}
{"timestamp":"2024-01-01T00:00:02Z","level":"INFO","fields":{"message":"close"},"span":{"name":"test_span"}}"#;
        create_log_file(&log_dir, "test.log", log_content);
        
        let response = server.get("/api/logs/test.log").await;
        response.assert_status_ok();
        
        let content: LogContentResponse = response.json();
        assert_eq!(content.name, "test.log");
        assert!(!content.entries.is_empty());
    }

    #[tokio::test]
    async fn test_get_log_file_not_found() {
        let (server, _log_dir, _workspace_dir) = create_test_app();
        
        let response = server.get("/api/logs/nonexistent.log").await;
        response.assert_status_not_found();
    }

    #[tokio::test]
    async fn test_get_log_path_traversal_blocked() {
        let (server, _log_dir, _workspace_dir) = create_test_app();
        
        // URL-encoded path traversal attempt
        let response = server.get("/api/logs/..%2Fsecret.log").await;
        response.assert_status_bad_request();
        
        // Backslash in filename
        let response = server.get("/api/logs/foo%5Cbar.log").await;
        response.assert_status_bad_request();
    }

    #[tokio::test]
    async fn test_search_log() {
        let (server, log_dir, _workspace_dir) = create_test_app();
        
        let log_content = r#"{"timestamp":"2024-01-01T00:00:00Z","level":"INFO","fields":{"message":"hello world"}}
{"timestamp":"2024-01-01T00:00:01Z","level":"DEBUG","fields":{"message":"goodbye world"}}
{"timestamp":"2024-01-01T00:00:02Z","level":"ERROR","fields":{"message":"error occurred"}}"#;
        create_log_file(&log_dir, "test.log", log_content);
        
        let response = server.get("/api/search/test.log")
            .add_query_param("q", "hello")
            .await;
        response.assert_status_ok();
        
        let result: SearchResponse = response.json();
        assert_eq!(result.query, "hello");
        assert!(result.total_matches > 0);
    }

    #[tokio::test]
    async fn test_search_log_with_level_filter() {
        let (server, log_dir, _workspace_dir) = create_test_app();
        
        let log_content = r#"{"timestamp":"2024-01-01T00:00:00Z","level":"INFO","fields":{"message":"info message"}}
{"timestamp":"2024-01-01T00:00:01Z","level":"DEBUG","fields":{"message":"debug message"}}
{"timestamp":"2024-01-01T00:00:02Z","level":"ERROR","fields":{"message":"error message"}}"#;
        create_log_file(&log_dir, "test.log", log_content);
        
        let response = server.get("/api/search/test.log")
            .add_query_param("q", "message")
            .add_query_param("level", "ERROR")
            .await;
        response.assert_status_ok();
        
        let result: SearchResponse = response.json();
        // Should only match ERROR level
        for entry in &result.matches {
            assert_eq!(entry.level, "ERROR");
        }
    }

    #[tokio::test]
    async fn test_search_invalid_regex() {
        let (server, log_dir, _workspace_dir) = create_test_app();
        
        create_log_file(&log_dir, "test.log", "INFO test");
        
        // Invalid regex with unclosed bracket
        let response = server.get("/api/search/test.log")
            .add_query_param("q", "[invalid")
            .await;
        response.assert_status_bad_request();
    }

    #[tokio::test]
    async fn test_get_source_file() {
        let (server, _log_dir, workspace_dir) = create_test_app();
        
        let source_content = r#"fn main() {
    println!("Hello, world!");
}
"#;
        create_source_file(&workspace_dir, "src/main.rs", source_content);
        
        let response = server.get("/api/source/src/main.rs").await;
        response.assert_status_ok();
        
        let result: serde_json::Value = response.json();
        assert_eq!(result["path"], "src/main.rs");
        assert_eq!(result["language"], "rust");
        assert!(result["content"].as_str().unwrap().contains("println"));
    }

    #[tokio::test]
    async fn test_get_source_snippet() {
        let (server, _log_dir, workspace_dir) = create_test_app();
        
        let source_content = "line1\nline2\nline3\nline4\nline5\nline6\nline7\nline8\nline9\nline10";
        create_source_file(&workspace_dir, "test.txt", source_content);
        
        let response = server.get("/api/source/test.txt")
            .add_query_param("line", "5")
            .add_query_param("context", "2")
            .await;
        response.assert_status_ok();
        
        let result: serde_json::Value = response.json();
        assert_eq!(result["highlight_line"], 5);
        assert_eq!(result["start_line"], 3);
        assert_eq!(result["end_line"], 7);
    }

    #[tokio::test]
    async fn test_get_source_path_traversal_blocked() {
        let (server, _log_dir, _workspace_dir) = create_test_app();
        
        // URL-encoded path traversal attempt
        let response = server.get("/api/source/..%2F..%2F..%2Fetc%2Fpasswd").await;
        response.assert_status_bad_request();
    }

    #[tokio::test]
    async fn test_get_source_not_found() {
        let (server, _log_dir, _workspace_dir) = create_test_app();
        
        let response = server.get("/api/source/nonexistent.rs").await;
        response.assert_status_not_found();
    }

    #[tokio::test]
    async fn test_detect_language() {
        assert_eq!(detect_language("test.rs"), "rust");
        assert_eq!(detect_language("test.ts"), "typescript");
        assert_eq!(detect_language("test.tsx"), "typescript");
        assert_eq!(detect_language("test.js"), "javascript");
        assert_eq!(detect_language("test.json"), "json");
        assert_eq!(detect_language("test.toml"), "toml");
        assert_eq!(detect_language("test.yaml"), "yaml");
        assert_eq!(detect_language("test.yml"), "yaml");
        assert_eq!(detect_language("test.md"), "markdown");
        assert_eq!(detect_language("test.unknown"), "plaintext");
    }

    #[tokio::test]
    async fn test_resolve_source_path_normalization() {
        let workspace = PathBuf::from("/workspace");
        
        // Forward slashes
        let result = resolve_source_path(&workspace, "src/main.rs");
        assert!(result.is_ok());
        
        // Backslashes (Windows)
        let result = resolve_source_path(&workspace, "src\\main.rs");
        assert!(result.is_ok());
        
        // Leading slashes removed
        let result = resolve_source_path(&workspace, "/src/main.rs");
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_resolve_source_path_traversal_blocked() {
        let workspace = PathBuf::from("/workspace");
        
        let result = resolve_source_path(&workspace, "../etc/passwd");
        assert!(result.is_err());
        
        let result = resolve_source_path(&workspace, "src/../../../etc/passwd");
        assert!(result.is_err());
    }
}
