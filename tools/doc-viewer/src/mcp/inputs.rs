//! MCP input types for documentation server tools.

use rmcp::schemars::{self, JsonSchema};
use serde::Deserialize;

// === Crate Documentation Tools ===

/// List all context-* crates with documentation
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListCratesInput {
    // No parameters - lists all discovered crates
}

/// Browse a crate's module tree
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BrowseCrateInput {
    /// Name of the crate (e.g., "context-trace", "context-search")
    pub crate_name: String,
}

/// Read crate or module documentation
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ReadCrateDocInput {
    /// Name of the crate (e.g., "context-trace")
    pub crate_name: String,
    /// Optional module path within the crate (e.g., "graph/path"). If omitted, reads crate root.
    #[serde(default)]
    pub module_path: Option<String>,
    /// Include README.md content (default: true)
    #[serde(default = "default_true")]
    pub include_readme: bool,
}

/// Update crate or module documentation
#[derive(Debug, Deserialize, JsonSchema)]
pub struct UpdateCrateDocInput {
    /// Name of the crate
    pub crate_name: String,
    /// Optional module path (if omitted, updates crate root)
    #[serde(default)]
    pub module_path: Option<String>,
    /// New index.yaml content (optional)
    #[serde(default)]
    pub index_yaml: Option<String>,
    /// New README.md content (optional)
    #[serde(default)]
    pub readme: Option<String>,
}

/// Create documentation for a new module
#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateModuleDocInput {
    /// Name of the crate
    pub crate_name: String,
    /// Module path to create (e.g., "new_module" or "parent/child")
    pub module_path: String,
    /// Name of the module
    pub name: String,
    /// Description of the module
    pub description: String,
}

/// Search crate documentation
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SearchCrateDocsInput {
    /// Search query. Supports: regex patterns (graph|path, init.*), quoted literals ("hello world"), backslash escaping (\|). Case-insensitive.
    pub query: String,
    /// Optional: filter to specific crate
    #[serde(default)]
    pub crate_filter: Option<String>,
    /// Search in type/trait/macro names (default: true)
    #[serde(default = "default_true")]
    pub search_types: bool,
    /// Search in README content (default: true)
    #[serde(default = "default_true")]
    pub search_content: bool,
}

/// Validate crate documentation
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ValidateCrateDocsInput {
    /// Optional: validate only specific crate
    #[serde(default)]
    pub crate_filter: Option<String>,
}

/// Check documentation staleness using git history
#[derive(Debug, Deserialize, JsonSchema)]
pub struct CheckStaleDocs {
    /// Optional: check only specific crate
    #[serde(default)]
    pub crate_filter: Option<String>,
    /// Days after which docs are considered stale (default: 7)
    #[serde(default = "default_stale_threshold")]
    pub stale_threshold_days: i64,
    /// Days after which docs are considered very stale (default: 30)
    #[serde(default = "default_very_stale_threshold")]
    pub very_stale_threshold_days: i64,
}

fn default_stale_threshold() -> i64 {
    7
}

fn default_very_stale_threshold() -> i64 {
    30
}

/// Analyze source files and suggest documentation updates
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SyncCrateDocs {
    /// Name of the crate to analyze
    pub crate_name: String,
    /// Optional: specific module path to analyze
    #[serde(default)]
    pub module_path: Option<String>,
    /// Update the last_synced timestamp in index.yaml (default: false)
    #[serde(default)]
    pub update_timestamp: bool,
    /// Return only summary counts instead of full item lists (default: false)
    #[serde(default)]
    pub summary_only: bool,
}

/// Update fields in a crate or module's index.yaml
#[derive(Debug, Deserialize, JsonSchema)]
pub struct UpdateCrateIndex {
    /// Name of the crate
    pub crate_name: String,
    /// Optional: specific module path to update
    #[serde(default)]
    pub module_path: Option<String>,
    /// Set source_files to this list (replaces existing)
    #[serde(default)]
    pub source_files: Option<Vec<String>>,
    /// Add these files to existing source_files
    #[serde(default)]
    pub add_source_files: Option<Vec<String>>,
    /// Remove these files from source_files
    #[serde(default)]
    pub remove_source_files: Option<Vec<String>>,
}

// === Agent Documentation Tools ===

/// Create a new document from template
#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateDocInput {
    /// Document type: "guide", "plan", "implemented", "bug-report", or "analysis"
    pub doc_type: String,
    /// Short name for the file (becomes UPPER_SNAKE_CASE in filename)
    pub name: String,
    /// Human-readable title for the document header
    pub title: String,
    /// One-line summary for the INDEX file
    pub summary: String,
    /// Tags for categorization (without #)
    #[serde(default)]
    pub tags: Vec<String>,
    /// Status for plans: "design", "in-progress", "completed", "blocked", "superseded"
    #[serde(default)]
    pub status: Option<String>,
}

/// List documents by type with optional filters
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListDocsInput {
    /// Document type: "guide", "plan", "implemented", "bug-report", or "analysis"
    pub doc_type: String,
    /// Filter by tag (optional, matches any document containing this tag)
    #[serde(default)]
    pub tag: Option<String>,
    /// Filter by status for plans: "design", "in-progress", "completed", "blocked", "superseded" (optional)
    #[serde(default)]
    pub status: Option<String>,
}

/// Update document metadata
#[derive(Debug, Deserialize, JsonSchema)]
pub struct UpdateMetaInput {
    /// Filename of the document to update
    pub filename: String,
    /// New tags (optional, replaces existing)
    #[serde(default)]
    pub tags: Option<Vec<String>>,
    /// New summary (optional)
    #[serde(default)]
    pub summary: Option<String>,
    /// New status for plans (optional)
    #[serde(default)]
    pub status: Option<String>,
}

/// Search documents by query and/or tag
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SearchDocsInput {
    /// Search query. Supports: regex patterns (graph|path, init.*), quoted literals ("hello world"), backslash escaping (\|). Case-insensitive. Optional if tag is provided.
    #[serde(default)]
    pub query: Option<String>,
    /// Tag to filter by (with or without #). Optional if query is provided.
    #[serde(default)]
    pub tag: Option<String>,
    /// Search within document content too (default: false, searches only metadata)
    #[serde(default)]
    pub search_content: bool,
    /// Filter to specific doc type
    #[serde(default)]
    pub doc_type: Option<String>,
}

/// Regenerate INDEX
#[derive(Debug, Deserialize, JsonSchema)]
pub struct RegenerateIndexInput {
    /// Document type: "guide", "plan", "implemented", "bug-report", or "analysis"
    pub doc_type: String,
}

/// Read a document with configurable detail level
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ReadDocInput {
    /// Filename of the document to read (e.g., "20250610_FEATURE_NAME.md")
    pub filename: String,
    /// Detail level: "outline" (headers only), "summary" (metadata, no body), "full" (everything). Default: "summary"
    #[serde(default = "default_detail_level")]
    pub detail: String,
}

fn default_detail_level() -> String {
    "summary".to_string()
}

/// Browse documentation structure (table of contents)
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BrowseDocsInput {
    /// Optional: filter to specific doc_type ("guide", "plan", etc.). If omitted, shows all categories.
    #[serde(default)]
    pub doc_type: Option<String>,
    /// Optional: filter by tag
    #[serde(default)]
    pub tag: Option<String>,
}

/// Get documents needing review
#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetDocsNeedingReviewInput {
    /// Maximum age in days - documents older than this will be included (default: 30)
    #[serde(default = "default_max_age_days")]
    pub max_age_days: u32,
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
    /// Search query. Supports: regex patterns (graph|path, init.*), quoted literals ("hello world"), backslash escaping (\|). Case-insensitive.
    pub query: String,
    /// Optional: filter to specific doc_type
    #[serde(default)]
    pub doc_type: Option<String>,
    /// Optional: filter by tag
    #[serde(default)]
    pub tag: Option<String>,
    /// Number of lines to include before each match (default: 2)
    #[serde(default = "default_context_lines")]
    pub lines_before: usize,
    /// Number of lines to include after each match (default: 2)
    #[serde(default = "default_context_lines")]
    pub lines_after: usize,
}

fn default_context_lines() -> usize {
    2
}

/// Add frontmatter to documents missing it
#[derive(Debug, Deserialize, JsonSchema)]
pub struct AddFrontmatterInput {
    /// Document type to process (or "all" for all types)
    pub doc_type: String,
    /// Preview changes without writing (default: false)
    #[serde(default)]
    pub dry_run: bool,
}

/// Get documentation health dashboard
#[derive(Debug, Deserialize, JsonSchema)]
pub struct HealthDashboardInput {
    /// Include detailed breakdown by category (default: true)
    #[serde(default = "default_true")]
    pub detailed: bool,
}
