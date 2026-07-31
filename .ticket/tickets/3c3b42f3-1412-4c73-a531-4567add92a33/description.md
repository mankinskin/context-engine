## Problem
Routing today is semantic-similarity guessing; AGENTS.md routes by prompt text, never by agent template or role. No canonical role taxonomy exists to disambiguate the 15 colliding template pairs.

## Scope
Land the contract that all other consolidation work depends on:
- Document the 14-role taxonomy (R1-R14, definitions from epic c608f5ac) in AGENTS.md.
- Author a first-match-wins request->role routing table (ordered list of signal patterns -> role -> target agent template name).
- Table must be deterministic: given a request, exactly one role/template resolves.
- Does NOT create/delete templates yet (that's C2/C3) — this ticket only defines the contract surface.

## Affected paths
- AGENTS.md

## Acceptance criteria
- [ ] AGENTS.md contains a role taxonomy section listing all 14 roles with one-line definitions
- [ ] AGENTS.md contains an ordered, first-match-wins routing table mapping request signals to roles/templates
- [ ] Table covers all 8 consolidated targets + Telemetry + KEEP-UNTOUCHED templates
- [ ] No two rows can match the same request ambiguously (documented tie-break: order of table = priority)
- [ ] Coordinate wording with spec ec3b13f1 (tool grant contract) so role definitions don't conflict with grant justifications
