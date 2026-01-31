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
                continue;
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

                // Check naming convention
                if !filename.chars().take(8).all(|c| c.is_ascii_digit()) {
                    report.issues.push(ValidationIssue {
                        file: filename.to_string(),
                        issue: "Missing YYYYMMDD_ prefix".to_string(),
                        severity: IssueSeverity::Error,
                    });
                }

                // Check frontmatter
                let content = fs::read_to_string(&path)?;
                if !content.starts_with("---") {
                    report.issues.push(ValidationIssue {
                        file: filename.to_string(),
                        issue: "Missing frontmatter".to_string(),
                        severity: IssueSeverity::Warning,
                    });
                }

                report.documents_checked += 1;
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

#[derive(Debug, Serialize)]
pub struct ValidationIssue {
    pub file: String,
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
