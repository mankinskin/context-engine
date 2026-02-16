//! MCP tool definitions for documentation management.
//!
//! This module is organized into:
//! - `agents`: Agent documentation management (guides, plans, bug reports, etc.)
//! - `crates`: Crate API documentation management (crates/*/agents/docs/)

pub mod agents;
pub mod crates;

use thiserror::Error;

/// Result type for tool operations.
pub type ToolResult<T> = Result<T, ToolError>;

#[derive(Debug, Error)]
pub enum ToolError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Document not found: {0}")]
    NotFound(String),
    #[error("Document already exists: {0}")]
    AlreadyExists(String),
    #[error("Parse error: {0}")]
    ParseError(String),
}

impl From<String> for ToolError {
    fn from(s: String) -> Self {
        ToolError::ParseError(s)
    }
}

/// Detail level for document reading
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DetailLevel {
    /// Headers/outline only - just structure
    Outline,
    /// Metadata without body content (default)
    Summary,
    /// Full content including body
    Full,
}

/// Filter criteria for listing documents
#[derive(Debug, Default)]
pub struct ListFilter {
    pub tag: Option<String>,
    pub status: Option<crate::schema::PlanStatus>,
}

// Re-export main manager types (other types available via agents:: and crates::)
pub use agents::DocsManager;
pub use crates::CrateDocsManager;
