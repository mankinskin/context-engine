## Objective
Implement the schema-modernization contract established in [e9c38d24 Schema modernization lifecycle and migration](.spec/specs/e9c38d24-42cc-4044-8b2c-6811b918530f/spec.toml).

## Decision Record
- Features are owned by specs; the new work-item types are `epic`, `bug`, `research`, `planning`, `implementation`, `review`, `interview`, and `testing`.
- Runtime schemas use strict single-parent inheritance and directed lifecycle graphs. Relation graphs remain separate ticket-edge graphs.
- Work-item lifecycle categories are `plan`, `act`, and `verify`; each concrete type must provide a path through all three.
- Shipped ticket schemas move to JSON; custom TOML and JSON remain supported with identical semantics. Type IDs are unique per schema kind; every collision is a full atomic-load error.
- Legacy records migrate through approved, idempotent transactional batches. Track 6 repairs only forward; it never reverts committed batches.

## Child Tracks
1. Engine and inheritance semantics
2. Dual-format schema loader
3. Resolved catalog and JSON built-ins
4. CLI and VS Code integration
5. Inventory and transactional migration
6. Cross-interface validation and release gate

## Done
All child tickets are done, linked validation evidence passes, and the linked spec is review-ready.


## Completion Evidence
Done requires every binding final-interview decision to be traceable to this spec and an owning acceptance/validation record, Track 5 migration-completion evidence, and an all-pass Track 6 validation-matrix artifact.