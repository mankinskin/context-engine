//! MCP tool definitions for documentation management.

use crate::{
    parser::{
        extract_metadata,
        parse_index,
    },
    schema::{
        Confidence,
        DocMetadata,
        DocType,
        IndexEntry,
        PlanStatus,
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
    collections::HashMap,
    fs,
    path::{
        Path,
        PathBuf,
    },
};
use walkdir::WalkDir;

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
    pub confidence: Option<Confidence>,
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
            confidence: params.confidence.unwrap_or(Confidence::Medium),
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
                        confidence: meta.confidence,
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
                confidence: d.confidence,
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

        if let Some(conf) = params.confidence {
            meta.confidence = conf;
        }
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
                        // Check confidence is set
                        if fm.confidence.is_none() {
                            report.issues.push(ValidationIssue {
                                file: filename.to_string(),
                                category: category.clone(),
                                issue:
                                    "Missing confidence field in frontmatter"
                                        .to_string(),
                                severity: IssueSeverity::Warning,
                            });
                        }

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
                    // Should have a table with columns: Date | File | Confidence | Summary
                    let has_doc_table = index_content
                        .contains("| Date | File | Confidence | Summary |")
                        || index_content
                            .contains("|------|------|------------|");

                    if !has_doc_table {
                        report.issues.push(ValidationIssue {
                            file: "INDEX.md".to_string(),
                            category: category.clone(),
                            issue: "INDEX.md should use minimal table format: | Date | File | Confidence | Summary |".to_string(),
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
            confidence: meta.confidence,
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
                // Filter by confidence
                if let Some(conf) = &filter.confidence {
                    if doc.confidence != *conf {
                        return false;
                    }
                }
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
                    confidence: d.confidence,
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

    /// Get documents that may need review (old or low confidence).
    pub fn get_docs_needing_review(
        &self,
        max_age_days: u32,
        include_low_confidence: bool,
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

                let mut reasons = Vec::new();

                // Check age
                if age_days > max_age_days as u64 {
                    reasons.push(format!("Old ({} days)", age_days));
                }

                // Check confidence
                if include_low_confidence
                    && matches!(doc.confidence, Confidence::Low)
                {
                    reasons.push("Low confidence".to_string());
                }

                if !reasons.is_empty() {
                    candidates.push(ReviewCandidate {
                        filename: doc.filename,
                        doc_type: doc_type.directory().to_string(),
                        date: doc.date,
                        age_days,
                        confidence: doc.confidence,
                        summary: doc.summary,
                        reason: reasons.join(", "),
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
        format!("confidence: {}", meta.confidence.emoji()),
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
    pub confidence: Option<Confidence>,
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
    pub confidence: Confidence,
    pub summary: String,
    pub tags: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<PlanStatus>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMetaParams {
    pub filename: String,
    pub confidence: Option<Confidence>,
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
    pub confidence: Confidence,
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
    pub confidence: Confidence,
    pub summary: String,
}

/// Document flagged for review
#[derive(Debug, Serialize)]
pub struct ReviewCandidate {
    pub filename: String,
    pub doc_type: String,
    pub date: String,
    pub age_days: u64,
    pub confidence: Confidence,
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
                md.push_str("| Date | File | Confidence | Summary |\n");
                md.push_str("|------|------|------------|--------|\n");
                for item in &cat.items {
                    md.push_str(&format!(
                        "| {} | {} | {} | {} |\n",
                        &item.date,
                        &item.filename,
                        item.confidence.emoji(),
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
            "**File:** `{}`  \n**Type:** {}  \n**Date:** {}  \n**Confidence:** {}  \n",
            self.filename,
            self.doc_type,
            self.date,
            self.confidence.emoji()
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
