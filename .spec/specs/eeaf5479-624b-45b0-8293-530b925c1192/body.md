## Summary

Ticket↔spec traceability is currently prose-only: `spec.toml` files carry an
untyped `related_tickets`/`ticket_ids` array of bare strings (sometimes a raw
UUID, sometimes a relative path such as `../../.ticket/...`), and
`TicketManifest` (an alias of `EntityManifest`) has no reciprocal field at
all. Nothing validates these links, so they rot silently and can resolve to
the wrong store (the nested-store bug: a relative path from
`memory-api/.spec/...` resolving against the root `.ticket/` store instead of
`memory-api/.ticket/`).

This spec defines the structured `TicketRef` / `SpecRef` schema, the typed
manifest fields that replace the untyped `related_tickets`/`ticket_ids`
arrays, the `validate-links` CLI contract for both `ticket` and `spec` CLIs,
the detection rules for dangling/wrong-store/bidirectional link defects, the
prose→structured migration approach, and the required test matrix.

Implements: [e82b4f88 Structured ticket↔spec linking with validation to replace prose-only references](../../../.ticket/tickets/e82b4f88-45e1-402b-ab59-de845c4930e0/ticket.toml)

## Scope

In scope:
- A `TicketRef` struct (spec-api) and `SpecRef` struct (ticket-api/memory-api)
  that each carry an explicit store identifier alongside the entity id.
- A typed `related_tickets: Vec<TicketRef>` field on `SpecManifest`.
- A typed `related_specs: Vec<SpecRef>` field on `TicketManifest`
  (`EntityManifest`).
- `ticket validate-links --workspace <ws>` and
  `spec validate-links --workspace <ws>` CLI commands (plus MCP-equivalent
  tool surfaces) that detect: dangling ticket refs, dangling spec refs,
  wrong-store refs, and bidirectional inconsistencies.
- A migration guide for converting existing untyped `related_tickets` /
  `ticket_ids` prose arrays into the structured field.
- The unit and integration test matrix that proves the schema, validator, and
  migration behave correctly, including a regression test for the
  nested-store bug.

Out of scope (non-goals):
- Automatic/background link repair (validation reports; it does not silently
  rewrite links).
- Changing `code_refs` (file/symbol code links) — this spec only covers
  ticket↔spec entity links.
- A generalized N-way entity-linking framework across every store type
  (doc-api, log-api, feedback-api, etc.); those remain prose/ad-hoc until a
  follow-on spec extends this pattern.
- UI/viewer rendering of the new fields (ticket-viewer/spec-viewer surfacing
  is follow-on work, not gated by this spec).

## Current State (baseline, verified)

- `SpecManifest` (`memory-api/crates/spec-api/src/manifest.rs`): `id`,
  `created_at`, `code_refs: Vec<CodeRef>`, `#[serde(flatten)] extra:
  BTreeMap<String, Value>`. No typed ticket-link field; `related_tickets` /
  `ticket_ids` only exist today as untyped entries inside `extra`, with
  observed values that are inconsistently either bare UUIDs or relative
  paths (see [memory-api/.spec/specs/76da5f2d-cea9-49d9-b223-730a0c2a5d6b/spec.toml](../../../memory-api/.spec/specs/76da5f2d-cea9-49d9-b223-730a0c2a5d6b/spec.toml)
  vs. [.spec/specs/0d205a9e-3add-40e7-a148-b7e2e65b260f/spec.toml](../../../.spec/specs/0d205a9e-3add-40e7-a148-b7e2e65b260f/spec.toml)).
- `EntityManifest` (`memory-api/crates/memory-api/src/model/entity.rs`,
  aliased as `TicketManifest` in `memory-api/crates/ticket-api/src/model/ticket.rs`):
  `id`, `created_at`, `#[serde(flatten)] extra`. No spec-link field of any
  kind, typed or untyped.
- No `validate-links` command exists in either the `ticket` or `spec` CLI.
- No `TicketRef` or `SpecRef` struct exists in the codebase today.

## TicketRef / SpecRef Schema

Both structs live next to the manifest type that consumes them and must
serde round-trip through TOML without loss.

```rust
/// A structured reference to a ticket, carried on SpecManifest.related_tickets.
/// Lives in spec-api (crate that owns SpecManifest).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TicketRef {
    /// Ticket UUID (EntityId / TicketId).
    pub ticket_id: Uuid,
    /// Named workspace the ticket store belongs to (matches ticket-api
    /// workspace resolution, e.g. "default", "memory-api").
    pub workspace: String,
    /// Store root the ticket resolves against, as a path relative to the
    /// repository root (e.g. ".ticket", "memory-api/.ticket"). Never a
    /// path relative to the referencing spec file.
    pub store_root: String,
}

/// A structured reference to a spec, carried on TicketManifest.related_specs.
/// Lives in memory-api (crate that owns EntityManifest) or ticket-api,
/// mirroring TicketRef's shape so validators can treat both symmetrically.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SpecRef {
    /// Spec UUID (SpecId).
    pub spec_id: Uuid,
    pub workspace: String,
    pub store_root: String,
}
```

Design decisions:
- `store_root` is always repo-root-relative, never relative to the
  referencing file. This is the direct fix for the nested-store bug, where a
  path relative to `memory-api/.spec/...` silently resolved against the
  wrong store.
- `workspace` and `store_root` are both required (not optional) so a
  reference is self-describing without needing to probe the filesystem to
  discover which store it belongs to.
- Field names (`ticket_id`/`spec_id` vs. a shared `id`) are kept
  type-specific rather than generic, so a `TicketRef` can never be
  accidentally constructed from spec data or vice versa.

## Structured Manifest Fields (as shipped)

**Correction (verified against the merged e82b4f88 implementation):** neither
manifest gained a literal `Vec<TicketRef>`/`Vec<SpecRef>` struct field with
its own `#[serde]` attributes. Both manifests already store all
non-hard-coded metadata inside a flattened `extra: BTreeMap<String, Value>`
(the same pattern `code_refs` predates), so the typed link is exposed as a
**typed accessor pair over `extra`**, not a new physical field:

```rust
// spec-api: memory-api/crates/spec-api/src/manifest.rs
pub struct SpecManifest {
    pub id: SpecId,
    pub created_at: DateTime<Utc>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub code_refs: Vec<CodeRef>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

impl SpecManifest {
    /// Parses the `related_tickets` extra key (never errors; empty vec if absent/malformed).
    pub fn related_tickets(&self) -> Vec<TicketRef> { self.parse_vec_field("related_tickets") }
    /// Also reads the legacy untyped `related_tickets`/`ticket_ids` prose keys during migration.
    pub fn related_tickets_and_legacy(&self) -> Vec<TicketRef> { /* ... */ }
    /// Writes `related_tickets`, removing the key entirely when empty.
    pub fn set_related_tickets(&mut self, related_tickets: Vec<TicketRef>) { /* ... */ }
}
```

```rust
// memory-api: memory-api/crates/memory-api/src/model/entity.rs
pub struct EntityManifest {
    pub id: EntityId,
    pub created_at: DateTime<Utc>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

impl EntityManifest {
    /// Parses the `related_specs` extra key (never errors; empty vec if absent/malformed).
    pub fn related_specs(&self) -> Vec<SpecRef> { /* ... */ }
    /// Writes `related_specs`, removing the key entirely when empty.
    pub fn set_related_specs(&mut self, related_specs: Vec<SpecRef>) -> Result<(), ...> { /* ... */ }
}
```

`TicketManifest` (`memory-api/crates/ticket-api/src/model/ticket.rs`) is a
type alias of `EntityManifest`, so it inherits `related_specs()` /
`set_related_specs()` directly.

Both accessors return an empty `Vec` (never an error) when the key is
absent or malformed, and `set_*` removes the key entirely when the vec is
empty — preserving backward compatibility with manifests that predate this
change. Existing untyped `related_tickets`/`ticket_ids` keys remain readable
inside `extra` until migrated (see Migration below); the validator treats
their presence as a migration signal, not an error, during a transition
window.

## `validate-links` CLI Contract

Two symmetric commands, one per store type:

```bash
ticket validate-links --workspace <ws> [--json]
spec validate-links --workspace <ws> [--json]
```

Behavior:
- Resolves every `related_specs` entry (ticket side) / `related_tickets`
  entry (spec side) via `(workspace, store_root, id)`, not id alone.
- Exit code is non-zero if any finding is reported; `--json` emits a
  structured report (list of findings + counts) suitable for CI and for an
  MCP `validate_ticket_spec_links`-equivalent tool wrapping the same report
  type.
- Read-only: this command never mutates a manifest. Fixing a finding is a
  separate, explicit edit.

## Detection Rules

The validator must detect and report each of the following distinctly
(distinguishable `kind` in the JSON report, not merged into one generic
"broken link" bucket):

1. **Dangling ticket ref** — a `SpecManifest.related_tickets` entry whose
   `(workspace, store_root, ticket_id)` does not resolve to an existing
   ticket in that store.
2. **Dangling spec ref** — a `TicketManifest.related_specs` entry whose
   `(workspace, store_root, spec_id)` does not resolve to an existing spec
   in that store.
3. **Wrong-store ref** — a reference whose `id` resolves to a real
   ticket/spec, but under a *different* `store_root` than the one recorded
   on the reference (the id exists, just not where the reference claims).
   This is the structural regression test for the nested-store bug.
4. **Bidirectional inconsistency** — ticket A's `related_specs` names spec
   B, but spec B's `related_tickets` does not name ticket A (or the
   reverse). Reported as a distinct finding kind so it can be triaged
   separately from hard dangling/wrong-store errors (a one-directional link
   may be intentional in some workflows, but must be visible).

## Migration Guide (Prose → Structured)

1. **Inventory**: run `spec validate-links --workspace <ws> --json` before
   any schema change to snapshot the current untyped `related_tickets` /
   `ticket_ids` entries per spec (values already observed in the wild: bare
   UUIDs, repo-root-relative ticket.toml paths, and spec-relative
   `../../.ticket/...` paths).
2. **Classify each entry**: bare UUID → resolve against the spec's own
   workspace store by id lookup; path-shaped entry → resolve the path
   against the repository root (not the referencing file) to recover the
   real store root, then extract the id from the resolved `ticket.toml`.
3. **Materialize `TicketRef`/`SpecRef`**: for each classified entry, write a
   structured `{ ticket_id, workspace, store_root }` (or `SpecRef`
   equivalent) into the new typed field.
4. **Remove the untyped key** from `extra` once every entry for that
   manifest has a structured replacement, so the manifest carries the
   typed field only.
5. **Re-run validation**: `ticket validate-links` / `spec validate-links`
   must report zero dangling/wrong-store findings for the migrated set
   (the 10 tickets / 8 specs from the prior mapping session are the
   reference dataset for this pass).
6. **Backfill the reverse edge**: for every migrated `related_tickets`
   entry, ensure the target ticket's `related_specs` gains the
   corresponding `SpecRef` (and vice versa) so bidirectional-inconsistency
   findings are cleared, not just suppressed.

This procedure is documentation-only in this spec (a written guide); the
migration script/tool itself is an implementation-ticket concern, not a
spec-level requirement, beyond it existing and being validated by the
integration test below.

## Test Matrix

Unit tests (`cargo test -p spec-api`, `cargo test -p ticket-api`):
- Serde round-trip: `TicketRef` and `SpecRef` survive a TOML
  serialize→deserialize cycle with all fields intact, including when
  embedded in `SpecManifest`/`EntityManifest`.
- Validation detection: one test per detection rule (dangling ticket ref,
  dangling spec ref, wrong-store ref, bidirectional inconsistency) using
  minimal fixture stores, each asserting the specific finding `kind` is
  reported and no other kind is falsely triggered.
- Cross-store scenarios: a reference whose `store_root` points at a
  workspace-scoped store (e.g. `memory-api/.ticket`) resolves correctly
  when validated from a different working directory / root store context.

Integration test:
- Reproduces the nested-store bug scenario directly: construct a spec in
  one store with a `related_tickets` entry whose `store_root` is
  deliberately the sibling nested store, run `spec validate-links`, and
  assert the wrong-store finding fires where today's prose-path resolution
  would have silently resolved to the incorrect ticket. This is the
  regression guard that proves the fix, not just the schema.

## Acceptance Criteria

- AC1 — `SpecManifest::related_tickets() -> Vec<TicketRef>` (extra-backed
  typed accessor, not a literal serde field) exists, `TicketRef` carries a
  workspace/store identifier. See "Structured Manifest Fields (as shipped)"
  and "TicketRef / SpecRef Schema".
- AC2 — `TicketManifest` (`EntityManifest`) has
  `related_specs() -> Vec<SpecRef>` (same extra-backed accessor pattern),
  `SpecRef` carries a workspace/store identifier. See "Structured Manifest
  Fields (as shipped)" and "TicketRef / SpecRef Schema".
- AC3 — `ticket validate-links --workspace <ws>` and
  `spec validate-links --workspace <ws>` exist. See "`validate-links` CLI
  Contract".
- AC4 — Validation detects dangling ticket refs, dangling spec refs,
  wrong-store refs, bidirectional inconsistencies. See "Detection Rules".
- AC5 — Migration guide documents prose→structured conversion. See
  "Migration Guide (Prose → Structured)".
- AC6 — Unit tests cover serde round-trip, validation detection,
  cross-store scenarios. See "Test Matrix" (Unit tests).
- AC7 — Integration test reproduces the nested-store bug scenario and
  confirms the fix. See "Test Matrix" (Integration test).

## Related Specs

- [spec-api/manifest](../../../.spec/specs/226ff55f-eebf-43b8-aa1e-5abf81b99101/spec.toml) —
  current `SpecManifest` shape this spec extends with `related_tickets`.
- [spec-api/code-ref](../../../.spec/specs/32eaa05c-cef6-4a3b-b506-b5a5410a4674/spec.toml) —
  sibling reference type (`CodeRef`) this spec's `TicketRef`/`SpecRef`
  pattern is modeled after (structured reference type living beside the
  manifest that consumes it).

## Related Tickets

- [e82b4f88 Structured ticket↔spec linking with validation to replace prose-only references](../../../.ticket/tickets/e82b4f88-45e1-402b-ab59-de845c4930e0/ticket.toml) —
  primary implementation ticket this spec defines the contract for.
- [fb14754e Carry verified physical repo paths in handoff packages and delegation prompts](../../../.ticket/tickets/fb14754e-2be8-40a5-a995-488842ba6367/ticket.toml) —
  related store-ownership issue (handoff packages) using the same
  workspace/store-identifier principle; not a dependency, but shares the
  "always carry the owning store, not just an id" design constraint.
