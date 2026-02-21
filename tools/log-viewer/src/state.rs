//! Application state and session management.

use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    env,
    path::PathBuf,
    sync::{Arc, RwLock},
};
use viewer_api::axum::http::HeaderMap;

use crate::config::Config;
use crate::log_parser::LogParser;

// Re-export session header from shared module
pub use viewer_api::session::SESSION_HEADER;

/// Session configuration for per-client logging behavior
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SessionConfig {
    /// Unique session identifier
    pub session_id: String,
    /// Whether to enable verbose logging for this session (default: false)
    #[serde(default)]
    pub verbose: bool,
    /// Number of source requests made in this session
    #[serde(skip_deserializing)]
    pub source_request_count: usize,
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            session_id: String::new(),
            verbose: false,
            source_request_count: 0,
        }
    }
}

/// Session store - maps session IDs to their configuration
pub type SessionStore = Arc<RwLock<HashMap<String, SessionConfig>>>;

/// Application state shared across handlers
#[derive(Clone)]
pub struct AppState {
    pub log_dir: PathBuf,
    pub workspace_root: PathBuf,
    pub parser: Arc<LogParser>,
    /// Session store for per-client configuration
    pub sessions: SessionStore,
}

/// Create the application state from config
pub fn create_app_state_from_config(config: &Config) -> AppState {
    AppState {
        log_dir: config.resolve_log_dir(),
        workspace_root: config.resolve_workspace_root(),
        parser: Arc::new(LogParser::new()),
        sessions: Arc::new(RwLock::new(HashMap::new())),
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
        sessions: Arc::new(RwLock::new(HashMap::new())),
    }
}

/// Get or create session config from headers
/// Returns None if no session ID is provided (anonymous request)
pub fn get_session_config(sessions: &SessionStore, headers: &HeaderMap) -> Option<SessionConfig> {
    let session_id = headers
        .get(SESSION_HEADER)
        .and_then(|v| v.to_str().ok())?;
    
    // Get or create session
    let sessions_guard = sessions.read().unwrap();
    if let Some(config) = sessions_guard.get(session_id) {
        Some(config.clone())
    } else {
        drop(sessions_guard);
        // Create new session with defaults
        let config = SessionConfig {
            session_id: session_id.to_string(),
            verbose: false,
            source_request_count: 0,
        };
        let mut sessions_guard = sessions.write().unwrap();
        sessions_guard.insert(session_id.to_string(), config.clone());
        Some(config)
    }
}

/// Increment source request counter for a session
pub fn increment_source_count(sessions: &SessionStore, session_id: &str) -> usize {
    let mut sessions_guard = sessions.write().unwrap();
    if let Some(config) = sessions_guard.get_mut(session_id) {
        config.source_request_count += 1;
        config.source_request_count
    } else {
        1
    }
}
