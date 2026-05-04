---
agent: agent
description: "Create tickets for issues and research each ticket systematically. Update with user interview answers, detailed implementation plans, or sub-tickets for composite work."
---

# Ticket System

Create tickets for issues and research each ticket systematically. Update tickets with user answers from interviews and a detailed implementation plan or appropriate sub-tickets for composite work items.

**Database:** `.ticket/` (resolved via `.ticket-workspace`)
**CLI binary:** `./target/debug/ticket.exe` (build with `cargo build -p ticket-cli`)
**MCP server:** `ticket-mcp` (tools prefixed `mcp_ticket-mcp_`)

---

## Workflow

1. **Research** — Read code, search existing tickets, understand the problem space
2. **Create tickets** — One per issue; include title, component, risk level, acceptance criteria
3. **Interview user** — Ask clarifying questions, capture answers as field updates on the ticket
4. **Plan** — Write an implementation plan in the ticket body, or break into sub-tickets and wire dependencies
5. **Execute** — Claim, implement, update state through the lifecycle
6. **Close** — Fast-forward to done when complete

---

## State Machine

```
new ──► ready ──► in-implementation ──► in-review ──► done
 │                                                     │
 │                                                     ▼
 │                                              in-implementation (review failed)
 │
 ▼
 cancelled ◄── (any non-terminal state)
```

**Transitions per state:**

| From | To |
|---|---|
| `new` | `ready`, `cancelled` |
| `ready` | `in-implementation`, `new`, `cancelled` |
| `in-implementation` | `in-review`, `cancelled` |
| `in-review` | `done`, `in-implementation`, `cancelled` |
| `done` | *(terminal)* |
| `cancelled` | *(terminal)* |

Use `ticket close --id <uuid>` to fast-forward through intermediate states to `done`.

---

## Edge Kinds

| Kind | Acyclic | Use |
|------|---------|-----|
| `depends_on` | yes | A cannot start until B is done |
| `blocks` | no | A is blocking B |
| `linked` | no | Related but independent |

---

## CLI Reference

### Global Flags

Every command accepts these:

```bash
ticket --json ...              # JSON output
ticket --index-root <path> ... # override ticket database location
ticket --schema-dir <path> ... # override schema directory
```

### Create

```bash
ticket create --title "Bug: parser panics on empty input" \
  --state new \
  --field component=context-trace \
  --field risk_level=high \
  --field "acceptance_criteria=Panic fixed; regression test added"

# With a body file (design doc, plan, etc.)
ticket create --title "Plan: refactor search API" \
  --body-file ./plan.md \
  --field component=context-search \
  --field workflow_stage=plan

# With explicit ID and type
ticket create --id <uuid> --type tracker-improvement --title "..."
```

**Flags:** `--id`, `--type`, `--title`, `--state`, `--field key=value` (repeatable), `--body-file`, `--root`

### Get / Read

```bash
ticket get --id <uuid>          # full manifest (fields, state, timestamps)
```

### Update

```bash
ticket update --id <uuid> --to-state in-implementation
ticket update --id <uuid> --field "acceptance_criteria=Updated criteria"
ticket update --id <uuid> --from-state new --to-state ready  # guarded transition
```

**Flags:** `--id`, `--to-state`, `--from-state`, `--field key=value` (repeatable)

### List

```bash
ticket list                                    # all active tickets
ticket list --state new                        # filter by state
ticket list --type tracker-improvement         # filter by type
ticket list --include-deleted                  # include soft-deleted
ticket list --where component=context-trace    # filter by field value
ticket list --limit 10
```

**Flags:** `--state`, `--type`, `--limit`, `--with-repro`, `--include-deleted`, `--where key=value` (repeatable)

### Search

```bash
ticket search "parser panic"        # full-text search
ticket search "refactor" --limit 5
```

### Delete

```bash
ticket delete --id <uuid>   # soft-delete
```

### Close / Cancel

```bash
ticket close --id <uuid>                    # fast-forward to done
ticket close --id <uuid> --to-state review  # fast-forward to a specific state
ticket cancel --id <uuid>                   # transition to cancelled
```

### Links (Edges)

```bash
ticket link --from <A> --to <B> --kind depends_on --reason "A needs B's API"
ticket unlink --from <A> --to <B> --kind depends_on
ticket links --id <uuid>   # list edges from this ticket
```

**Kinds:** `depends_on` (acyclic-enforced), `blocks`, `linked`

### Claim / Unclaim (Leases)

```bash
ticket claim --id <uuid> --agent copilot --ttl-secs 600 --intent "implementing fix"
ticket unclaim --id <uuid>
ticket leases   # list all active leases
```

### History / Diff / Revert

```bash
ticket history --id <uuid> --limit 10
ticket diff --id <uuid> --from 1 --to 3     # diff two revisions
ticket revert --id <uuid> --to 2            # revert to revision
```

### Batch Operations

Atomic execution — rolls back all changes on first error.

```bash
# From stdin (NDJSON, one JSON object per line)
ticket exec --batch <<'EOF'
{"command":"create","title":"Sub-task A","type":"tracker-improvement","state":"new","fields":{"component":"context-trace"}}
{"command":"create","title":"Sub-task B","type":"tracker-improvement","state":"new"}
{"command":"link","from":"<uuid-a>","to":"<uuid-b>","kind":"depends_on"}
EOF

# From file
ticket batch --file ./bulk-ops.ndjson
```

**Supported batch commands:** `create`, `get`, `update`, `delete`, `list`, `link`, `links`, `search`

### Scan / Recovery

```bash
ticket scan              # scan registered roots for new ticket folders
ticket scan --reindex    # full rebuild of SQLite + search index from filesystem
```

### Workspace Management

```bash
ticket workspace current
ticket workspace list
ticket workspace new <name> [--path <dir>]
ticket workspace use <name> [--local]
```

### Other Commands

```bash
ticket next [--limit 20] [--filter <prefix>]  # unblocked ready tickets, priority-ordered
ticket status [--filter <state>] [--show-blocked]
ticket ready-overview [--filter <state>]
ticket audit                    # summary of all tickets by state
ticket assets --id <uuid>       # list attached files
ticket attach --id <uuid> <filepath> [--as <name>]
ticket watch [--debounce-ms 200]
ticket serve [--port 8080]
```

---

## MCP Tools Reference

All MCP tools require a `workspace` parameter (use `mcp_ticket-mcp_list_workspaces` to find it).

### Read Tools

```
mcp_ticket-mcp_list_tickets     workspace, state?, query?, limit?
mcp_ticket-mcp_get_ticket       workspace, id
mcp_ticket-mcp_get_ticket_description  workspace, id
mcp_ticket-mcp_list_edges       workspace, kind?
mcp_ticket-mcp_subgraph         workspace, root, direction?, edge_kind?, depth?, limit_nodes?, limit_edges?
```

### Write Tools

```
mcp_ticket-mcp_update_ticket    workspace, id, to_state?, fields[]?
mcp_ticket-mcp_close_ticket     workspace, id, to_state? (default: "done")
mcp_ticket-mcp_cancel_ticket    workspace, id
```

### Utility Tools

```
mcp_ticket-mcp_health
mcp_ticket-mcp_list_workspaces
mcp_ticket-mcp_next_tickets     workspace, limit?, filter?  — unblocked ready tickets in priority order
mcp_ticket-mcp_workflow         name (list|triage_open_tickets|fetch_ticket_context|inspect_dependencies), workspace?, id?, query?
mcp_ticket-mcp_help
```

---

## Fields Reference

**Required:** `title`, `type`

**Standard optional fields:**

| Field | Values | Purpose |
|-------|--------|---------|
| `component` | crate or tool name | Which part of the codebase |
| `risk_level` | `low`, `medium`, `high` | Impact assessment |
| `acceptance_criteria` | free text | Testable done condition |
| `workflow_stage` | `design`, `plan`, `implementation`, `validation` | Current phase |
| `blocked_reason` | free text | Why the ticket is blocked |
| `tags` | free text | Searchable labels |

---

## Common Patterns

### Find next work

```bash
# Priority-ordered list of unblocked tickets in "ready" state
ticket next --json

# Scoped to a track/prefix
ticket next --filter "[bootstrap]" --limit 5 --json
```

Dependency convention: parents/epics `depends_on` their children (done when all children done). Children depend on sibling prerequisites, not on their parent.

### Single issue → research → plan → close

```bash
# 1. Create
ticket create --title "Bug: X crashes on Y" --state new \
  --field component=context-trace --field risk_level=high \
  --field "acceptance_criteria=Crash fixed; test added"

# 2. Research and update with findings
ticket update --id <uuid> --field "root_cause=Off-by-one in boundary check"

# 3. Implement, then close
ticket close --id <uuid>
```

### Composite work → parent + sub-tickets

```bash
# 1. Create parent
ticket create --title "Refactor: search API" --state new \
  --field component=context-search --field workflow_stage=plan

# 2. Create sub-tickets and wire dependencies
ticket create --title "Impl: extract QueryBuilder" --state new \
  --field component=context-search
ticket link --from <child> --to <parent> --kind depends_on

ticket create --title "Impl: migrate callers" --state new \
  --field component=context-search
ticket link --from <child2> --to <child1> --kind depends_on

# 3. Work through sub-tickets, then close parent
ticket close --id <parent>
```

### Interview-driven refinement

```bash
# Create with what you know
ticket create --title "Feature: configurable retry policy" --state new \
  --field component=context-read

# After interviewing user, update with answers
ticket update --id <uuid> \
  --field "acceptance_criteria=Exponential backoff with jitter; max 3 retries; configurable per-request" \
  --field risk_level=medium
```
