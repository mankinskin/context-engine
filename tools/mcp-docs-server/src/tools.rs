//! MCP tool definitions for documentation management.

use crate::{
    parser::{
        extract_metadata,
        parse_crate_index,
        parse_module_index,
        read_markdown_file,
    },
    schema::{
        DocMetadata,
        DocType,
        IndexEntry,
        PlanStatus,
        CrateMetadata,
        ModuleMetadata,
        CrateSummary,
        ModuleTreeNode,
        CrateSearchResult,
        CrateValidationIssue,
        CrateValidationReport,
        TypeEntry,
    },
    templates::{
        generate_document,
        generate_index,
    },
};
use serde::{
    Deserialize,
    Serialize,
};
use std::{
    fs,
    path::{
        Path,
        PathBuf,
    },
};

/// Result type for tool operations.
pub type ToolResult<T> = Result<T, ToolError>;

#[derive(Debug, thiserror::Error)]
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
    pub status: Option<PlanStatus>,
}

/// Documentation manager handling all operations.
pub struct DocsManager {
    agents_dir: PathBuf,
}

impl DocsManager {
    pub fn new(agents_dir: PathBuf) -> Self {
        Self { agents_dir }
    }

    /// Create a new document from parameters.
    pub fn create_document(
        &self,
        params: CreateDocParams,
    ) -> ToolResult<CreateDocResult> {
        let date = chrono::Local::now().format("%Y%m%d").to_string();
        let prefix = params.doc_type.file_prefix();
        let name_upper = params
            .name
            .to_uppercase()
            .replace(' ', "_")
            .replace('-', "_");
        let filename = format!("{}{}_{}.md", date, prefix, name_upper);

        let dir = self.agents_dir.join(params.doc_type.directory());
        let path = dir.join(&filename);

        if path.exists() {
            return Err(ToolError::AlreadyExists(filename));
        }

        let meta = DocMetadata {
            doc_type: params.doc_type,
            date,
            title: params.title,
            filename: filename.clone(),
            tags: params.tags.unwrap_or_default(),
            summary: params.summary,
            status: params.status,
        };

        let content = generate_document(&meta);
        fs::create_dir_all(&dir)?;
        fs::write(&path, &content)?;

        // Update INDEX
        self.update_index(params.doc_type)?;

        Ok(CreateDocResult {
            path: path.display().to_string(),
            filename,
        })
    }

    /// List all documents of a given type.
    pub fn list_documents(
        &self,
        doc_type: DocType,
    ) -> ToolResult<Vec<DocSummary>> {
        let dir = self.agents_dir.join(doc_type.directory());
        let mut docs = Vec::new();

        if !dir.exists() {
            return Ok(docs);
        }

        for entry in fs::read_dir(&dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().map_or(false, |e| e == "md") {
                let filename = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or_default()
                    .to_string();

                if filename == "INDEX.md" {
                    continue;
                }

                let content = fs::read_to_string(&path)?;
                if let Some(meta) = extract_metadata(&path, &content) {
                    docs.push(DocSummary {
                        filename: meta.filename,
                        title: meta.title,
                        date: meta.date,
                        summary: meta.summary,
                        tags: meta.tags,
                        status: meta.status,
                    });
                }
            }
        }

        // Sort by date descending
        docs.sort_by(|a, b| b.date.cmp(&a.date));

        Ok(docs)
    }

    /// Update the INDEX.md for a document type.
    pub fn update_index(
        &self,
        doc_type: DocType,
    ) -> ToolResult<String> {
        let docs = self.list_documents(doc_type)?;
        let entries: Vec<IndexEntry> = docs
            .iter()
            .map(|d| IndexEntry {
                date: d.date.clone(),
                filename: d.filename.clone(),
                summary: d.summary.clone(),
                status: d.status,
            })
            .collect();

        let content = generate_index(doc_type, &entries);
        let path = self.agents_dir.join(doc_type.directory()).join("INDEX.md");
        fs::write(&path, &content)?;

        Ok(path.display().to_string())
    }

    /// Update metadata for an existing document.
    pub fn update_document_metadata(
        &self,
        params: UpdateMetaParams,
    ) -> ToolResult<()> {
        let path = self.find_document(&params.filename)?;
        let content = fs::read_to_string(&path)?;

        // Parse existing and merge updates
        let mut meta = extract_metadata(&path, &content).ok_or_else(|| {
            ToolError::InvalidInput("Cannot parse document".into())
        })?;

        if let Some(tags) = params.tags {
            meta.tags = tags;
        }
        if let Some(summary) = params.summary {
            meta.summary = summary;
        }
        if let Some(status) = params.status {
            meta.status = Some(status);
        }

        // Regenerate frontmatter only (preserve body)
        let body = extract_body(&content);
        let new_content = format!("{}\n{}", generate_frontmatter(&meta), body);
        fs::write(&path, new_content)?;

        // Update index
        self.update_index(meta.doc_type)?;

        Ok(())
    }

    /// Search documents by tag.
    pub fn search_by_tag(
        &self,
        tag: &str,
    ) -> ToolResult<Vec<DocSummary>> {
        let mut results = Vec::new();
        let tag_lower = tag.to_lowercase().trim_start_matches('#').to_string();

        for doc_type in [
            DocType::Guide,
            DocType::Plan,
            DocType::Implemented,
            DocType::BugReport,
            DocType::Analysis,
        ] {
            let docs = self.list_documents(doc_type)?;
            for doc in docs {
                if doc.tags.iter().any(|t| t.to_lowercase() == tag_lower) {
                    results.push(doc);
                }
            }
        }

        results.sort_by(|a, b| b.date.cmp(&a.date));
        Ok(results)
    }

    /// Validate all documents and indexes.
    pub fn validate(&self) -> ToolResult<ValidationReport> {
        use crate::parser::{
            parse_filename,
            parse_frontmatter,
            parse_title,
        };

        let mut report = ValidationReport::default();

        for doc_type in [
            DocType::Guide,
            DocType::Plan,
            DocType::Implemented,
            DocType::BugReport,
            DocType::Analysis,
        ] {
            let dir = self.agents_dir.join(doc_type.directory());
            if !dir.exists() {
                report.issues.push(ValidationIssue {
                    file: doc_type.directory().to_string(),
                    category: doc_type.directory().to_string(),
                    issue: "Directory does not exist".to_string(),
                    severity: IssueSeverity::Warning,
                });
                continue;
            }

            // Check INDEX.md exists
            let index_path = dir.join("INDEX.md");
            if !index_path.exists() {
                report.issues.push(ValidationIssue {
                    file: "INDEX.md".to_string(),
                    category: doc_type.directory().to_string(),
                    issue: "Missing INDEX.md file".to_string(),
                    severity: IssueSeverity::Error,
                });
            }

            for entry in fs::read_dir(&dir)? {
                let entry = entry?;
                let path = entry.path();
                let filename = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or_default();

                if filename == "INDEX.md" || !filename.ends_with(".md") {
                    continue;
                }

                let category = doc_type.directory().to_string();

                // Check naming convention (YYYYMMDD_ prefix)
                if parse_filename(filename).is_none() {
                    report.issues.push(ValidationIssue {
                        file: filename.to_string(),
                        category: category.clone(),
                        issue: "Invalid filename format - expected YYYYMMDD_NAME.md".to_string(),
                        severity: IssueSeverity::Error,
                    });
                }

                // Check file content
                let content = fs::read_to_string(&path)?;

                // Check frontmatter exists
                if !content.starts_with("---") {
                    report.issues.push(ValidationIssue {
                        file: filename.to_string(),
                        category: category.clone(),
                        issue: "Missing frontmatter (should start with ---)"
                            .to_string(),
                        severity: IssueSeverity::Error,
                    });
                } else {
                    // Parse and validate frontmatter
                    if let Some(fm) = parse_frontmatter(&content) {
                        // Check tags exist
                        if fm.tags.is_empty() {
                            report.issues.push(ValidationIssue {
                                file: filename.to_string(),
                                category: category.clone(),
                                issue: "No tags defined in frontmatter"
                                    .to_string(),
                                severity: IssueSeverity::Warning,
                            });
                        }

                        // Plans should have status
                        if doc_type == DocType::Plan && fm.status.is_none() {
                            report.issues.push(ValidationIssue {
                                file: filename.to_string(),
                                category: category.clone(),
                                issue: "Plan document missing status field"
                                    .to_string(),
                                severity: IssueSeverity::Warning,
                            });
                        }
                    } else {
                        report.issues.push(ValidationIssue {
                            file: filename.to_string(),
                            category: category.clone(),
                            issue: "Could not parse frontmatter".to_string(),
                            severity: IssueSeverity::Warning,
                        });
                    }
                }

                // Check for H1 title
                if parse_title(&content).is_none() {
                    report.issues.push(ValidationIssue {
                        file: filename.to_string(),
                        category: category.clone(),
                        issue: "Missing H1 title (# Title)".to_string(),
                        severity: IssueSeverity::Warning,
                    });
                }

                report.documents_checked += 1;
            }

            // Validate INDEX.md content
            if index_path.exists() {
                let category = doc_type.directory().to_string();
                if let Ok(index_content) = fs::read_to_string(&index_path) {
                    // Collect all document filenames in directory
                    let mut doc_files: Vec<String> = Vec::new();
                    if let Ok(entries) = fs::read_dir(&dir) {
                        for entry in entries.flatten() {
                            let fname =
                                entry.file_name().to_string_lossy().to_string();
                            if fname.ends_with(".md") && fname != "INDEX.md" {
                                doc_files.push(fname);
                            }
                        }
                    }

                    // Check each document is mentioned in INDEX
                    for doc_file in &doc_files {
                        if !index_content.contains(doc_file) {
                            report.issues.push(ValidationIssue {
                                file: "INDEX.md".to_string(),
                                category: category.clone(),
                                issue: format!(
                                    "Document '{}' not listed in INDEX",
                                    doc_file
                                ),
                                severity: IssueSeverity::Warning,
                            });
                        }
                    }

                    // Check for stale entries in INDEX (files mentioned but don't exist)
                    // Look for patterns like "### YYYYMMDD_*.md" or "| YYYYMMDD | filename.md |"
                    let filename_pattern =
                        regex::Regex::new(r"\d{8}_[A-Za-z0-9_-]+\.md").unwrap();
                    for caps in filename_pattern.find_iter(&index_content) {
                        let mentioned_file = caps.as_str();
                        if !doc_files.contains(&mentioned_file.to_string()) {
                            report.issues.push(ValidationIssue {
                                file: "INDEX.md".to_string(),
                                category: category.clone(),
                                issue: format!(
                                    "Stale entry '{}' - file does not exist",
                                    mentioned_file
                                ),
                                severity: IssueSeverity::Error,
                            });
                        }
                    }

                    // Check INDEX has H1 title
                    if parse_title(&index_content).is_none() {
                        report.issues.push(ValidationIssue {
                            file: "INDEX.md".to_string(),
                            category: category.clone(),
                            issue: "INDEX.md missing H1 title".to_string(),
                            severity: IssueSeverity::Warning,
                        });
                    }

                    // Check INDEX uses minimal table format
                    // Should have a table with columns: Date | File | Summary
                    let has_doc_table = index_content
                        .contains("| Date | File | Summary |")
                        || index_content
                            .contains("|------|------|");

                    if !has_doc_table {
                        report.issues.push(ValidationIssue {
                            file: "INDEX.md".to_string(),
                            category: category.clone(),
                            issue: "INDEX.md should use minimal table format: | Date | File | Summary |".to_string(),
                            severity: IssueSeverity::Warning,
                        });
                    }

                    // Check for verbose formatting patterns that should be avoided
                    let verbose_patterns = [
                        "**What it provides:**",
                        "**Key locations:**",
                        "**Solves:**",
                        "**Benefits:**",
                        "**Technique:**",
                        "**Tags:** `#", // Tags should be in document, not INDEX
                    ];

                    for pattern in verbose_patterns {
                        if index_content.contains(pattern) {
                            report.issues.push(ValidationIssue {
                                file: "INDEX.md".to_string(),
                                category: category.clone(),
                                issue: format!("INDEX.md too verbose - remove '{}' sections (use table format)", pattern),
                                severity: IssueSeverity::Warning,
                            });
                        }
                    }

                    // Check for excessive line count (INDEX should be concise)
                    let line_count = index_content.lines().count();
                    let expected_max = 20 + (doc_files.len() * 2); // Header + 2 lines per doc max
                    if line_count > expected_max {
                        report.issues.push(ValidationIssue {
                            file: "INDEX.md".to_string(),
                            category: category.clone(),
                            issue: format!("INDEX.md too long ({} lines, expected <{}). Use minimal table format.", line_count, expected_max),
                            severity: IssueSeverity::Warning,
                        });
                    }
                }
            }
        }

        Ok(report)
    }

    fn find_document(
        &self,
        filename: &str,
    ) -> ToolResult<PathBuf> {
        for doc_type in [
            DocType::Guide,
            DocType::Plan,
            DocType::Implemented,
            DocType::BugReport,
            DocType::Analysis,
        ] {
            let path =
                self.agents_dir.join(doc_type.directory()).join(filename);
            if path.exists() {
                return Ok(path);
            }
        }
        Err(ToolError::NotFound(filename.to_string()))
    }

    /// Read the full content of a document.
    pub fn read_document(
        &self,
        filename: &str,
        detail: DetailLevel,
    ) -> ToolResult<ReadDocResult> {
        let path = self.find_document(filename)?;
        let content = fs::read_to_string(&path)?;

        let meta = extract_metadata(&path, &content).ok_or_else(|| {
            ToolError::InvalidInput("Cannot parse document".into())
        })?;

        let body = match detail {
            DetailLevel::Outline => extract_outline(&content),
            DetailLevel::Summary => None,
            DetailLevel::Full => Some(extract_body(&content)),
        };

        Ok(ReadDocResult {
            filename: meta.filename,
            doc_type: meta.doc_type.directory().to_string(),
            title: meta.title,
            date: meta.date,
            summary: meta.summary,
            tags: meta.tags,
            status: meta.status,
            body,
        })
    }

    /// List documents with optional filters.
    pub fn list_documents_filtered(
        &self,
        doc_type: DocType,
        filter: &ListFilter,
    ) -> ToolResult<Vec<DocSummary>> {
        let docs = self.list_documents(doc_type)?;

        let filtered = docs
            .into_iter()
            .filter(|doc| {
                // Filter by tag
                if let Some(tag) = &filter.tag {
                    let tag_lower =
                        tag.to_lowercase().trim_start_matches('#').to_string();
                    if !doc.tags.iter().any(|t| t.to_lowercase() == tag_lower) {
                        return false;
                    }
                }
                // Filter by status
                if let Some(status) = &filter.status {
                    if doc.status.as_ref() != Some(status) {
                        return false;
                    }
                }
                true
            })
            .collect();

        Ok(filtered)
    }

    /// Browse documentation structure (TOC view).
    pub fn browse_docs(
        &self,
        doc_type: Option<DocType>,
        filter: &ListFilter,
    ) -> ToolResult<BrowseResult> {
        let doc_types = match doc_type {
            Some(dt) => vec![dt],
            None => vec![
                DocType::Guide,
                DocType::Plan,
                DocType::Implemented,
                DocType::BugReport,
                DocType::Analysis,
            ],
        };

        let mut categories = Vec::new();
        let mut total_docs = 0;

        for dt in doc_types {
            let docs = self.list_documents_filtered(dt, filter)?;
            let count = docs.len();
            total_docs += count;

            let items: Vec<TocItem> = docs
                .into_iter()
                .map(|d| TocItem {
                    filename: d.filename,
                    date: d.date,
                    summary: d.summary,
                })
                .collect();

            categories.push(CategorySummary {
                category: dt.directory().to_string(),
                doc_count: count,
                items,
            });
        }

        Ok(BrowseResult {
            total_documents: total_docs,
            categories,
        })
    }

    /// Get documents that may need review (old documents).
    pub fn get_docs_needing_review(
        &self,
        max_age_days: u32,
    ) -> ToolResult<Vec<ReviewCandidate>> {
        let mut candidates = Vec::new();
        let today = chrono::Local::now().date_naive();

        for doc_type in [
            DocType::Guide,
            DocType::Plan,
            DocType::Implemented,
            DocType::BugReport,
            DocType::Analysis,
        ] {
            let docs = self.list_documents(doc_type)?;

            for doc in docs {
                // Parse date from YYYYMMDD format
                let doc_date =
                    chrono::NaiveDate::parse_from_str(&doc.date, "%Y%m%d")
                        .unwrap_or(today);
                let age_days = (today - doc_date).num_days().max(0) as u64;

                // Check age
                if age_days > max_age_days as u64 {
                    candidates.push(ReviewCandidate {
                        filename: doc.filename,
                        doc_type: doc_type.directory().to_string(),
                        date: doc.date,
                        age_days,
                        summary: doc.summary,
                        reason: format!("Old ({} days)", age_days),
                    });
                }
            }
        }

        // Sort by age descending (oldest first)
        candidates.sort_by(|a, b| b.age_days.cmp(&a.age_days));

        Ok(candidates)
    }

    /// Search document content for a query string.
    pub fn search_content(
        &self,
        query: &str,
        doc_type: Option<DocType>,
        filter: &ListFilter,
        lines_before: usize,
        lines_after: usize,
    ) -> ToolResult<ContentSearchResult> {
        let doc_types = match doc_type {
            Some(dt) => vec![dt],
            None => vec![
                DocType::Guide,
                DocType::Plan,
                DocType::Implemented,
                DocType::BugReport,
                DocType::Analysis,
            ],
        };

        let query_lower = query.to_lowercase();
        let mut matches = Vec::new();
        let mut files_searched = 0;
        let mut total_matches = 0;

        for dt in doc_types {
            let docs = self.list_documents_filtered(dt, filter)?;

            for doc in docs {
                files_searched += 1;
                let path =
                    self.agents_dir.join(dt.directory()).join(&doc.filename);
                let content = fs::read_to_string(&path)?;
                let lines: Vec<&str> = content.lines().collect();

                let mut excerpts = Vec::new();

                for (idx, line) in lines.iter().enumerate() {
                    if line.to_lowercase().contains(&query_lower) {
                        total_matches += 1;

                        // Gather context before
                        let start = idx.saturating_sub(lines_before);
                        let context_before: Vec<String> = lines[start..idx]
                            .iter()
                            .map(|s| s.to_string())
                            .collect();

                        // Gather context after
                        let end = (idx + 1 + lines_after).min(lines.len());
                        let context_after: Vec<String> = lines[idx + 1..end]
                            .iter()
                            .map(|s| s.to_string())
                            .collect();

                        excerpts.push(MatchExcerpt {
                            line_number: idx + 1,
                            line: line.to_string(),
                            context_before,
                            context_after,
                        });
                    }
                }

                if !excerpts.is_empty() {
                    matches.push(FileMatch {
                        filename: doc.filename,
                        doc_type: dt.directory().to_string(),
                        match_count: excerpts.len(),
                        excerpts,
                    });
                }
            }
        }

        Ok(ContentSearchResult {
            query: query.to_string(),
            total_matches,
            files_searched,
            matches,
        })
    }

    /// Enhanced search: search by query and/or tag, optionally searching content
    pub fn search_docs(
        &self,
        query: Option<&str>,
        tag: Option<&str>,
        search_content: bool,
        doc_type: Option<DocType>,
    ) -> ToolResult<Vec<DocSummary>> {
        let doc_types = match doc_type {
            Some(dt) => vec![dt],
            None => vec![
                DocType::Guide,
                DocType::Plan,
                DocType::Implemented,
                DocType::BugReport,
                DocType::Analysis,
            ],
        };

        let query_lower = query.map(|q| q.to_lowercase());
        let tag_lower = tag.map(|t| t.to_lowercase().trim_start_matches('#').to_string());

        let mut results = Vec::new();

        for dt in doc_types {
            let docs = self.list_documents(dt)?;

            for doc in docs {
                let mut matches = false;

                // Check tag if provided
                if let Some(ref tag_l) = tag_lower {
                    if doc.tags.iter().any(|t| t.to_lowercase() == *tag_l) {
                        matches = true;
                    }
                }

                // Check query if provided
                if let Some(ref query_l) = query_lower {
                    // Search in title and summary
                    if doc.title.to_lowercase().contains(query_l) 
                        || doc.summary.to_lowercase().contains(query_l) 
                    {
                        matches = true;
                    }

                    // Search in content if requested
                    if !matches && search_content {
                        let path = self.agents_dir.join(dt.directory()).join(&doc.filename);
                        if let Ok(content) = fs::read_to_string(&path) {
                            if content.to_lowercase().contains(query_l) {
                                matches = true;
                            }
                        }
                    }
                }

                // If only tag provided and it matches, or if query matches
                // Handle the case where only one filter is provided
                if matches || (tag.is_none() && query.is_none()) {
                    // This shouldn't happen as we validate input, but be safe
                }

                // More precise logic: if both provided, need tag match; if only one, need that one
                let tag_ok = tag_lower.as_ref().map_or(true, |tag_l| {
                    doc.tags.iter().any(|t| t.to_lowercase() == *tag_l)
                });
                
                let query_ok = query_lower.as_ref().map_or(true, |query_l| {
                    let title_match = doc.title.to_lowercase().contains(query_l);
                    let summary_match = doc.summary.to_lowercase().contains(query_l);
                    
                    if title_match || summary_match {
                        return true;
                    }
                    
                    if search_content {
                        let path = self.agents_dir.join(dt.directory()).join(&doc.filename);
                        if let Ok(content) = fs::read_to_string(&path) {
                            return content.to_lowercase().contains(query_l);
                        }
                    }
                    false
                });

                if tag_ok && query_ok {
                    results.push(doc);
                }
            }
        }

        results.sort_by(|a, b| b.date.cmp(&a.date));
        Ok(results)
    }

    /// Add frontmatter to documents that are missing it
    pub fn add_frontmatter(
        &self,
        doc_type: Option<DocType>,
        dry_run: bool,
    ) -> ToolResult<AddFrontmatterResult> {
        use crate::parser::{parse_filename, parse_frontmatter, parse_title};
        
        let doc_types = match doc_type {
            Some(dt) => vec![dt],
            None => vec![
                DocType::Guide,
                DocType::Plan,
                DocType::Implemented,
                DocType::BugReport,
                DocType::Analysis,
            ],
        };

        let mut result = AddFrontmatterResult {
            processed: 0,
            updated: 0,
            skipped: 0,
            errors: Vec::new(),
            changes: Vec::new(),
        };

        for dt in doc_types {
            let dir = self.agents_dir.join(dt.directory());
            if !dir.exists() {
                continue;
            }

            for entry in fs::read_dir(&dir)? {
                let entry = entry?;
                let path = entry.path();
                let filename = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or_default();

                // Skip non-md files and INDEX.md
                if !filename.ends_with(".md") || filename == "INDEX.md" {
                    continue;
                }

                result.processed += 1;

                let content = match fs::read_to_string(&path) {
                    Ok(c) => c,
                    Err(e) => {
                        result.errors.push(format!("{}: {}", filename, e));
                        continue;
                    }
                };

                // Check if frontmatter exists
                if content.trim_start().starts_with("---") {
                    if let Some(_) = parse_frontmatter(&content) {
                        result.skipped += 1;
                        continue;
                    }
                }

                // Needs frontmatter - infer metadata
                let (date, _name) = parse_filename(filename).unwrap_or_else(|| {
                    ("00000000".to_string(), filename.trim_end_matches(".md").to_string())
                });

                let title = parse_title(&content).unwrap_or_else(|| filename.to_string());

                // Try to extract summary from first paragraph
                let summary = extract_summary(&content).unwrap_or_default();

                // Infer tags from filename and content
                let tags = infer_tags(filename, &content, dt);

                let meta = DocMetadata {
                    doc_type: dt,
                    date: date.clone(),
                    filename: filename.to_string(),
                    tags: tags.clone(),
                    summary: summary.clone(),
                    status: if dt == DocType::Plan { Some(PlanStatus::Design) } else { None },
                    title: title.clone(),
                };

                let frontmatter = generate_frontmatter(&meta);
                let new_content = format!("{}\n\n{}", frontmatter, content.trim_start());

                result.changes.push(FrontmatterChange {
                    filename: filename.to_string(),
                    doc_type: dt.directory().to_string(),
                    inferred_tags: tags,
                    inferred_summary: summary,
                });

                if !dry_run {
                    if let Err(e) = fs::write(&path, new_content) {
                        result.errors.push(format!("{}: {}", filename, e));
                        continue;
                    }
                }

                result.updated += 1;
            }
        }

        Ok(result)
    }

    /// Get a health dashboard summarizing documentation status
    pub fn health_dashboard(&self, detailed: bool) -> ToolResult<HealthDashboard> {
        let validation = self.validate()?;
        
        let mut dashboard = HealthDashboard {
            total_documents: 0,
            frontmatter_coverage: 0.0,
            index_sync_issues: 0,
            naming_issues: 0,
            old_documents: 0,
            categories: Vec::new(),
        };

        let mut docs_with_frontmatter = 0;

        for doc_type in [
            DocType::Guide,
            DocType::Plan,
            DocType::Implemented,
            DocType::BugReport,
            DocType::Analysis,
        ] {
            let docs = self.list_documents(doc_type).unwrap_or_default();
            let count = docs.len();
            dashboard.total_documents += count;

            // Count frontmatter
            let dir = self.agents_dir.join(doc_type.directory());
            let mut fm_count = 0;
            let mut old_count = 0;

            for doc in &docs {
                let path = dir.join(&doc.filename);
                if let Ok(content) = fs::read_to_string(&path) {
                    if content.trim_start().starts_with("---") {
                        if let Some(_) = crate::parser::parse_frontmatter(&content) {
                            fm_count += 1;
                            docs_with_frontmatter += 1;
                        }
                    }
                }
                
                let today = chrono::Local::now().format("%Y%m%d").to_string();
                if calculate_age_days(&doc.date, &today) > 30 {
                    old_count += 1;
                    dashboard.old_documents += 1;
                }
            }

            if detailed {
                dashboard.categories.push(CategoryHealth {
                    name: doc_type.directory().to_string(),
                    total: count,
                    with_frontmatter: fm_count,
                    old: old_count,
                });
            }
        }

        // Calculate metrics from validation
        for issue in &validation.issues {
            match issue.issue.as_str() {
                s if s.contains("not listed in INDEX") => dashboard.index_sync_issues += 1,
                s if s.contains("Invalid filename") => dashboard.naming_issues += 1,
                _ => {}
            }
        }

        dashboard.frontmatter_coverage = if dashboard.total_documents > 0 {
            (docs_with_frontmatter as f64 / dashboard.total_documents as f64) * 100.0
        } else {
            100.0
        };

        Ok(dashboard)
    }
}

/// Calculate age in days between two YYYYMMDD dates
fn calculate_age_days(date: &str, today: &str) -> u32 {
    use chrono::NaiveDate;
    let parse_date = |s: &str| NaiveDate::parse_from_str(s, "%Y%m%d").ok();
    
    if let (Some(d), Some(t)) = (parse_date(date), parse_date(today)) {
        (t - d).num_days().max(0) as u32
    } else {
        0
    }
}

/// Extract summary from document content (first non-header paragraph)
fn extract_summary(content: &str) -> Option<String> {
    let body = extract_body(content);
    for line in body.lines() {
        let trimmed = line.trim();
        if !trimmed.is_empty() 
            && !trimmed.starts_with('#') 
            && !trimmed.starts_with('-')
            && !trimmed.starts_with('|')
            && !trimmed.starts_with("**")
        {
            // Truncate to reasonable length
            let summary = if trimmed.len() > 150 {
                format!("{}...", &trimmed[..147])
            } else {
                trimmed.to_string()
            };
            return Some(summary);
        }
    }
    None
}

/// Infer tags from filename and content
fn infer_tags(filename: &str, content: &str, doc_type: DocType) -> Vec<String> {
    let mut tags = Vec::new();
    
    // Add doc type as tag
    tags.push(doc_type.directory().trim_end_matches('s').to_string());
    
    // Check for crate mentions
    let crates = ["context-trace", "context-search", "context-insert", "context-read"];
    for crate_name in crates {
        if filename.to_lowercase().contains(&crate_name.replace('-', "_"))
            || content.to_lowercase().contains(crate_name)
        {
            tags.push(crate_name.to_string());
        }
    }
    
    // Check for common concepts
    let concepts = [
        ("algorithm", "algorithm"),
        ("bug", "debugging"),
        ("test", "testing"),
        ("refactor", "refactoring"),
        ("api", "api"),
        ("performance", "performance"),
    ];
    
    let lower_name = filename.to_lowercase();
    let lower_content = content.to_lowercase();
    for (pattern, tag) in concepts {
        if lower_name.contains(pattern) || lower_content.contains(pattern) {
            if !tags.contains(&tag.to_string()) {
                tags.push(tag.to_string());
            }
        }
    }
    
    tags
}

fn generate_frontmatter(meta: &DocMetadata) -> String {
    let tags_str = meta
        .tags
        .iter()
        .map(|t| format!("`#{}`", t))
        .collect::<Vec<_>>()
        .join(" ");

    let mut lines = vec![
        "---".to_string(),
        format!("tags: {}", tags_str),
        format!("summary: {}", meta.summary),
    ];

    if let Some(status) = &meta.status {
        lines.push(format!("status: {}", status.emoji()));
    }

    lines.push("---".to_string());
    lines.join("\n")
}

fn extract_body(content: &str) -> String {
    let lines: Vec<&str> = content.lines().collect();

    if lines.first().map_or(true, |l| l.trim() != "---") {
        return content.to_string();
    }

    // Find end of frontmatter
    if let Some(end_idx) = lines.iter().skip(1).position(|l| l.trim() == "---")
    {
        lines[end_idx + 2..].join("\n")
    } else {
        content.to_string()
    }
}

/// Extract just the headers/outline from document content
fn extract_outline(content: &str) -> Option<String> {
    let body = extract_body(content);
    let headers: Vec<&str> =
        body.lines().filter(|line| line.starts_with('#')).collect();

    if headers.is_empty() {
        None
    } else {
        Some(headers.join("\n"))
    }
}

// === Parameter and Result Types ===

#[derive(Debug, Deserialize)]
pub struct CreateDocParams {
    pub doc_type: DocType,
    pub name: String,
    pub title: String,
    pub summary: String,
    pub tags: Option<Vec<String>>,
    pub status: Option<PlanStatus>,
}

#[derive(Debug, Serialize)]
pub struct CreateDocResult {
    pub path: String,
    pub filename: String,
}

#[derive(Debug, Serialize)]
pub struct DocSummary {
    pub filename: String,
    pub title: String,
    pub date: String,
    pub summary: String,
    pub tags: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<PlanStatus>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMetaParams {
    pub filename: String,
    pub tags: Option<Vec<String>>,
    pub summary: Option<String>,
    pub status: Option<PlanStatus>,
}

#[derive(Debug, Default, Serialize)]
pub struct ValidationReport {
    pub documents_checked: usize,
    pub issues: Vec<ValidationIssue>,
}

impl ValidationReport {
    pub fn to_markdown(&self) -> String {
        let mut md = String::new();

        md.push_str("# Documentation Validation Report\n\n");
        md.push_str(&format!(
            "**Documents Checked:** {}\n",
            self.documents_checked
        ));

        let errors: Vec<_> = self
            .issues
            .iter()
            .filter(|i| matches!(i.severity, IssueSeverity::Error))
            .collect();
        let warnings: Vec<_> = self
            .issues
            .iter()
            .filter(|i| matches!(i.severity, IssueSeverity::Warning))
            .collect();

        md.push_str(&format!("**Errors:** {}\n", errors.len()));
        md.push_str(&format!("**Warnings:** {}\n\n", warnings.len()));

        if self.issues.is_empty() {
            md.push_str("✅ **All documents pass validation!**\n");
        } else {
            if !errors.is_empty() {
                md.push_str("## ❌ Errors\n\n");
                md.push_str("| Category | File | Issue |\n");
                md.push_str("|----------|------|-------|\n");
                for issue in &errors {
                    md.push_str(&format!(
                        "| {} | {} | {} |\n",
                        issue.category, issue.file, issue.issue
                    ));
                }
                md.push_str("\n");
            }

            if !warnings.is_empty() {
                md.push_str("## ⚠️ Warnings\n\n");
                md.push_str("| Category | File | Issue |\n");
                md.push_str("|----------|------|-------|\n");
                for issue in &warnings {
                    md.push_str(&format!(
                        "| {} | {} | {} |\n",
                        issue.category, issue.file, issue.issue
                    ));
                }
            }
        }

        md
    }
}

#[derive(Debug, Serialize)]
pub struct ValidationIssue {
    pub file: String,
    pub category: String,
    pub issue: String,
    pub severity: IssueSeverity,
}

#[derive(Debug, Serialize)]
pub enum IssueSeverity {
    Error,
    Warning,
}

/// Result of reading a document
#[derive(Debug, Serialize)]
pub struct ReadDocResult {
    pub filename: String,
    pub doc_type: String,
    pub title: String,
    pub date: String,
    pub summary: String,
    pub tags: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<PlanStatus>,
    /// The body content (only present for 'full' detail level, or outline for 'outline')
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
}

/// Result of browsing documentation structure
#[derive(Debug, Serialize)]
pub struct BrowseResult {
    pub total_documents: usize,
    pub categories: Vec<CategorySummary>,
}

/// Summary of a document category
#[derive(Debug, Serialize)]
pub struct CategorySummary {
    pub category: String,
    pub doc_count: usize,
    pub items: Vec<TocItem>,
}

/// Table of contents item (minimal info)
#[derive(Debug, Serialize)]
pub struct TocItem {
    pub filename: String,
    pub date: String,
    pub summary: String,
}

/// Document flagged for review
#[derive(Debug, Serialize)]
pub struct ReviewCandidate {
    pub filename: String,
    pub doc_type: String,
    pub date: String,
    pub age_days: u64,
    pub summary: String,
    pub reason: String,
}

/// Result of content search
#[derive(Debug, Serialize)]
pub struct ContentSearchResult {
    pub query: String,
    pub total_matches: usize,
    pub files_searched: usize,
    pub matches: Vec<FileMatch>,
}

/// Matches within a single file
#[derive(Debug, Serialize)]
pub struct FileMatch {
    pub filename: String,
    pub doc_type: String,
    pub match_count: usize,
    pub excerpts: Vec<MatchExcerpt>,
}

/// A single match with context
#[derive(Debug, Serialize)]
pub struct MatchExcerpt {
    /// Line number where the match occurs (1-indexed)
    pub line_number: usize,
    /// The matching line
    pub line: String,
    /// Lines before the match
    pub context_before: Vec<String>,
    /// Lines after the match
    pub context_after: Vec<String>,
}

/// Result of add_frontmatter operation
#[derive(Debug, Serialize)]
pub struct AddFrontmatterResult {
    pub processed: usize,
    pub updated: usize,
    pub skipped: usize,
    pub errors: Vec<String>,
    pub changes: Vec<FrontmatterChange>,
}

/// A single frontmatter change
#[derive(Debug, Serialize)]
pub struct FrontmatterChange {
    pub filename: String,
    pub doc_type: String,
    pub inferred_tags: Vec<String>,
    pub inferred_summary: String,
}

impl AddFrontmatterResult {
    pub fn to_markdown(&self) -> String {
        let mut md = String::new();
        md.push_str(&format!(
            "# Add Frontmatter Results\n\n**Processed:** {} | **Updated:** {} | **Skipped:** {} | **Errors:** {}\n\n",
            self.processed, self.updated, self.skipped, self.errors.len()
        ));

        if !self.changes.is_empty() {
            md.push_str("## Changes\n\n");
            md.push_str("| File | Type | Tags |\n");
            md.push_str("|------|------|------|\n");
            for change in &self.changes {
                let tags = change.inferred_tags.join(", ");
                md.push_str(&format!(
                    "| {} | {} | {} |\n",
                    change.filename, change.doc_type, tags
                ));
            }
            md.push('\n');
        }

        if !self.errors.is_empty() {
            md.push_str("## Errors\n\n");
            for err in &self.errors {
                md.push_str(&format!("- {}\n", err));
            }
        }

        md
    }
}

/// Health dashboard metrics
#[derive(Debug, Serialize)]
pub struct HealthDashboard {
    pub total_documents: usize,
    pub frontmatter_coverage: f64,
    pub index_sync_issues: usize,
    pub naming_issues: usize,
    pub old_documents: usize,
    pub categories: Vec<CategoryHealth>,
}

/// Health metrics for a single category
#[derive(Debug, Serialize)]
pub struct CategoryHealth {
    pub name: String,
    pub total: usize,
    pub with_frontmatter: usize,
    pub old: usize,
}

impl HealthDashboard {
    pub fn to_markdown(&self) -> String {
        let mut md = String::new();
        md.push_str("# Documentation Health Dashboard\n\n");
        
        // Overall metrics
        md.push_str("## Overview\n\n");
        md.push_str("| Metric | Value | Status |\n");
        md.push_str("|--------|-------|--------|\n");
        
        let fm_status = if self.frontmatter_coverage >= 90.0 { "✅" } 
            else if self.frontmatter_coverage >= 50.0 { "⚠️" } 
            else { "❌" };
        md.push_str(&format!(
            "| Frontmatter Coverage | {:.1}% | {} |\n",
            self.frontmatter_coverage, fm_status
        ));
        
        let idx_status = if self.index_sync_issues == 0 { "✅" } 
            else if self.index_sync_issues <= 5 { "⚠️" } 
            else { "❌" };
        md.push_str(&format!(
            "| INDEX Sync Issues | {} | {} |\n",
            self.index_sync_issues, idx_status
        ));
        
        let name_status = if self.naming_issues == 0 { "✅" } 
            else if self.naming_issues <= 3 { "⚠️" } 
            else { "❌" };
        md.push_str(&format!(
            "| Naming Issues | {} | {} |\n",
            self.naming_issues, name_status
        ));
        
        md.push_str(&format!("| Total Documents | {} | ℹ️ |\n", self.total_documents));
        md.push_str(&format!("| Old Documents (>30d) | {} | ℹ️ |\n", self.old_documents));
        md.push('\n');
        
        // Category breakdown
        if !self.categories.is_empty() {
            md.push_str("## By Category\n\n");
            md.push_str("| Category | Total | Frontmatter | Old |\n");
            md.push_str("|----------|-------|-------------|-----|\n");
            for cat in &self.categories {
                let fm_pct = if cat.total > 0 {
                    (cat.with_frontmatter as f64 / cat.total as f64) * 100.0
                } else {
                    100.0
                };
                md.push_str(&format!(
                    "| {} | {} | {} ({:.0}%) | {} |\n",
                    cat.name, cat.total, cat.with_frontmatter, fm_pct, cat.old
                ));
            }
            md.push('\n');
        }
        
        // Recommendations
        md.push_str("## Recommendations\n\n");
        if self.frontmatter_coverage < 100.0 {
            let missing = self.total_documents - (self.total_documents as f64 * self.frontmatter_coverage / 100.0) as usize;
            md.push_str(&format!(
                "- 🔧 Run `add_frontmatter` to add frontmatter to {} documents\n",
                missing
            ));
        }
        if self.index_sync_issues > 0 {
            md.push_str("- 🔧 Run `regenerate_index` for categories with INDEX sync issues\n");
        }
        if self.naming_issues > 0 {
            md.push_str("- 📝 Rename files with invalid naming conventions to YYYYMMDD_NAME.md format\n");
        }
        if self.old_documents > 10 {
            md.push_str("- 📋 Review old documents with `get_docs_needing_review` for potential updates\n");
        }
        
        md
    }
}

// === Markdown Formatting ===

impl BrowseResult {
    pub fn to_markdown(&self) -> String {
        let mut md = String::new();
        md.push_str(&format!(
            "# Documentation Overview\n\n**Total Documents:** {}\n\n",
            self.total_documents
        ));

        for cat in &self.categories {
            md.push_str(&format!(
                "## {} ({} docs)\n\n",
                capitalize(&cat.category),
                cat.doc_count
            ));

            if cat.items.is_empty() {
                md.push_str("*No documents*\n\n");
            } else {
                md.push_str("| Date | File | Summary |\n");
                md.push_str("|------|------|---------|\n");
                for item in &cat.items {
                    md.push_str(&format!(
                        "| {} | {} | {} |\n",
                        &item.date,
                        &item.filename,
                        truncate(&item.summary, 50)
                    ));
                }
                md.push('\n');
            }
        }

        md
    }
}

impl ReadDocResult {
    pub fn to_markdown(&self) -> String {
        let mut md = String::new();
        md.push_str(&format!("# {}\n\n", self.title));
        md.push_str(&format!(
            "**File:** `{}`  \n**Type:** {}  \n**Date:** {}  \n",
            self.filename,
            self.doc_type,
            self.date,
        ));

        if !self.tags.is_empty() {
            let tags: Vec<String> =
                self.tags.iter().map(|t| format!("`#{}`", t)).collect();
            md.push_str(&format!("**Tags:** {}  \n", tags.join(" ")));
        }

        if let Some(status) = &self.status {
            md.push_str(&format!("**Status:** {}  \n", status.emoji()));
        }

        md.push_str(&format!("\n**Summary:** {}\n", self.summary));

        if let Some(body) = &self.body {
            md.push_str("\n---\n\n");
            md.push_str(body);
        }

        md
    }
}

impl ContentSearchResult {
    pub fn to_markdown(&self) -> String {
        let mut md = String::new();
        md.push_str(&format!("# Search Results: \"{}\"\n\n", self.query));
        md.push_str(&format!(
            "**Matches:** {} in {} files searched\n\n",
            self.total_matches, self.files_searched
        ));

        for file_match in &self.matches {
            md.push_str(&format!(
                "## {} ({})\n\n",
                file_match.filename, file_match.doc_type
            ));
            md.push_str(&format!("*{} match(es)*\n\n", file_match.match_count));

            for excerpt in &file_match.excerpts {
                md.push_str(&format!("**Line {}:**\n", excerpt.line_number));
                md.push_str("```\n");

                for line in &excerpt.context_before {
                    md.push_str(&format!("  {}\n", line));
                }
                md.push_str(&format!("> {}\n", excerpt.line));
                for line in &excerpt.context_after {
                    md.push_str(&format!("  {}\n", line));
                }

                md.push_str("```\n\n");
            }
        }

        md
    }
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn truncate(
    s: &str,
    max_len: usize,
) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    }
}

// =============================================================================
// Crate Documentation Manager
// =============================================================================

/// Result of crate discovery with diagnostics
#[derive(Debug, Serialize)]
pub struct CrateDiscoveryResult {
    pub crates: Vec<CrateSummary>,
    pub diagnostics: Vec<String>,
    pub crates_dir: String,
    pub crates_dir_exists: bool,
}

/// Manager for crate API documentation in crates/*/agents/docs/
pub struct CrateDocsManager {
    crates_dir: PathBuf,
}

impl CrateDocsManager {
    pub fn new(crates_dir: PathBuf) -> Self {
        Self { crates_dir }
    }

    /// Get the crates directory path for diagnostics
    pub fn crates_dir(&self) -> &Path {
        &self.crates_dir
    }

    /// Discover all context-* crates with agents/docs directories
    /// Returns both successful crates and diagnostic information about failures
    pub fn discover_crates_with_diagnostics(&self) -> ToolResult<CrateDiscoveryResult> {
        let mut result = CrateDiscoveryResult {
            crates: Vec::new(),
            diagnostics: Vec::new(),
            crates_dir: self.crates_dir.display().to_string(),
            crates_dir_exists: self.crates_dir.exists(),
        };

        if !self.crates_dir.exists() {
            result.diagnostics.push(format!(
                "Crates directory does not exist: {}",
                self.crates_dir.display()
            ));
            return Ok(result);
        }

        for entry in fs::read_dir(&self.crates_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if !path.is_dir() {
                continue;
            }

            let name = path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or_default();

            // Only include context-* crates
            if !name.starts_with("context-") {
                continue;
            }

            let docs_path = path.join("agents").join("docs");
            let index_path = docs_path.join("index.yaml");

            if !docs_path.exists() {
                result.diagnostics.push(format!(
                    "{}: agents/docs directory not found",
                    name
                ));
                continue;
            }

            if !index_path.exists() {
                result.diagnostics.push(format!(
                    "{}: index.yaml not found at {}",
                    name, index_path.display()
                ));
                continue;
            }

            match parse_crate_index(&index_path) {
                Ok(meta) => {
                    let readme_path = docs_path.join("README.md");
                    result.crates.push(CrateSummary {
                        name: meta.name,
                        version: meta.version,
                        description: meta.description,
                        module_count: meta.modules.len(),
                        has_readme: readme_path.exists(),
                        docs_path: docs_path.to_string_lossy().to_string(),
                    });
                }
                Err(e) => {
                    result.diagnostics.push(format!(
                        "{}: YAML parse error - {}",
                        name, e
                    ));
                }
            }
        }

        result.crates.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(result)
    }

    /// Discover all context-* crates (simplified, no diagnostics)
    pub fn discover_crates(&self) -> ToolResult<Vec<CrateSummary>> {
        Ok(self.discover_crates_with_diagnostics()?.crates)
    }

    /// Browse a crate's module tree
    pub fn browse_crate(&self, crate_name: &str) -> ToolResult<ModuleTreeNode> {
        let crate_path = self.crates_dir.join(crate_name);
        let docs_path = crate_path.join("agents").join("docs");
        let index_path = docs_path.join("index.yaml");

        if !index_path.exists() {
            return Err(ToolError::NotFound(format!(
                "Crate docs not found: {}",
                crate_name
            )));
        }

        let meta = parse_crate_index(&index_path)?;
        let readme_path = docs_path.join("README.md");

        let mut children = Vec::new();
        for module_ref in &meta.modules {
            let module_path = docs_path.join(&module_ref.path);
            if let Ok(node) = self.build_module_tree(&module_path, &module_ref.name) {
                children.push(node);
            }
        }

        // Collect exported items as key_types
        let mut key_types = Vec::new();
        if let Some(exported) = &meta.exported_items {
            key_types.extend(exported.types.clone());
            key_types.extend(exported.traits.clone());
            key_types.extend(exported.macros.clone());
        }

        Ok(ModuleTreeNode {
            name: meta.name,
            path: String::new(),
            description: meta.description,
            children,
            files: Vec::new(),
            key_types,
            has_readme: readme_path.exists(),
        })
    }

    /// Build a module tree node recursively
    fn build_module_tree(&self, module_path: &Path, name: &str) -> ToolResult<ModuleTreeNode> {
        let index_path = module_path.join("index.yaml");
        
        if !index_path.exists() {
            return Err(ToolError::NotFound(format!(
                "Module docs not found: {}",
                module_path.display()
            )));
        }

        let meta = parse_module_index(&index_path)?;
        let readme_path = module_path.join("README.md");

        let mut children = Vec::new();
        for submodule in &meta.submodules {
            let sub_path = module_path.join(&submodule.path);
            if let Ok(node) = self.build_module_tree(&sub_path, &submodule.name) {
                children.push(node);
            }
        }

        Ok(ModuleTreeNode {
            name: name.to_string(),
            path: module_path.to_string_lossy().to_string(),
            description: meta.description,
            children,
            files: meta.files,
            key_types: meta.key_types,
            has_readme: readme_path.exists(),
        })
    }

    /// Read documentation for a crate or module
    pub fn read_crate_doc(
        &self,
        crate_name: &str,
        module_path: Option<&str>,
        include_readme: bool,
    ) -> ToolResult<CrateDocResult> {
        let crate_path = self.crates_dir.join(crate_name);
        let docs_path = crate_path.join("agents").join("docs");

        let target_path = match module_path {
            Some(rel_path) => docs_path.join(rel_path),
            None => docs_path.clone(),
        };

        let index_path = target_path.join("index.yaml");
        let readme_path = target_path.join("README.md");

        if !index_path.exists() {
            return Err(ToolError::NotFound(format!(
                "Documentation not found: {}/{}",
                crate_name,
                module_path.unwrap_or("")
            )));
        }

        let index_content = fs::read_to_string(&index_path)?;
        let readme_content = if include_readme && readme_path.exists() {
            Some(fs::read_to_string(&readme_path)?)
        } else {
            None
        };

        Ok(CrateDocResult {
            crate_name: crate_name.to_string(),
            module_path: module_path.map(|s| s.to_string()),
            index_yaml: index_content,
            readme: readme_content,
        })
    }

    /// Update documentation for a crate or module
    pub fn update_crate_doc(
        &self,
        crate_name: &str,
        module_path: Option<&str>,
        index_yaml: Option<&str>,
        readme: Option<&str>,
    ) -> ToolResult<()> {
        let crate_path = self.crates_dir.join(crate_name);
        let docs_path = crate_path.join("agents").join("docs");

        let target_path = match module_path {
            Some(rel_path) => docs_path.join(rel_path),
            None => docs_path.clone(),
        };

        if !target_path.exists() {
            return Err(ToolError::NotFound(format!(
                "Documentation path not found: {}/{}",
                crate_name,
                module_path.unwrap_or("")
            )));
        }

        // Validate YAML before writing
        if let Some(yaml) = index_yaml {
            // Try to parse the YAML to validate it
            if module_path.is_some() {
                serde_yaml::from_str::<ModuleMetadata>(yaml)
                    .map_err(|e| ToolError::InvalidInput(format!("Invalid module YAML: {}", e)))?;
            } else {
                serde_yaml::from_str::<CrateMetadata>(yaml)
                    .map_err(|e| ToolError::InvalidInput(format!("Invalid crate YAML: {}", e)))?;
            }
            fs::write(target_path.join("index.yaml"), yaml)?;
        }

        if let Some(md) = readme {
            fs::write(target_path.join("README.md"), md)?;
        }

        Ok(())
    }

    /// Create documentation for a new module
    pub fn create_module_doc(
        &self,
        crate_name: &str,
        module_path: &str,
        name: &str,
        description: &str,
    ) -> ToolResult<String> {
        let crate_path = self.crates_dir.join(crate_name);
        let docs_path = crate_path.join("agents").join("docs").join(module_path);

        if docs_path.exists() {
            return Err(ToolError::AlreadyExists(format!(
                "Module docs already exist: {}/{}",
                crate_name,
                module_path
            )));
        }

        fs::create_dir_all(&docs_path)?;

        let meta = ModuleMetadata {
            name: name.to_string(),
            description: description.to_string(),
            submodules: Vec::new(),
            files: Vec::new(),
            key_types: Vec::new(),
        };

        let yaml = serde_yaml::to_string(&meta)
            .map_err(|e| ToolError::InvalidInput(format!("YAML serialization error: {}", e)))?;

        fs::write(docs_path.join("index.yaml"), yaml)?;

        Ok(docs_path.to_string_lossy().to_string())
    }

    /// Search crate documentation
    pub fn search_crate_docs(
        &self,
        query: &str,
        crate_filter: Option<&str>,
        search_types: bool,
        search_content: bool,
    ) -> ToolResult<Vec<CrateSearchResult>> {
        let mut results = Vec::new();
        let query_lower = query.to_lowercase();

        let crates = self.discover_crates()?;
        
        for crate_summary in crates {
            if let Some(filter) = crate_filter {
                if crate_summary.name != filter {
                    continue;
                }
            }

            let crate_path = self.crates_dir.join(&crate_summary.name);
            let docs_path = crate_path.join("agents").join("docs");

            // Search crate-level
            if let Ok(meta) = parse_crate_index(&docs_path.join("index.yaml")) {
                // Search description
                if meta.description.to_lowercase().contains(&query_lower) {
                    results.push(CrateSearchResult {
                        crate_name: crate_summary.name.clone(),
                        module_path: String::new(),
                        match_type: "crate".to_string(),
                        name: meta.name.clone(),
                        description: Some(meta.description.clone()),
                        context: None,
                    });
                }

                // Search exported items
                if search_types {
                    if let Some(exported) = &meta.exported_items {
                        results.extend(self.search_type_entries(
                            &exported.types,
                            &query_lower,
                            &crate_summary.name,
                            "",
                            "type",
                        ));
                        results.extend(self.search_type_entries(
                            &exported.traits,
                            &query_lower,
                            &crate_summary.name,
                            "",
                            "trait",
                        ));
                        results.extend(self.search_type_entries(
                            &exported.macros,
                            &query_lower,
                            &crate_summary.name,
                            "",
                            "macro",
                        ));
                    }
                }

                // Search modules recursively
                for module_ref in &meta.modules {
                    if module_ref.name.to_lowercase().contains(&query_lower)
                        || module_ref.description.to_lowercase().contains(&query_lower)
                    {
                        results.push(CrateSearchResult {
                            crate_name: crate_summary.name.clone(),
                            module_path: module_ref.path.clone(),
                            match_type: "module".to_string(),
                            name: module_ref.name.clone(),
                            description: Some(module_ref.description.clone()),
                            context: None,
                        });
                    }

                    // Search within module
                    let module_path = docs_path.join(&module_ref.path);
                    results.extend(self.search_module(
                        &module_path,
                        &query_lower,
                        &crate_summary.name,
                        &module_ref.path,
                        search_types,
                        search_content,
                    ));
                }
            }

            // Search README content
            if search_content {
                let readme_path = docs_path.join("README.md");
                if let Ok(content) = read_markdown_file(&readme_path) {
                    if let Some(context) = self.find_context_in_content(&content, &query_lower) {
                        results.push(CrateSearchResult {
                            crate_name: crate_summary.name.clone(),
                            module_path: String::new(),
                            match_type: "content".to_string(),
                            name: "README.md".to_string(),
                            description: None,
                            context: Some(context),
                        });
                    }
                }
            }
        }

        Ok(results)
    }

    fn search_type_entries(
        &self,
        entries: &[TypeEntry],
        query: &str,
        crate_name: &str,
        module_path: &str,
        match_type: &str,
    ) -> Vec<CrateSearchResult> {
        entries
            .iter()
            .filter(|entry| entry.name().to_lowercase().contains(query))
            .map(|entry| CrateSearchResult {
                crate_name: crate_name.to_string(),
                module_path: module_path.to_string(),
                match_type: match_type.to_string(),
                name: entry.name().to_string(),
                description: entry.description().map(|s| s.to_string()),
                context: None,
            })
            .collect()
    }

    fn search_module(
        &self,
        module_path: &Path,
        query: &str,
        crate_name: &str,
        rel_path: &str,
        search_types: bool,
        search_content: bool,
    ) -> Vec<CrateSearchResult> {
        let mut results = Vec::new();
        let index_path = module_path.join("index.yaml");

        if let Ok(meta) = parse_module_index(&index_path) {
            // Search key_types
            if search_types {
                results.extend(self.search_type_entries(
                    &meta.key_types,
                    query,
                    crate_name,
                    rel_path,
                    "type",
                ));
            }

            // Search files
            for file in &meta.files {
                if file.name.to_lowercase().contains(query)
                    || file.description.to_lowercase().contains(query)
                {
                    results.push(CrateSearchResult {
                        crate_name: crate_name.to_string(),
                        module_path: rel_path.to_string(),
                        match_type: "file".to_string(),
                        name: file.name.clone(),
                        description: Some(file.description.clone()),
                        context: None,
                    });
                }
            }

            // Search submodules recursively
            for submodule in &meta.submodules {
                if submodule.name.to_lowercase().contains(query)
                    || submodule.description.to_lowercase().contains(query)
                {
                    let sub_rel_path = format!("{}/{}", rel_path, submodule.path);
                    results.push(CrateSearchResult {
                        crate_name: crate_name.to_string(),
                        module_path: sub_rel_path.clone(),
                        match_type: "module".to_string(),
                        name: submodule.name.clone(),
                        description: Some(submodule.description.clone()),
                        context: None,
                    });
                }

                let sub_path = module_path.join(&submodule.path);
                let sub_rel_path = format!("{}/{}", rel_path, submodule.path);
                results.extend(self.search_module(
                    &sub_path,
                    query,
                    crate_name,
                    &sub_rel_path,
                    search_types,
                    search_content,
                ));
            }

            // Search README content
            if search_content {
                let readme_path = module_path.join("README.md");
                if let Ok(content) = read_markdown_file(&readme_path) {
                    if let Some(context) = self.find_context_in_content(&content, query) {
                        results.push(CrateSearchResult {
                            crate_name: crate_name.to_string(),
                            module_path: rel_path.to_string(),
                            match_type: "content".to_string(),
                            name: "README.md".to_string(),
                            description: None,
                            context: Some(context),
                        });
                    }
                }
            }
        }

        results
    }

    fn find_context_in_content(&self, content: &str, query: &str) -> Option<String> {
        for line in content.lines() {
            if line.to_lowercase().contains(query) {
                return Some(truncate(line.trim(), 100));
            }
        }
        None
    }

    /// Validate crate documentation for consistency
    pub fn validate_crate_docs(&self, crate_filter: Option<&str>) -> ToolResult<CrateValidationReport> {
        let mut report = CrateValidationReport::default();
        let crates = self.discover_crates()?;

        for crate_summary in crates {
            if let Some(filter) = crate_filter {
                if crate_summary.name != filter {
                    continue;
                }
            }

            report.crates_checked += 1;

            let crate_path = self.crates_dir.join(&crate_summary.name);
            let docs_path = crate_path.join("agents").join("docs");
            let index_path = docs_path.join("index.yaml");

            // Check crate index
            match parse_crate_index(&index_path) {
                Ok(meta) => {
                    // Check all referenced modules exist
                    for module_ref in &meta.modules {
                        let module_path = docs_path.join(&module_ref.path);
                        if !module_path.exists() {
                            report.issues.push(CrateValidationIssue {
                                crate_name: crate_summary.name.clone(),
                                module_path: Some(module_ref.path.clone()),
                                issue: format!("Referenced module '{}' does not exist", module_ref.path),
                                severity: "error".to_string(),
                            });
                        } else {
                            // Recursively validate module
                            self.validate_module(
                                &module_path,
                                &crate_summary.name,
                                &module_ref.path,
                                &mut report,
                            );
                        }
                    }

                    // Warn about missing README
                    if !docs_path.join("README.md").exists() {
                        report.issues.push(CrateValidationIssue {
                            crate_name: crate_summary.name.clone(),
                            module_path: None,
                            issue: "Missing README.md".to_string(),
                            severity: "warning".to_string(),
                        });
                    }
                }
                Err(e) => {
                    report.issues.push(CrateValidationIssue {
                        crate_name: crate_summary.name.clone(),
                        module_path: None,
                        issue: format!("Failed to parse index.yaml: {}", e),
                        severity: "error".to_string(),
                    });
                }
            }
        }

        Ok(report)
    }

    fn validate_module(
        &self,
        module_path: &Path,
        crate_name: &str,
        rel_path: &str,
        report: &mut CrateValidationReport,
    ) {
        report.modules_checked += 1;

        let index_path = module_path.join("index.yaml");

        match parse_module_index(&index_path) {
            Ok(meta) => {
                // Check all referenced submodules exist
                for submodule in &meta.submodules {
                    let sub_path = module_path.join(&submodule.path);
                    if !sub_path.exists() {
                        report.issues.push(CrateValidationIssue {
                            crate_name: crate_name.to_string(),
                            module_path: Some(format!("{}/{}", rel_path, submodule.path)),
                            issue: format!("Referenced submodule '{}' does not exist", submodule.path),
                            severity: "error".to_string(),
                        });
                    } else {
                        let sub_rel_path = format!("{}/{}", rel_path, submodule.path);
                        self.validate_module(&sub_path, crate_name, &sub_rel_path, report);
                    }
                }

                // Warn about missing description
                if meta.description.is_empty() {
                    report.issues.push(CrateValidationIssue {
                        crate_name: crate_name.to_string(),
                        module_path: Some(rel_path.to_string()),
                        issue: "Empty description".to_string(),
                        severity: "warning".to_string(),
                    });
                }
            }
            Err(e) => {
                report.issues.push(CrateValidationIssue {
                    crate_name: crate_name.to_string(),
                    module_path: Some(rel_path.to_string()),
                    issue: format!("Failed to parse index.yaml: {}", e),
                    severity: "error".to_string(),
                });
            }
        }
    }
}

/// Result of reading crate documentation
#[derive(Debug, Serialize)]
pub struct CrateDocResult {
    pub crate_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_path: Option<String>,
    pub index_yaml: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readme: Option<String>,
}

impl CrateDocResult {
    pub fn to_markdown(&self) -> String {
        let mut md = String::new();
        let location = match &self.module_path {
            Some(path) => format!("{}::{}", self.crate_name, path.replace('/', "::")),
            None => self.crate_name.clone(),
        };
        md.push_str(&format!("# Documentation: {}\n\n", location));
        md.push_str("## index.yaml\n\n```yaml\n");
        md.push_str(&self.index_yaml);
        md.push_str("```\n\n");
        if let Some(readme) = &self.readme {
            md.push_str("## README.md\n\n");
            md.push_str(readme);
        }
        md
    }
}
