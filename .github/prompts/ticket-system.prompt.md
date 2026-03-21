---
mode: agent
description: "Reference for using the ticket CLI to track tasks, plans, and bugs. Use when asked about task status, creating plans, or managing work in this repo."
---

# Ticket System — context-engine Task Tracker

The repo uses a local ticket database at `.ticket/` (resolved from `.ticket-workspace`).
Binary: `./target/debug/ticket.exe` (build: `cargo build -p context-tasks --bin ticket`)

## Core Commands

```bash
# Create a ticket (embed plan doc as body)
ticket create --title "<title>" --state <state> \
  --field component=<crate|tool> \
  --field risk_level=<low|medium|high> \
  --field "acceptance_criteria=<criteria>" \
  [--body-file <path/to/plan.md>]

# List / filter
ticket list
ticket list --state open
ticket list --state in-progress

# Get full ticket
ticket get --id <uuid>

# Update state or fields
ticket update --id <uuid> --to-state in-progress
ticket update --id <uuid> --field key=value

# Full-text + metadata search (searches description.md content too)
ticket search "keyword"

# Dependency edges (directed, acyclic-enforced)
ticket link --from <uuid> --to <uuid> --kind depends_on [--reason "why"]
ticket links --id <uuid>          # outgoing edges from ticket

# Delete
ticket delete --id <uuid>

# Claim / unclaim for active work (lease system)
ticket claim --id <uuid> --worker-id <agent-name>
ticket unclaim --id <uuid> --worker-id <agent-name>
```

## States (tracker-improvement schema)

| State | Meaning |
|---|---|
| `open` | Not started, unblocked |
| `in-progress` | Actively being worked |
| `review` | Implementation done, awaiting review |
| `validating` | Under validation |
| `validated` | Validation passed |
| `release-candidate` | Ready to merge |
| `released` | Merged/deployed |
| `monitoring` | Released but under observation |
| `done` | Fully complete |
| `blocked` | Blocked on something external |
| `cancelled` | No longer needed |

## Edge Kinds

| Kind | Direction | Cycle check |
|---|---|---|
| `depends_on` | directed | ✅ acyclic enforced |
| `blocks` | directed | none |
| `linked` | undirected | none |

## Standard Fields

| Field | Values | Notes |
|---|---|---|
| `component` | crate or tool name | e.g. `context-trace`, `cli`, `storage` |
| `risk_level` | `low` / `medium` / `high` | |
| `acceptance_criteria` | free text | explicit done condition |
| `bootstrap_blocker` | `true` / `false` | blocks the bootstrap plan |
| `rollout_stage` | `mirror` / `hybrid` / `tracker-first` | |

## Workflow: Large Task → Ticket (replaces agents/plans/)

**Instead of creating `agents/plans/YYYYMMDD_PLAN_*.md`, create a ticket:**

```bash
ticket create \
  --title "<concise title>" \
  --state open \
  --field component=<crate> \
  --field risk_level=<level> \
  --field "acceptance_criteria=<explicit done condition>" \
  --body-file <path/to/existing/plan.md>   # optional: embed existing doc
```

**Execution session:**
```bash
# 1. Load the plan
ticket get --id <uuid>

# 2. Claim it
ticket claim --id <uuid> --worker-id <your-agent-name>

# 3. Transition to in-progress
ticket update --id <uuid> --to-state in-progress

# 4. Work, then update to review
ticket update --id <uuid> --to-state review

# 5. Unclaim when done
ticket unclaim --id <uuid> --worker-id <your-agent-name>
```

**Dependencies** — model them explicitly, not via text fields:
```bash
ticket link --from <blocked-uuid> --to <blocker-uuid> --kind depends_on
```

**Search for related work:**
```bash
ticket search "context-read"           # full-text over all ticket bodies
ticket search --field component=cli    # metadata filter
```

## Workspace

The active workspace is `.ticket/` in the repo root (via `.ticket-workspace`).
To override: `ticket --index-root /path/to/other/.ticket <command>`

## What still goes in agents/

| Type | Where |
|---|---|
| How-to guides, patterns | `agents/guides/` |
| Bug reports | `agents/bug-reports/` |
| Algorithm analysis | `agents/analysis/` |
| Completed feature summaries | `agents/implemented/` |
| Temporary scratch | `agents/tmp/` |
| Quick-reference | `CHEAT_SHEET.md` |

Plans and task backlog → **ticket database only**.
