Define and adopt a hybrid cross-store contract layer so domain crates interact via inversion of control rather than direct domain coupling.

Decisions locked:
- hybrid topology: one small shared core contract crate plus domain extension contract crates
- binary crates are composition roots and wire trigger-domain workflows to dependency-domain implementations using static typing

Implementation plan:
1. Define core contract crate for shared reference and resolution traits.
2. Define domain extension contracts where trigger-domain ownership is clearer.
3. Migrate one end-to-end workflow path to trait-driven wiring as reference implementation.
4. Add dependency checks to prevent domain crate cycles.

Contract candidates:
- cross-store reference resolver
- evidence lookup and projection
- validation provider abstraction
- graph traversal/query adapter

Validation evidence:
- cargo metadata/dependency checks proving DAG compliance
- unit/integration tests for one migrated workflow path
- docs showing binary composition root patterns and migration template

Acceptance criteria:
- contract boundaries are documented and enforced in Cargo metadata
- at least one end-to-end path uses trait-based static wiring
- no new cyclic domain dependencies are introduced
- migration template exists for converting additional workflows
