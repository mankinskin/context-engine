//! Configuration loading for log-viewer
//!
//! Config file search order:
//! 1. Path in `LOG_VIEWER_CONFIG` environment variable
//! 2. `./log-viewer.toml` (current directory)
//! 3. `./config/log-viewer.toml` (config subdirectory)
//! 4. `~/.config/log-viewer/config.toml` (user config directory)
//!
//! Environment variables override config file values.

use serde::{Deserialize, Serialize};
use std::{env, fs, path::PathBuf};
use tracing::{debug, info, warn};

/// Convert a path to Unix-style string (forward slashes)
fn to_unix_path(path: &std::path::Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

/// Main configuration structure
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Config {
    /// Directory containing log files
    pub log_dir: Option<PathBuf>,
    /// Workspace root for source file resolution
    pub workspace_root: Option<PathBuf>,
    /// Server configuration
    pub server: ServerConfig,
    /// Logging configuration
    pub logging: LoggingConfig,
}

/// Server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ServerConfig {
    /// Host to bind to
    pub host: String,
    /// Port to listen on
    pub port: u16,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 3000,
        }
    }
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct LoggingConfig {
    /// Log level: trace, debug, info, warn, error
    pub level: String,
    /// Enable file logging
    pub file_logging: bool,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            file_logging: false,
        }
    }
}

impl Config {
    /// Load configuration from file and environment variables
    pub fn load() -> Self {
        let mut config = Self::load_from_file().unwrap_or_default();
        config.apply_env_overrides();
        config
    }

    /// Search for and load config file
    fn load_from_file() -> Option<Self> {
        let config_paths = Self::config_search_paths();
        
        for path in config_paths {
            if path.exists() {
                match fs::read_to_string(&path) {
                    Ok(content) => match toml::from_str(&content) {
                        Ok(config) => {
                            info!("Loaded config from: {}", to_unix_path(&path));
                            return Some(config);
                        }
                        Err(e) => {
                            warn!("Failed to parse config file {}: {}", to_unix_path(&path), e);
                        }
                    },
                    Err(e) => {
                        debug!("Could not read config file {}: {}", to_unix_path(&path), e);
                    }
                }
            }
        }
        
        debug!("No config file found, using defaults");
        None
    }

    /// Get list of paths to search for config file
    fn config_search_paths() -> Vec<PathBuf> {
        let mut paths = Vec::new();
        
        // 1. Environment variable
        if let Ok(path) = env::var("LOG_VIEWER_CONFIG") {
            paths.push(PathBuf::from(path));
        }
        
        // 2. Current directory
        if let Ok(cwd) = env::current_dir() {
            paths.push(cwd.join("log-viewer.toml"));
            paths.push(cwd.join("config").join("log-viewer.toml"));
        }
        
        // 3. User config directory
        if let Some(home) = dirs_path() {
            paths.push(home.join(".config").join("log-viewer").join("config.toml"));
        }
        
        paths
    }

    /// Apply environment variable overrides
    fn apply_env_overrides(&mut self) {
        // LOG_DIR overrides config file
        if let Ok(log_dir) = env::var("LOG_DIR") {
            self.log_dir = Some(PathBuf::from(log_dir));
        }
        
        // WORKSPACE_ROOT overrides config file
        if let Ok(workspace_root) = env::var("WORKSPACE_ROOT") {
            self.workspace_root = Some(PathBuf::from(workspace_root));
        }
        
        // LOG_LEVEL overrides config file
        if let Ok(level) = env::var("LOG_LEVEL") {
            self.logging.level = level;
        }
        
        // LOG_FILE enables file logging
        if env::var("LOG_FILE").is_ok() {
            self.logging.file_logging = true;
        }
    }

    /// Resolve log_dir with fallback logic
    pub fn resolve_log_dir(&self) -> PathBuf {
        self.log_dir.clone().unwrap_or_else(|| {
            // Default to target/test-logs in workspace root
            let mut path = env::current_dir().expect("Failed to get current directory");
            // Try to find workspace root by looking for Cargo.toml
            while !path.join("Cargo.toml").exists() && path.parent().is_some() {
                path = path.parent().unwrap().to_path_buf();
            }
            path.join("target").join("test-logs")
        })
    }

    /// Resolve workspace_root with fallback logic
    pub fn resolve_workspace_root(&self) -> PathBuf {
        self.workspace_root.clone().unwrap_or_else(|| {
            let mut path = env::current_dir().expect("Failed to get current directory");
            while !path.join("Cargo.toml").exists() && path.parent().is_some() {
                path = path.parent().unwrap().to_path_buf();
            }
            path
        })
    }
}

/// Get home directory path (cross-platform)
fn dirs_path() -> Option<PathBuf> {
    #[cfg(target_os = "windows")]
    {
        env::var("USERPROFILE").map(PathBuf::from).ok()
    }
    #[cfg(not(target_os = "windows"))]
    {
        env::var("HOME").map(PathBuf::from).ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.server.port, 3000);
        assert_eq!(config.server.host, "127.0.0.1");
        assert_eq!(config.logging.level, "info");
        assert!(!config.logging.file_logging);
    }

    #[test]
    fn test_parse_config() {
        let toml_content = r#"
            log_dir = "/path/to/logs"
            workspace_root = "/path/to/workspace"
            
            [server]
            host = "0.0.0.0"
            port = 8080
            
            [logging]
            level = "debug"
            file_logging = true
        "#;
        
        let config: Config = toml::from_str(toml_content).unwrap();
        assert_eq!(config.log_dir, Some(PathBuf::from("/path/to/logs")));
        assert_eq!(config.server.port, 8080);
        assert_eq!(config.logging.level, "debug");
    }
}
