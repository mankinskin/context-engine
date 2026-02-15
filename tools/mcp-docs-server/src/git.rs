//! Git integration for stale detection.
//!
//! Uses git commands to determine file modification times and detect
//! when documentation is out of sync with source files.

use std::path::Path;
use std::process::Command;

use chrono::{DateTime, Utc};

use crate::schema::FileModificationInfo;

/// Error type for git operations
#[derive(Debug)]
pub enum GitError {
    /// Git command failed or not available
    CommandFailed(String),
    /// Failed to parse git output
    ParseError(String),
    /// Not a git repository
    NotARepository,
}

impl std::fmt::Display for GitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GitError::CommandFailed(msg) => write!(f, "Git command failed: {}", msg),
            GitError::ParseError(msg) => write!(f, "Failed to parse git output: {}", msg),
            GitError::NotARepository => write!(f, "Not a git repository"),
        }
    }
}

impl std::error::Error for GitError {}

/// Result type for git operations
pub type GitResult<T> = Result<T, GitError>;

/// Check if a directory is a git repository
pub fn is_git_repository(path: &Path) -> bool {
    Command::new("git")
        .args(["rev-parse", "--git-dir"])
        .current_dir(path)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Get the root of the git repository containing the given path
pub fn get_repo_root(path: &Path) -> GitResult<String> {
    let output = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .current_dir(path)
        .output()
        .map_err(|e| GitError::CommandFailed(e.to_string()))?;

    if !output.status.success() {
        return Err(GitError::NotARepository);
    }

    String::from_utf8(output.stdout)
        .map(|s| s.trim().to_string())
        .map_err(|e| GitError::ParseError(e.to_string()))
}

/// Information about a file's last modification from git
#[derive(Debug, Clone)]
pub struct GitFileInfo {
    /// ISO 8601 timestamp of last modification
    pub last_modified: String,
    /// Short commit hash
    pub commit_hash: String,
    /// Commit message (first line)
    pub commit_message: String,
}

/// Get git information for a file
///
/// Returns None if the file is not tracked by git or doesn't exist.
pub fn get_file_info(repo_path: &Path, file_path: &str) -> Option<GitFileInfo> {
    // Get the last commit that modified this file
    // Format: %H = full hash, %h = short hash, %aI = author date ISO 8601, %s = subject
    let output = Command::new("git")
        .args([
            "log",
            "-1",
            "--format=%h|%aI|%s",
            "--",
            file_path,
        ])
        .current_dir(repo_path)
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8(output.stdout).ok()?;
    let line = stdout.trim();
    
    if line.is_empty() {
        return None; // File not tracked or no commits
    }

    let parts: Vec<&str> = line.splitn(3, '|').collect();
    if parts.len() < 3 {
        return None;
    }

    Some(GitFileInfo {
        commit_hash: parts[0].to_string(),
        last_modified: parts[1].to_string(),
        commit_message: parts[2].to_string(),
    })
}

/// Get modification info for multiple files
pub fn get_files_info(repo_path: &Path, file_paths: &[String]) -> Vec<FileModificationInfo> {
    file_paths
        .iter()
        .map(|path| {
            let full_path = repo_path.join(path);
            let exists = full_path.exists();
            
            match get_file_info(repo_path, path) {
                Some(info) => FileModificationInfo {
                    path: path.clone(),
                    last_modified: Some(info.last_modified),
                    last_commit: Some(info.commit_hash),
                    commit_message: Some(info.commit_message),
                    exists,
                },
                None => FileModificationInfo {
                    path: path.clone(),
                    last_modified: None,
                    last_commit: None,
                    commit_message: None,
                    exists,
                },
            }
        })
        .collect()
}

/// Get the most recent modification time from a list of file infos
pub fn get_most_recent_modification(files: &[FileModificationInfo]) -> Option<String> {
    files
        .iter()
        .filter_map(|f| f.last_modified.as_ref())
        .max()
        .cloned()
}

/// Parse an ISO 8601 timestamp to DateTime<Utc>
pub fn parse_timestamp(timestamp: &str) -> Option<DateTime<Utc>> {
    DateTime::parse_from_rfc3339(timestamp)
        .ok()
        .map(|dt| dt.with_timezone(&Utc))
}

/// Calculate days between two ISO 8601 timestamps
pub fn days_between(earlier: &str, later: &str) -> Option<i64> {
    let earlier_dt = parse_timestamp(earlier)?;
    let later_dt = parse_timestamp(later)?;
    Some((later_dt - earlier_dt).num_days())
}

/// Calculate days from a timestamp until now
pub fn days_since(timestamp: &str) -> Option<i64> {
    let dt = parse_timestamp(timestamp)?;
    let now = Utc::now();
    Some((now - dt).num_days())
}

/// Get files modified since a given timestamp
pub fn get_files_modified_since(
    files: &[FileModificationInfo],
    since_timestamp: &str,
) -> Vec<String> {
    let since_dt = match parse_timestamp(since_timestamp) {
        Some(dt) => dt,
        None => return Vec::new(),
    };

    files
        .iter()
        .filter(|f| {
            f.last_modified
                .as_ref()
                .and_then(|ts| parse_timestamp(ts))
                .map(|dt| dt > since_dt)
                .unwrap_or(false)
        })
        .map(|f| f.path.clone())
        .collect()
}

/// Get current timestamp in ISO 8601 format
pub fn current_timestamp() -> String {
    Utc::now().to_rfc3339()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_timestamp() {
        let ts = "2026-02-15T10:30:00+00:00";
        let dt = parse_timestamp(ts);
        assert!(dt.is_some());
    }

    #[test]
    fn test_days_between() {
        let earlier = "2026-02-10T00:00:00+00:00";
        let later = "2026-02-15T00:00:00+00:00";
        let days = days_between(earlier, later);
        assert_eq!(days, Some(5));
    }
}
