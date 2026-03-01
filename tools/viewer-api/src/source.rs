//! Source file utilities for resolving and serving source code.
//!
//! Provides common functions for source file resolution and language detection,
//! shared between viewer tools that need to display source code.

use std::path::PathBuf;

/// Detect programming language from file extension.
///
/// Returns a language identifier string suitable for syntax highlighting.
pub fn detect_language(path: &str) -> String {
    let ext = path.rsplit('.').next().unwrap_or("");
    match ext {
        "rs" => "rust",
        "ts" | "tsx" => "typescript",
        "js" | "jsx" => "javascript",
        "json" => "json",
        "toml" => "toml",
        "yaml" | "yml" => "yaml",
        "md" => "markdown",
        "html" => "html",
        "css" => "css",
        _ => "plaintext",
    }
    .to_string()
}

/// Sanitize and resolve a source path relative to a workspace root.
///
/// Normalizes path separators, strips leading slashes, and checks for
/// path traversal attacks.
pub fn resolve_source_path(
    workspace_root: &PathBuf,
    path: &str,
) -> Result<PathBuf, String> {
    // Normalize path separators
    let normalized = path.replace('\\', "/");

    // Remove leading slashes
    let clean_path = normalized.trim_start_matches('/');

    // Check for path traversal
    if clean_path.contains("..") {
        return Err("Path traversal not allowed".to_string());
    }

    let full_path = workspace_root.join(clean_path);

    // Verify the path is within workspace
    if !full_path.starts_with(workspace_root) {
        return Err("Path outside workspace".to_string());
    }

    Ok(full_path)
}

/// Extract a snippet of lines from source content.
///
/// Returns (snippet, start_line, end_line) where lines are 1-based.
/// `context` is the number of lines to include above and below `line`.
pub fn extract_snippet(
    content: &str,
    line: usize,
    context: usize,
) -> (String, usize, usize) {
    let lines: Vec<&str> = content.lines().collect();
    let total_lines = lines.len();

    let line = line.min(total_lines).max(1);
    let start_line = line.saturating_sub(context).max(1);
    let end_line = (line + context).min(total_lines);

    let snippet_lines: Vec<&str> = lines[(start_line - 1)..end_line].to_vec();
    let snippet_content = snippet_lines.join("\n");

    (snippet_content, start_line, end_line)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_detect_language() {
        assert_eq!(detect_language("main.rs"), "rust");
        assert_eq!(detect_language("index.ts"), "typescript");
        assert_eq!(detect_language("app.tsx"), "typescript");
        assert_eq!(detect_language("script.js"), "javascript");
        assert_eq!(detect_language("config.json"), "json");
        assert_eq!(detect_language("Cargo.toml"), "toml");
        assert_eq!(detect_language("config.yaml"), "yaml");
        assert_eq!(detect_language("config.yml"), "yaml");
        assert_eq!(detect_language("readme.md"), "markdown");
        assert_eq!(detect_language("page.html"), "html");
        assert_eq!(detect_language("style.css"), "css");
        assert_eq!(detect_language("Makefile"), "plaintext");
    }

    #[test]
    fn test_resolve_source_path_normal() {
        let root = PathBuf::from("/workspace");
        let result = resolve_source_path(&root, "src/main.rs");
        assert_eq!(result.unwrap(), Path::new("/workspace/src/main.rs"));
    }

    #[test]
    fn test_resolve_source_path_strips_leading_slash() {
        let root = PathBuf::from("/workspace");
        let result = resolve_source_path(&root, "/src/main.rs");
        assert_eq!(result.unwrap(), Path::new("/workspace/src/main.rs"));
    }

    #[test]
    fn test_resolve_source_path_normalizes_backslashes() {
        let root = PathBuf::from("/workspace");
        let result = resolve_source_path(&root, "src\\lib\\mod.rs");
        assert_eq!(result.unwrap(), Path::new("/workspace/src/lib/mod.rs"));
    }

    #[test]
    fn test_resolve_source_path_rejects_traversal() {
        let root = PathBuf::from("/workspace");
        let result = resolve_source_path(&root, "../etc/passwd");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Path traversal not allowed");
    }

    #[test]
    fn test_extract_snippet_middle() {
        let content = "line1\nline2\nline3\nline4\nline5\nline6\nline7";
        let (snippet, start, end) = extract_snippet(content, 4, 1);
        assert_eq!(snippet, "line3\nline4\nline5");
        assert_eq!(start, 3);
        assert_eq!(end, 5);
    }

    #[test]
    fn test_extract_snippet_start_clamped() {
        let content = "line1\nline2\nline3\nline4\nline5";
        let (snippet, start, end) = extract_snippet(content, 1, 2);
        assert_eq!(start, 1);
        assert_eq!(end, 3);
        assert_eq!(snippet, "line1\nline2\nline3");
    }

    #[test]
    fn test_extract_snippet_end_clamped() {
        let content = "line1\nline2\nline3\nline4\nline5";
        let (snippet, start, end) = extract_snippet(content, 5, 2);
        assert_eq!(start, 3);
        assert_eq!(end, 5);
        assert_eq!(snippet, "line3\nline4\nline5");
    }
}
