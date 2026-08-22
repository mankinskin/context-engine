# README — Spec-System Redesign and Full Workflow-Cycle Implementation (Merged Dossier)

## Reading Order

1. [ROADMAP.md](ROADMAP.md) — entry point: outcome summary, resolved decisions, validation gates, and the ordered waypoint route.
2. [ARTIFACTS.md](ARTIFACTS.md) — merged, deduplicated artifact inventory across both source dossiers.
3. [duplication-reviews/22-08-2026_spec-system-workflow-cycle-merge/duplication-report.md](../../duplication-reviews/22-08-2026_spec-system-workflow-cycle-merge/duplication-report.md) — the shared-vs-unique map this merge's `ARTIFACTS.md`/`ROADMAP.md` were consolidated from.
4. `sources/21-08-2026_spec-ticket-cycle/` — the original closed-loop production-workflow cycle dossier (request → spec → tickets → tests → implementation → validated response → next iteration), unedited. Start at its own `README.md`.
5. `sources/20-08-2026_specification-architecture-guidelines/` — the original specification-structure dossier (component/criterion/evidence-reference/directed-contract-edge model, Presentation System case study), unedited. Start at its own `README.md`.

## Scope

This merged dossier combines two independently-refined dossiers that turned out to describe one shared effort — a redesigned, component-oriented specification system, and the full production workflow cycle it should describe — and extends them, per the merge's directing instructions, into a roadmap aimed at a **full implementation**: specifying the cycle and the new spec system (and adjacent tooling: tickets, docs, tests) in the current spec tooling, reviewing those specs with the user, creating and then actually completing implementation tickets, migrating the spec system's own specification into the new format, updating its technical documentation, updating the presentation deck with a cycle diagram, and updating agent guidance — the last three are how the roadmap actually faces the user with the finished results, not just the plan. The main outcome the roadmap targets is the new spec system implemented, tested, and documented, in service of better specification and verification for both new and existing products — `ROADMAP.md`'s Waypoint 7 is the waypoint that delivers that outcome; Waypoint 6 only creates the tickets that track it.

## Decision Boundary

This merge, and the dossiers it combines, are read-only with respect to code and specs beyond the roadmap-compilation ticket-creation exception:

- No spec was created or edited by this merge.
- No ticket was created by this merge. Note: Dossier A's own `ROADMAP.md` recorded ticket `5b50329b` (ticket-depends-on-spec gating edge) as already created during its roadmap compilation. The requester later confirmed it was **intentionally deleted** because it was created before its own governing spec existed — correct under the spec-before-ticket ordering below, not a gap. `ROADMAP.md` now reflects this; Waypoint 12 depends on Waypoint 4's adjacent-tooling spec existing first, rather than being treated as an unblocked recreate-anytime task.
- Dossier B's `ROADMAP.md` was completed as a precondition for this merge (it did not exist before this pass — see `sources/20-08-2026_specification-architecture-guidelines/ROADMAP.md`), resolving its previously-open contract-ownership question; that completion is itself a read-only roadmap-compilation step, not an implementation.
- `ROADMAP.md`'s ticket-sized waypoints (6, 8, 12) and its two direct spec-authoring waypoints (3, 4) are named and ordered here, but **not created/authored**; Waypoints 7, 9, 10, and 11 (implementing/closing Waypoint 6's tickets, then updating technical docs, the deck, and agent guidance against the finished system) are likewise not performed by this dossier. Authoring the specs, creating and closing the tickets, and updating docs/deck/guidance is explicit, separate, later work — the next step is `/spec` (Waypoints 3, 4), then `/tickets` (Waypoint 6), then an implementation session (Waypoints 7, 9, 10, 11), not this dossier itself.
