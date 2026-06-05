---
description: "Use when editing ticket system crates/tools. Covers state transitions, storage/search boundaries, and workflow expectations."
applyTo: "**"
---

## Scope

Applies to:
- `crates/ticket-api/`
- `tools/ticket-cli/`
- `tools/ticket-http/`
- `tools/ticket-mcp/`
- `tools/ticket-viewer/`

## Design Constraints

- Respect ticket lifecycle/state machine invariants.
- Keep storage/index behavior backward compatible unless explicitly requested.
- Preserve clear separation between API, storage, transport, and UI layers.

## Ticket Quality — Standing Obligations

These rules apply during **every session**, not only when working on ticket-system code.

### Orientation (start of every session)

Before writing any code, run a quick orientation to understand the current ticket landscape:

# Check the draftboard (active agents, WIP limit, stale warnings)
ticket board show --toon
```

Alternatively, use the MCP ticket tools (`mcp_ticket-mcp_next_tickets`, `mcp_ticket-mcp_list_tickets`, `mcp_ticket-mcp_health`, `mcp_ticket-mcp_board_show`) when the MCP server is running.

### Discovery Before Creating

Always search for existing tickets before creating new ones. Duplicate tickets degrade store quality.

```bash
./target/debug/ticket.exe search "<keywords>" --toon
```

Or via MCP: `mcp_ticket-mcp_list_tickets` with a `where` filter, or `mcp_ticket-mcp_get_ticket_description`.

### Continuous State Updates

Update ticket state immediately when the work status changes — do not defer to the end of a session:

| Situation | Action |
|---|---|
| Starting implementation | `update --to-state in-implementation` |
| Implementation complete, moving to review | `update --to-state in-review` |
| All acceptance criteria met and validated | `close <id>` |
| Ticket is no longer relevant | `cancel <id>` with a reason |

> **IMPORTANT — state machine is one-way.** Transitions only go forward.
> Progress through **every** state in order — do not skip states. The schema
> defines `required_states` (e.g. `["in-review"]`) that **must** appear in a
> ticket's history before it can reach a terminal state (`done`). Attempting to
> close a ticket without visiting all required states will be rejected by the
> store.
>
> If a state was reached prematurely, use `update --undo` to revert the last
> transition and re-progress correctly.

### Correcting State Transitions (Undo / Revert)

If a ticket was advanced to the wrong state, use `--undo` to roll back the
last transition:

# Check for stale in-implementation tickets that may conflict with your work
./target/debug/ticket.exe list --where state=in-implementation --json

# Survey all new tickets
./target/debug/ticket.exe list --where state=new --json

# Check overall graph health
./target/debug/ticket.exe health --all --json

```bash
# Undo the most recent state change (reverts to the previous state)
./target/debug/ticket.exe update <id> --undo --json
```

For deeper rollbacks, use `revert --to <rev>` to restore a specific historical
revision:

```bash
# Revert to revision 6 (re-applies fields from that point in history)
./target/debug/ticket.exe revert <id> --to 6 --json
```

> `--undo` is a convenience for the common case of "I advanced too far" and is
> equivalent to reverting to rev N-2. Neither command deletes history — a new
> revision is appended recording the rollback.

### Schema-Enforced Workflow (`required_states`)

The ticket type schema can declare `required_states` — a list of states that
**must** appear in a ticket's history before the store allows a transition to
a terminal state (default terminal: `done`).

For `tracker-improvement` tickets, the schema enforces:

```toml
required_states = ["in-review"]
```

This means the store will **reject** `close` (or `update --to-state done`) if
`in-review` has never been visited. This is enforced at the API layer, so it
applies to CLI, MCP, and HTTP equally.

To customize enforcement per ticket type, edit the corresponding schema file
under `crates/ticket-api/schemas/<type>.toml`.

### Review Gate Before Closing

**Never `close` a ticket directly from `in-implementation`.** Always move
through `in-review` first, even for small changes.
The schema's `required_states` enforcement prevents skipping `in-review`,
but you should still follow the full progression diligently.
Review readiness means the implementation, required validation, documentation updates, and spec traceability are current before the state change.

#### Step 1 — Move to in-review

```bash
./target/debug/ticket.exe update <id> --to-state in-review
```

#### Step 2 — Code Review Checklist

Before moving to validation, verify each of the following. Fix any issue
found before proceeding, including missing documentation or spec traceability needed for review.

**Correctness & Reactivity (frontend)**
- [ ] All signal reads that must re-run on change are inside reactive closures,
      not computed once outside the `view!` macro.
- [ ] State updated correctly on all paths (including edge cases like empty data).

**Memory & Cleanup**
- [ ] No unbounded `Closure::forget()` calls; use `Closure::into_js_value()` to
      transfer ownership to the JS GC instead.
- [ ] Document-level event listeners registered with a `on_cleanup` removal hook
      so they are unregistered if the component unmounts mid-gesture.
- [ ] No `Rc`/`RefCell` or wasm-bindgen closures that outlive component scope
      without an explicit cleanup path.

**CSS & Layout**
- [ ] Elements with negative positioning checked against any `overflow: hidden`
      ancestors — they will be clipped.
- [ ] Responsive/resize behavior tested at both min-width and large widths.
- [ ] `aria-label` or role attributes on interactive elements without visible text.

**Security**
- [ ] User-controlled strings inserted into the DOM use text-node APIs (e.g.
      `set_text_content`) — not `set_inner_html` — to prevent XSS.
- [ ] URLs derived from external data are validated before fetch/navigation.

**General**
- [ ] No dead code, unused imports, or unreachable branches left behind.
- [ ] Public API changes reflected in docs/changelogs if applicable.
- [ ] The relevant spec links the exact ticket folder path(s), the updated docs, and the passing or blocked validation results for this work.
- [ ] The implementation summary captures implementation, validation, and documentation status.

#### Step 3 — Validate Acceptance Criteria

Run the relevant test suite(s) against the ticket's acceptance criteria.
Keep iterating on the nearest required validation until it passes or you have a clearly repeated failure with enough evidence to stop and report the blocker:

```bash
# Native unit tests (pure-Rust logic, no browser needed)
cargo test -p <crate>

# WASM browser tests (requires wasm-pack + Chrome)
wasm-pack test --headless --chrome <path/to/crate>

# Cargo check for WASM target (quick compile gate)
cargo check --target wasm32-unknown-unknown -p <crate>
```

Confirm each acceptance criterion listed in the ticket description is met with
a passing test or a documented manual verification step.

#### Step 3 — Close

Only close after the review checklist is complete and tests pass:

```bash
./target/debug/ticket.exe close <id>
```

### Picking Next Work

Use `ticket next` to find the highest-priority unblocked tickets:

```bash
# Find unblocked ready tickets you can work on now (priority-ordered)
ticket next --json

```bash
# List unblocked tickets sorted by progress, then priority
ticket next --json

# With a title prefix filter for a specific track
./target/debug/ticket.exe next --filter "[bootstrap]" --json

# Limit results
./target/debug/ticket.exe next --limit 5 --json
```

Or via MCP: `mcp_ticket-mcp_next_tickets` with `workspace`, optional `limit` and `filter`.

The command returns tickets in **any non-terminal state** whose `depends_on`
edges all point to `done`/`cancelled` tickets. Results are sorted by:

1. **State progress** — tickets closest to `done` appear first (e.g.
   `in-review` > `in-implementation` > `ready` > `new`).
   Progress is determined by the state's index in the schema's `states` list.
2. **Priority** — `critical > high > medium > low > none`.
3. **Creation date** — oldest first (FIFO tiebreaker).

### Dependency Semantics

Use these rules to model planning, design, tracker, and implementation ticket relationships correctly.

**Dependency direction convention:** Parents/epics `depends_on` their children (an epic is done when all children are done). Children do **not** depend on their parent — they depend on sibling prerequisites.

Planning or design tickets track the creation and refinement of specs, tickets, and execution shape. Implementation tickets depend on the planning or design ticket being completed before implementation starts. Tracker or epic tickets are separate execution parents: the tracker ticket depends on the child implementation tickets and closes when those children are done. Do not use a planning or design ticket as the tracker for its own implementation work.

### Board Coordination

The draftboard tracks which agent is working on each ticket and which files
are owned. Check in when starting implementation; check out when done.

#### Check-In / Check-Out / Heartbeat

```bash
# Register yourself as actively working a ticket
./target/debug/ticket.exe board check-in <ticket-id> \
  --agent <agent-id> \
  --intent "brief description of planned work" \
  --file "src/foo.rs" \
  --file "src/bar.rs" \
  --ttl-secs 3600 \
  --json

# Refresh your heartbeat before TTL elapses
./target/debug/ticket.exe board heartbeat <entry-id> --json

# Check out when done (records handoff reason in audit trail)
./target/debug/ticket.exe board check-out <ticket-id> \
  --agent <agent-id> \
  --reason "implemented and tested" \
  --json
```

#### WIP Limit

`board show` reports `wip_limit_reached` and `next` surfaces a warning when
the limit is hit. Do not start new implementation work when the WIP limit is
reached — finish or hand off an existing entry first.

Default limit: 5 simultaneous active entries. Configure:

```bash
./target/debug/ticket.exe board configure --max-wip 3 --json
```

#### Stale-Entry Response

An entry becomes **stale** when its heartbeat TTL elapses. `board show`
lists stale entries under `warnings[]` and `stale_count`.

Required responses:
1. Agent still active: run `board heartbeat <entry-id>` to renew.
2. Work abandoned: run `board check-out <ticket-id>` then clean.
3. Remove stale entries: `board clean preview --include-stale`, then
   `board clean apply --token <token> --include-stale`.

#### File Ownership

Owned files block other agents from checking in with overlapping paths.
Keep owned file lists narrow and release them (via check-out or update-files)
when no longer needed.

Use the short flag forms shown below as the canonical CLI shape. The board
parser keeps the older `--agent-id`, `--files`, `--old-path`, and `--new-path`
spellings as compatibility aliases, but help text and docs should use the same
flag names as the rest of `ticket-cli`.

```bash
# Add / remove files from an active entry
./target/debug/ticket.exe board update-files <ticket-id> \
  --agent <agent-id> --add "new.rs" --remove "old.rs" --json

# Rename a file in an active entry (atomic)
./target/debug/ticket.exe board rename-file <ticket-id> \
  --agent <agent-id> --from "old.rs" --to "new.rs" --json
```

### Dependency Maintenance

After completing significant work, check whether finished tickets unblock others and update those links:

```bash
# Find what a completed ticket blocks
./target/debug/ticket.exe topgraph <id> --json \
  | jq -r '.payload.nodes[] | select(.state=="new" or .state=="ready") | .id'
```

Add missing `depends_on` edges when you discover undocumented dependencies. Use `--reason` on every link to explain *why* the dependency exists.

### Commit Checkpoint Suggestions

Suggest a `git commit` checkpoint to the user when any of the following is true:

- A ticket transitions to `closed` (work milestone reached).
- A batch of related tickets all reach `closed` or `in-implementation` together.
- A dependency graph changes materially (new links added/removed).
- A tracked bug is fixed and its ticket closed.

Phrase suggestions like:

> "Ticket `<title>` is now closed — good checkpoint for a commit. Suggested message: `<imperative summary of what was done>`."

### Aggressive Quality Improvement

Opportunistically improve ticket quality whenever you touch the store:

- Fill in missing `description`, `priority`, or `type` fields on tickets you encounter.
- Split vague tickets into concrete, actionable child tickets linked with `depends_on`.
- Remove or merge duplicate tickets.
- Verify that `in-implementation` tickets actually have an active owner/context; flag stale ones.
- After any structural refactor, re-run `ticket health --all` and resolve reported issues.

## Workflow Expectations

- Start implementation work by searching for existing tickets and creating or updating the required ticket set before code changes.
- Update or create the relevant spec before implementation when requirements, goals, or behavior are new or changing.
- For each ticket, implement the scoped change, run the required validation until it passes or repeatedly fails, update docs, verify the spec links the related tickets with openable `ticket.toml` targets plus the updated docs and validation results, then move the ticket to `in-review`.
- When capturing validation or documentation evidence in this repository, prefer the repo-local `workflow` CLI so the resulting artifact can be linked from specs and tickets.
- If validation repeatedly fails, do not silently skip it. Record the failing command or manual verification result and the blocker in the ticket/spec status summary.
- Summaries and handoffs must report implementation, validation, and documentation status.
- When dedicated test, doc, or cross-store-link tooling is missing or partial, use the strongest available substitute and note the gap explicitly.
- When mentioning tickets in chat output, use the exact canonical ticket folder path returned by ticket-api output as the base path for the markdown link target.
- Never synthesize a ticket folder path from a UUID, the current store root, or an example path; if the first ticket-api response omits the path, run a follow-up ticket-api command that returns the authoritative path before responding.
- Render ticket references in chat markdown as `[<short-id> <title>](<canonical ticket folder path>/ticket.toml)`, where `<short-id>` is the first 8 characters of the authoritative ticket id, `<title>` is the authoritative ticket title, and the link target appends `/ticket.toml` to the exact returned folder path so editors can open the ticket file directly.

### Health Checks

```bash
# Health-check a subgraph rooted at a ticket (BFS traversal)
ticket health abcd1234 --json

# Health-check a subgraph, filtering to a specific type
ticket health abcd1234 --where type=tracker-improvement --json
```

# Health-check all tickets
ticket health --all --json

# Health-check all new tickets (--where filter)
ticket health --all --where state=new --json

### Command Chaining (pipe via --stdin)

```bash
# List tickets → pipe IDs → health check
ticket list --where priority=high --json \
  | jq -r '.payload.items[].id' \
  | ticket health --stdin --json

# Subgraph → filter new tickets → health check
ticket subgraph abcd1234 --json \
  | jq -r '.payload.nodes[] | select(.state=="new") | .id' \
  | ticket health --stdin --json

# Topgraph → health check all reverse dependencies
ticket topgraph abcd1234 --json \
  | jq -r '.payload.nodes[].id' \
  | ticket health --stdin --json
```

### Batch (CLI-syntax, transactional)

`ticket batch` reads one CLI command per line from stdin (or `--file`). All
commands execute against the same store in order. If any command fails, all
prior writes are rolled back automatically. Blank lines and `#` comments are
ignored.

# From a checked-in batch file
ticket batch --file scripts/bootstrap-tickets.txt --json

```bash
# Heredoc — create tickets + link, all atomic
ticket batch --json <<'EOF'
create --title "Extract GPU pipeline" --type tracker-improvement
create --title "Add shader cache" --type tracker-improvement
# link is resolved after creates succeed
link --from <UUID-A> --to <UUID-B> --kind depends_on
EOF

# Stdin from another process
echo -e "create --title 'Setup CI' --type tracker-improvement\nclose <UUID>" \
  | ticket batch --json
```

**Rules:**
- Each line is parsed identically to a top-level `ticket <subcommand>` call.
- `serve`, `watch`, nested `batch`, `scan`, lease commands, and
  config commands (`add-root`, `workspace`, `export-command-schema`) are
  rejected with a clear error — they cannot be used inside a batch.
- On rollback: `create` → deleted, `update` → state/fields restored, `link`
  → edge removed. Read-only commands (`get`, `list`, `search`, `health`, etc.)
  produce no undo entry and are not affected by rollback.
- No `--index-root` requirement — uses normal workspace resolution like any
  other CLI command.

## HTTP Endpoints

```
GET /api/graph/subgraph?workspace=default&root=<UUID>&depth=2
GET /api/graph/topgraph?workspace=default&root=<UUID>&depth=2
GET /api/graph/health?workspace=default&all=true
GET /api/graph/health?workspace=default&root=<UUID>&depth=4&direction=out
```

## Index Reconciliation (`scan --force`)

`scan` normally only integrates new/changed files it discovers. Use
`scan --force` to force a full reconciliation — every ticket.toml is re-read
from disk and both the SQLite index and Tantivy search index are rebuilt:

```bash
# Force-reconcile all indexes from disk
./target/debug/ticket.exe scan --force --json
```

Output includes `"force": true` and `"reconciled": <count>` showing how many
tickets were re-indexed. Use this after manual edits to ticket.toml files or
when the index seems stale.

## Validation

- Prefer focused tests for changed modules before broader suites.
- Verify search/index behavior when touching ticket query paths.
- Confirm no regressions in CLI or MCP-facing flows for changed endpoints.
