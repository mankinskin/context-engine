<!-- aligned-structure:v2 -->

# Document API Repository, Identity, and Typed Resolver Contract

## Target Code Location

[workflow-tools/doc/crates/doc-api/src/lib.rs](workflow-tools/doc/crates/doc-api/src/lib.rs) exports the Document API public boundary; [workflow-tools/doc/crates/doc-api/src/evidence.rs](workflow-tools/doc/crates/doc-api/src/evidence.rs) owns the current `DocEvidenceRecord` path-bearing record; and [workflow-tools/doc/crates/doc-api/src/workspace.rs](workflow-tools/doc/crates/doc-api/src/workspace.rs) owns deterministic `DocWorkspace` discovery.

## Naming Conventions

The planned public Rust-library types are `DocumentIdentity`, `DocumentRecord`, `DocumentRepository`, `DocumentTarget`, `DocumentResolver`, and `ResolveDocumentOutcome`. `DocumentIdentity` is exactly `(workspace_slug, repo_relative_path)`. The versioned target grammar begins with `document/v1/<workspace_slug>/<repo_relative_path>`; a target without its versioned form has no implicit free-form fallback. This child owns `document-identity`, `document-target-grammar`, `document-resolver-outcomes`, `document-index-lifecycle`, and `document-resolver-test-coverage` criterion ids.

## Requester Input

> D12 Document provider: Create the already-proposed child spec titled exactly **Document API Repository, Identity, and Typed Resolver Contract** under parent `73817390` (not the root). Boundary: `doc-api` owns persisted document records, deterministic repository indexing, typed target grammar, and typed resolution; `spec-api` consumes outcomes only.

## Reading Order

1. [73817390 Document Store Evidence Integration](.spec/specs/73817390-7e6a-427a-a644-626718d9f25d/body.md) - parent consumer/integration contract.
2. [7498bed7 Evidence Reference Contract](.spec/specs/7498bed7-ac74-4484-b50e-8a9cf96d8431/body.md) - outcome-only evidence consumer.
3. [workflow-tools/doc/crates/doc-api/src/evidence.rs](workflow-tools/doc/crates/doc-api/src/evidence.rs) - current record location that contains the deprecated free-form `document_paths` shape.
4. [workflow-tools/doc/crates/doc-api/src/workspace.rs](workflow-tools/doc/crates/doc-api/src/workspace.rs) - deterministic repository-discovery baseline.

## Responsibility

If this spec is implemented, dependents can rely on doc-api to persist and resolve a readable, repository-scoped document identity without spec-api interpreting filesystem paths. doc-api exclusively owns records, repository indexing, target parsing, and resolution; spec-api consumes typed outcomes only.

## Interfaces And Dependencies

`DocumentRepository` scans a declared repository and persists `DocumentRecord` entries keyed by `DocumentIdentity`. `DocumentResolver` is a typed Rust library boundary in doc-api, not an MCP or HTTP endpoint and not a filesystem helper in spec-api. It accepts a versioned `DocumentTarget` and returns `Resolved { record }`, `Missing { identity }`, or `Unsupported { target }`; malformed input returns a typed request error before outcome selection.

`DocWorkspace` supplies the existing deterministic workspace-discovery baseline. The new repository index must declare when an initial scan occurs and when an explicit refresh rescans the repository; resolution reads the persisted index rather than performing an implicit scan.

## Behavior

- `document-identity`: every persisted record is identified exactly by `(workspace_slug, repo_relative_path)`. This stable, readable, repository-scoped identity replaces `DocEvidenceRecord.document_paths` as the reference identity; UUIDs and content hashes are not identities.
- `document-target-grammar`: parse only the documented versioned `document/v1/<workspace_slug>/<repo_relative_path>` grammar. A valid but unsupported version or target kind returns `Unsupported { target }`; malformed syntax is a typed request error.
- `document-resolver-outcomes`: return `Resolved { record }` for one indexed identity and `Missing { identity }` for an absent valid identity. Repository identity collisions and ambiguous indexed matches are deterministic typed errors, never arbitrary record selection.
- `document-index-lifecycle`: an initial repository scan builds the index, and explicit refresh replaces or reconciles it deterministically. Scan order, collision detection, and refresh results are independent of filesystem enumeration order.
- `document-resolver-test-coverage`: focused doc-api tests cover initial scan, explicit refresh, identity collision, ambiguity, grammar parsing, `Resolved`, `Missing`, `Unsupported`, and malformed requests.

## Boundaries And Failure Cases

`document_paths` remains legacy input/output compatibility data only until migration; it is not a resolver identity and cannot be silently interpreted as one. A resolver never uses UUIDs, content hashes, free-form paths, an implicit scan, or an external transport. Malformed target input, duplicate identities, and ambiguous matches are typed errors; unsupported targets are an explicit outcome. The current code positions are `partial` for `DocEvidenceRecord` and `DocWorkspace`, and `not-implemented` for the named repository, identity, target, parser, resolver, and refresh symbols.

## Provider/Consumer Contract

Provides `document-identity`, `document-target-grammar`, `document-resolver-outcomes`, and `document-index-lifecycle` to [73817390 Document Store Evidence Integration](.spec/specs/73817390-7e6a-427a-a644-626718d9f25d/body.md), which in turn provides document evidence integration to [7498bed7 Evidence Reference Contract](.spec/specs/7498bed7-ac74-4484-b50e-8a9cf96d8431/body.md). Evidence Reference consumes only `Resolved`, `Missing`, and `Unsupported`; it does not parse targets, read the filesystem, or decide index lifecycle.

## Examples

After an explicit scan indexes `docs/guide.md` in workspace `context-engine`, `document/v1/context-engine/docs/guide.md` resolves to `Resolved { record }` with identity `(context-engine, docs/guide.md)`. Refreshing after deletion makes the same valid target return `Missing { identity }`. `document/v2/context-engine/docs/guide.md` returns `Unsupported { target }`, while `docs/guide.md` is malformed and returns a typed request error. Two indexed records with the same identity cause a deterministic collision error rather than selecting one.

## Evidence

Guards: no `test-api` `ValidationSpec` id is allocated yet; verification requires focused doc-api unit tests for every named outcome, identity collision/ambiguity, grammar, scan, and refresh. Planned command: `cargo test --manifest-path workflow-tools/doc/Cargo.toml -p doc-api`. The governing-rule requirement is a PolicyRule that introduces this draft as `coming-soon / not-implemented` until the provider symbols and tests exist.

## Scope

Defines the doc-api provider contract and its consumer boundary only. It does not implement storage, introduce an MCP/HTTP surface, define spec-api adapter parsing, or determine criterion fulfillment.
