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
    CrateDocsManager,
};

/// MCP Server for documentation management.
#[derive(Clone)]
pub struct DocsServer {
    manager: Arc<DocsManager>,
    crate_manager: Arc<CrateDocsManager>,
    tool_router: ToolRouter<Self>,
}

impl DocsServer {
    pub fn new(agents_dir: PathBuf, crates_dir: PathBuf) -> Self {
        Self {
            manager: Arc::new(DocsManager::new(agents_dir)),
            crate_manager: Arc::new(CrateDocsManager::new(crates_dir)),
            tool_router: Self::tool_router(),
        }
    }
}

// === Tool Input Types ===

// --- Crate Documentation Tools ---

/// List all context-* crates with documentation
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListCratesInput {
    // No parameters - lists all discovered crates
}

/// Browse a crate's module tree
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BrowseCrateInput {
    /// Name of the crate (e.g., "context-trace", "context-search")
    crate_name: String,
}

/// Read crate or module documentation
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ReadCrateDocInput {
    /// Name of the crate (e.g., "context-trace")
    crate_name: String,
    /// Optional module path within the crate (e.g., "graph/path"). If omitted, reads crate root.
    #[serde(default)]
    module_path: Option<String>,
    /// Include README.md content (default: true)
    #[serde(default = "default_true")]
    include_readme: bool,
}

/// Update crate or module documentation
#[derive(Debug, Deserialize, JsonSchema)]
pub struct UpdateCrateDocInput {
    /// Name of the crate
    crate_name: String,
    /// Optional module path (if omitted, updates crate root)
    #[serde(default)]
    module_path: Option<String>,
    /// New index.yaml content (optional)
    #[serde(default)]
    index_yaml: Option<String>,
    /// New README.md content (optional)
    #[serde(default)]
    readme: Option<String>,
}

/// Create documentation for a new module
#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateModuleDocInput {
    /// Name of the crate
    crate_name: String,
    /// Module path to create (e.g., "new_module" or "parent/child")
    module_path: String,
    /// Name of the module
    name: String,
    /// Description of the module
    description: String,
}

/// Search crate documentation
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SearchCrateDocsInput {
    /// Search query (case-insensitive)
    query: String,
    /// Optional: filter to specific crate
    #[serde(default)]
    crate_filter: Option<String>,
    /// Search in type/trait/macro names (default: true)
    #[serde(default = "default_true")]
    search_types: bool,
    /// Search in README content (default: true)
    #[serde(default = "default_true")]
    search_content: bool,
}

/// Validate crate documentation
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ValidateCrateDocsInput {
    /// Optional: validate only specific crate
    #[serde(default)]
    crate_filter: Option<String>,
}

// --- Agent Documentation Tools ---

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

/// Search document content for strings
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SearchContentInput {
    /// The search query (case-insensitive substring match)
    query: String,
    /// Optional: filter to specific doc_type
    #[serde(default)]
    doc_type: Option<String>,
    /// Optional: filter by confidence level
    #[serde(default)]
    confidence: Option<String>,
    /// Optional: filter by tag
    #[serde(default)]
    tag: Option<String>,
    /// Number of lines to include before each match (default: 2)
    #[serde(default = "default_context_lines")]
    lines_before: usize,
    /// Number of lines to include after each match (default: 2)
    #[serde(default = "default_context_lines")]
    lines_after: usize,
}

fn default_context_lines() -> usize {
    2
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
        description = "Validate all documents for naming conventions, frontmatter, and other requirements. Returns a report of errors and warnings."
    )]
    async fn validate_docs(&self) -> Result<CallToolResult, McpError> {
        match self.manager.validate() {
            Ok(report) => Ok(CallToolResult::success(vec![Content::text(
                report.to_markdown(),
            )])),
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
            Ok(result) => Ok(CallToolResult::success(vec![Content::text(
                result.to_markdown(),
            )])),
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
            Ok(toc) => Ok(CallToolResult::success(vec![Content::text(
                toc.to_markdown(),
            )])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(format!(
                "Error: {}",
                e
            ))])),
        }
    }

    /// Search document content for strings
    #[tool(
        description = "Search for text within document content. Returns matching lines with surrounding context. Use filters to narrow the search scope."
    )]
    async fn search_content(
        &self,
        Parameters(input): Parameters<SearchContentInput>,
    ) -> Result<CallToolResult, McpError> {
        let doc_type = input.doc_type.as_ref().and_then(|s| parse_doc_type(s));
        let filter = tools::ListFilter {
            confidence: input.confidence.as_ref().map(|s| parse_confidence(s)),
            tag: input.tag,
            status: None,
        };

        match self.manager.search_content(
            &input.query,
            doc_type,
            &filter,
            input.lines_before,
            input.lines_after,
        ) {
            Ok(results) => Ok(CallToolResult::success(vec![Content::text(
                results.to_markdown(),
            )])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(format!(
                "Error: {}",
                e
            ))])),
        }
    }

    // === Crate Documentation Tools ===

    /// List all context-* crates with documentation
    #[tool(
        description = "List all context-* crates that have agents/docs directories. Shows crate name, version, description, module count, and whether they have a README."
    )]
    async fn list_crates(
        &self,
        #[allow(unused_variables)]
        Parameters(_input): Parameters<ListCratesInput>,
    ) -> Result<CallToolResult, McpError> {
        match self.crate_manager.discover_crates() {
            Ok(crates) => {
                let mut md = String::from("# Documented Crates\n\n");
                md.push_str("| Crate | Version | Modules | README | Description |\n");
                md.push_str("|-------|---------|---------|--------|-------------|\n");
                for c in &crates {
                    let version = c.version.as_deref().unwrap_or("-");
                    let readme = if c.has_readme { "✅" } else { "❌" };
                    md.push_str(&format!(
                        "| {} | {} | {} | {} | {} |\n",
                        c.name, version, c.module_count, readme, c.description
                    ));
                }
                Ok(CallToolResult::success(vec![Content::text(md)]))
            }
            Err(e) => Ok(CallToolResult::error(vec![Content::text(format!(
                "Error: {}",
                e
            ))])),
        }
    }

    /// Browse a crate's module tree
    #[tool(
        description = "Browse the module tree of a specific crate. Shows all modules, submodules, files, and key types hierarchically."
    )]
    async fn browse_crate(
        &self,
        Parameters(input): Parameters<BrowseCrateInput>,
    ) -> Result<CallToolResult, McpError> {
        match self.crate_manager.browse_crate(&input.crate_name) {
            Ok(tree) => {
                let md = format_module_tree(&tree, 0);
                Ok(CallToolResult::success(vec![Content::text(md)]))
            }
            Err(e) => Ok(CallToolResult::error(vec![Content::text(format!(
                "Error: {}",
                e
            ))])),
        }
    }

    /// Read crate or module documentation
    #[tool(
        description = "Read the documentation for a crate or specific module. Returns the index.yaml metadata and optionally the README.md content."
    )]
    async fn read_crate_doc(
        &self,
        Parameters(input): Parameters<ReadCrateDocInput>,
    ) -> Result<CallToolResult, McpError> {
        match self.crate_manager.read_crate_doc(
            &input.crate_name,
            input.module_path.as_deref(),
            input.include_readme,
        ) {
            Ok(result) => Ok(CallToolResult::success(vec![Content::text(
                result.to_markdown(),
            )])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(format!(
                "Error: {}",
                e
            ))])),
        }
    }

    /// Update crate or module documentation
    #[tool(
        description = "Update the documentation for a crate or module. Can update index.yaml and/or README.md. Validates YAML before writing."
    )]
    async fn update_crate_doc(
        &self,
        Parameters(input): Parameters<UpdateCrateDocInput>,
    ) -> Result<CallToolResult, McpError> {
        match self.crate_manager.update_crate_doc(
            &input.crate_name,
            input.module_path.as_deref(),
            input.index_yaml.as_deref(),
            input.readme.as_deref(),
        ) {
            Ok(()) => {
                let location = match &input.module_path {
                    Some(p) => format!("{}::{}", input.crate_name, p.replace('/', "::")),
                    None => input.crate_name.clone(),
                };
                Ok(CallToolResult::success(vec![Content::text(format!(
                    "Updated documentation for: {}",
                    location
                ))]))
            }
            Err(e) => Ok(CallToolResult::error(vec![Content::text(format!(
                "Error: {}",
                e
            ))])),
        }
    }

    /// Create documentation for a new module
    #[tool(
        description = "Create a new module documentation directory with an initial index.yaml. Use this when documenting a new module that doesn't have docs yet."
    )]
    async fn create_module_doc(
        &self,
        Parameters(input): Parameters<CreateModuleDocInput>,
    ) -> Result<CallToolResult, McpError> {
        match self.crate_manager.create_module_doc(
            &input.crate_name,
            &input.module_path,
            &input.name,
            &input.description,
        ) {
            Ok(path) => Ok(CallToolResult::success(vec![Content::text(format!(
                "Created module documentation at: {}",
                path
            ))])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(format!(
                "Error: {}",
                e
            ))])),
        }
    }

    /// Search crate documentation
    #[tool(
        description = "Search across all crate documentation for types, traits, macros, modules, or content. Returns matches with context."
    )]
    async fn search_crate_docs(
        &self,
        Parameters(input): Parameters<SearchCrateDocsInput>,
    ) -> Result<CallToolResult, McpError> {
        match self.crate_manager.search_crate_docs(
            &input.query,
            input.crate_filter.as_deref(),
            input.search_types,
            input.search_content,
        ) {
            Ok(results) => {
                let mut md = format!("# Search Results: \"{}\"\n\n", input.query);
                md.push_str(&format!("**{} matches found**\n\n", results.len()));
                
                if results.is_empty() {
                    md.push_str("No matches found.\n");
                } else {
                    md.push_str("| Crate | Module | Type | Name | Description |\n");
                    md.push_str("|-------|--------|------|------|-------------|\n");
                    for r in &results {
                        let module = if r.module_path.is_empty() { "-" } else { &r.module_path };
                        let desc = r.description.as_deref()
                            .or(r.context.as_deref())
                            .unwrap_or("-");
                        md.push_str(&format!(
                            "| {} | {} | {} | {} | {} |\n",
                            r.crate_name, module, r.match_type, r.name, desc
                        ));
                    }
                }
                
                Ok(CallToolResult::success(vec![Content::text(md)]))
            }
            Err(e) => Ok(CallToolResult::error(vec![Content::text(format!(
                "Error: {}",
                e
            ))])),
        }
    }

    /// Validate crate documentation
    #[tool(
        description = "Validate crate documentation for consistency: check that all referenced modules exist, YAML is valid, etc. Returns a report of errors and warnings."
    )]
    async fn validate_crate_docs(
        &self,
        Parameters(input): Parameters<ValidateCrateDocsInput>,
    ) -> Result<CallToolResult, McpError> {
        match self.crate_manager.validate_crate_docs(input.crate_filter.as_deref()) {
            Ok(report) => Ok(CallToolResult::success(vec![Content::text(
                report.to_markdown(),
            )])),
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
                "MCP Docs Server for managing structured agent documentation and crate API docs.\n\n\
                 Agent Docs (guides, plans, bug-reports, etc.):\n\
                 - create_doc, list_docs, read_doc, update_doc_meta, search_docs, browse_docs\n\
                 - regenerate_index, validate_docs, get_docs_needing_review, search_content\n\n\
                 Crate API Docs (crates/*/agents/docs/):\n\
                 - list_crates, browse_crate, read_crate_doc, update_crate_doc\n\
                 - create_module_doc, search_crate_docs, validate_crate_docs"
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

/// Format a module tree node as markdown
fn format_module_tree(node: &schema::ModuleTreeNode, depth: usize) -> String {
    use std::fmt::Write;
    let mut md = String::new();
    let indent = "  ".repeat(depth);
    let prefix = if depth == 0 { "#" } else { &"#".repeat((depth + 1).min(4)) };
    
    let _ = writeln!(md, "{} {}", prefix, node.name);
    if !node.description.is_empty() {
        let _ = writeln!(md, "{}*{}*\n", indent, node.description);
    }
    
    // Show key types
    if !node.key_types.is_empty() {
        let _ = writeln!(md, "{}**Key Types:**", indent);
        for t in &node.key_types {
            let desc = t.description().map(|d| format!(" - {}", d)).unwrap_or_default();
            let _ = writeln!(md, "{}- `{}`{}", indent, t.name(), desc);
        }
        let _ = writeln!(md);
    }
    
    // Show files
    if !node.files.is_empty() {
        let _ = writeln!(md, "{}**Files:**", indent);
        for f in &node.files {
            let _ = writeln!(md, "{}- `{}` - {}", indent, f.name, f.description);
        }
        let _ = writeln!(md);
    }
    
    // Recurse into children
    for child in &node.children {
        md.push_str(&format_module_tree(child, depth + 1));
    }
    
    md
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get agents directory from environment or use default
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let workspace_root = manifest_dir
        .parent() // tools/
        .and_then(|p| p.parent()) // context-engine/
        .unwrap_or(&manifest_dir);
    
    let agents_dir = std::env::var("AGENTS_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| workspace_root.join("agents"));
    
    let crates_dir = std::env::var("CRATES_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| workspace_root.join("crates"));

    eprintln!("MCP Docs Server starting...");
    eprintln!("Agents directory: {}", agents_dir.display());
    eprintln!("Crates directory: {}", crates_dir.display());

    let server = DocsServer::new(agents_dir, crates_dir);

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

    #[test]
    fn test_format_module_tree() {
        use crate::schema::{FileEntry, TypeEntry, ModuleTreeNode};
        let tree = ModuleTreeNode {
            name: "test".to_string(),
            path: "".to_string(),
            description: "Test module".to_string(),
            children: vec![],
            files: vec![FileEntry {
                name: "mod.rs".to_string(),
                description: "Module root".to_string(),
            }],
            key_types: vec![TypeEntry::Simple("TestType".to_string())],
            has_readme: true,
        };
        let md = format_module_tree(&tree, 0);
        assert!(md.contains("# test"));
        assert!(md.contains("*Test module*"));
        assert!(md.contains("`TestType`"));
        assert!(md.contains("`mod.rs`"));
    }
}
