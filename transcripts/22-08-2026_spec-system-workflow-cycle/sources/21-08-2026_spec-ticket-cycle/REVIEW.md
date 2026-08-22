# Review — First Informed Loop

**Verdict: Approved as scoped.**

## Critique

The merged intent ([merged.clean.md](merged.clean.md)) describes a closed-loop production-workflow cycle: user request → spec (goal/contract, optional depending on maturity) → tickets (plan) → tests (evidence for the spec's acceptance criteria) → implementation → validated response back to the user → next iteration. The transcript explicitly frames this as more important than the existing scattered "tickets before code" mentions, and asks for it to be (a) documented as a first-class principle and (b) shown in the presentation deck as the core production cycle.

Checked against [ARTIFACTS.md](ARTIFACTS.md):

- The principle is currently **stated piecemeal** across `AGENTS.md`, `workflow.instructions.md`, `lifecycle.instructions.md`, and `phase-separation.instructions.md` — no single file unifies it as "the cycle," confirming the transcript's own observation.
- Most of the cycle's mechanical pieces **already exist**: spec store, ticket store with `depends_on`/`blocks`/`linked` edges, `[[refs]]`/`spec_refs` for informational spec pointers, and test-api executions already linking to both `spec_ids` and `ticket_ids`. This matches the transcript's closing line ("All the components I mentioned already have initial implementations").
- One genuine gap: the transcript proposes tickets **depend on or fulfill a spec** as a gating relationship (blocking readiness the way `depends_on` between tickets does today). Research confirms no such gating edge exists — `[[refs]]`/`spec_refs` is informational only. This is a real ticket-api behavior gap, not a docs gap, and is too large to implement inline in this dossier (schema, gating logic, and `next_tickets`/readiness-check changes).
- The presentation ask resolves cleanly against evidence: [.presentation/](../../.presentation/README.md) at the repo root is the composing, repo-wide "context-engine" overview deck — the correct target for "our complete cycle," no ambiguity requiring an interview.

No finding in this pass required interviewing the user: every open question (which deck, whether the ticket-spec edge capability already exists) resolved directly from repository evidence.

## Scope Decision

**In scope for this dossier:**

1. A single, unified instruction file defining the closed-loop cycle as a named, citable principle, cross-linked from `AGENTS.md` (not inlined into it, per `AGENTS.md`'s own "keep this file small" constraint).
2. A presentation-deck work package: add the cycle as a slide/section in the root `.presentation/` deck.
3. A ticket recommendation (not implemented here) for the ticket→spec gating-edge-kind change, since it is an architecture-level ticket-api behavior change.
4. A documentation-clarification work package tying test-api's existing `spec_ids`/`ticket_ids` linkage explicitly to the cycle's "executable measurements validate the spec" step.

**Out of scope (non-goals) for this dossier:**

- No ticket-api code changes (schema, edge kinds, gating logic) — scoped as a ticket recommendation only.
- No rewrite of existing ticket/spec/lifecycle instruction files beyond adding cross-references to the new cycle file.
- No redesign of the presentation deck beyond adding the cycle content.
- No spec creation in this pass — spec creation is the separate, later step this dossier's `README.md` decision boundary reserves for `/spec` after the roadmap is picked up.
