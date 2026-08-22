# Roadmap — Component-Oriented Specification Model

## Outcome Summary

Replace the current monolithic-prose specification shape (one `body.md` mixing decisions, phases, requirements, acceptance criteria, and deferred ideas) with a component-oriented model: independently addressable components, measurable acceptance criteria linked to test evidence, typed external evidence references, and directed consumer/provider contract edges that permit cycles. The Presentation System specification is the concrete case study and eventual pilot target; the model itself must be generic enough for any spec.

## Relevant Artifact IDs

- `.spec/specs/2ccde9ee-85ac-4c87-9601-f6099f5be01c` — Presentation System spec (case study and pilot-migration candidate).
- [workflow-tools/spec/crates/spec-api/src/manifest.rs](../../workflow-tools/spec/crates/spec-api/src/manifest.rs) — existing structured manifest fields (acceptance criteria, evidence requirements, fulfillment summaries, related tickets).
- [workflow-tools/spec/crates/spec-api/schemas/specification.toml](../../workflow-tools/spec/crates/spec-api/schemas/specification.toml) — existing typed edge schema (direction/cycle rules).
- [workflow-tools/spec/crates/spec-api/src/store/sections.rs](../../workflow-tools/spec/crates/spec-api/src/store/sections.rs), [workflow-tools/spec/crates/spec-api/src/store/hierarchy.rs](../../workflow-tools/spec/crates/spec-api/src/store/hierarchy.rs) — existing section/hierarchy primitives, reusable building blocks.
- [workflow-tools/spec/crates/spec-api/src/ticket_ref.rs](../../workflow-tools/spec/crates/spec-api/src/ticket_ref.rs) — existing structured cross-store ticket reference, a template for the evidence-reference shape.
- `05-target-artifact-contract.md` (this dossier) — drafted component/criterion/evidence/contract-edge record shapes.

## Active Blockers

None. The contract-ownership question left open by `05-target-artifact-contract.md` is resolved below.

## Resolved Decision — Contract Ownership

Adopted: **each component declares only its own outward-facing contract** (the acceptance criteria it owns). A consuming component's edge references the provider's owned criteria rather than restating them. This is the recommendation `05-target-artifact-contract.md` already carried; it is now adopted rather than pending, because a duplicate-declaration model has no clear conflict-resolution rule and the reference-only model does.

## Validation Gates

- `cargo test -p spec-api --test schema_test` — must keep passing as the schema gains new artifact kinds.
- `./target/debug/spec.exe health --all` — structural health check across all specs, including any newly modeled components.
- `./target/debug/spec.exe get 2ccde9ee-85ac-4c87-9601-f6099f5be01c --json` — before/after snapshot for the eventual Presentation System pilot migration.
- A representative two-component cycle example (each serving and consuming the other) must assign every criterion, provider obligation, and consumer claim exactly once under the adopted ownership rule.

## Roadmap Waypoints

1. **Finalize the target artifact contract.** Single-session. Take `05-target-artifact-contract.md`'s drafted component/criterion/evidence/contract-edge shapes and the resolved ownership decision above, and confirm they cover the current spec-api manifest fields without contradiction. Validation: the two-component cycle check above.
2. **Map the Presentation System spec to components.** Single-session (scope: read-only partition, no edits to the live spec). Partition the current body's requirements (R1-R15), acceptance criteria (AC1-AC17), and deferred material into a proposed component map, one owning component per active item, deferred material explicitly marked deferred. Validation: every active requirement/criterion has exactly one owning component or an explicit cross-component contract edge.
3. **Bind mapped criteria to evidence.** Single-session, depends on Waypoint 2. For each candidate acceptance criterion from the component map, identify the validation-spec/test-execution reference and expected evidence artifact, reusing the existing `acceptance_criteria`/`evidence_requirements`/`fulfillment_summaries` manifest fields where they fit. Validation: every criterion is measurable and names at least one evidence source; every contract edge references only provider-owned criteria.
4. **Decide and execute the migration slice.** Not sized for one session — this waypoint spans schema/tooling changes, a real spec migration, and validation, and is exactly the kind of oversized, cross-session work the ticket-creation exception exists for. Create a tracking ticket when this waypoint is picked up (see Heads-up notes); do not decompose it further inline here.

## Heads-up Notes

- The Presentation System spec's manifest currently records only basic identity/classification metadata — the spec API projection reports no code references today, so Waypoint 3's evidence binding starts from prose, not existing structured links.
- Waypoint 4 (migration) is deliberately left as a ticket-sized waypoint rather than expanded here; a later `/tickets` pass should size it against the actual schema-change scope once Waypoints 1-3 are validated.
- This dossier does not create tickets, edit specs, or implement code — see `README.md`'s Decision Boundary. Waypoint 4 becoming a ticket is the one exception the pipeline allows during roadmap compilation, and even that ticket is not created by this pass; it is named as the next step.
