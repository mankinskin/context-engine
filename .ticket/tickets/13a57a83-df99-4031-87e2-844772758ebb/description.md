# Bootstrap: Spec Files for the Spec System

## Objective

Write the initial specification files for the spec system itself, establishing the self-documenting foundation. These specs serve as both documentation and as test fixtures for the spec tooling.

## Specs to Create

### Root Spec: `spec-system` (scope: domain)
- Overview of the entire specification system
- Architecture diagram: memory-api → spec-api → CLI/MCP/HTTP
- Design decisions and rationale

### Module Specs

| Slug | Scope | Component | Code Target |
|------|-------|-----------|-------------|
| `memory-api` | crate | memory-api | `crates/memory-api/` |
| `memory-api/entity` | module | memory-api | EntityManifest, EntityStore |
| `memory-api/schema` | module | memory-api | TypeSchema, SchemaRegistry |
| `memory-api/edge` | module | memory-api | EdgeRecord, EdgeRegistry |
| `memory-api/query` | module | memory-api | Expr, parse_query |
| `memory-api/storage` | module | memory-api | EntityFs, RedbIndexStore, TantivySearchIndex |
| `spec-api` | crate | spec-api | `crates/spec-api/` |
| `spec-api/manifest` | module | spec-api | SpecManifest, CodeRef, FeatureStatus |
| `spec-api/store` | module | spec-api | SpecStore |
| `spec-api/schema` | module | spec-api | specification.toml state machine |
| `spec-api/skill-gen` | module | spec-api | Skill generation engine |
| `spec-cli` | crate | spec-cli | spec binary |
| `spec-mcp` | crate | spec-mcp | MCP tool surface |
| `spec-http` | crate | spec-http | HTTP endpoints |

## Acceptance Criteria

- [ ] All specs above created in the spec store
- [ ] Parent-child hierarchy correctly wired
- [ ] Code refs point to actual source files
- [ ] Body content describes current implementation state
- [ ] Specs pass `spec health` validation