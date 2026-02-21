//! Log Viewer HTTP Server and MCP Server
//!
//! Serves a web interface for viewing and querying tracing logs.
//!
//! # HTTP Endpoints
//! - GET /api/logs - List available log files
//! - GET /api/logs/:name - Get log file content
//! - GET /api/search/:name?q=query - Search within a log file
//! - GET /api/source/*path - Get source file content
//! - Static files served from /static
//!
//! # MCP Server
//! Run with `--mcp` flag to start the MCP server on stdio for agent integration.
//! The MCP server provides tools for querying logs using JQ syntax.
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
mod handlers;
mod log_parser;
mod mcp_server;
mod query;
mod router;
mod source;
mod state;
mod types;

use config::Config;
use std::{env, net::SocketAddr, path::PathBuf};
use tracing::info;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use handlers::to_unix_path;
use router::create_router;
use state::create_app_state_from_config;

// Re-export types needed by tests
pub use log_parser::{LogEntry, LogParser};
pub use state::{AppState, SessionConfig, SessionStore, SESSION_HEADER, create_app_state};
pub use types::{
    ErrorResponse, JqQuery, JqQueryResponse, LogContentResponse, LogFileInfo,
    SearchQuery, SearchResponse,
};

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

#[tokio::main]
async fn main() {
    // Check for --mcp flag to run in MCP server mode
    let args: Vec<String> = env::args().collect();
    let mcp_mode = args.iter().any(|arg| arg == "--mcp");
    
    // Load configuration from file and environment
    let config = Config::load();
    
    let log_dir = config.resolve_log_dir();
    let workspace_root = config.resolve_workspace_root();
    
    if mcp_mode {
        // MCP-only mode - run stdio server
        if let Err(e) = mcp_server::run_mcp_server(log_dir, workspace_root).await {
            eprintln!("MCP server error: {}", e);
            std::process::exit(1);
        }
    } else {
        // HTTP server mode (default)
        init_tracing(&config);

        let state = create_app_state_from_config(&config);
        info!(log_dir = %to_unix_path(&state.log_dir), exists = state.log_dir.exists(), "Log directory");
        info!(workspace_root = %to_unix_path(&state.workspace_root), "Workspace root");

        // Static file serving for the frontend
        let static_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("static");
        info!(static_dir = %to_unix_path(&static_dir), "Static directory");

        let app = create_router(state, Some(static_dir));

        // Bind to address from config
        let addr: SocketAddr = format!("{}:{}", config.server.host, config.server.port)
            .parse()
            .expect("Invalid server address in config");
        info!(%addr, "Starting HTTP server");

        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        viewer_api::axum::serve(listener, app).await.unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum_test::TestServer;
    use tempfile::TempDir;
    use std::fs;
    use std::collections::HashMap;
    use std::sync::{Arc, RwLock};
    use source::detect_language;
    use source::resolve_source_path;

    /// Create a test app with a temporary log directory
    fn create_test_app() -> (TestServer, TempDir, TempDir) {
        let log_dir = TempDir::new().unwrap();
        let workspace_dir = TempDir::new().unwrap();
        
        let state = AppState {
            log_dir: log_dir.path().to_path_buf(),
            workspace_root: workspace_dir.path().to_path_buf(),
            parser: Arc::new(LogParser::new()),
            sessions: Arc::new(RwLock::new(HashMap::new())),
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

    #[tokio::test]
    async fn test_query_log_jq() {
        let (server, log_dir, _workspace_dir) = create_test_app();
        
        // Create test log file with multiple entries (message inside fields)
        let log_content = r#"{"timestamp":"0.001","level":"INFO","fields":{"message":"test info 1","target":"test"}}
{"timestamp":"0.002","level":"ERROR","fields":{"message":"test error","target":"test"}}
{"timestamp":"0.003","level":"INFO","fields":{"message":"test info 2","target":"test"}}"#;
        create_log_file(&log_dir, "test.log", log_content);
        
        // Filter for ERROR level using JQ
        let response = server.get("/api/query/test.log")
            .add_query_param("jq", r#"select(.level == "ERROR")"#)
            .await;
        response.assert_status_ok();
        
        let result: JqQueryResponse = response.json();
        assert_eq!(result.total_matches, 1);
        assert_eq!(result.matches[0].level, "ERROR");
        assert_eq!(result.matches[0].message, "test error");
    }

    #[tokio::test]
    async fn test_query_log_jq_invalid() {
        let (server, log_dir, _workspace_dir) = create_test_app();
        
        create_log_file(&log_dir, "test.log", r#"{"level":"INFO","message":"test"}"#);
        
        // Invalid JQ syntax
        let response = server.get("/api/query/test.log")
            .add_query_param("jq", "select(.invalid syntax")
            .await;
        response.assert_status_bad_request();
    }
}
