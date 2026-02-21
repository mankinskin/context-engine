//! Agent documentation management (guides, plans, bug reports, etc.)
//!
//! This module handles the `agents/` directory documentation including:
//! - guides/ - How-to guides and troubleshooting
//! - plans/ - Task plans before execution
//! - implemented/ - Completed feature documentation
//! - bug-reports/ - Known issues and analyses
//! - analysis/ - Algorithm analysis and comparisons

use crate::{
    parser::{
        extract_metadata,
        parse_frontmatter,
        parse_filename,
        parse_title,
    },
    schema::{
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
use super::{ToolResult, ToolError, DetailLevel, ListFilter, compile_search_regex, regex_matches};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::PathBuf,
};

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

    /// Delete a document and update the index.
    pub fn delete_document(&self, filename: &str) -> ToolResult<String> {
        let path = self.find_document(filename)?;
        
        // Get doc type to update index
        let content = fs::read_to_string(&path)?;
        let meta = extract_metadata(&path, &content)
            .ok_or_else(|| ToolError::InvalidInput("Cannot parse document".into()))?;
        
        // Delete the file
        fs::remove_file(&path)?;
        
        // Update index
        self.update_index(meta.doc_type)?;
        
        Ok(path.display().to_string())
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

    /// Search document content using regex pattern (case-insensitive).
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

        let regex = compile_search_regex(query)?;
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
                    // Check if regex matches the line
                    if regex_matches(line, &regex) {
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

    /// Enhanced search: search by regex query and/or tag, optionally searching content.
    /// Query is a case-insensitive regex pattern.
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

        let regex = query.map(compile_search_regex).transpose()?.flatten();
        let tag_lower = tag.map(|t| t.to_lowercase().trim_start_matches('#').to_string());

        let mut results = Vec::new();

        for dt in doc_types {
            let docs = self.list_documents(dt)?;

            for doc in docs {
                // Check tag if provided
                let tag_ok = tag_lower.as_ref().map_or(true, |tag_l| {
                    doc.tags.iter().any(|t| t.to_lowercase() == *tag_l)
                });
                
                // Check regex if provided
                let query_ok = if regex.is_none() {
                    true
                } else {
                    // Combine title/summary/tags for searching
                    let searchable = format!(
                        "{} {} {}",
                        doc.title,
                        doc.summary,
                        doc.tags.join(" ")
                    );
                    
                    if regex_matches(&searchable, &regex) {
                        true
                    } else if search_content {
                        let path = self.agents_dir.join(dt.directory()).join(&doc.filename);
                        if let Ok(content) = fs::read_to_string(&path) {
                            regex_matches(&content, &regex)
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                };

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
                        if let Some(_) = parse_frontmatter(&content) {
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

// =============================================================================
// Helper Functions
// =============================================================================

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

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    }
}

// =============================================================================
// Parameter and Result Types
// =============================================================================

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

// =============================================================================
// Markdown Formatting
// =============================================================================

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
