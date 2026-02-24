//! JQ-style query language for filtering and querying documentation.
//!
//! Uses jaq (a jq clone) to provide powerful JSON filtering on YAML and
//! Markdown documents converted to JSON.
//!
//! # Example Queries
//!
//! ```text
//! # Filter by document type
//! select(.doc_type == "guide")
//!
//! # Filter by tag
//! select(.tags | any(. == "testing"))
//!
//! # Search in title
//! select(.title | contains("search"))
//!
//! # Combine with regex
//! select(.title | test("error|bug"; "i"))
//!
//! # Filter by date range
//! select(.date >= "20250101")
//!
//! # Extract specific fields
//! {title, tags, summary}
//!
//! # For crate docs - filter by module path
//! select(.path | startswith("search/"))
//! ```

use jaq_interpret::{
    Ctx,
    FilterT,
    ParseCtx,
    RcIter,
    Val,
};
use serde_json::Value;

/// Error type for query operations
#[derive(Debug)]
pub struct QueryError {
    pub message: String,
}

impl std::fmt::Display for QueryError {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for QueryError {}

/// A compiled jq filter ready to execute
pub struct JqFilter {
    #[allow(dead_code)]
    ctx: ParseCtx,
    filter: jaq_interpret::Filter,
}

impl JqFilter {
    /// Compile a jq filter string
    pub fn compile(query: &str) -> Result<Self, QueryError> {
        // Create parsing context and add standard definitions
        let mut ctx = ParseCtx::new(Vec::new());
        ctx.insert_natives(jaq_core::core());
        ctx.insert_defs(jaq_std::std());

        // Parse the query - jaq_syn::parse returns Option<T>
        let parsed = jaq_syn::parse(query, |p| p.module(|p| p.term()));

        let main = parsed.ok_or_else(|| QueryError {
            message: "Parse error: invalid jq expression".to_string(),
        })?;

        // Convert and compile
        let filter = main.conv(query);
        let filter = ctx.compile(filter);

        if !ctx.errs.is_empty() {
            let msg = ctx
                .errs
                .iter()
                .map(|(e, _span)| format!("{}", e))
                .collect::<Vec<_>>()
                .join(", ");
            return Err(QueryError {
                message: format!("Compilation error: {}", msg),
            });
        }

        Ok(Self { ctx, filter })
    }

    /// Run the filter on a JSON value, returning all outputs
    pub fn run(
        &self,
        input: &Value,
    ) -> Vec<Result<Value, String>> {
        let inputs = RcIter::new(std::iter::empty());
        let val = Val::from(input.clone());
        let ctx = Ctx::new([], &inputs);

        self.filter
            .run((ctx, val))
            .map(|result| {
                result
                    .map(|v| serde_json::Value::from(v))
                    .map_err(|e| format!("{}", e))
            })
            .collect()
    }

    /// Run the filter on a value, returning true if any output is truthy
    pub fn matches(
        &self,
        input: &Value,
    ) -> bool {
        let results = self.run(input);
        results.into_iter().any(|r| match r {
            Ok(Value::Bool(true)) => true,
            Ok(Value::Null) | Ok(Value::Bool(false)) => false,
            Ok(_) => true, // Non-null, non-false values are truthy
            Err(_) => false,
        })
    }
}

/// Filter a slice of JSON values using a jq query (returns matching values)
pub fn filter_values<'a>(
    values: impl IntoIterator<Item = &'a Value>,
    query: &str,
) -> Result<Vec<Value>, QueryError> {
    let filter = JqFilter::compile(query)?;

    let mut results = Vec::new();
    for value in values {
        if filter.matches(value) {
            results.push(value.clone());
        }
    }

    Ok(results)
}

/// Transform values using a jq query (can produce multiple/different outputs per input)
pub fn transform_values<'a>(
    values: impl IntoIterator<Item = &'a Value>,
    query: &str,
) -> Result<Vec<Value>, QueryError> {
    let filter = JqFilter::compile(query)?;

    let mut results = Vec::new();
    for value in values {
        for result in filter.run(value) {
            if let Ok(v) = result {
                results.push(v);
            }
        }
    }

    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_compile_simple_filter() {
        let filter = JqFilter::compile(".");
        assert!(filter.is_ok());
    }

    #[test]
    fn test_compile_invalid_filter() {
        let filter = JqFilter::compile("[invalid");
        assert!(filter.is_err());
    }

    #[test]
    fn test_filter_select() {
        let filter = JqFilter::compile("select(.level == \"ERROR\")").unwrap();

        let error_doc = json!({"level": "ERROR", "title": "Bug"});
        let info_doc = json!({"level": "INFO", "title": "Guide"});

        assert!(filter.matches(&error_doc));
        assert!(!filter.matches(&info_doc));
    }

    #[test]
    fn test_filter_contains() {
        // Note: contains() is case-sensitive in jq
        let filter =
            JqFilter::compile("select(.title | contains(\"search\"))").unwrap();

        let match_doc = json!({"title": "search guide"});
        let no_match = json!({"title": "Other guide"});

        assert!(filter.matches(&match_doc));
        assert!(!filter.matches(&no_match));
    }

    #[test]
    fn test_filter_case_insensitive() {
        // Use test() with "i" flag for case-insensitive matching
        let filter =
            JqFilter::compile("select(.title | test(\"search\"; \"i\"))")
                .unwrap();

        let match_upper = json!({"title": "Search Guide"});
        let match_lower = json!({"title": "search guide"});
        let no_match = json!({"title": "Other guide"});

        assert!(filter.matches(&match_upper));
        assert!(filter.matches(&match_lower));
        assert!(!filter.matches(&no_match));
    }

    #[test]
    fn test_filter_array_any() {
        let filter =
            JqFilter::compile("select(.tags | any(. == \"testing\"))").unwrap();

        let match_doc = json!({"tags": ["testing", "debug"]});
        let no_match = json!({"tags": ["production"]});

        assert!(filter.matches(&match_doc));
        assert!(!filter.matches(&no_match));
    }

    #[test]
    fn test_transform_values() {
        let docs = vec![
            json!({"title": "Doc A", "date": "20250101"}),
            json!({"title": "Doc B", "date": "20250102"}),
        ];

        let results = transform_values(docs.iter(), "{title}").unwrap();

        assert_eq!(results.len(), 2);
        assert_eq!(results[0], json!({"title": "Doc A"}));
        assert_eq!(results[1], json!({"title": "Doc B"}));
    }

    #[test]
    fn test_filter_values() {
        let docs = vec![
            json!({"doc_type": "guide", "title": "Guide"}),
            json!({"doc_type": "plan", "title": "Plan"}),
            json!({"doc_type": "guide", "title": "Another Guide"}),
        ];

        let results =
            filter_values(docs.iter(), "select(.doc_type == \"guide\")")
                .unwrap();

        assert_eq!(results.len(), 2);
        assert_eq!(results[0]["title"], "Guide");
        assert_eq!(results[1]["title"], "Another Guide");
    }

    #[test]
    fn test_date_comparison() {
        let filter =
            JqFilter::compile("select(.date >= \"20250201\")").unwrap();

        let after = json!({"date": "20250215"});
        let before = json!({"date": "20250115"});

        assert!(filter.matches(&after));
        assert!(!filter.matches(&before));
    }
}
