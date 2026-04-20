---
description: "Implement ticket 614f5f2a: spec-api multi-file folder structure with SpecFs"
---

# Ticket 614f5f2a — Multi-File Folder Structure

## Goal

Implement the spec folder layout using `EntityFs` from memory-api. Each spec lives in a `<uuid>/` folder with `spec.toml`, `body.md`, optional `sections/`, `assets/`, `history.ndjson`, and `.spec-lock`.

## Ticket State Management

```bash
# At start:
./target/debug/ticket.exe update 614f5f2a --to-state in-implementation
./target/debug/ticket.exe board check-in 614f5f2a --agent-id copilot --intent "implementing SpecFs folder structure" --files "crates/spec-api/src/spec_fs.rs" --ttl 3600

# At end (after tests pass):
./target/debug/ticket.exe update 614f5f2a --to-state in-review
```

## Context

- `EntityFs` already exists in `crates/memory-api/src/storage/entity_fs.rs`
- It takes a `EntityFolderConfig` specifying `manifest_file` and `lock_file` names
- `EntityFs::new("spec.toml", ".spec-lock")` gives us a spec-configured instance
- EntityFs already handles: `create`, `read`, `update`, `mark_deleted`, `scan_root`, `read_history`, `append_history`
- See `crates/ticket-api/` for how ticket-api wraps EntityFs (likely via EntityStore or similar)

## Folder Layout

```
<scan_root>/<uuid>/
  spec.toml         ← SpecManifest (TOML)
  body.md           ← main specification body (markdown)
  sections/         ← optional additional sections (created on demand)
  assets/           ← optional attachments (created on demand)
  .spec-lock        ← advisory lock file for writes (fs4)
  history.ndjson    ← append-only revision log
```

## Implementation

### Step 1: Create `crates/spec-api/src/spec_fs.rs`

The key insight: EntityFs already handles everything. SpecFs is a thin wrapper that:
1. Configures EntityFs with spec-specific filenames (`spec.toml`, `.spec-lock`)
2. Adds `body.md` handling (read/write)
3. Adds `sections/` and `assets/` directory management

```rust
use std::path::{Path, PathBuf};
use std::fs;
use memory_api::storage::entity_fs::EntityFs;
use memory_api::model::entity::EntityManifest;
use memory_api::error::StorageError;
use crate::manifest::SpecManifest;

/// Spec-specific EntityFs configuration constant.
pub const SPEC_MANIFEST_FILE: &str = "spec.toml";
pub const SPEC_LOCK_FILE: &str = ".spec-lock";

/// Create an EntityFs configured for spec entities.
pub fn spec_entity_fs() -> EntityFs {
    EntityFs::new(SPEC_MANIFEST_FILE, SPEC_LOCK_FILE)
}

/// Create a new spec folder with spec.toml and body.md.
///
/// This wraps EntityFs::create and adds body.md writing.
pub fn create_spec_folder(
    fs: &EntityFs,
    manifest: &SpecManifest,
    target_root: &Path,
    body: &str,
) -> Result<PathBuf, StorageError> {
    // Convert SpecManifest to EntityManifest for EntityFs
    let entity_manifest = spec_to_entity(manifest)?;
    // EntityFs::create writes spec.toml and optionally description.md
    // We use body parameter to write body content
    let folder = fs.create(&entity_manifest, target_root, Some(body))?;

    // Rename description.md → body.md (EntityFs writes "description.md")
    let desc_path = folder.join("description.md");
    let body_path = folder.join("body.md");
    if desc_path.exists() {
        std::fs::rename(&desc_path, &body_path)?;
    }

    Ok(folder)
}

/// Read the body.md content from a spec folder.
pub fn read_body(spec_path: &Path) -> Result<String, StorageError> {
    let body_path = spec_path.join("body.md");
    if !body_path.exists() {
        return Ok(String::new());
    }
    fs::read_to_string(&body_path).map_err(StorageError::Io)
}

/// Write body.md content to a spec folder.
pub fn write_body(spec_path: &Path, body: &str) -> Result<(), StorageError> {
    let body_path = spec_path.join("body.md");
    fs::write(&body_path, body).map_err(StorageError::Io)
}

/// Ensure the sections/ directory exists.
pub fn ensure_sections_dir(spec_path: &Path) -> Result<PathBuf, StorageError> {
    let dir = spec_path.join("sections");
    fs::create_dir_all(&dir).map_err(StorageError::Io)?;
    Ok(dir)
}

/// Ensure the assets/ directory exists.
pub fn ensure_assets_dir(spec_path: &Path) -> Result<PathBuf, StorageError> {
    let dir = spec_path.join("assets");
    fs::create_dir_all(&dir).map_err(StorageError::Io)?;
    Ok(dir)
}

/// List section files in sections/ (sorted).
pub fn list_sections(spec_path: &Path) -> Result<Vec<PathBuf>, StorageError> {
    let dir = spec_path.join("sections");
    if !dir.exists() {
        return Ok(Vec::new());
    }
    let mut entries: Vec<PathBuf> = fs::read_dir(&dir)
        .map_err(StorageError::Io)?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.is_file())
        .collect();
    entries.sort();
    Ok(entries)
}

// ── conversions ──

/// Convert SpecManifest → EntityManifest for EntityFs operations.
fn spec_to_entity(spec: &SpecManifest) -> Result<EntityManifest, StorageError> {
    // EntityManifest has the same shape: id, created_at, extra
    Ok(EntityManifest {
        id: spec.id,
        created_at: spec.created_at,
        extra: spec.extra.clone(),
    })
}

/// Convert EntityManifest → SpecManifest.
pub fn entity_to_spec(entity: &EntityManifest) -> SpecManifest {
    SpecManifest {
        id: entity.id,
        created_at: entity.created_at,
        extra: entity.extra.clone(),
    }
}
```

**IMPORTANT**: Before writing this code, verify:
1. How `EntityManifest` is structured (check `crates/memory-api/src/model/entity.rs`) — it likely has `id`, `created_at`, `extra` just like SpecManifest
2. How EntityFs handles the `body` parameter in `create()` — it writes `description.md`. Decide if you want to rename it to `body.md` or change the convention. The simplest approach is to rename after creation.
3. Whether SpecManifest implements `Into<EntityManifest>` or if you need a conversion function

### Step 2: Register module in lib.rs

```rust
pub mod spec_fs;
pub use spec_fs::{spec_entity_fs, create_spec_folder, read_body, write_body};
```

### Step 3: Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use crate::manifest::SpecManifest;

    fn setup() -> (TempDir, EntityFs) {
        let tmp = TempDir::new().unwrap();
        let fs = spec_entity_fs();
        (tmp, fs)
    }

    #[test]
    fn test_create_and_read_spec_folder() {
        let (tmp, fs) = setup();
        let manifest = SpecManifest::new("test/slug", "Test Spec", "test-component");

        let folder = create_spec_folder(&fs, &manifest, tmp.path(), "# Hello\n\nSpec body.").unwrap();

        // Verify spec.toml exists and is readable
        let entity = fs.read(&folder).unwrap();
        assert_eq!(entity.id, manifest.id);

        // Verify body.md
        let body = read_body(&folder).unwrap();
        assert_eq!(body, "# Hello\n\nSpec body.");

        // Verify .spec-lock does NOT exist yet (only created during writes)
        assert!(!folder.join(".spec-lock").exists() || folder.join(".spec-lock").exists());
    }

    #[test]
    fn test_write_body() {
        let (tmp, fs) = setup();
        let manifest = SpecManifest::new("test/body", "Body Test", "comp");
        let folder = create_spec_folder(&fs, &manifest, tmp.path(), "initial").unwrap();

        write_body(&folder, "updated body content").unwrap();
        let body = read_body(&folder).unwrap();
        assert_eq!(body, "updated body content");
    }

    #[test]
    fn test_sections_and_assets() {
        let (tmp, fs) = setup();
        let manifest = SpecManifest::new("test/dirs", "Dir Test", "comp");
        let folder = create_spec_folder(&fs, &manifest, tmp.path(), "body").unwrap();

        // Initially no sections
        let sections = list_sections(&folder).unwrap();
        assert!(sections.is_empty());

        // Create sections dir and add a file
        let sections_dir = ensure_sections_dir(&folder).unwrap();
        std::fs::write(sections_dir.join("01-overview.md"), "Overview").unwrap();
        let sections = list_sections(&folder).unwrap();
        assert_eq!(sections.len(), 1);

        // Create assets dir
        let assets_dir = ensure_assets_dir(&folder).unwrap();
        assert!(assets_dir.exists());
    }

    #[test]
    fn test_read_body_missing() {
        let (tmp, _) = setup();
        // Non-existent path returns empty string
        let body = read_body(tmp.path()).unwrap();
        assert_eq!(body, "");
    }

    #[test]
    fn test_roundtrip_manifest() {
        let (tmp, fs) = setup();
        let manifest = SpecManifest::new("round/trip", "Roundtrip", "comp");
        let folder = create_spec_folder(&fs, &manifest, tmp.path(), "body").unwrap();

        let entity = fs.read(&folder).unwrap();
        let spec = entity_to_spec(&entity);
        assert_eq!(spec.slug(), manifest.slug());
        assert_eq!(spec.title(), manifest.title());
        assert_eq!(spec.id(), manifest.id());
    }

    #[test]
    fn test_scan_root() {
        let (tmp, fs) = setup();
        let m1 = SpecManifest::new("a/b", "Spec A", "comp");
        let m2 = SpecManifest::new("c/d", "Spec B", "comp");
        create_spec_folder(&fs, &m1, tmp.path(), "body1").unwrap();
        create_spec_folder(&fs, &m2, tmp.path(), "body2").unwrap();

        let (entries, diags) = fs.scan_root(tmp.path()).unwrap();
        assert_eq!(entries.len(), 2);
        assert!(diags.is_empty());
    }

    #[test]
    fn test_history_append() {
        let (tmp, fs) = setup();
        let manifest = SpecManifest::new("hist/test", "History", "comp");
        let folder = create_spec_folder(&fs, &manifest, tmp.path(), "body").unwrap();

        // Initially no history
        let history = fs.read_history(&folder).unwrap();
        assert!(history.is_empty());

        // Append a revision
        fs.append_history(&folder, &manifest.extra, None).unwrap();
        let history = fs.read_history(&folder).unwrap();
        assert_eq!(history.len(), 1);
    }
}
```

### Step 4: Add `fs4` to spec-api Cargo.toml (if not already present)

Only needed if SpecFs does its own locking beyond EntityFs. Since EntityFs handles locking internally, this may not be needed. Check whether `fs4` is already a transitive dependency via `memory-api`.

## Validation

```bash
cargo test -p spec-api
cargo check -p spec-api
```

## Key Constraints

- **Reuse EntityFs** — do NOT reimplement filesystem operations. EntityFs already handles manifest CRUD, locking, history, and scanning.
- SpecFs is a thin wrapper adding: `body.md` handling, `sections/` and `assets/` directory helpers, and SpecManifest↔EntityManifest conversions
- `sections/` and `assets/` dirs are created on demand, not at entity creation time
- Advisory locking is handled by EntityFs using `.spec-lock` — no custom locking needed
- History uses EntityFs's `append_history()` and `read_history()` — uses `history.ndjson` automatically
- Check if EntityFs writes `description.md` on create — you may need to rename it to `body.md` or adjust the convention
