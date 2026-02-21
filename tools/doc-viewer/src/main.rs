//! Doc Viewer Server - HTTP and MCP server for documentation management.
//!
//! This server provides tools for viewing, creating, updating, and managing
//! documentation files in the agents/ directory structure.
//!
//! # Usage
//!
//! ```bash
//! # Run HTTP server only (default if no flags)
//! doc-viewer --http
//!
//! # Run MCP server only (for AI assistant integration)
//! doc-viewer --mcp
//!
//! # Run both servers
//! doc-viewer --http --mcp
//! ```
//!
//! # Environment Variables
//! - `AGENTS_DIR` - Directory containing agent documentation (default: <workspace>/agents)
//! - `CRATES_DIRS` - Path-separated list of crate directories (default: <workspace>/crates:<workspace>/tools)
//! - `STATIC_DIR` - Directory for static frontend files (default: <manifest>/static)
//! - `PORT` - HTTP server port (default: 3001)

mod git;
mod helpers;
mod http;
mod mcp;
mod parser;
mod schema;
mod templates;
mod tools;

use rmcp::{transport::stdio, ServiceExt};
use std::{env, path::PathBuf, sync::Arc};
use tracing::info;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use mcp::DocsServer;

/// Initialize tracing with optional file output
fn init_tracing() {
    let log_level = env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string());
    let file_logging = env::var("LOG_FILE").is_ok();

    let filter = EnvFilter::try_new(&log_level)
        .unwrap_or_else(|_| EnvFilter::new("info"));

    let fmt_layer = fmt::layer()
        .with_target(true)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true);

    // Check if file logging is enabled
    if file_logging {
        let log_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("logs");
        std::fs::create_dir_all(&log_dir).ok();
        
        let file_appender = tracing_appender::rolling::daily(&log_dir, "doc-viewer.log");
        let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
        
        // Store the guard to keep the appender alive
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
        
        info!("File logging enabled to logs/doc-viewer.log");
    } else {
        tracing_subscriber::registry()
            .with(filter)
            .with(fmt_layer)
            .init();
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command line arguments
    let args: Vec<String> = std::env::args().collect();
    let http_mode = args.iter().any(|arg| arg == "--http");
    let mcp_mode = args.iter().any(|arg| arg == "--mcp");
    
    // If no flags specified, default to HTTP mode
    let (run_http, run_mcp) = if !http_mode && !mcp_mode {
        (true, false)
    } else {
        (http_mode, mcp_mode)
    };
    
    // Get agents directory from environment or use default
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    // manifest_dir = tools/doc-viewer/
    // .parent() = tools/
    // .parent().parent() = context-engine/ (workspace root)
    let workspace_root = manifest_dir
        .parent() // tools/
        .and_then(|p| p.parent()) // context-engine/
        .unwrap_or(&manifest_dir);
    
    let agents_dir = std::env::var("AGENTS_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| workspace_root.join("agents"));
    
    // Parse CRATES_DIRS as a path-separated list (like PATH)
    // Default includes both crates/ and tools/ directories
    let crates_dirs: Vec<PathBuf> = std::env::var("CRATES_DIRS")
        .or_else(|_| std::env::var("CRATES_DIR")) // Backwards compatibility
        .map(|val| std::env::split_paths(&val).collect())
        .unwrap_or_else(|_| vec![
            workspace_root.join("crates"),
            workspace_root.join("tools"),
        ]);

    // Initialize tracing for HTTP mode (MCP mode uses stderr to avoid stdio conflicts)
    if run_http {
        init_tracing();
    }

    let mode = match (run_http, run_mcp) {
        (true, true) => "HTTP + MCP",
        (true, false) => "HTTP only",
        (false, true) => "MCP only",
        (false, false) => unreachable!(),
    };
    let crates_dirs_display: Vec<_> = crates_dirs.iter().map(|d| d.display().to_string()).collect();

    if run_http {
        info!(mode, agents_dir = %agents_dir.display(), crates_dirs = ?crates_dirs_display, "Doc Viewer Server starting");
    } else {
        // MCP-only mode - use stderr to avoid interfering with stdio transport
        eprintln!("Doc Viewer Server starting...");
        eprintln!("  Mode: {}", mode);
        eprintln!("  Agents directory: {}", agents_dir.display());
        for dir in &crates_dirs {
            eprintln!("  Crates directory: {}", dir.display());
        }
    }

    if run_mcp && !run_http {
        // MCP-only mode - run stdio server
        let server = DocsServer::new(agents_dir, crates_dirs);
        let service = server.serve(stdio()).await.inspect_err(|e| {
            eprintln!("MCP server error: {:?}", e);
        })?;
        service.waiting().await?;
    } else if run_http && !run_mcp {
        // HTTP-only mode
        run_http_server(manifest_dir, agents_dir, crates_dirs).await?;
    } else {
        // Both servers - run MCP in background, HTTP in foreground
        let agents_dir_clone = agents_dir.clone();
        let crates_dirs_clone = crates_dirs.clone();
        
        // Spawn MCP server in background task
        tokio::spawn(async move {
            let server = DocsServer::new(agents_dir_clone, crates_dirs_clone);
            match server.serve(stdio()).await {
                Ok(service) => {
                    if let Err(e) = service.waiting().await {
                        eprintln!("MCP server error while waiting: {:?}", e);
                    }
                }
                Err(e) => {
                    eprintln!("MCP server initialization error: {:?}", e);
                }
            }
        });
        
        // Run HTTP server in main task
        run_http_server(manifest_dir, agents_dir, crates_dirs).await?;
    }

    Ok(())
}

/// Run the HTTP server
async fn run_http_server(
    manifest_dir: PathBuf,
    agents_dir: PathBuf,
    crates_dirs: Vec<PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Static files directory
    let static_dir = std::env::var("STATIC_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| manifest_dir.join("static"));

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3001);

    info!(static_dir = %static_dir.display(), "Static directory");

    let state = http::HttpState {
        docs_manager: Arc::new(tools::DocsManager::new(agents_dir)),
        crate_manager: Arc::new(tools::CrateDocsManager::new(crates_dirs)),
    };

    let app = http::create_router(state, Some(static_dir));

    let addr = format!("0.0.0.0:{}", port);
    info!(%addr, "Starting HTTP server");
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    viewer_api::axum::serve(listener, app).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use helpers::{parse_doc_type, format_module_tree};
    use schema::DocType;

    #[test]
    fn test_parse_doc_type() {
        assert_eq!(parse_doc_type("guide"), Some(DocType::Guide));
        assert_eq!(parse_doc_type("bug-report"), Some(DocType::BugReport));
        assert_eq!(parse_doc_type("invalid"), None);
    }

    #[test]
    fn test_format_module_tree() {
        use schema::{FileEntry, TypeEntry, ModuleTreeNode};
        let tree = ModuleTreeNode {
            name: "test".to_string(),
            path: "".to_string(),
            description: "Test module".to_string(),
            children: vec![],
            files: vec![FileEntry {
                name: "mod.rs".to_string(),
                description: "Module root".to_string(),
            }],
            key_types: vec![TypeEntry {
                name: "TestType".to_string(),
                description: None,
            }],
            has_readme: true,
            all_types: vec![],
        };
        let md = format_module_tree(&tree, 0);
        assert!(md.contains("# test"));
        assert!(md.contains("*Test module*"));
        assert!(md.contains("`TestType`"));
        assert!(md.contains("`mod.rs`"));
    }
}
