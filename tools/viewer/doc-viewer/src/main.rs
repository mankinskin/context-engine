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
mod markdown_ast;
mod mcp;
mod parser;
mod query;
mod schema;
mod templates;
mod tools;

use rmcp::{
    transport::stdio,
    ServiceExt,
};
use std::{
    env,
    path::PathBuf,
    sync::Arc,
};
use viewer_api::{
    display_host,
    init_tracing_full,
    session::SessionStore,
    to_unix_path,
    tracing::info,
    TracingConfig,
};

use mcp::DocsServer;

/// Initialize tracing with optional file output
fn init_tracing() {
    // Use WORKSPACE_ROOT or fall back to compile-time path
    let log_dir = std::env::var("WORKSPACE_ROOT")
        .map(|root| PathBuf::from(root).join("tools/doc-viewer/logs"))
        .unwrap_or_else(|_| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("logs")
        });
    let config = TracingConfig::from_env("doc-viewer", log_dir);
    init_tracing_full(&config);
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

    // Get workspace root from environment, or fall back to compile-time path
    // Priority: WORKSPACE_ROOT env > current working directory detection > compile-time CARGO_MANIFEST_DIR
    let workspace_root: PathBuf = std::env::var("WORKSPACE_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            // Try to detect workspace root from current directory (look for Cargo.toml with [workspace])
            if let Ok(cwd) = std::env::current_dir() {
                if cwd.join("Cargo.toml").exists()
                    && cwd.join("agents").exists()
                {
                    return cwd;
                }
            }
            // Fall back to compile-time manifest directory
            let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            manifest_dir
                .parent() // tools/
                .and_then(|p| p.parent()) // context-engine/
                .map(|p| p.to_path_buf())
                .unwrap_or(manifest_dir)
        });

    // Get agents directory - explicit env var or workspace_root/agents
    let agents_dir = std::env::var("AGENTS_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| workspace_root.join("agents"));

    // Parse CRATES_DIRS as a path-separated list (like PATH)
    // Default includes both crates/ and tools/ directories under workspace root
    let crates_dirs: Vec<PathBuf> = std::env::var("CRATES_DIRS")
        .or_else(|_| std::env::var("CRATES_DIR")) // Backwards compatibility
        .map(|val| std::env::split_paths(&val).collect())
        .unwrap_or_else(|_| {
            vec![workspace_root.join("crates"), workspace_root.join("tools")]
        });

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
    let crates_dirs_display: Vec<_> =
        crates_dirs.iter().map(|d| to_unix_path(d)).collect();

    if run_http {
        info!(mode, agents_dir = %to_unix_path(&agents_dir), crates_dirs = ?crates_dirs_display, "Doc Viewer Server starting");
    }
    // MCP-only mode: skip startup messages to avoid VS Code warnings (stderr shows as warnings)

    if run_mcp && !run_http {
        // MCP-only mode - run stdio server
        let server = DocsServer::new(agents_dir, crates_dirs);
        let service = server.serve(stdio()).await.inspect_err(|e| {
            eprintln!("MCP server error: {:?}", e);
        })?;
        service.waiting().await?;
    } else if run_http && !run_mcp {
        // HTTP-only mode
        run_http_server(workspace_root, agents_dir, crates_dirs).await?;
    } else {
        // Both servers - run MCP in background, HTTP in foreground
        let agents_dir_clone = agents_dir.clone();
        let crates_dirs_clone = crates_dirs.clone();

        // Spawn MCP server in background task
        tokio::spawn(async move {
            let server = DocsServer::new(agents_dir_clone, crates_dirs_clone);
            match server.serve(stdio()).await {
                Ok(service) =>
                    if let Err(e) = service.waiting().await {
                        eprintln!("MCP server error while waiting: {:?}", e);
                    },
                Err(e) => {
                    eprintln!("MCP server initialization error: {:?}", e);
                },
            }
        });

        // Run HTTP server in main task
        run_http_server(workspace_root, agents_dir, crates_dirs).await?;
    }

    Ok(())
}

/// Run the HTTP server
async fn run_http_server(
    workspace_root: PathBuf,
    agents_dir: PathBuf,
    crates_dirs: Vec<PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Static files directory - use STATIC_DIR env or workspace_root/tools/doc-viewer/static
    let static_dir = std::env::var("STATIC_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| workspace_root.join("tools/doc-viewer/static"));

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3001);

    info!(static_dir = %to_unix_path(&static_dir), "Static directory");

    let state = http::HttpState {
        docs_manager: Arc::new(tools::DocsManager::new(agents_dir)),
        crate_manager: Arc::new(tools::CrateDocsManager::new(crates_dirs)),
        sessions: SessionStore::new(),
    };

    let app = http::create_router(state, Some(static_dir));

    let bind_addr = format!("0.0.0.0:{}", port);
    let display_addr = format!("{}:{}", display_host("0.0.0.0"), port);
    info!(addr = %format!("http://{}", display_addr), "Starting HTTP server");
    let listener = tokio::net::TcpListener::bind(&bind_addr).await?;
    viewer_api::axum::serve(listener, app).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use helpers::{
        format_module_tree,
        parse_doc_type,
    };
    use schema::DocType;

    #[test]
    fn test_parse_doc_type() {
        assert_eq!(parse_doc_type("guide"), Some(DocType::Guide));
        assert_eq!(parse_doc_type("bug-report"), Some(DocType::BugReport));
        assert_eq!(parse_doc_type("invalid"), None);
    }

    #[test]
    fn test_format_module_tree() {
        use schema::{
            FileEntry,
            ModuleTreeNode,
            TypeEntry,
        };
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
