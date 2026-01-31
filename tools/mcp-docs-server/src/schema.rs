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
