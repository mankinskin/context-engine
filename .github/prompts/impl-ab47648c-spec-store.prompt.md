---
description: "Implement ticket ab47648c: SpecStore — storage layer wrapping EntityStore with slug resolution, hierarchy, and section management"
---

# Ticket ab47648c — SpecStore: Storage Layer

## Goal

Build `SpecStore` on top of `memory_api::EntityStore` adding spec-specific features: parent-child hierarchy, slug uniqueness via `SlugIndex`, multi-file folder support via `EntityFs` configured for specs, and section CRUD.

This is the critical-path ticket — 12 downstream tickets (CLI, MCP, HTTP, health, search, etc.) all depend on it.

## Ticket State Management

```bash
# At start:
./target/debug/ticket.exe update ab47648c --to-state in-implementation
./target/debug/ticket.exe board check-in ab47648c --agent-id copilot \
  --intent "implementing SpecStore storage layer" \
  --files "crates/spec-api/src/store.rs,crates/spec-api/src/lib.rs,crates/spec-api/src/error.rs,crates/spec-api/Cargo.toml" \
  --ttl 3600

# At end (after tests pass):
./target/debug/ticket.exe update ab47648c --to-state in-review
```

## Architecture

```
SpecStore
├── inner: EntityStore         (memory-api — SQLite index + Tantivy search + EntityFs)
├── slug_index: SlugIndex      (spec-api — in-memory slug→UUID cache)
└── EntityFs configured with ("spec.toml", ".spec-lock")
```

**Key design**: SpecStore wraps `EntityStore` from memory-api. It does NOT duplicate index/search/fs logic. The `EntityStore` facade handles all persistence; SpecStore adds:
1. Slug validation + uniqueness enforcement (via `SlugIndex`)
2. SpecManifest ↔ EntityManifest conversion
3. `body.md` file management (specs use `body.md` not `description.md`)
4. `sections/` directory management
5. Parent-child hierarchy traversal (via the `parent` extra field)

## Existing Code to Build On

The following already exist in `crates/spec-api/src/`:

| File | Contents |
|---|---|
| `manifest.rs` | `SpecManifest` with `id`, `created_at`, `code_refs: Vec<CodeRef>`, `extra: BTreeMap` + typed accessors (slug, title, state, component, scope, parent) |
| `slug.rs` | `validate_slug()`, `SlugIndex` (HashMap-based, uniqueness enforcement, rebuild) |
| `code_ref.rs` | `CodeRef`, `SymbolKind`, `validate_refs()`, `find_refs_for_file()` |
| `default_schema.rs` | `specification_schema()`, `spec_schema_registry()` — loads `schemas/specification.toml` |
| `error.rs` | `SpecError` with variants: `NotFound`, `InvalidSlug`, `DuplicateSlug`, `Storage(StorageError)`, `Serialization` |

**EntityStore** (in `crates/memory-api/src/storage/entity_store.rs`) provides:
- `open(index_root, EntityFs)` / `open_with(index_root, EntityFs, SchemaRegistry)`
- `add_scan_root(ScanRoot)`, `list_scan_roots()`
- `get_indexed(Uuid)`, `list_indexed(bool)`
- `search(query, limit)`
- `add_edge(EdgeRecord)`, `remove_edge(EdgeRecord)`, `edges_from(Uuid)`, `list_all_edges()`
- `scan(reindex: bool) → ScanReport`
- Public fields: `index: RedbIndexStore`, `fs: EntityFs`, `search: TantivySearchIndex`, `schema_registry: SchemaRegistry`, `index_root: PathBuf`

**EntityFs** (in `crates/memory-api/src/storage/entity_fs.rs`) provides:
- `new(manifest_file, lock_file)` — configured with `("spec.toml", ".spec-lock")` for specs
- `create(EntityManifest, target_root, body)` — creates `<uuid>/` folder with manifest + optional `description.md`
- `read(entity_path)` → `EntityManifest`
- `update(entity_path, patch, new_state)` → `EntityManifest`
- `mark_deleted(entity_path)`
- `scan_root(scan_root)` → `(Vec<EntityScanEntry>, Vec<ParseDiagnostic>)`
- `read_history(entity_path)`, `append_history(entity_path, fields, author)`
- `read_description(entity_path)`, `write_description(entity_path, text)`
- `ensure_assets_dir(entity_path)`

**EntityManifest** shape:
```rust
pub struct EntityManifest {
    pub id: EntityId,           // Uuid
    pub created_at: DateTime<Utc>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}
```

## Implementation Plan

### Step 1: Create `crates/spec-api/src/store.rs`

```rust
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::fs;

use chrono::Utc;
use serde_json::Value;
use uuid::Uuid;

use memory_api::error::StorageError;
use memory_api::model::entity::EntityManifest;
use memory_api::model::filesystem::ScanRoot;
use memory_api::storage::entity_fs::EntityFs;
use memory_api::storage::entity_store::EntityStore;

use crate::error::SpecError;
use crate::manifest::{SpecId, SpecManifest};
use crate::slug::SlugIndex;
use crate::default_schema::spec_schema_registry;

/// Spec filesystem configuration constants.
const SPEC_MANIFEST_FILE: &str = "spec.toml";
const SPEC_LOCK_FILE: &str = ".spec-lock";

/// The central spec store: wraps EntityStore with spec-specific features.
pub struct SpecStore {
    inner: EntityStore,
    slug_index: SlugIndex,
}
```

### Step 2: Constructor

```rust
impl SpecStore {
    /// Open a SpecStore at `index_root`.
    pub fn open(index_root: &Path) -> Result<Self, SpecError> {
        let fs = EntityFs::new(SPEC_MANIFEST_FILE, SPEC_LOCK_FILE);
        let registry = spec_schema_registry();
        let inner = EntityStore::open_with(index_root, fs, registry)?;
        Ok(Self {
            inner,
            slug_index: SlugIndex::new(),
        })
    }

    /// Access the underlying EntityStore.
    pub fn entity_store(&self) -> &EntityStore {
        &self.inner
    }
}
```

### Step 3: Scan & slug index rebuild

```rust
/// Scan all roots and rebuild the slug index.
pub fn scan(&mut self, reindex: bool) -> Result<memory_api::storage::entity_store::ScanReport, SpecError> {
    let report = self.inner.scan(reindex)?;
    self.rebuild_slug_index()?;
    Ok(report)
}

/// Rebuild the SlugIndex from the SQLite index.
fn rebuild_slug_index(&mut self) -> Result<(), SpecError> {
    let all = self.inner.list_indexed(false)?;
    let entries = all.iter().filter_map(|e| {
        // Read the manifest to get slug
        let manifest = self.inner.fs.read(&e.path).ok()?;
        let slug = manifest.extra.get("slug")?.as_str()?.to_string();
        Some((slug, e.id))
    });
    self.slug_index = SlugIndex::rebuild(entries)?;
    Ok(())
}
```

### Step 4: CRUD operations

```rust
/// Create a new spec.
pub fn create(
    &mut self,
    manifest: &SpecManifest,
    body: &str,
    target_root: Option<&Path>,
) -> Result<SpecId, SpecError> {
    // Validate slug
    let slug = manifest.slug().ok_or_else(|| SpecError::InvalidSlug("missing slug".into()))?;
    crate::slug::validate_slug(slug)?;

    // Check slug uniqueness
    if let Some(existing) = self.slug_index.resolve(slug) {
        if existing != manifest.id {
            return Err(SpecError::DuplicateSlug(slug.to_string()));
        }
    }

    // Resolve target root
    let root = match target_root {
        Some(p) => p.to_path_buf(),
        None => {
            let roots = self.inner.list_scan_roots()?;
            roots.into_iter()
                .next()
                .map(|r| r.path)
                .unwrap_or_else(|| self.inner.index_root.join("specs"))
        }
    };
    fs::create_dir_all(&root).map_err(StorageError::Io)?;

    // Convert SpecManifest → EntityManifest
    let entity = spec_to_entity(manifest);

    // Create folder via EntityFs (writes spec.toml + description.md)
    let folder = self.inner.fs.create(&entity, &root, Some(body))?;

    // Rename description.md → body.md
    let desc_path = folder.join("description.md");
    let body_path = folder.join("body.md");
    if desc_path.exists() {
        fs::rename(&desc_path, &body_path).map_err(StorageError::Io)?;
    }

    // Index the entity
    // ... integrate into SQLite + search (reuse inner.scan pattern or manually insert)

    // Register slug
    self.slug_index.insert(slug.to_string(), manifest.id)?;

    // Append initial history
    let _ = self.inner.fs.append_history(&folder, entity.extra.clone(), None);

    Ok(manifest.id)
}
```

**IMPORTANT**: Look at how `EntityStore::scan` calls `integrate_entry` to index entities in SQLite + search. You'll need to do the same after creating a spec. Either:
- (a) Call `self.inner.scan(false)` after create to pick it up, or
- (b) Directly call `self.inner.index.insert_ticket(...)` + `self.inner.search.upsert(...)` like TicketStore does

Option (b) is more efficient. Study `TicketStore::create` in `crates/ticket-api/src/storage/store.rs` (lines 130-185) for the exact pattern of indexing after creation.

### Step 5: Get operations

```rust
/// Resolve an ID or slug to a UUID.
pub fn resolve_id(&self, id_or_slug: &str) -> Result<Uuid, SpecError> {
    // Try parsing as UUID first
    if let Ok(uuid) = id_or_slug.parse::<Uuid>() {
        return Ok(uuid);
    }
    // Try UUID prefix match
    if let Some(uuid) = self.resolve_prefix(id_or_slug)? {
        return Ok(uuid);
    }
    // Try slug resolution
    self.slug_index.resolve(id_or_slug)
        .ok_or_else(|| SpecError::NotFound(id_or_slug.to_string()))
}

/// Get a spec manifest by ID or slug.
pub fn get(&self, id_or_slug: &str) -> Result<SpecManifest, SpecError> {
    let uuid = self.resolve_id(id_or_slug)?;
    let indexed = self.inner.get_indexed(&uuid)?
        .ok_or(SpecError::NotFound(uuid.to_string()))?;
    let entity = self.inner.fs.read(&indexed.path)?;
    Ok(entity_to_spec(&entity))
}

/// Get manifest + body.
pub fn get_full(&self, id_or_slug: &str) -> Result<(SpecManifest, String), SpecError> {
    let uuid = self.resolve_id(id_or_slug)?;
    let indexed = self.inner.get_indexed(&uuid)?
        .ok_or(SpecError::NotFound(uuid.to_string()))?;
    let entity = self.inner.fs.read(&indexed.path)?;
    let body = read_body(&indexed.path);
    Ok((entity_to_spec(&entity), body))
}
```

### Step 6: Update operations

```rust
/// Update manifest fields.
pub fn update(
    &mut self,
    id_or_slug: &str,
    patch: BTreeMap<String, Value>,
    to_state: Option<&str>,
) -> Result<SpecManifest, SpecError> {
    let uuid = self.resolve_id(id_or_slug)?;
    let indexed = self.inner.get_indexed(&uuid)?
        .ok_or(SpecError::NotFound(uuid.to_string()))?;

    // If slug is changing, validate + update slug index
    if let Some(new_slug_val) = patch.get("slug") {
        if let Some(new_slug) = new_slug_val.as_str() {
            crate::slug::validate_slug(new_slug)?;
            // Remove old slug, insert new
            let old = self.inner.fs.read(&indexed.path)?;
            if let Some(old_slug) = old.extra.get("slug").and_then(|v| v.as_str()) {
                self.slug_index.remove(old_slug);
            }
            self.slug_index.insert(new_slug.to_string(), uuid)?;
        }
    }

    // State transition validation via schema registry
    if let Some(to) = to_state {
        let current = indexed.state.as_deref().unwrap_or("draft");
        if let Some(schema) = self.inner.schema_registry().get("specification") {
            schema.ensure_transition(current, to)?;
        }
    }

    let updated = self.inner.fs.update(&indexed.path, &patch, to_state)?;

    // Update SQLite + search indexes
    // ... (follow TicketStore::update pattern)

    // Append history
    let _ = self.inner.fs.append_history(&indexed.path, updated.extra.clone(), None);

    Ok(entity_to_spec(&updated))
}

/// Update body.md content.
pub fn update_body(&self, id_or_slug: &str, content: &str) -> Result<(), SpecError> {
    let uuid = self.resolve_id(id_or_slug)?;
    let indexed = self.inner.get_indexed(&uuid)?
        .ok_or(SpecError::NotFound(uuid.to_string()))?;
    write_body(&indexed.path, content)?;
    Ok(())
}

/// Soft-delete a spec.
pub fn delete(&mut self, id_or_slug: &str) -> Result<(), SpecError> {
    let uuid = self.resolve_id(id_or_slug)?;
    let indexed = self.inner.get_indexed(&uuid)?
        .ok_or(SpecError::NotFound(uuid.to_string()))?;
    // Remove slug
    let entity = self.inner.fs.read(&indexed.path)?;
    if let Some(slug) = entity.extra.get("slug").and_then(|v| v.as_str()) {
        self.slug_index.remove(slug);
    }
    self.inner.fs.mark_deleted(&indexed.path)?;
    // Update index
    // ...
    Ok(())
}
```

### Step 7: Section CRUD

```rust
/// Add a section file.
pub fn add_section(&self, id_or_slug: &str, name: &str, content: &str) -> Result<(), SpecError> {
    let uuid = self.resolve_id(id_or_slug)?;
    let indexed = self.inner.get_indexed(&uuid)?
        .ok_or(SpecError::NotFound(uuid.to_string()))?;
    let sections_dir = indexed.path.join("sections");
    fs::create_dir_all(&sections_dir).map_err(StorageError::Io)?;
    let file_name = if name.ends_with(".md") { name.to_string() } else { format!("{}.md", name) };
    fs::write(sections_dir.join(&file_name), content).map_err(StorageError::Io)?;
    Ok(())
}

/// Update a section file.
pub fn update_section(&self, id_or_slug: &str, name: &str, content: &str) -> Result<(), SpecError> {
    let uuid = self.resolve_id(id_or_slug)?;
    let indexed = self.inner.get_indexed(&uuid)?
        .ok_or(SpecError::NotFound(uuid.to_string()))?;
    let file_name = if name.ends_with(".md") { name.to_string() } else { format!("{}.md", name) };
    let path = indexed.path.join("sections").join(&file_name);
    if !path.exists() {
        return Err(SpecError::NotFound(format!("section: {}", name)));
    }
    fs::write(&path, content).map_err(StorageError::Io)?;
    Ok(())
}

/// Delete a section file.
pub fn delete_section(&self, id_or_slug: &str, name: &str) -> Result<(), SpecError> {
    let uuid = self.resolve_id(id_or_slug)?;
    let indexed = self.inner.get_indexed(&uuid)?
        .ok_or(SpecError::NotFound(uuid.to_string()))?;
    let file_name = if name.ends_with(".md") { name.to_string() } else { format!("{}.md", name) };
    let path = indexed.path.join("sections").join(&file_name);
    if path.exists() {
        fs::remove_file(&path).map_err(StorageError::Io)?;
    }
    Ok(())
}

/// List section file names (sorted).
pub fn list_sections(&self, id_or_slug: &str) -> Result<Vec<String>, SpecError> {
    let uuid = self.resolve_id(id_or_slug)?;
    let indexed = self.inner.get_indexed(&uuid)?
        .ok_or(SpecError::NotFound(uuid.to_string()))?;
    let sections_dir = indexed.path.join("sections");
    if !sections_dir.exists() {
        return Ok(Vec::new());
    }
    let mut names: Vec<String> = fs::read_dir(&sections_dir)
        .map_err(StorageError::Io)?
        .filter_map(|e| e.ok())
        .map(|e| e.file_name().to_string_lossy().to_string())
        .filter(|n| n.ends_with(".md"))
        .collect();
    names.sort();
    Ok(names)
}
```

### Step 8: Hierarchy traversal

```rust
/// Get direct children of a spec (specs whose `parent` field == this spec's ID).
pub fn children(&self, id_or_slug: &str) -> Result<Vec<SpecManifest>, SpecError> {
    let uuid = self.resolve_id(id_or_slug)?;
    let uuid_str = uuid.to_string();
    let all = self.inner.list_indexed(false)?;
    let mut children = Vec::new();
    for indexed in all {
        let entity = self.inner.fs.read(&indexed.path)?;
        if entity.extra.get("parent").and_then(|v| v.as_str()) == Some(&uuid_str) {
            children.push(entity_to_spec(&entity));
        }
    }
    Ok(children)
}

/// Walk the parent chain from a spec to the root.
pub fn ancestors(&self, id_or_slug: &str) -> Result<Vec<SpecManifest>, SpecError> {
    let mut result = Vec::new();
    let mut current = self.get(id_or_slug)?;
    while let Some(parent_str) = current.parent().map(String::from) {
        let parent = self.get(&parent_str)?;
        result.push(parent.clone());
        current = parent;
    }
    Ok(result)
}

/// BFS all descendants of a spec.
pub fn subtree(&self, id_or_slug: &str) -> Result<Vec<SpecManifest>, SpecError> {
    let uuid = self.resolve_id(id_or_slug)?;
    let mut result = Vec::new();
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(uuid);
    while let Some(current_id) = queue.pop_front() {
        let uuid_str = current_id.to_string();
        let all = self.inner.list_indexed(false)?;
        for indexed in all {
            let entity = self.inner.fs.read(&indexed.path)?;
            if entity.extra.get("parent").and_then(|v| v.as_str()) == Some(&uuid_str) {
                let spec = entity_to_spec(&entity);
                queue.push_back(spec.id);
                result.push(spec);
            }
        }
    }
    Ok(result)
}
```

### Step 9: Conversion helpers

```rust
fn spec_to_entity(spec: &SpecManifest) -> EntityManifest {
    EntityManifest {
        id: spec.id,
        created_at: spec.created_at,
        extra: spec.extra.clone(),
    }
}

fn entity_to_spec(entity: &EntityManifest) -> SpecManifest {
    SpecManifest {
        id: entity.id,
        created_at: entity.created_at,
        code_refs: Vec::new(),  // code_refs stored in extra, need special handling
        extra: entity.extra.clone(),
    }
}

fn read_body(spec_path: &Path) -> String {
    let body_path = spec_path.join("body.md");
    fs::read_to_string(&body_path).unwrap_or_default()
}

fn write_body(spec_path: &Path, content: &str) -> Result<(), SpecError> {
    let body_path = spec_path.join("body.md");
    fs::write(&body_path, content).map_err(|e| SpecError::Storage(StorageError::Io(e)))
}
```

**NOTE on `code_refs`**: SpecManifest has `code_refs: Vec<CodeRef>` with `#[serde(default, skip_serializing_if = "Vec::is_empty")]`. When reading from EntityManifest (which doesn't have this typed field), the code_refs will be in the `extra` map as a JSON value. The `entity_to_spec` conversion needs to handle this — either by:
- (a) Parsing `code_refs` from `extra` if present and moving to the typed field
- (b) Letting serde handle it if SpecManifest is deserialized directly from TOML (not via EntityManifest)

Since EntityFs reads into EntityManifest (not SpecManifest), you need approach (a) or you need to read the TOML directly into SpecManifest instead of going through EntityManifest. Consider adding a `read_spec` method that reads `spec.toml` directly into `SpecManifest` using `toml::from_str`.

### Step 10: Register in lib.rs

Add to `crates/spec-api/src/lib.rs`:
```rust
pub mod store;
pub use store::SpecStore;
```

### Step 11: Add error variants if needed

Check if `SpecError` needs additional variants for state transition errors. The `SchemaRegistry::ensure_transition` returns `StorageError`, which already converts via `From<StorageError>`.

You may need:
```rust
#[error("invalid state transition: {0}")]
InvalidTransition(String),
```

### Step 12: Tests

Write integration tests in `crates/spec-api/src/store.rs` or a separate `tests/` file:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn setup() -> (TempDir, SpecStore) {
        let tmp = TempDir::new().unwrap();
        let mut store = SpecStore::open(tmp.path()).unwrap();
        let root = tmp.path().join("specs");
        std::fs::create_dir_all(&root).unwrap();
        store.entity_store().add_scan_root(ScanRoot {
            path: root,
            label: "test".into(),
        }).unwrap();
        (tmp, store)
    }

    #[test]
    fn test_create_and_get() {
        let (tmp, mut store) = setup();
        let manifest = SpecManifest::new("test/example", "Example Spec", "test-component");
        let id = store.create(&manifest, "# Example\n\nBody text.", None).unwrap();

        let retrieved = store.get(&id.to_string()).unwrap();
        assert_eq!(retrieved.slug(), Some("test/example"));
        assert_eq!(retrieved.title(), Some("Example Spec"));
    }

    #[test]
    fn test_get_full_with_body() {
        let (tmp, mut store) = setup();
        let manifest = SpecManifest::new("test/body", "Body Test", "comp");
        store.create(&manifest, "Hello body", None).unwrap();

        let (spec, body) = store.get_full("test/body").unwrap();
        assert_eq!(body, "Hello body");
    }

    #[test]
    fn test_slug_resolution() {
        let (tmp, mut store) = setup();
        let manifest = SpecManifest::new("api/storage", "Storage", "api");
        let id = store.create(&manifest, "body", None).unwrap();

        // Resolve by slug
        assert_eq!(store.resolve_id("api/storage").unwrap(), id);
        // Resolve by full UUID
        assert_eq!(store.resolve_id(&id.to_string()).unwrap(), id);
    }

    #[test]
    fn test_duplicate_slug_rejected() {
        let (tmp, mut store) = setup();
        let m1 = SpecManifest::new("unique/slug", "Spec 1", "comp");
        store.create(&m1, "body", None).unwrap();

        let m2 = SpecManifest::new("unique/slug", "Spec 2", "comp");
        assert!(store.create(&m2, "body", None).is_err());
    }

    #[test]
    fn test_update_body() {
        let (tmp, mut store) = setup();
        let manifest = SpecManifest::new("test/update", "Update", "comp");
        store.create(&manifest, "initial", None).unwrap();

        store.update_body("test/update", "updated body").unwrap();
        let (_, body) = store.get_full("test/update").unwrap();
        assert_eq!(body, "updated body");
    }

    #[test]
    fn test_sections_crud() {
        let (tmp, mut store) = setup();
        let manifest = SpecManifest::new("test/sections", "Sections", "comp");
        store.create(&manifest, "body", None).unwrap();

        store.add_section("test/sections", "overview", "# Overview").unwrap();
        store.add_section("test/sections", "details", "# Details").unwrap();

        let sections = store.list_sections("test/sections").unwrap();
        assert_eq!(sections, vec!["details.md", "overview.md"]);

        store.update_section("test/sections", "overview", "# Updated Overview").unwrap();
        store.delete_section("test/sections", "details").unwrap();

        let sections = store.list_sections("test/sections").unwrap();
        assert_eq!(sections, vec!["overview.md"]);
    }

    #[test]
    fn test_parent_child_hierarchy() {
        let (tmp, mut store) = setup();
        let parent = SpecManifest::new("api/parent", "Parent", "api");
        let parent_id = store.create(&parent, "parent body", None).unwrap();

        let mut child = SpecManifest::new("api/child", "Child", "api");
        child.set_parent(&parent_id.to_string());
        store.create(&child, "child body", None).unwrap();

        let children = store.children(&parent_id.to_string()).unwrap();
        assert_eq!(children.len(), 1);
        assert_eq!(children[0].slug(), Some("api/child"));

        let ancestors = store.ancestors("api/child").unwrap();
        assert_eq!(ancestors.len(), 1);
        assert_eq!(ancestors[0].slug(), Some("api/parent"));
    }

    #[test]
    fn test_delete_removes_slug() {
        let (tmp, mut store) = setup();
        let manifest = SpecManifest::new("test/delete", "Delete", "comp");
        store.create(&manifest, "body", None).unwrap();

        store.delete("test/delete").unwrap();
        assert!(store.get("test/delete").is_err());
    }
}
```

## Validation

```bash
cargo test -p spec-api
cargo check -p spec-api
```

## Key Constraints

1. **Wrap EntityStore** — do NOT duplicate SQLite/Tantivy/filesystem logic. EntityStore handles persistence; SpecStore adds domain logic.
2. **body.md vs description.md**: EntityFs writes `description.md` on create. SpecStore renames it to `body.md`. For reads, use `body.md` directly (not EntityFs `read_description`).
3. **Slug index is ephemeral** — rebuilt from disk on `scan()`. No persistent slug storage beyond the spec.toml `slug` field.
4. **SpecManifest ↔ EntityManifest conversion** — handle `code_refs` field carefully. Consider reading `spec.toml` directly into `SpecManifest` via `toml::from_str` instead of going through EntityManifest for `get` operations.
5. **State transitions** — delegate to `SchemaRegistry::ensure_transition` from the inner EntityStore's schema registry.
6. **Hierarchy** — the `parent` field stores a UUID string. `children()` scans all specs. For large stores this is O(n); acceptable for now, can be indexed later.
7. **`&mut self`** — methods that modify `slug_index` need `&mut self`. This is fine for single-threaded use. If concurrent access is needed later, wrap in `RwLock`.
