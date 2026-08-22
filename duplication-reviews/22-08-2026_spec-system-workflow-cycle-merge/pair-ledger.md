# Pair Ledger — spec-system-workflow-cycle-merge

Comparison scope (n=19, explicit file subset — dossier-merge, not corpus-wide):

- Dossier B = `transcripts/22-08-2026_spec-system-workflow-cycle/sources/20-08-2026_specification-architecture-guidelines/`
- Dossier A = `transcripts/22-08-2026_spec-system-workflow-cycle/sources/21-08-2026_spec-ticket-cycle/`

Sort order (directory path, then filename, ASCII):

| # | File |
|---|------|
| F1 | B/01-case-study-and-target.md |
| F2 | B/02-existing-capability-and-decision.md |
| F3 | B/03-migration-pilot-roadmap.md |
| F4 | B/04-completion-checklist.md |
| F5 | B/05-target-artifact-contract.md |
| F6 | B/ARTIFACTS.md |
| F7 | B/README.md |
| F8 | B/REVIEW.md |
| F9 | B/ROADMAP.md |
| F10 | B/merged.clean.md |
| F11 | A/01-document-closed-loop-cycle.md |
| F12 | A/02-presentation-deck-slide.md |
| F13 | A/03-ticket-spec-gating-edge.md |
| F14 | A/04-test-evidence-link.md |
| F15 | A/ARTIFACTS.md |
| F16 | A/README.md |
| F17 | A/REVIEW.md |
| F18 | A/ROADMAP.md |
| F19 | A/merged.clean.md |

Total unordered pairs = 19×18/2 = 171. Batches = 33 (anchor-fixed, MAX_FILES_PER_BATCH=8). Phases = 6 (PHASE_WIDTH=6).

## Ledger

| Batch | Anchor | Targets | Verdict | Status |
|---|---|---|---|---|
| 1 | F1 | F2,F3,F4,F5,F6,F7,F8 | near-dup,near-dup,thematic,near-dup,thematic,thematic,near-dup | done |
| 2 | F1 | F9,F10,F11,F12,F13,F14,F15 | near-dup,near-dup,thematic,thematic,thematic,near-dup,near-dup | done |
| 3 | F1 | F16,F17,F18,F19 | thematic,near-dup,near-dup,near-dup | done |
| 4 | F2 | F3,F4,F5,F6,F7,F8,F9 | exact,near-dup,near-dup,near-dup,thematic,near-dup,exact | done |
| 5 | F2 | F10,F11,F12,F13,F14,F15,F16 | near-dup,thematic,thematic,near-dup,thematic,thematic,near-dup | done |
| 6 | F2 | F17,F18,F19 | thematic,thematic,thematic | done |
| 7 | F3 | F4,F5,F6,F7,F8,F9,F10 | thematic,near-dup,thematic,thematic,thematic,exact,near-dup | done |
| 8 | F3 | F11,F12,F13,F14,F15,F16,F17 | thematic,thematic,thematic,thematic,thematic,near-dup,thematic | done |
| 9 | F3 | F18,F19 | thematic,thematic | done |
| 10 | F4 | F5,F6,F7,F8,F9,F10,F11 | thematic,thematic,thematic,thematic,thematic,thematic,thematic | done |
| 11 | F4 | F12,F13,F14,F15,F16,F17,F18 | thematic,thematic,thematic,thematic,thematic,thematic,thematic | done |
| 12 | F4 | F19 | thematic | done |
| 13 | F5 | F6,F7,F8,F9,F10,F11,F12 | near-dup,near-dup,near-dup,near-dup,near-dup,thematic,thematic | done |
| 14 | F5 | F13,F14,F15,F16,F17,F18,F19 | thematic(distinct concepts),near-dup,near-dup,thematic,near-dup,near-dup,near-dup | done |
| 15 | F6 | F7,F8,F9,F10,F11,F12,F13 | near-dup,near-dup,exact,near-dup,no-overlap,no-overlap,thematic | done |
| 16 | F6 | F14,F15,F16,F17,F18,F19 | thematic,thematic,thematic,thematic,thematic,thematic | done |
| 17 | F7 | F8,F9,F10,F11,F12,F13,F14 | near-dup,near-dup,near-dup,thematic,thematic,thematic,thematic | done |
| 18 | F7 | F15,F16,F17,F18,F19 | thematic,near-dup(structural),thematic,thematic,thematic | done |
| 19 | F8 | F9,F10,F11,F12,F13,F14,F15 | near-dup,thematic,thematic,thematic,thematic,thematic,thematic | done |
| 20 | F8 | F16,F17,F18,F19 | thematic,near-dup,near-dup,thematic | done |
| 21 | F9 | F10,F11,F12,F13,F14,F15,F16 | near-dup,thematic,thematic,thematic,thematic,thematic,thematic | done |
| 22 | F9 | F17,F18,F19 | thematic,near-dup(shared spec id+waypoints),thematic | done |
| 23 | F10 | F11,F12,F13,F14,F15,F16,F17 | near-dup,thematic,near-dup,near-dup,near-dup,thematic,near-dup | done |
| 24 | F10 | F18,F19 | near-dup,thematic | done |
| 25 | F11 | F12,F13,F14,F15,F16,F17,F18 | near-dup,thematic,near-dup,near-dup,near-dup,near-dup,thematic | done |
| 26 | F11 | F19 | near-dup | done |
| 27 | F12 | F13,F14,F15,F16,F17,F18,F19 | thematic,thematic,near-dup,near-dup,thematic,thematic,exact | done |
| 28 | F13 | F14,F15,F16,F17,F18,F19 | thematic,near-dup,near-dup,near-dup,near-dup,exact | done |
| 29 | F14 | F15,F16,F17,F18,F19 | near-dup,thematic,near-dup,near-dup,exact | done |
| 30 | F15 | F16,F17,F18,F19 | thematic,near-dup,thematic,thematic | done |
| 31 | F16 | F17,F18,F19 | near-dup,exact,thematic | done |
| 32 | F17 | F18,F19 | near-dup,exact | done |
| 33 | F18 | F19 | thematic | done |
