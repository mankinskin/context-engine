use crate::analyzer::{
    TracingKind,
    TracingLocation,
};

/// Collects tracing macro invocations from source code
///
/// This uses a simple line-based approach rather than full AST parsing
/// because macro invocations can be complex and span multiple lines.
pub struct TracingCollector;

impl TracingCollector {
    /// Collect all tracing statements from source content
    pub fn collect(content: &str) -> Vec<TracingLocation> {
        let mut locations = Vec::new();

        // Patterns to look for
        let patterns = [
            // Standard tracing macros
            ("trace!", TracingKind::Trace),
            ("tracing::trace!", TracingKind::Trace),
            ("debug!", TracingKind::Debug),
            ("tracing::debug!", TracingKind::Debug),
            ("info!", TracingKind::Info),
            ("tracing::info!", TracingKind::Info),
            ("warn!", TracingKind::Warn),
            ("tracing::warn!", TracingKind::Warn),
            ("error!", TracingKind::Error),
            ("tracing::error!", TracingKind::Error),
            // Span macros
            ("trace_span!", TracingKind::Trace),
            ("debug_span!", TracingKind::Debug),
            ("info_span!", TracingKind::Info),
            ("warn_span!", TracingKind::Warn),
            ("error_span!", TracingKind::Error),
            // Instrument attribute
            ("#[instrument", TracingKind::Instrument),
            ("#[tracing::instrument", TracingKind::Instrument),
        ];

        for (line_num, line) in content.lines().enumerate() {
            let line_number = line_num + 1; // 1-indexed

            // Skip comments
            let trimmed = line.trim();
            if trimmed.starts_with("//") {
                continue;
            }

            // Check for each pattern
            for (pattern, kind) in &patterns {
                if line.contains(pattern) {
                    // Avoid false positives from string literals
                    // Simple heuristic: check if pattern appears outside of quotes
                    if is_outside_string(line, pattern) {
                        locations.push(TracingLocation {
                            line: line_number,
                            kind: kind.clone(),
                        });
                        break; // Only count once per line
                    }
                }
            }
        }

        locations
    }
}

/// Simple heuristic to check if a pattern appears outside of string literals
/// This is not perfect but works for most cases
fn is_outside_string(
    line: &str,
    pattern: &str,
) -> bool {
    if let Some(pos) = line.find(pattern) {
        // Count quotes before the pattern
        let before = &line[..pos];
        let double_quotes =
            before.matches('"').count() - before.matches("\\\"").count();

        // If even number of unescaped quotes, we're outside a string
        double_quotes % 2 == 0
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collect_trace() {
        let content = r#"
fn example() {
    trace!("hello");
    debug!("world");
    info!("test");
}
"#;
        let locations = TracingCollector::collect(content);
        assert_eq!(locations.len(), 3);
        assert_eq!(locations[0].kind, TracingKind::Trace);
        assert_eq!(locations[1].kind, TracingKind::Debug);
        assert_eq!(locations[2].kind, TracingKind::Info);
    }

    #[test]
    fn test_skip_comments() {
        let content = r#"
fn example() {
    // trace!("commented out");
    debug!("real");
}
"#;
        let locations = TracingCollector::collect(content);
        assert_eq!(locations.len(), 1);
        assert_eq!(locations[0].kind, TracingKind::Debug);
    }

    #[test]
    fn test_instrument_attribute() {
        let content = r#"
#[instrument]
fn example() {
    debug!("hello");
}
"#;
        let locations = TracingCollector::collect(content);
        assert_eq!(locations.len(), 2);
        assert_eq!(locations[0].kind, TracingKind::Instrument);
        assert_eq!(locations[1].kind, TracingKind::Debug);
    }

    #[test]
    fn test_namespaced_macros() {
        let content = r#"
fn example() {
    tracing::trace!("a");
    tracing::debug!("b");
    tracing::info!("c");
}
"#;
        let locations = TracingCollector::collect(content);
        assert_eq!(locations.len(), 3);
    }
}
