//! MCP tool handlers for the documentation server.

use rmcp::{
    handler::server::wrapper::Parameters,
    model::*,
    tool,
    tool_router,
    ErrorData as McpError,
};

use crate::helpers::{parse_doc_type, parse_detail_level, parse_status, format_module_tree};
use crate::tools::{self, agents::CreateDocParams};
use super::inputs::*;
use super::DocsServer;

// The #[tool_router] proc macro generates the tool_router() method which must be
// accessible from mod.rs. We need to impl the handlers directly in mod.rs or
// make this module's impl block visible.

#[tool_router]
impl DocsServer {
    /// Create a new documentation file from template
    #[tool(
        description = "Create a new documentation file from template. Automatically generates the dated filename, populates the template, and updates the INDEX."
    )]
    pub async fn create_doc(
        &self,
        Parameters(input): Parameters<CreateDocInput>,
    ) -> Result<CallToolResult, McpError> {
        let doc_type = parse_doc_type(&input.doc_type).ok_or_else(|| {
            McpError::invalid_params(
                format!("Invalid doc_type: {}", input.doc_type),
                None,
            )
        })?;

        let status = input.status.as_ref().and_then(|s| parse_status(s));

        let params = CreateDocParams {
            doc_type,
            name: input.name,
            title: input.title,
            summary: input.summary,
            tags: Some(input.tags),
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
    pub async fn list_docs(
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
        description = "Update metadata (tags, summary, status) for an existing document. Also regenerates the INDEX."
    )]
    pub async fn update_doc_meta(
        &self,
        Parameters(input): Parameters<UpdateMetaInput>,
    ) -> Result<CallToolResult, McpError> {
        let params = tools::agents::UpdateMetaParams {
            filename: input.filename.clone(),
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

    /// Search documents by query and/or tag
    #[tool(description = "Search for documents using query text (in titles, summaries, content) and/or filter by tag. At least one of query or tag must be provided.")]
    pub async fn search_docs(
        &self,
        Parameters(input): Parameters<SearchDocsInput>,
    ) -> Result<CallToolResult, McpError> {
        if input.query.is_none() && input.tag.is_none() {
            return Ok(CallToolResult::error(vec![Content::text(
                "Error: At least one of 'query' or 'tag' must be provided"
            )]));
        }
        
        let doc_type = input.doc_type.as_ref().and_then(|s| parse_doc_type(s));
        
        match self.manager.search_docs(
            input.query.as_deref(),
            input.tag.as_deref(),
            input.search_content,
            doc_type,
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

    /// Regenerate INDEX.md for a document type
    #[tool(
        description = "Regenerate the INDEX.md file for a document category by scanning all documents in the directory."
    )]
    pub async fn regenerate_index(
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
    pub async fn validate_docs(&self) -> Result<CallToolResult, McpError> {
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
    pub async fn read_doc(
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
        description = "Get documents that may need review based on age. Useful for maintenance and summarization workflows."
    )]
    pub async fn get_docs_needing_review(
        &self,
        Parameters(input): Parameters<GetDocsNeedingReviewInput>,
    ) -> Result<CallToolResult, McpError> {
        match self.manager.get_docs_needing_review(
            input.max_age_days,
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
    pub async fn browse_docs(
        &self,
        Parameters(input): Parameters<BrowseDocsInput>,
    ) -> Result<CallToolResult, McpError> {
        let doc_type = input.doc_type.as_ref().and_then(|s| parse_doc_type(s));
        let filter = tools::ListFilter {
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
    pub async fn search_content(
        &self,
        Parameters(input): Parameters<SearchContentInput>,
    ) -> Result<CallToolResult, McpError> {
        let doc_type = input.doc_type.as_ref().and_then(|s| parse_doc_type(s));
        let filter = tools::ListFilter {
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

    /// Add frontmatter to documents missing it
    #[tool(
        description = "Add YAML frontmatter to documents that are missing it. Can process a specific doc_type or 'all'. Use dry_run=true to preview changes without writing. Automatically infers tags from content."
    )]
    pub async fn add_frontmatter(
        &self,
        Parameters(input): Parameters<AddFrontmatterInput>,
    ) -> Result<CallToolResult, McpError> {
        let doc_type = if input.doc_type.to_lowercase() == "all" {
            None
        } else {
            match parse_doc_type(&input.doc_type) {
                Some(dt) => Some(dt),
                None => return Ok(CallToolResult::error(vec![Content::text(
                    format!("Invalid doc_type: {}. Use guide, plan, implemented, bug-report, analysis, or all", input.doc_type)
                )])),
            }
        };

        match self.manager.add_frontmatter(doc_type, input.dry_run) {
            Ok(result) => Ok(CallToolResult::success(vec![Content::text(
                result.to_markdown(),
            )])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(format!(
                "Error: {}",
                e
            ))])),
        }
    }

    /// Get documentation health dashboard
    #[tool(
        description = "Get a comprehensive health dashboard showing documentation metrics: frontmatter coverage, INDEX sync status, naming convention compliance, and document age distribution. Provides actionable recommendations."
    )]
    pub async fn health_dashboard(
        &self,
        Parameters(input): Parameters<HealthDashboardInput>,
    ) -> Result<CallToolResult, McpError> {
        match self.manager.health_dashboard(input.detailed) {
            Ok(dashboard) => Ok(CallToolResult::success(vec![Content::text(
                dashboard.to_markdown(),
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
    pub async fn list_crates(
        &self,
        #[allow(unused_variables)]
        Parameters(_input): Parameters<ListCratesInput>,
    ) -> Result<CallToolResult, McpError> {
        match self.crate_manager.discover_crates_with_diagnostics() {
            Ok(result) => {
                let mut md = String::from("# Documented Crates\n\n");
                
                // Show directory info
                md.push_str("**Crates Directories:**\n");
                for (dir, exists) in result.crates_dirs.iter().zip(result.dirs_exist.iter()) {
                    let status = if *exists { "✅" } else { "❌" };
                    md.push_str(&format!("- `{}` {}\n", dir, status));
                }
                md.push('\n');
                
                if result.crates.is_empty() {
                    md.push_str("*No documented crates found.*\n\n");
                } else {
                    md.push_str("| Crate | Version | Modules | README | Description |\n");
                    md.push_str("|-------|---------|---------|--------|-------------|\n");
                    for c in &result.crates {
                        let version = c.version.as_deref().unwrap_or("-");
                        let readme = if c.has_readme { "✅" } else { "❌" };
                        md.push_str(&format!(
                            "| {} | {} | {} | {} | {} |\n",
                            c.name, version, c.module_count, readme, c.description
                        ));
                    }
                    md.push('\n');
                }
                
                // Show diagnostics if any
                if !result.diagnostics.is_empty() {
                    md.push_str("## Diagnostics\n\n");
                    for diag in &result.diagnostics {
                        md.push_str(&format!("- {}\n", diag));
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

    /// Browse a crate's module tree
    #[tool(
        description = "Browse the module tree of a specific crate. Shows all modules, submodules, files, and key types hierarchically."
    )]
    pub async fn browse_crate(
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
    pub async fn read_crate_doc(
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
    pub async fn update_crate_doc(
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
    pub async fn create_module_doc(
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
    pub async fn search_crate_docs(
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
    pub async fn validate_crate_docs(
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

    /// Check documentation staleness
    #[tool(
        description = "Check if crate documentation is stale by comparing git modification times of source files against the last_synced timestamp in index.yaml. Returns a report showing which docs need updating."
    )]
    pub async fn check_stale_docs(
        &self,
        Parameters(input): Parameters<CheckStaleDocs>,
    ) -> Result<CallToolResult, McpError> {
        match self.crate_manager.check_stale_docs(
            input.crate_filter.as_deref(),
            input.stale_threshold_days,
            input.very_stale_threshold_days,
        ) {
            Ok(report) => Ok(CallToolResult::success(vec![Content::text(
                report.to_markdown(),
            )])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(format!(
                "Error: {}",
                e
            ))])),
        }
    }

    /// Sync crate documentation with source
    #[tool(
        description = "Analyze source files and suggest documentation updates. Parses Rust source files to find public types, traits, and macros, then compares with current documentation to suggest additions and removals. Optionally updates the last_synced timestamp. Use summary_only=true for a quick overview."
    )]
    pub async fn sync_crate_docs(
        &self,
        Parameters(input): Parameters<SyncCrateDocs>,
    ) -> Result<CallToolResult, McpError> {
        match self.crate_manager.sync_crate_docs(
            &input.crate_name,
            input.module_path.as_deref(),
            input.update_timestamp,
            input.summary_only,
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

    /// Update a crate's index.yaml configuration
    #[tool(
        description = "Update fields in a crate or module's index.yaml, such as source_files for stale detection. Can set, add, or remove source file entries."
    )]
    pub async fn update_crate_index(
        &self,
        Parameters(input): Parameters<UpdateCrateIndex>,
    ) -> Result<CallToolResult, McpError> {
        match self.crate_manager.update_crate_index(
            &input.crate_name,
            input.module_path.as_deref(),
            input.source_files,
            input.add_source_files,
            input.remove_source_files,
        ) {
            Ok(result) => Ok(CallToolResult::success(vec![Content::text(result)])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(format!(
                "Error: {}",
                e
            ))])),
        }
    }
}
