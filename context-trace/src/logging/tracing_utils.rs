//! Tracing setup utilities for tests
//!
//! Provides per-test logging to files with automatic cleanup on success.
//! Logs are written to `<target-dir>/test-logs/<test_name>.log` and deleted if the test passes.
//! The target directory is automatically detected from the Cargo build environment.

use std::{
    env,
    fs,
    path::{
        Path,
        PathBuf,
    },
    sync::Once,
};
use tracing::Level;
use tracing_subscriber::{
    EnvFilter,
    fmt::format::FmtSpan,
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

static GLOBAL_INIT: Once = Once::new();

/// Get the target directory used by Cargo
///
/// This respects the workspace structure by checking:
/// 1. CARGO_TARGET_DIR environment variable (if set by user/CI)
/// 2. CARGO_MANIFEST_DIR at runtime to find workspace root
/// 3. Falls back to "target" relative to current directory
fn get_target_dir() -> PathBuf {
    // First check if CARGO_TARGET_DIR is set (user override or CI)
    if let Ok(target_dir) = env::var("CARGO_TARGET_DIR") {
        return PathBuf::from(target_dir);
    }

    // During test execution, CARGO_MANIFEST_DIR points to the crate being tested
    // For workspace, we want to use the workspace root's target directory
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let manifest_path = PathBuf::from(&manifest_dir);
        let mut current = manifest_path.clone();

        // Walk up to find workspace root (has Cargo.toml with [workspace])
        while let Some(parent) = current.parent() {
            let workspace_toml = parent.join("Cargo.toml");
            if workspace_toml.exists() {
                // Check if this is a workspace root by looking for [workspace] section
                if let Ok(contents) = fs::read_to_string(&workspace_toml) {
                    if contents.contains("[workspace]") {
                        return parent.join("target");
                    }
                }
            }
            current = parent.to_path_buf();
        }

        // If no workspace found, use the manifest dir's target
        return manifest_path.join("target");
    }

    // Fallback to relative "target" directory (current directory)
    PathBuf::from("target")
}

/// Configuration for test tracing
#[derive(Debug, Clone)]
pub struct TracingConfig {
    /// Directory where log files are stored
    pub log_dir: PathBuf,
    /// Default log level
    pub default_level: Level,
    /// Whether to log to stdout
    pub log_to_stdout: bool,
    /// Whether to log to file
    pub log_to_file: bool,
    /// Custom filter directives (e.g., "context_search=debug,context_trace=info")
    pub filter_directives: Option<String>,
    /// Which spans to log
    pub span_events: FmtSpan,
}

impl Default for TracingConfig {
    fn default() -> Self {
        Self {
            log_dir: get_target_dir().join("test-logs"),
            default_level: Level::DEBUG,
            log_to_stdout: true,
            log_to_file: true,
            filter_directives: None,
            span_events: FmtSpan::NEW | FmtSpan::CLOSE,
        }
    }
}

impl TracingConfig {
    /// Create config with custom log level
    pub fn with_level(
        mut self,
        level: Level,
    ) -> Self {
        self.default_level = level;
        self
    }

    /// Create config with custom filter directives
    ///
    /// Example: `"context_search::search=trace,context_trace=debug"`
    pub fn with_filter(
        mut self,
        filter: impl Into<String>,
    ) -> Self {
        self.filter_directives = Some(filter.into());
        self
    }

    /// Enable/disable stdout logging
    pub fn stdout(
        mut self,
        enabled: bool,
    ) -> Self {
        self.log_to_stdout = enabled;
        self
    }

    /// Enable/disable file logging
    pub fn file(
        mut self,
        enabled: bool,
    ) -> Self {
        self.log_to_file = enabled;
        self
    }

    /// Set which span events to log
    pub fn span_events(
        mut self,
        events: FmtSpan,
    ) -> Self {
        self.span_events = events;
        self
    }

    /// Set custom log directory
    pub fn log_dir(
        mut self,
        dir: impl Into<PathBuf>,
    ) -> Self {
        self.log_dir = dir.into();
        self
    }
}

/// Guard that handles test logging lifecycle
///
/// Automatically cleans up log files when the test succeeds (guard is dropped without panic).
pub struct TestTracing {
    log_file_path: Option<PathBuf>,
}

impl TestTracing {
    /// Initialize tracing for a test
    ///
    /// # Example
    /// ```no_run
    /// use context_trace::tracing_utils::TestTracing;
    ///
    /// #[test]
    /// fn my_test() {
    ///     let _tracing = TestTracing::init("my_test");
    ///     // Test code with automatic tracing
    ///     // Log file will be deleted if test passes
    /// }
    /// ```
    pub fn init(test_name: &str) -> Self {
        Self::init_with_config(test_name, TracingConfig::default())
    }

    /// Initialize tracing with custom configuration
    pub fn init_with_config(
        test_name: &str,
        config: TracingConfig,
    ) -> Self {
        // Initialize global tracing only once
        GLOBAL_INIT.call_once(|| {
            // This is a placeholder - actual subscriber will be set per-test
        });

        // Create log directory
        if config.log_to_file {
            fs::create_dir_all(&config.log_dir).ok();
        }

        let log_file_path = if config.log_to_file {
            Some(config.log_dir.join(format!("{}.log", test_name)))
        } else {
            None
        };

        // Build the filter
        let filter = if let Some(directives) = &config.filter_directives {
            EnvFilter::try_new(directives).unwrap_or_else(|_| {
                EnvFilter::new(config.default_level.as_str())
            })
        } else {
            // Check for RUST_LOG env var, otherwise use default level
            EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                EnvFilter::new(config.default_level.as_str())
            })
        };

        // Create the subscriber with layers
        let registry = tracing_subscriber::registry().with(filter);

        // Extract config values to avoid partial move issues
        let span_events = config.span_events;
        let log_to_stdout = config.log_to_stdout;

        // Build layers based on configuration
        match (log_to_stdout, log_file_path.as_ref()) {
            (true, Some(path)) => {
                // Both stdout and file - use multiple layers
                let file =
                    fs::File::create(path).expect("Failed to create log file");

                let stdout_layer = tracing_subscriber::fmt::layer()
                    .with_writer(std::io::stdout)
                    .with_span_events(span_events.clone())
                    .with_thread_ids(true)
                    .with_thread_names(true)
                    .with_target(true)
                    .with_file(true)
                    .with_line_number(true)
                    .with_ansi(true)
                    .pretty();

                let file_layer = tracing_subscriber::fmt::layer()
                    .with_writer(move || {
                        file.try_clone().expect("Failed to clone file")
                    })
                    .with_span_events(span_events)
                    .with_thread_ids(true)
                    .with_thread_names(true)
                    .with_target(true)
                    .with_file(true)
                    .with_line_number(true)
                    .with_ansi(false)
                    .pretty();

                registry.with(stdout_layer).with(file_layer).try_init().ok();
            },
            (true, None) => {
                // Only stdout
                let stdout_layer = tracing_subscriber::fmt::layer()
                    .with_writer(std::io::stdout)
                    .with_span_events(span_events)
                    .with_thread_ids(true)
                    .with_thread_names(true)
                    .with_target(true)
                    .with_file(true)
                    .with_line_number(true)
                    .with_ansi(true)
                    .pretty();

                registry.with(stdout_layer).try_init().ok();
            },
            (false, Some(path)) => {
                // Only file
                let file =
                    fs::File::create(path).expect("Failed to create log file");

                let file_layer = tracing_subscriber::fmt::layer()
                    .with_writer(move || {
                        file.try_clone().expect("Failed to clone file")
                    })
                    .with_span_events(span_events)
                    .with_thread_ids(true)
                    .with_thread_names(true)
                    .with_target(true)
                    .with_file(true)
                    .with_line_number(true)
                    .with_ansi(false)
                    .pretty();

                registry.with(file_layer).try_init().ok();
            },
            (false, None) => {
                // No output - minimal subscriber
                registry.try_init().ok();
            },
        }

        tracing::info!(
            test_name = %test_name,
            log_file = ?log_file_path,
            "Test tracing initialized"
        );

        Self { log_file_path }
    }

    /// Get the path to the log file for this test
    pub fn log_file(&self) -> Option<&Path> {
        self.log_file_path.as_deref()
    }

    /// Explicitly keep the log file (don't delete on drop)
    ///
    /// Useful if you want to preserve logs even for passing tests
    pub fn keep_log(mut self) -> Self {
        self.log_file_path = None;
        self
    }
}

impl Drop for TestTracing {
    fn drop(&mut self) {
        // Check if we're unwinding (test panicked/failed)
        let is_panicking = std::thread::panicking();

        if !is_panicking {
            // Test passed - clean up log file
            if let Some(ref path) = self.log_file_path {
                tracing::info!(
                    log_file = ?path,
                    "Test passed, removing log file"
                );
                fs::remove_file(path).ok();
            }
        } else {
            // Test failed - keep log file
            if let Some(ref path) = self.log_file_path {
                eprintln!(
                    "\n❌ Test failed! Log file preserved at: {}",
                    path.display()
                );
            }
        }
    }
}

/// Convenience macro to initialize tracing for a test
///
/// # Example
/// ```no_run
/// use context_trace::init_test_tracing;
///
/// #[test]
/// fn my_test() {
///     init_test_tracing!();
///     // Your test code
/// }
/// ```
#[macro_export]
macro_rules! init_test_tracing {
    () => {{
        // Extract test name from function
        let test_name = {
            fn f() {}
            fn type_name_of<T>(_: T) -> &'static str {
                std::any::type_name::<T>()
            }
            let name = type_name_of(f);
            // Extract function name from the path
            name.strip_suffix("::f")
                .and_then(|s| s.split("::").last())
                .unwrap_or("unknown")
        };
        $crate::logging::tracing_utils::TestTracing::init(test_name)
    }};
    ($config:expr) => {{
        let test_name = {
            fn f() {}
            fn type_name_of<T>(_: T) -> &'static str {
                std::any::type_name::<T>()
            }
            let name = type_name_of(f);
            name.strip_suffix("::f")
                .and_then(|s| s.split("::").last())
                .unwrap_or("unknown")
        };
        $crate::logging::tracing_utils::TestTracing::init_with_config(
            test_name, $config,
        )
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tracing_config_builder() {
        let config = TracingConfig::default()
            .with_level(Level::TRACE)
            .with_filter("context_search=trace")
            .stdout(false)
            .file(true)
            .log_dir("custom/logs");

        assert_eq!(config.default_level, Level::TRACE);
        assert_eq!(
            config.filter_directives,
            Some("context_search=trace".to_string())
        );
        assert!(!config.log_to_stdout);
        assert!(config.log_to_file);
        assert_eq!(config.log_dir, PathBuf::from("custom/logs"));
    }
}
