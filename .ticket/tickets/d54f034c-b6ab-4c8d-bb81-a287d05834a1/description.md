# Simplify ticket state machine: drop in-refinement and in-validation

## Problem

The current `tracker-improvement` state machine has **8 states** with 6
intermediate states between creation and completion. Two of these states add
ceremony without proportional value:

1. **`in-refinement`**: Separates "new" from "ready" but in practice tickets
   are either new (not yet triaged) or ready (actionable). Refinement is part
   of the new→ready transition, not a distinct durable state.
2. **`in-validation`**: Separates "in-review" from "done" but review already
   implies validation. Acceptance criteria should be checked as part of the
   review, not as a separate state gate.

### Current state machine (8 states):
```
new → in-refinement → ready → in-implementation → in-review → in-validation → done
                                                                              ↗
                                    (any non-terminal) ──────────────→ cancelled
```

### Proposed state machine (6 states):
```
new → ready → in-implementation → in-review → done
                                              ↗
                  (any non-terminal) → cancelled
```

## Motivation

- **Reduced friction**: Agents and humans skip states or fast-forward through
  them. Fewer states = fewer accidental missteps.
- **Clearer semantics**: Each state has a distinct, unambiguous meaning.
- **Easier tooling**: The VS Code extension, MCP tools, and CLI all benefit
  from a smaller state set.
- **Better enforcement**: With fewer states, `required_states = ["in-review"]`
  remains the key quality gate, and it's harder to rubber-stamp through.

## Proposed State Definitions

| State | Meaning | Entry Condition |
|-------|---------|----------------|
| `new` | Ticket created, not yet triaged/ready | Default on create |
| `ready` | Requirements clear, actionable | Triage/refinement complete |
| `in-implementation` | Active development work | Developer starts coding |
| `in-review` | Code review + acceptance criteria validation | Implementation complete |
| `done` | All criteria met, review passed | Review approved |
| `cancelled` | No longer relevant | Explicit cancellation |

## Proposed Transitions

```toml
[[transitions]]
from = "new"
to = "ready"

[[transitions]]
from = "new"
to = "cancelled"

[[transitions]]
from = "ready"
to = "in-implementation"

[[transitions]]
from = "ready"
to = "new"

[[transitions]]
from = "ready"
to = "cancelled"

[[transitions]]
from = "in-implementation"
to = "in-review"

[[transitions]]
from = "in-implementation"
to = "cancelled"

[[transitions]]
from = "in-review"
to = "done"

[[transitions]]
from = "in-review"
to = "in-implementation"

[[transitions]]
from = "in-review"
to = "cancelled"
```

## Acceptance Criteria

- [ ] AC1: `tracker-improvement.toml` schema updated to 6 states with
      transitions as specified above.
- [ ] AC2: `required_states = ["in-review"]` preserved — tickets must pass
      through review before done.
- [ ] AC3: Existing tickets in `in-refinement` state are migrated to `new`
      (or `ready` if refinement was complete).
- [ ] AC4: Existing tickets in `in-validation` state are migrated to
      `in-review` (or `done` if validation was complete).
- [ ] AC5: All references to removed states updated in:
      - `.agents/instructions/ticket-system.instructions.md`
      - `AGENTS.md`
      - `.agents/prompts/` files
      - CLI help text and examples
- [ ] AC6: `ticket scan --force` succeeds after migration with no orphaned
      state references.
- [ ] AC7: E2E tests validate the full lifecycle through the simplified
      state machine (new → ready → in-implementation → in-review → done).
- [ ] AC8: E2E tests validate rejection of removed states
      (`in-refinement`, `in-validation`).
- [ ] AC9: E2E tests validate that `close` fast-forward from any state
      correctly walks through `in-review` → `done`.
- [ ] AC10: User must explicitly approve the migration plan before
       execution (review gate).

## Implementation Plan

### Phase 1: Schema Update

1. Edit `crates/ticket-api/schemas/tracker-improvement.toml`:
   - Remove `in-refinement` and `in-validation` from `states`
   - Update `transitions` to the new set
   - Keep `required_states = ["in-review"]`
   - Keep `terminal_states = ["done", "cancelled"]` (implicit default)

2. Update `TicketTypeSchema` validation in `crates/ticket-api/src/model/schema.rs`
   if any hardcoded references to removed states exist.

### Phase 2: Migration

1. Create a migration script or CLI command:
   ```bash
   # Find tickets in removed states
   ticket list --where state=in-refinement --json
   ticket list --where state=in-validation --json

   # Migrate: in-refinement → new (safe default)
   # Migrate: in-validation → in-review (was already reviewed)
   ```

2. For each affected ticket, use `update --to-state` to move to the
   replacement state. Document the migration in ticket history.

3. Run `ticket scan --force` to reconcile indexes.

### Phase 3: Documentation Updates

Update all documentation referencing the old states:

- `.agents/instructions/ticket-system.instructions.md`
  - State transition table
  - Review gate instructions
  - CLI examples
- `AGENTS.md` — remove references to `in-refinement`, `in-validation`
- `.agents/prompts/*.prompt.md` — update any state references

### Phase 4: E2E Testing

#### Test infrastructure

Use the wdio-vscode-service framework (shared with the tree view ticket)
or cargo integration tests for CLI/API level:

```rust
#[test]
fn test_simplified_lifecycle() {
    // new → ready → in-implementation → in-review → done
    let store = create_test_store();
    let id = store.create("tracker-improvement", "test ticket");
    store.update(id, "ready").unwrap();
    store.update(id, "in-implementation").unwrap();
    store.update(id, "in-review").unwrap();
    store.close(id).unwrap();
    assert_eq!(store.get(id).state, "done");
}

#[test]
fn test_removed_states_rejected() {
    let store = create_test_store();
    let id = store.create("tracker-improvement", "test");
    let result = store.update(id, "in-refinement");
    assert!(result.is_err()); // InvalidTransition
}

#[test]
fn test_fast_forward_through_review() {
    let store = create_test_store();
    let id = store.create("tracker-improvement", "test");
    store.update(id, "ready").unwrap();
    store.update(id, "in-implementation").unwrap();
    store.close(id).unwrap();
    // Should have walked: in-implementation → in-review → done
    let history = store.get_history(id);
    assert!(history.contains_state("in-review"));
}

#[test]
fn test_required_states_enforced() {
    let store = create_test_store();
    let id = store.create("tracker-improvement", "test");
    store.update(id, "ready").unwrap();
    store.update(id, "in-implementation").unwrap();
    // Try to go directly to done (skipping in-review)
    let result = store.update(id, "done"); // No direct transition exists
    assert!(result.is_err());
}
```

#### CLI integration tests

```bash
# Full lifecycle
ticket create --title "lifecycle test" --type tracker-improvement --json
ticket update <id> --to-state ready --json
ticket update <id> --to-state in-implementation --json
ticket update <id> --to-state in-review --json
ticket close <id> --json

# Rejection of old states
ticket update <id> --to-state in-refinement --json  # → error
ticket update <id> --to-state in-validation --json  # → error
```

### Phase 5: User Review Gate

Before merging, present the full migration plan to the user:
- List of affected tickets with current → new state mapping
- Diff of schema changes
- Documentation updates summary
- Test results

## Risk Assessment

- **Medium**: Existing tickets in removed states need migration. Data loss
  risk mitigated by using state transitions (preserves history) rather than
  raw edits.
- **Low**: Schema change is backward-compatible for tickets already in valid
  states (new, ready, in-implementation, in-review, done, cancelled).
- **Medium**: Documentation spread — many files reference the old states.
  Grep search required to find all references.
- **Low**: `close` fast-forward via `find_path()` BFS will automatically
  adapt to the new transition graph.

## Dependency

This ticket should be completed **before** the tree view ticket
(`5bf1951a-dce4-4efb-80d6-89fe4fa01573`) since the tree view will
dynamically load schema states. Simplifying the schema first means the
tree view immediately benefits from fewer, clearer states.

## Files to Modify

- `crates/ticket-api/schemas/tracker-improvement.toml` — schema update
- `crates/ticket-api/src/model/schema.rs` — verify no hardcoded state refs
- `crates/ticket-api/src/storage/store.rs` — verify transition logic
- `.agents/instructions/ticket-system.instructions.md` — doc update
- `AGENTS.md` — doc update
- `.agents/prompts/*.prompt.md` — doc updates
- Migration script or batch commands for existing tickets
