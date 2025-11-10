/// Utilities for test logging and tracing with automatic cleanup
pub mod tracing_utils;

/// Formatting utilities for pretty-printing debug output in logs
pub mod format_utils;

// Re-export commonly used items for convenience
pub use format_utils::{
    PrettyDebug,
    pretty,
};
pub use tracing_utils::{
    TestTracing,
    TracingConfig,
};
