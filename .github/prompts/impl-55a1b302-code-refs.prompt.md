---
description: "Implement ticket 55a1b302: spec-api code references — CodeRef struct and validation"
---

# Ticket 55a1b302 — Code References: Symbol-Level Links

## Goal

Add `CodeRef` and `SymbolKind` types to `spec-api` for linking spec features to exact symbols in implementation code. Include validation for file existence and line range plausibility.

## Ticket State Management

```bash
# At start:
./target/debug/ticket.exe update 55a1b302 --to-state in-implementation
./target/debug/ticket.exe board check-in 55a1b302 --agent-id copilot --intent "implementing CodeRef system" --files "crates/spec-api/src/code_ref.rs" --ttl 3600

# At end (after tests pass):
./target/debug/ticket.exe update 55a1b302 --to-state in-review
```

## Context

- `spec-api` crate exists at `crates/spec-api/` with `manifest.rs` (SpecManifest), `error.rs`, `lib.rs`
- SpecManifest has `extra: BTreeMap<String, Value>` with `#[serde(flatten)]`
- Code refs are stored inline in `spec.toml` as `[[code_refs]]` TOML array entries
- This ticket does NOT touch SpecStore (that's ab47648c) — just defines the types and standalone validation functions

## Implementation

### Step 1: Create `crates/spec-api/src/code_ref.rs`

```rust
use serde::{Deserialize, Serialize};

/// The kind of symbol a code reference points to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SymbolKind {
    Struct,
    Function,
    Trait,
    Impl,
    Enum,
    Module,
    Const,
    Type,
}

/// A reference from a spec to a specific symbol in implementation code.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CodeRef {
    /// Workspace-relative file path, e.g. "crates/ticket-api/src/storage/store.rs"
    pub file: String,
    /// Symbol name, e.g. "TicketStore::create"
    pub symbol: String,
    /// Kind of symbol
    pub kind: SymbolKind,
    /// Start line (1-based)
    pub line_start: u32,
    /// End line (1-based, inclusive)
    pub line_end: u32,
    /// Optional description of what this reference covers
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
```

### Step 2: TOML serialization

Ensure CodeRef round-trips as TOML `[[code_refs]]` array entries:

```toml
[[code_refs]]
file = "crates/ticket-api/src/storage/store.rs"
symbol = "TicketStore"
kind = "struct"
line_start = 45
line_end = 52

[[code_refs]]
file = "crates/ticket-api/src/storage/store.rs"
symbol = "TicketStore::create"
kind = "function"
line_start = 120
line_end = 180
description = "Creates a new ticket with generated UUID"
```

This means SpecManifest needs to support `code_refs` as a field. There are two approaches:
- **(a)** Store code_refs in the `extra` BTreeMap as a JSON value (array of objects). This is consistent with the existing pattern but requires manual serialization.
- **(b)** Add `code_refs: Vec<CodeRef>` as a direct field on SpecManifest with `#[serde(default, skip_serializing_if = "Vec::is_empty")]`.

Choose **(b)** — it's cleaner for TOML round-tripping. Add the field to SpecManifest:

```rust
// In manifest.rs, add to SpecManifest struct:
#[serde(default, skip_serializing_if = "Vec::is_empty")]
pub code_refs: Vec<CodeRef>,
```

### Step 3: Validation functions

Create validation functions (standalone, not on SpecStore):

```rust
use std::path::Path;

/// Validation result for a single code ref.
pub struct RefValidation {
    pub code_ref: CodeRef,
    pub file_exists: bool,
    pub line_range_valid: bool,  // line_end >= line_start, line_end <= file line count
    pub message: Option<String>,
}

/// Validate code refs against a workspace root.
pub fn validate_refs(code_refs: &[CodeRef], workspace_root: &Path) -> Vec<RefValidation> {
    code_refs.iter().map(|r| {
        let file_path = workspace_root.join(&r.file);
        let file_exists = file_path.exists();
        let line_range_valid = r.line_end >= r.line_start && if file_exists {
            // Count lines in file, check line_end <= line count
            let content = std::fs::read_to_string(&file_path).unwrap_or_default();
            let line_count = content.lines().count() as u32;
            r.line_end <= line_count
        } else {
            false
        };
        RefValidation {
            code_ref: r.clone(),
            file_exists,
            line_range_valid,
            message: if !file_exists {
                Some(format!("file not found: {}", r.file))
            } else if !line_range_valid {
                Some(format!("invalid line range: {}-{}", r.line_start, r.line_end))
            } else {
                None
            },
        }
    }).collect()
}

/// Reverse lookup: find which code refs reference a given file path.
pub fn find_refs_for_file<'a>(code_refs: &'a [CodeRef], file_path: &str) -> Vec<&'a CodeRef> {
    code_refs.iter().filter(|r| r.file == file_path).collect()
}
```

### Step 4: Register module in lib.rs

```rust
pub mod code_ref;
pub use code_ref::{CodeRef, SymbolKind};
```

### Step 5: Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_code_ref_serde_roundtrip() {
        let cr = CodeRef {
            file: "src/main.rs".to_string(),
            symbol: "main".to_string(),
            kind: SymbolKind::Function,
            line_start: 1,
            line_end: 10,
            description: None,
        };
        let toml_str = toml::to_string(&cr).unwrap();
        let parsed: CodeRef = toml::from_str(&toml_str).unwrap();
        assert_eq!(cr, parsed);
    }

    #[test]
    fn test_symbol_kind_serde() {
        // Verify snake_case serialization
        let cr = CodeRef {
            file: "src/lib.rs".to_string(),
            symbol: "MyStruct".to_string(),
            kind: SymbolKind::Struct,
            line_start: 5,
            line_end: 20,
            description: Some("A struct".to_string()),
        };
        let toml_str = toml::to_string(&cr).unwrap();
        assert!(toml_str.contains("kind = \"struct\""));
    }

    #[test]
    fn test_validate_refs_missing_file() {
        let refs = vec![CodeRef {
            file: "nonexistent/file.rs".to_string(),
            symbol: "foo".to_string(),
            kind: SymbolKind::Function,
            line_start: 1,
            line_end: 5,
            description: None,
        }];
        let results = validate_refs(&refs, std::path::Path::new("/tmp"));
        assert!(!results[0].file_exists);
    }

    #[test]
    fn test_find_refs_for_file() {
        let refs = vec![
            CodeRef { file: "a.rs".into(), symbol: "A".into(), kind: SymbolKind::Struct, line_start: 1, line_end: 5, description: None },
            CodeRef { file: "b.rs".into(), symbol: "B".into(), kind: SymbolKind::Struct, line_start: 1, line_end: 5, description: None },
            CodeRef { file: "a.rs".into(), symbol: "C".into(), kind: SymbolKind::Function, line_start: 10, line_end: 20, description: None },
        ];
        let found = find_refs_for_file(&refs, "a.rs");
        assert_eq!(found.len(), 2);
    }
}
```

## Validation

```bash
cargo test -p spec-api
cargo check -p spec-api
```

## Key Constraints

- Do NOT add code ref management to SpecStore — that's ticket ab47648c
- `validate_refs` and `find_refs_for_file` are standalone functions, not methods on any store
- `code_refs` field on SpecManifest uses `#[serde(default, skip_serializing_if = "Vec::is_empty")]` so existing specs without code refs still parse
- SymbolKind uses `#[serde(rename_all = "snake_case")]` for TOML compatibility
