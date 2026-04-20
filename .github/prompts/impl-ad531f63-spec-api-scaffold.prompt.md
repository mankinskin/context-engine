---
description: "Implement ticket ad531f63: spec-api crate scaffold + SpecManifest model"
---

# Ticket ad531f63 — spec-api Crate Scaffold + SpecManifest Model

## Goal

Create the `crates/spec-api/` crate with its Cargo.toml, module structure, and `SpecManifest` model using the same `extra: BTreeMap<String, Value>` pattern as `EntityManifest`/`TicketManifest`.

## Ticket State Management

```bash
# At start:
./target/debug/ticket.exe update ad531f63 --to-state in-implementation
./target/debug/ticket.exe board check-in ad531f63 --agent-id copilot --intent "creating spec-api crate scaffold" --files "crates/spec-api/Cargo.toml,crates/spec-api/src/lib.rs,crates/spec-api/src/manifest.rs" --ttl 3600

# At end (after tests pass):
./target/debug/ticket.exe update ad531f63 --to-state in-review
```

## Context

- `memory-api` at `crates/memory-api/` provides `EntityManifest` with `extra: BTreeMap<String, Value>`
- `ticket-api` follows the same pattern — `TicketManifest` is essentially `EntityManifest` with typed accessors for `title`, `state`, `priority` etc. stored in `extra`
- `SpecManifest` will follow the exact same pattern

## Design Decision (Already Resolved)

SpecManifest uses the `EntityManifest` + `extra: BTreeMap` pattern. All spec-specific fields are stored in the `extra` map and accessed via typed helper methods. This is the same approach as tickets.

## Implementation

### Step 1: Create `crates/spec-api/Cargo.toml`

```toml
[package]
name = "spec-api"
version = "0.1.0"
edition = "2024"
description = "Specification system: manifest model, storage, and domain logic"

[dependencies]
memory-api = { path = "../memory-api" }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
toml = "0.8"
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1", features = ["serde", "v4"] }
thiserror = "2"

[dev-dependencies]
pretty_assertions = "1"
tempfile = "3"
```

### Step 2: Add to workspace Cargo.toml

Add `"crates/spec-api"` to the `[workspace] members` list in the root `Cargo.toml`.

### Step 3: Create `crates/spec-api/src/lib.rs`

```rust
pub mod error;
pub mod manifest;

pub use manifest::SpecManifest;
```

### Step 4: Create `crates/spec-api/src/error.rs`

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SpecError {
    #[error("spec not found: {0}")]
    NotFound(String),

    #[error("invalid slug: {0}")]
    InvalidSlug(String),

    #[error("duplicate slug: {0}")]
    DuplicateSlug(String),

    #[error("storage error: {0}")]
    Storage(#[from] memory_api::error::StorageError),

    #[error("serialization error: {0}")]
    Serialization(String),
}
```

### Step 5: Create `crates/spec-api/src/manifest.rs`

Study how `TicketManifest` works in ticket-api. Look at:
- `crates/ticket-api/src/model/ticket.rs` — how TicketManifest wraps EntityManifest
- How typed accessor methods (`.title()`, `.state()`, `.priority()`) read from `extra`

Then create `SpecManifest` with the same pattern:

```rust
use memory_api::model::entity::EntityManifest;
use serde_json::Value;
use uuid::Uuid;

/// A specification manifest — metadata about a spec stored in spec.toml.
///
/// Uses the same `extra: BTreeMap<String, Value>` storage pattern as
/// EntityManifest/TicketManifest. Spec-specific fields are stored in
/// the extra map and accessed via typed methods.
pub struct SpecManifest {
    inner: EntityManifest,
}

impl SpecManifest {
    /// Create a new spec manifest with required fields.
    pub fn new(slug: &str, title: &str, component: &str) -> Self {
        let mut inner = EntityManifest::new();  // check actual constructor
        inner.set("slug", slug);
        inner.set("title", title);
        inner.set("component", component);
        inner.set("type", "specification");
        inner.set("state", "draft");
        Self { inner }
    }

    // ── typed accessors ──

    pub fn id(&self) -> Uuid { self.inner.id }
    pub fn slug(&self) -> Option<&str> { self.inner.get_str("slug") }
    pub fn title(&self) -> Option<&str> { self.inner.get_str("title") }
    pub fn state(&self) -> Option<&str> { self.inner.get_str("state") }
    pub fn component(&self) -> Option<&str> { self.inner.get_str("component") }
    pub fn scope(&self) -> Option<&str> { self.inner.get_str("scope") }
    pub fn parent(&self) -> Option<&str> { self.inner.get_str("parent") }

    // ── setters ──

    pub fn set_slug(&mut self, slug: &str) { self.inner.set("slug", slug); }
    pub fn set_title(&mut self, title: &str) { self.inner.set("title", title); }
    pub fn set_state(&mut self, state: &str) { self.inner.set("state", state); }
    pub fn set_component(&mut self, comp: &str) { self.inner.set("component", comp); }
    pub fn set_scope(&mut self, scope: &str) { self.inner.set("scope", scope); }
    pub fn set_parent(&mut self, parent: &str) { self.inner.set("parent", parent); }

    /// Access the underlying EntityManifest.
    pub fn as_entity(&self) -> &EntityManifest { &self.inner }
    pub fn into_entity(self) -> EntityManifest { self.inner }
    pub fn from_entity(inner: EntityManifest) -> Self { Self { inner } }
}
```

**IMPORTANT**: The accessor pattern above is approximate. Before implementing, read `crates/ticket-api/src/model/ticket.rs` (or the memory-api EntityManifest) to see the ACTUAL accessor API (`.get_str()`, `.set()`, etc). Match whatever pattern exists.

### Step 6: Implement Serde

SpecManifest should serialize to/from TOML as `spec.toml`. Make sure it round-trips:

```rust
// Serialize: SpecManifest → TOML string
// Deserialize: TOML string → SpecManifest

impl Serialize for SpecManifest { ... }  // or derive, or delegate to inner
impl Deserialize for SpecManifest { ... }
```

If EntityManifest already derives Serialize/Deserialize, just delegate.

### Step 7: Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_spec_manifest() {
        let m = SpecManifest::new("ticket-api/store", "TicketStore", "ticket-api");
        assert_eq!(m.slug(), Some("ticket-api/store"));
        assert_eq!(m.title(), Some("TicketStore"));
        assert_eq!(m.component(), Some("ticket-api"));
        assert_eq!(m.state(), Some("draft"));
    }

    #[test]
    fn test_serde_round_trip() {
        let m = SpecManifest::new("ticket-api/store", "TicketStore", "ticket-api");
        let toml_str = toml::to_string_pretty(m.as_entity()).unwrap();
        let parsed: EntityManifest = toml::from_str(&toml_str).unwrap();
        let m2 = SpecManifest::from_entity(parsed);
        assert_eq!(m2.slug(), Some("ticket-api/store"));
        assert_eq!(m2.title(), Some("TicketStore"));
    }

    #[test]
    fn test_set_parent() {
        let mut m = SpecManifest::new("ticket-api/store/create", "create", "ticket-api");
        let parent_id = uuid::Uuid::new_v4().to_string();
        m.set_parent(&parent_id);
        assert_eq!(m.parent(), Some(parent_id.as_str()));
    }
}
```

## Validation

```bash
cargo test -p spec-api
cargo check --workspace  # ensure no workspace breakage
```

## Key Constraints

- Follow the EXACT same pattern as ticket-api's TicketManifest
- Do NOT add storage, slug validation, or folder structure — those are separate tickets
- Do NOT add CodeRef or FeatureStatus — those are separate tickets (55a1b302, c4c9e9d4)
- Keep scope minimal: Cargo.toml + lib.rs + error.rs + manifest.rs + tests
