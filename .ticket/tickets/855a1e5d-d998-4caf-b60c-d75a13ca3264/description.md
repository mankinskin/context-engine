Build a generator that runs the audit-api against the repository and emits a compact markdown summary of the current audit status directly to `.audit/README.md` along with its co-located machine sidecar under `.audit/index.toon`.

## Scope
- Implement an `audit-index` subcommand (or extend `audit-cli`) that runs `audit run .` and formats the AuditReport into `.audit/README.md` and `.audit/index.toon` (TOON primary, D8).
- Include overall rating, severities table, and active findings summaries.
- Conforms to the ContextNode schema (0dba399a); committed to git (D5).
- Emit an `.agents/` agent-hook node pointing agents at the audit summary (D1).
- Highly performant; can be run during a profiled local pre-commit hook (D2) or on demand without overhead.

## Acceptance criteria
- Running the generator writes outputs under `.audit/` plus the `.agents/` hook.
- Findings are mapped as ContextRefs with severities and stable identifiers.
- Re-running with unchanged code is digest-stable.

## Non-goals
- No central `.context/` store.
- Does not add new audit checks.

## Resolved design decisions
- D5: committed to git. D8: TOON sidecar. D2: profiled git hook / on-demand. D1: `.agents/` hook + `.audit/` workspace index.