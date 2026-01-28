use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::function_collector::FunctionCollector;
use crate::tracing_collector::TracingCollector;

/// Information about a function and its tracing coverage
#[derive(Debug, Clone, Serialize)]
pub struct FunctionInfo {
    /// Source file path
    pub file: PathBuf,
    /// Module path (namespace)
    pub module_path: String,
    /// Function name
    pub name: String,
    /// Start line (including attributes)
    pub start_line: usize,
    /// End line
    pub end_line: usize,
    /// Number of tracing statements in this function
    pub tracing_count: usize,
    /// Whether function has #[instrument] attribute
    pub has_instrument: bool,
}

impl FunctionInfo {
    /// Calculate the number of lines in this function
    pub fn line_count(&self) -> usize {
        if self.end_line >= self.start_line {
            self.end_line - self.start_line + 1
        } else {
            0
        }
    }

    /// Calculate tracing density (statements per 100 lines)
    pub fn density(&self) -> f64 {
        let lines = self.line_count();
        if lines == 0 {
            0.0
        } else {
            (self.tracing_count as f64) / (lines as f64) * 100.0
        }
    }

    /// Get full qualified path
    pub fn full_path(&self) -> String {
        if self.module_path.is_empty() {
            self.name.clone()
        } else {
            format!("{}::{}", self.module_path, self.name)
        }
    }
}

/// Represents a tracing statement location
#[derive(Debug, Clone, Serialize)]
pub struct TracingLocation {
    pub line: usize,
    pub kind: TracingKind,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum TracingKind {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Instrument,
}

/// Analyze a single Rust source file
pub fn analyze_file(path: &Path) -> Result<Vec<FunctionInfo>, String> {
    let content = fs::read_to_string(path).map_err(|e| format!("Failed to read file: {}", e))?;

    // Parse the file
    let syntax = syn::parse_file(&content).map_err(|e| format!("Failed to parse: {}", e))?;

    // Collect all functions
    let mut function_collector = FunctionCollector::new(path);
    function_collector.visit_file(&syntax);
    let mut functions = function_collector.functions;

    // Collect all tracing statements (by line number)
    let tracing_locations = TracingCollector::collect(&content);

    // Build ordered map of line -> tracing statements
    let tracing_map: BTreeMap<usize, Vec<&TracingLocation>> = {
        let mut map: BTreeMap<usize, Vec<&TracingLocation>> = BTreeMap::new();
        for loc in &tracing_locations {
            map.entry(loc.line).or_default().push(loc);
        }
        map
    };

    // For each function, count tracing statements in its range
    for func in &mut functions {
        let mut count = 0;

        // Count statements within the function's line range
        for (_line, locs) in tracing_map.range(func.start_line..=func.end_line) {
            count += locs.len();
        }

        func.tracing_count = count;
    }

    Ok(functions)
}
