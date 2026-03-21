---
mode: agent
description: "Comprehensive workflow for using the ticket system to manage all design, planning, implementation, review, validation, bug tracking, and task completion in this repo. Use whenever working on any non-trivial task."
---

# Ticket System — context-engine Workflow

All tasks, plans, bugs, and designs are tracked in the ticket database at `.ticket/` (resolved via `.ticket-workspace`).
Binary: `./target/debug/ticket.exe` (build: `cargo build -p context-tasks --bin ticket`)

---

## 1. State Machine

Every ticket moves through a defined lifecycle. Choose the right entry point for your work:

```
open ──► in-progress ──► review ──► validating ──► validated ──► release-candidate ──► released ──► monitoring ──► done
  ▲                                                                                                              │
  └─────────────────────── blocked ◄──────────────────────────────────────────────────────────────────────────┘
                           cancelled (terminal)
```

| State | When to use |
|---|---|
| `open` | Created, not yet started. Waiting for assignment. |
| `in-progress` | Claimed and actively being worked. |
| `review` | Work submitted; awaiting human or agent review. |
| `validating` | Under automated or manual validation (tests, CI). |
| `validated` | Validation passed; ready to merge. |
| `release-candidate` | Merge-ready; linked to a release target. |
| `released` | Merged / deployed. |
| `monitoring` | Live but watching for regressions. |
| `done` | Fully complete — no further action needed. |
| `blocked` | Cannot progress; blocker recorded in fields. |
| `cancelled` | Dropped; reason recorded. |

---

## 2. Dependency Model

**Always wire dependencies before working.** Use `depends_on` for hard prerequisites; `blocks` for reverse lookups; `linked` for loose associations.

```bash
# A cannot start until B is done
ticket link --from <A-uuid> --to <B-uuid> --kind depends_on --reason "why"

# Show what A is waiting on
ticket links --id <A-uuid>

# Find what's currently unblocked (no outgoing depends_on edges in open state)
ticket list --state open
```

Edge kinds:

| Kind | Direction | Cycle guard | Use for |
|---|---|---|---|
| `depends_on` | A → B | ✅ acyclic | A cannot start until B is done |
| `blocks` | A → B | none | A is blocking progress on B |
| `linked` | undirected | none | Related but independent |

---

## 3. Standard Fields

```bash
--field component=<name>              # crate / tool: context-trace, cli, storage, …
--field risk_level=<low|medium|high>
--field "acceptance_criteria=<text>"  # explicit, testable done condition
--field workflow_stage=<design|plan|implementation|validation>
--field blocked_reason=<text>         # when transitioning to blocked
```

---

## 4. Workflow by Work Type

### 4a. Design

Used for decisions, RFCs, architecture choices. Body holds the design document.

```bash
# Open a design ticket with the doc embedded
ticket create --title "Design: <topic>" --state open \
  --field component=<crate> --field risk_level=<level> \
  --field workflow_stage=design \
  --field "acceptance_criteria=Design reviewed, approach agreed, implementation tickets created" \
  --body-file <path/to/design.md>

# When writing the design
ticket claim --id <uuid> --worker-id <agent>
ticket update --id <uuid> --to-state in-progress

# When ready for review
ticket update --id <uuid> --to-state review

# After review, spawn implementation tickets linked as dependants, then close design
ticket update --id <uuid> --to-state done
```

### 4b. Planning (large feature)

Used to break a large feature into implementation tickets.

```bash
# Create the plan ticket (body = full plan document)
ticket create --title "Plan: <feature>" --state open \
  --field component=<crate> --field risk_level=<level> \
  --field workflow_stage=plan \
  --field "acceptance_criteria=Implementation tickets created; all deps wired; plan approved" \
  --body-file <plan.md>

# Link prerequisites
ticket link --from <plan-uuid> --to <blocker-uuid> --kind depends_on

# Claim and produce implementation sub-tickets
ticket claim --id <plan-uuid> --worker-id <agent>
ticket update --id <plan-uuid> --to-state in-progress

# Create each impl sub-ticket, link the plan as depended-upon
ticket create --title "Impl: <step>" --state open --field component=<crate> \
  --field risk_level=<level> --field "acceptance_criteria=<step done condition>"
ticket link --from <impl-uuid> --to <plan-uuid> --kind depends_on

# Mark plan done once all impl tickets are created and wired
ticket update --id <plan-uuid> --to-state done
ticket unclaim --id <plan-uuid> --worker-id <agent>
```

### 4c. Implementation

Used for concrete code changes. One ticket per cohesive piece of work.

```bash
# Create (or work from existing open ticket)
ticket create --title "Impl: <what>" --state open \
  --field component=<crate> --field risk_level=<level> \
  --field "acceptance_criteria=Tests pass; API matches spec; no regressions"

# Start work
ticket claim --id <uuid> --worker-id <agent>
ticket update --id <uuid> --to-state in-progress

# Implementation complete, tests pass
ticket update --id <uuid> --to-state review

# After review approval — run final validation
ticket update --id <uuid> --to-state validating

# All checks green
ticket update --id <uuid> --to-state validated

# Ready to merge
ticket update --id <uuid> --to-state release-candidate

# Merged
ticket update --id <uuid> --to-state done
ticket unclaim --id <uuid> --worker-id <agent>
```

### 4d. Bug Tracking

```bash
# File the bug
ticket create --title "Bug: <component> — <symptom>" --state open \
  --field component=<crate> --field risk_level=<high|medium|low> \
  --field "acceptance_criteria=Root cause identified; fix verified; regression test added"

# Link the ticket this bug blocks
ticket link --from <bug-uuid> --to <affected-uuid> --kind blocks

# Work through fix lifecycle same as Implementation (4c)
```

### 4e. Validation Checklist (per-ticket)

Before marking `validated`:
- [ ] All acceptance criteria met (from `ticket get --id <uuid>`)
- [ ] Relevant tests run and green: `cargo test -p <crate>`
- [ ] No warnings introduced
- [ ] Dependent tickets re-evaluated (do they still have correct state?)

---

## 5. Bootstrap Plan Execution

The active bootstrap backlog is tracked as tickets. Check current state:

```bash
ticket list --state open       # what's available
ticket list --state in-progress  # what's being worked
```

**Dependency graph root — pick this up first:**
```
4f2d2a5e  [bootstrap] wire create/get/update/list/delete to storage backend
          → 6 tickets unlock when this is done
```

**Working a bootstrap ticket:**
```bash
# 1. Understand the work (body contains the full phase plan)
ticket get --id <uuid>

# 2. Check what it depends on — all must be done first
ticket links --id <uuid>

# 3. Claim and start
ticket claim --id <uuid> --worker-id <agent-name>
ticket update --id <uuid> --to-state in-progress

# 4. If the work needs sub-tasks, create child tickets and link them
ticket create --title "Impl: <sub-step>" --state open \
  --field component=<crate> --field risk_level=<level> \
  --field "acceptance_criteria=<sub-step done condition>"
ticket link --from <child-uuid> --to <parent-uuid> --kind depends_on

# 5. When all sub-tasks done, validate parent
ticket update --id <uuid> --to-state validating
cargo test -p <relevant-crate>
ticket update --id <uuid> --to-state validated
ticket update --id <uuid> --to-state done
ticket unclaim --id <uuid> --worker-id <agent-name>
```

**Validation tests for bootstrap tickets:**
- `4f2d2a5e` (CRUD): `cargo test -p context-tasks` — all 45+ contract + integration tests pass
- `2a1fa2f2` (leases): lease claim/unclaim/stale loop integration test pass
- `de6c3391` (crash recovery): simulated crash test via `ticket scan --reindex`
- `a8d6c1d2`–`f5d7e9a2` (T1–T6): integration tests in HOST_EXECUTOR_AUTH_PROVIDER.md

---

## 6. CLI Quick Reference

```bash
# Create
ticket create --title "<title>" --state <state> \
  --field component=<name> --field risk_level=<l|m|h> \
  --field "acceptance_criteria=<criteria>" \
  [--body-file <doc.md>]

# Query
ticket list [--state <state>]
ticket get --id <uuid>
ticket search "<keywords>"

# Mutate
ticket update --id <uuid> --to-state <state>
ticket update --id <uuid> --field key=value

# Edges
ticket link --from <uuid> --to <uuid> --kind <depends_on|blocks|linked> [--reason <text>]
ticket links --id <uuid>

# Lease
ticket claim --id <uuid> --agent <name>
ticket unclaim --id <uuid> --agent <name>
ticket leases

# Workspace
ticket workspace current
ticket workspace list

# Recovery — rebuild derived indexes after a fresh clone or crash
# (tickets.redb and search_index/ are NOT tracked in git; only tickets/**/*.toml
# and tickets/**/*.md are committed. Run this once after cloning.)
ticket scan --reindex
```
