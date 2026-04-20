---
description: "Implement ticket 4b6dc9d5: spec-api schema — specification lifecycle state machine"
---

# Ticket 4b6dc9d5 — spec-api Schema: Lifecycle State Machine

## Goal

Create the `specification.toml` schema file for spec-api that defines the specification lifecycle state machine (draft → reviewed → approved → implemented → verified) with required states enforcement.

## Ticket State Management

```bash
# At start:
./target/debug/ticket.exe update 4b6dc9d5 --to-state in-implementation
./target/debug/ticket.exe board check-in 4b6dc9d5 --agent-id copilot --intent "creating specification schema TOML" --files "crates/spec-api/schemas/specification.toml" --ttl 3600

# At end (after tests pass):
./target/debug/ticket.exe update 4b6dc9d5 --to-state in-review
```

## Context

- `spec-api` crate exists at `crates/spec-api/` with `manifest.rs`, `error.rs`, `lib.rs`
- The existing ticket schema is at `crates/ticket-api/schemas/tracker-improvement.toml` — use it as the reference template
- `memory-api` has `SchemaRegistry` in `crates/memory-api/src/model/schema_registry.rs` that loads these TOML files
- The schema enforces states, transitions, required_states, fields, and edge_rules

## State Machine

```
draft → reviewed → approved → implemented → verified
  ↓        ↓          ↓           ↓
cancelled cancelled  cancelled  cancelled
                      ↓
                   deprecated
```

### Required States

`required_states = ["reviewed", "approved"]` — specs cannot reach terminal state (`verified`) without having visited `reviewed` AND `approved` in their history.

## Implementation

### Step 1: Create `crates/spec-api/schemas/specification.toml`

Model this EXACTLY after `crates/ticket-api/schemas/tracker-improvement.toml`. That file uses:
- `type_id` — the type identifier string
- `states` — ordered list of all valid states
- `required_states` — states that must appear in history before terminal
- `[[transitions]]` — pairs of `from`/`to` defining valid state transitions
- `[fields.<name>]` — field definitions with `field_type` and `required`
- `[edge_rules.<kind>]` — edge kind rules with `directed` and `acyclic_enforced`

```toml
type_id = "specification"

states = [
    "draft",
    "reviewed",
    "approved",
    "implemented",
    "verified",
    "deprecated",
    "cancelled",
]

required_states = ["reviewed", "approved"]

# ── Transitions ──

# From draft
[[transitions]]
from = "draft"
to = "reviewed"

[[transitions]]
from = "draft"
to = "cancelled"

# From reviewed
[[transitions]]
from = "reviewed"
to = "approved"

[[transitions]]
from = "reviewed"
to = "draft"

[[transitions]]
from = "reviewed"
to = "cancelled"

# From approved
[[transitions]]
from = "approved"
to = "implemented"

[[transitions]]
from = "approved"
to = "deprecated"

[[transitions]]
from = "approved"
to = "cancelled"

# From implemented
[[transitions]]
from = "implemented"
to = "verified"

[[transitions]]
from = "implemented"
to = "approved"

[[transitions]]
from = "implemented"
to = "cancelled"

# From verified (terminal — only deprecation)
[[transitions]]
from = "verified"
to = "deprecated"

# ── Fields ──

[fields.title]
field_type = "string"
required = true

[fields.slug]
field_type = "string"
required = true

[fields.type]
field_type = "string"
required = true

[fields.state]
field_type = "string"
required = false

[fields.component]
field_type = "string"
required = false

[fields.scope]
field_type = "string"
required = false

[fields.parent]
field_type = "string"
required = false

# ── Edge rules ──

[edge_rules.depends_on]
directed = true
acyclic_enforced = true

[edge_rules.linked]
directed = false
acyclic_enforced = false

[edge_rules.parent_of]
directed = true
acyclic_enforced = true
```

### Step 2: Register schema in spec-api

Check how ticket-api loads its schemas. Look at:
- `crates/ticket-api/src/model/default_schema.rs` or similar
- How `SchemaRegistry::with_builtins()` includes the tracker-improvement schema

For spec-api, you may need to:
1. Create a `default_schema.rs` that embeds the TOML via `include_str!`
2. Provide a `spec_schema_registry()` function or equivalent
3. Or, if SchemaRegistry supports loading from a directory, just point it at `schemas/`

### Step 3: Add test

```rust
#[test]
fn test_specification_schema_loads() {
    let toml_content = include_str!("../schemas/specification.toml");
    // Parse using SchemaRegistry's schema parsing
    // Verify states, transitions, required_states, fields, edge_rules
}
```

Also test that:
- All transitions are between valid states
- required_states are a subset of non-terminal states
- The `parent_of` edge kind is present (this is new vs ticket schemas)

## Validation

```bash
cargo test -p spec-api
cargo check -p spec-api
```

## Key Constraints

- Follow the EXACT same TOML format as `crates/ticket-api/schemas/tracker-improvement.toml`
- The `parent_of` edge kind is NEW (tickets don't have it) — this enables hierarchical spec relationships
- `deprecated` is reachable from `approved`, `verified` — not from early states
- Do NOT implement state machine enforcement logic — that's in memory-api's SchemaRegistry already
