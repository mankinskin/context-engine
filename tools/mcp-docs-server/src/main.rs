//! MCP Documentation Server for structured agent documentation management.
//!
//! This server provides tools for creating, updating, and managing
//! documentation files in the agents/ directory structure.

mod parser;
mod schema;
mod templates;
mod tools;

use rmcp::{
    handler::server::{
        tool::ToolRouter,
        wrapper::Parameters,
    },
    model::*,
    schemars,
    schemars::JsonSchema,
    tool,
    tool_handler,
    tool_router,
    transport::stdio,
    ErrorData as McpError,
    ServerHandler,
    ServiceExt,
};
use schema::{
    Confidence,
    DocType,
    PlanStatus,
};
use serde::Deserialize;
use std::{
    path::PathBuf,
    sync::Arc,
};
use tools::{
    CreateDocParams,
    DocsManager,
};

/// MCP Server for documentation management.
#[derive(Clone)]
pub struct DocsServer {
    manager: Arc<DocsManager>,
    tool_router: ToolRouter<Self>,
}

impl DocsServer {
    pub fn new(agents_dir: PathBuf) -> Self {
        Self {
            manager: Arc::new(DocsManager::new(agents_dir)),
            tool_router: Self::tool_router(),
        }
    }
}

// === Tool Input Types ===

/// Create a new document from template
#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateDocInput {
    /// Document type: "guide", "plan", "implemented", "bug-report", or "analysis"
    doc_type: String,
    /// Short name for the file (becomes UPPER_SNAKE_CASE in filename)
    name: String,
    /// Human-readable title for the document header
    title: String,
    /// One-line summary for the INDEX file
    summary: String,
    /// Tags for categorization (without #)
    #[serde(default)]
    tags: Vec<String>,
    /// Confidence level: "high", "medium", or "low" (default: "medium")
    #[serde(default = "default_confidence")]
    confidence: String,
    /// Status for plans: "design", "in-progress", "completed", "blocked", "superseded"
    #[serde(default)]
    status: Option<String>,
}

fn default_confidence() -> String {
    "medium".to_string()
}

/// List documents by type with optional filters
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListDocsInput {
    /// Document type: "guide", "plan", "implemented", "bug-report", or "analysis"
    doc_type: String,
    /// Filter by confidence level: "high", "medium", or "low" (optional)
    #[serde(default)]
    confidence: Option<String>,
    /// Filter by tag (optional, matches any document containing this tag)
    #[serde(default)]
    tag: Option<String>,
    /// Filter by status for plans: "design", "in-progress", "completed", "blocked", "superseded" (optional)
    #[serde(default)]
    status: Option<String>,
}

/// Update document metadata
#[derive(Debug, Deserialize, JsonSchema)]
pub struct UpdateMetaInput {
    /// Filename of the document to update
    filename: String,
    /// New confidence level (optional)
    #[serde(default)]
    confidence: Option<String>,
    /// New tags (optional, replaces existing)
    #[serde(default)]
    tags: Option<Vec<String>>,
    /// New summary (optional)
    #[serde(default)]
    summary: Option<String>,
    /// New status for plans (optional)
    #[serde(default)]
    status: Option<String>,
}

/// Search by tag
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SearchTagInput {
    /// Tag to search for (with or without #)
    tag: String,
}

/// Regenerate INDEX
#[derive(Debug, Deserialize, JsonSchema)]
pub struct RegenerateIndexInput {
    /// Document type: "guide", "plan", "implemented", "bug-report", or "analysis"
    doc_type: String,
}

/// Read a document with configurable detail level
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ReadDocInput {
    /// Filename of the document to read (e.g., "20250610_FEATURE_NAME.md")
    filename: String,
    /// Detail level: "outline" (headers only), "summary" (metadata, no body), "full" (everything). Default: "summary"
    #[serde(default = "default_detail_level")]
    detail: String,
}

fn default_detail_level() -> String {
    "summary".to_string()
}

/// Browse documentation structure (table of contents)
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BrowseDocsInput {
    /// Optional: filter to specific doc_type ("guide", "plan", etc.). If omitted, shows all categories.
    #[serde(default)]
    doc_type: Option<String>,
    /// Optional: filter by confidence level
    #[serde(default)]
    confidence: Option<String>,
    /// Optional: filter by tag
    #[serde(default)]
    tag: Option<String>,
}

/// Get documents needing review
#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetDocsNeedingReviewInput {
    /// Maximum age in days - documents older than this will be included (default: 30)
    #[serde(default = "default_max_age_days")]
    max_age_days: u32,
    /// Include low-confidence documents (default: true)
    #[serde(default = "default_true")]
    include_low_confidence: bool,
}

fn default_max_age_days() -> u32 {
    30
}

fn default_true() -> bool {
    true
}

// === Tool Implementations ===

#[tool_router]
impl DocsServer {
    /// Create a new documentation file from template
    #[tool(
        description = "Create a new documentation file from template. Automatically generates the dated filename, populates the template, and updates the INDEX."
    )]
    async fn create_doc(
        &self,
        Parameters(input): Parameters<CreateDocInput>,
    ) -> Result<CallToolResult, McpError> {
        let doc_type = parse_doc_type(&input.doc_type).ok_or_else(|| {
            McpError::invalid_params(
                format!("Invalid doc_type: {}", input.doc_type),
                None,
            )
        })?;

        let confidence = parse_confidence(&input.confidence);
        let status = input.status.as_ref().and_then(|s| parse_status(s));

        let params = CreateDocParams {
            doc_type,
            name: input.name,
            title: input.title,
            summary: input.summary,
            tags: Some(input.tags),
            confidence: Some(confidence),
            status,
        };

        match self.manager.create_document(params) {
            Ok(result) => Ok(CallToolResult::success(vec![Content::text(
                format!("Created: {}\nPath: {}", result.filename, result.path),
            )])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(format!(
                "Error: {}",
                e
            ))])),
        }
    }

    /// List all documents of a specific type
    #[tool(
        description = "List all documents of a specific type (guide, plan, implemented, bug-report, analysis) with their metadata."
    )]
    async fn list_docs(
        &self,
        Parameters(input): Parameters<ListDocsInput>,
    ) -> Result<CallToolResult, McpError> {
        let doc_type = parse_doc_type(&input.doc_type).ok_or_else(|| {
            McpError::invalid_params(
                format!("Invalid doc_type: {}", input.doc_type),
                None,
            )
        })?;

        let filter = tools::ListFilter {
            confidence: input.confidence.as_ref().map(|s| parse_confidence(s)),
            tag: input.tag,
            status: input.status.as_ref().and_then(|s| parse_status(s)),
        };

        match self.manager.list_documents_filtered(doc_type, &filter) {
            Ok(docs) => {
                let json =
                    serde_json::to_string_pretty(&docs).unwrap_or_default();
                Ok(CallToolResult::success(vec![Content::text(json)]))
            },
            Err(e) => Ok(CallToolResult::error(vec![Content::text(format!(
                "Error: {}",
                e
            ))])),
        }
    }

    /// Update metadata for an existing document
    #[tool(
        description = "Update metadata (confidence, tags, summary, status) for an existing document. Also regenerates the INDEX."
    )]
    async fn update_doc_meta(
        &self,
        Parameters(input): Parameters<UpdateMetaInput>,
    ) -> Result<CallToolResult, McpError> {
        let params = tools::UpdateMetaParams {
            filename: input.filename.clone(),
            confidence: input.confidence.as_ref().map(|s| parse_confidence(s)),
            tags: input.tags,
            summary: input.summary,
            status: input.status.as_ref().and_then(|s| parse_status(s)),
        };

        match self.manager.update_document_metadata(params) {
            Ok(()) => Ok(CallToolResult::success(vec![Content::text(
                format!("Updated: {}", input.filename),
            )])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(format!(
                "Error: {}",
                e
            ))])),
        }
    }

    /// Search documents by tag
    #[tool(description = "Search for documents across all categories by tag.")]
    async fn search_docs(
        &self,
        Parameters(input): Parameters<SearchTagInput>,
    ) -> Result<CallToolResult, McpError> {
        match self.manager.search_by_tag(&input.tag) {
            Ok(docs) => {
                let json =
                    serde_json::to_string_pretty(&docs).unwrap_or_default();
                Ok(CallToolResult::success(vec![Content::text(json)]))
            },
            Err(e) => Ok(CallToolResult::error(vec![Content::text(format!(
                "Error: {}",
                e
            ))])),
        }
    }

    /// Regenerate INDEX.md for a document type
    #[tool(
        description = "Regenerate the INDEX.md file for a document category by scanning all documents in the directory."
    )]
    async fn regenerate_index(
        &self,
        Parameters(input): Parameters<RegenerateIndexInput>,
    ) -> Result<CallToolResult, McpError> {
        let doc_type = parse_doc_type(&input.doc_type).ok_or_else(|| {
            McpError::invalid_params(
                format!("Invalid doc_type: {}", input.doc_type),
                None,
            )
        })?;

        match self.manager.update_index(doc_type) {
            Ok(path) => Ok(CallToolResult::success(vec![Content::text(
                format!("Regenerated INDEX: {}", path),
            )])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(format!(
                "Error: {}",
                e
            ))])),
        }
    }

    /// Validate all documents for convention compliance
    #[tool(
        description = "Validate all documents for naming conventions, frontmatter, and other requirements."
    )]
    async fn validate_docs(&self) -> Result<CallToolResult, McpError> {
        match self.manager.validate() {
            Ok(report) => {
                let json =
                    serde_json::to_string_pretty(&report).unwrap_or_default();
                Ok(CallToolResult::success(vec![Content::text(json)]))
            },
            Err(e) => Ok(CallToolResult::error(vec![Content::text(format!(
                "Error: {}",
                e
            ))])),
        }
    }

    /// Read the full content of a document
    #[tool(
        description = "Read a document with configurable detail level. Use 'outline' for headers only, 'summary' for metadata without body (default), 'full' for complete content."
    )]
    async fn read_doc(
        &self,
        Parameters(input): Parameters<ReadDocInput>,
    ) -> Result<CallToolResult, McpError> {
        let detail = parse_detail_level(&input.detail);
        match self.manager.read_document(&input.filename, detail) {
            Ok(result) => {
                let json =
                    serde_json::to_string_pretty(&result).unwrap_or_default();
                Ok(CallToolResult::success(vec![Content::text(json)]))
            },
            Err(e) => Ok(CallToolResult::error(vec![Content::text(format!(
                "Error: {}",
                e
            ))])),
        }
    }

    /// Find documents that may need review or updates
    #[tool(
        description = "Get documents that may need review: old documents (configurable age) and/or low-confidence documents. Useful for maintenance and summarization workflows."
    )]
    async fn get_docs_needing_review(
        &self,
        Parameters(input): Parameters<GetDocsNeedingReviewInput>,
    ) -> Result<CallToolResult, McpError> {
        match self.manager.get_docs_needing_review(
            input.max_age_days,
            input.include_low_confidence,
        ) {
            Ok(docs) => {
                let json =
                    serde_json::to_string_pretty(&docs).unwrap_or_default();
                Ok(CallToolResult::success(vec![Content::text(json)]))
            },
            Err(e) => Ok(CallToolResult::error(vec![Content::text(format!(
                "Error: {}",
                e
            ))])),
        }
    }

    /// Browse documentation structure
    #[tool(
        description = "Browse the documentation structure as a table of contents. Shows document counts and summaries per category. Use filters to narrow results. This is the recommended starting point for exploring docs."
    )]
    async fn browse_docs(
        &self,
        Parameters(input): Parameters<BrowseDocsInput>,
    ) -> Result<CallToolResult, McpError> {
        let doc_type = input.doc_type.as_ref().and_then(|s| parse_doc_type(s));
        let filter = tools::ListFilter {
            confidence: input.confidence.as_ref().map(|s| parse_confidence(s)),
            tag: input.tag,
            status: None,
        };

        match self.manager.browse_docs(doc_type, &filter) {
            Ok(toc) => {
                let json =
                    serde_json::to_string_pretty(&toc).unwrap_or_default();
                Ok(CallToolResult::success(vec![Content::text(json)]))
            },
            Err(e) => Ok(CallToolResult::error(vec![Content::text(format!(
                "Error: {}",
                e
            ))])),
        }
    }
}

#[tool_handler]
impl ServerHandler for DocsServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some(
                "MCP Docs Server for managing structured agent documentation. \
                 Tools: create_doc, list_docs, update_doc_meta, search_docs, \
                 regenerate_index, validate_docs."
                    .into(),
            ),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}

// === Helper Functions ===

fn parse_doc_type(s: &str) -> Option<DocType> {
    match s.to_lowercase().as_str() {
        "guide" | "guides" => Some(DocType::Guide),
        "plan" | "plans" => Some(DocType::Plan),
        "implemented" => Some(DocType::Implemented),
        "bug-report" | "bug-reports" | "bug_report" | "bugreport" =>
            Some(DocType::BugReport),
        "analysis" => Some(DocType::Analysis),
        _ => None,
    }
}

fn parse_detail_level(s: &str) -> tools::DetailLevel {
    match s.to_lowercase().as_str() {
        "outline" => tools::DetailLevel::Outline,
        "full" => tools::DetailLevel::Full,
        _ => tools::DetailLevel::Summary,
    }
}

fn parse_confidence(s: &str) -> Confidence {
    match s.to_lowercase().as_str() {
        "high" => Confidence::High,
        "low" => Confidence::Low,
        _ => Confidence::Medium,
    }
}

fn parse_status(s: &str) -> Option<PlanStatus> {
    match s.to_lowercase().as_str() {
        "design" => Some(PlanStatus::Design),
        "in-progress" | "in_progress" | "inprogress" =>
            Some(PlanStatus::InProgress),
        "completed" | "complete" | "done" => Some(PlanStatus::Completed),
        "blocked" => Some(PlanStatus::Blocked),
        "superseded" | "abandoned" => Some(PlanStatus::Superseded),
        _ => None,
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get agents directory from environment or use default
    let agents_dir = std::env::var("AGENTS_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            // Default: assume we're in context-engine/agents/mcp-docs-server
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .unwrap()
                .to_path_buf()
        });

    eprintln!("MCP Docs Server starting...");
    eprintln!("Agents directory: {}", agents_dir.display());

    let server = DocsServer::new(agents_dir);

    let service = server.serve(stdio()).await.inspect_err(|e| {
        eprintln!("Server error: {:?}", e);
    })?;

    service.waiting().await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_doc_type() {
        assert_eq!(parse_doc_type("guide"), Some(DocType::Guide));
        assert_eq!(parse_doc_type("bug-report"), Some(DocType::BugReport));
        assert_eq!(parse_doc_type("invalid"), None);
    }

    #[test]
    fn test_parse_confidence() {
        assert_eq!(parse_confidence("high"), Confidence::High);
        assert_eq!(parse_confidence("LOW"), Confidence::Low);
        assert_eq!(parse_confidence("unknown"), Confidence::Medium);
    }
}
