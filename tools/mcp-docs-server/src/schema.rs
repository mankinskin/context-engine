//! Document schema definitions for structured agent documentation.

use serde::{
    Deserialize,
    Serialize,
};

/// Confidence rating for documentation entries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Confidence {
    /// 🟢 High - Verified, current, complete
    High,
    /// 🟡 Medium - Mostly accurate, may have gaps
    Medium,
    /// 🔴 Low - Outdated or incomplete
    Low,
}

impl Confidence {
    pub fn emoji(&self) -> &'static str {
        match self {
            Confidence::High => "🟢",
            Confidence::Medium => "🟡",
            Confidence::Low => "🔴",
        }
    }

    pub fn from_emoji(s: &str) -> Option<Self> {
        match s.trim() {
            "🟢" | "high" | "High" => Some(Confidence::High),
            "🟡" | "medium" | "Medium" => Some(Confidence::Medium),
            "🔴" | "low" | "Low" => Some(Confidence::Low),
            _ => None,
        }
    }
}

/// Document category/type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DocType {
    Guide,
    Plan,
    Implemented,
    BugReport,
    Analysis,
}

impl DocType {
    pub fn directory(&self) -> &'static str {
        match self {
            DocType::Guide => "guides",
            DocType::Plan => "plans",
            DocType::Implemented => "implemented",
            DocType::BugReport => "bug-reports",
            DocType::Analysis => "analysis",
        }
    }

    pub fn from_directory(dir: &str) -> Option<Self> {
        match dir {
            "guides" => Some(DocType::Guide),
            "plans" => Some(DocType::Plan),
            "implemented" => Some(DocType::Implemented),
            "bug-reports" => Some(DocType::BugReport),
            "analysis" => Some(DocType::Analysis),
            _ => None,
        }
    }

    pub fn file_prefix(&self) -> &'static str {
        match self {
            DocType::Guide => "",
            DocType::Plan => "PLAN_",
            DocType::Implemented => "",
            DocType::BugReport => "BUG_",
            DocType::Analysis => "",
        }
    }
}

/// Status for plans (only applicable to DocType::Plan).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PlanStatus {
    /// 📋 Design/planning phase
    Design,
    /// 🚧 In progress
    InProgress,
    /// ✅ Completed (should move to implemented/)
    Completed,
    /// ⚠️ Blocked
    Blocked,
    /// ❌ Superseded/abandoned
    Superseded,
}

impl PlanStatus {
    pub fn emoji(&self) -> &'static str {
        match self {
            PlanStatus::Design => "📋",
            PlanStatus::InProgress => "🚧",
            PlanStatus::Completed => "✅",
            PlanStatus::Blocked => "⚠️",
            PlanStatus::Superseded => "❌",
        }
    }
}

/// Metadata extracted from or written to a document.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocMetadata {
    pub doc_type: DocType,
    pub date: String,     // YYYYMMDD format
    pub title: String,    // Human-readable title
    pub filename: String, // Full filename with date prefix
    pub confidence: Confidence,
    pub tags: Vec<String>,
    pub summary: String, // One-line summary for INDEX
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<PlanStatus>, // Only for plans
}

/// INDEX entry (simplified table row).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexEntry {
    pub date: String,
    pub filename: String,
    pub confidence: Confidence,
    pub summary: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<PlanStatus>,
}

impl From<&DocMetadata> for IndexEntry {
    fn from(meta: &DocMetadata) -> Self {
        IndexEntry {
            date: meta.date.clone(),
            filename: meta.filename.clone(),
            confidence: meta.confidence,
            summary: meta.summary.clone(),
            status: meta.status,
        }
    }
}

// =============================================================================
// Crate Documentation Schema
// =============================================================================

/// Reference to a module in a crate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleRef {
    pub name: String,
    pub description: String,
    pub path: String,
}

/// Reference to a submodule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmoduleRef {
    pub name: String,
    pub path: String,
    pub description: String,
}

/// A file entry in a module
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEntry {
    pub name: String,
    pub description: String,
}

/// A type entry (for key_types)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TypeEntry {
    /// Simple type name
    Simple(String),
    /// Type with description (name: description format in YAML)
    WithDescription { name: String, description: String },
}

impl TypeEntry {
    pub fn name(&self) -> &str {
        match self {
            TypeEntry::Simple(n) => n,
            TypeEntry::WithDescription { name, .. } => name,
        }
    }
    
    pub fn description(&self) -> Option<&str> {
        match self {
            TypeEntry::Simple(_) => None,
            TypeEntry::WithDescription { description, .. } => Some(description),
        }
    }
}

/// Exported items from a crate
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExportedItems {
    #[serde(default)]
    pub types: Vec<TypeEntry>,
    #[serde(default)]
    pub traits: Vec<TypeEntry>,
    #[serde(default)]
    pub macros: Vec<TypeEntry>,
}

/// Crate-level metadata (from index.yaml at crate root)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrateMetadata {
    pub name: String,
    #[serde(default)]
    pub version: Option<String>,
    pub description: String,
    #[serde(default)]
    pub modules: Vec<ModuleRef>,
    #[serde(default)]
    pub exported_items: Option<ExportedItems>,
    #[serde(default)]
    pub dependencies: Vec<String>,
    #[serde(default)]
    pub features: Vec<String>,
}

/// Module-level metadata (from index.yaml in module directories)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleMetadata {
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub submodules: Vec<SubmoduleRef>,
    #[serde(default)]
    pub files: Vec<FileEntry>,
    #[serde(default)]
    pub key_types: Vec<TypeEntry>,
}

/// Summary info for a crate (used in list_crates output)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrateSummary {
    pub name: String,
    pub version: Option<String>,
    pub description: String,
    pub module_count: usize,
    pub has_readme: bool,
    pub docs_path: String,
}

/// A node in the module tree for browsing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleTreeNode {
    pub name: String,
    pub path: String,
    pub description: String,
    #[serde(default)]
    pub children: Vec<ModuleTreeNode>,
    #[serde(default)]
    pub files: Vec<FileEntry>,
    #[serde(default)]
    pub key_types: Vec<TypeEntry>,
    pub has_readme: bool,
}

/// Search result for crate documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrateSearchResult {
    pub crate_name: String,
    pub module_path: String,
    pub match_type: String, // "type", "trait", "macro", "module", "file", "content"
    pub name: String,
    pub description: Option<String>,
    pub context: Option<String>,
}

/// Validation issue for crate documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrateValidationIssue {
    pub crate_name: String,
    pub module_path: Option<String>,
    pub issue: String,
    pub severity: String, // "error", "warning"
}

/// Validation report for crate documentation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CrateValidationReport {
    pub crates_checked: usize,
    pub modules_checked: usize,
    pub issues: Vec<CrateValidationIssue>,
}

impl CrateValidationReport {
    pub fn to_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!(
            "# Crate Documentation Validation Report\n\n\
             **Crates checked:** {}\n\
             **Modules checked:** {}\n\
             **Issues found:** {}\n\n",
            self.crates_checked,
            self.modules_checked,
            self.issues.len()
        ));

        if self.issues.is_empty() {
            out.push_str("✅ No issues found!\n");
        } else {
            out.push_str("## Issues\n\n");
            out.push_str("| Severity | Crate | Module | Issue |\n");
            out.push_str("|----------|-------|--------|-------|\n");
            for issue in &self.issues {
                let module = issue.module_path.as_deref().unwrap_or("-");
                let severity_icon = if issue.severity == "error" { "❌" } else { "⚠️" };
                out.push_str(&format!(
                    "| {} | {} | {} | {} |\n",
                    severity_icon, issue.crate_name, module, issue.issue
                ));
            }
        }

        out
    }
}

