<!-- aligned-structure:v1 -->

# Summary

Creating new durable stores is still too dependent on reverse-engineering prior domains. This spec defines one foundational memory-store bootstrap contract for a fully operational store, then layers extension profiles on top so domain-specific behaviors stay powerful without fragmenting the common base.

## Behavior Story

An engineer starting a new durable store should be able to identify the minimum shared store mechanics, the required transport and validation surfaces, and the extension points for richer domain workflows without guessing whether a domain belongs to a different bootstrap family.

## Provided Surface Contracts

- The repository owns one canonical foundational bootstrap contract for durable stores built on shared `memory-api` store mechanics.
- `ticket`, `spec`, `rule`, `test`, `log`, and `session` all participate in that foundational contract where they expose durable records, indexes, workspace resolution, and transport-facing query/write behavior.
- Domain-specific attached artifacts, richer workflow logic, and specialized query semantics belong in extension profiles layered above the foundational store contract.
- Prompt-driven scaffolding, templates, and slash-skill automation are downstream consumers of this policy and must not invent alternate bootstrap categories.

## Core Bootstrap Profile

The core profile defines the minimum contract for a fully operational generic store:

- durable record storage using shared `memory-api` open/create and workspace-resolution primitives
- schema or equivalent domain-shape registration sufficient to validate writes and query stable fields
- transport foundations across the real surfaces the domain claims to support, with CLI and MCP expected by default and HTTP required where the domain already exposes HTTP
- generic store features: create, get, update, list or search, move or scan where applicable, nested-workspace resolution, and cross-store link fields when the domain interoperates with other durable stores
- focused validation proving CRUD behavior, schema enforcement, workspace-root discovery, and transport parity instead of compile-only bootstrap checks

## Extension Profile Model

Extension profiles add domain-owned behavior on top of the core profile without redefining the foundational store model:

- workflow and lifecycle semantics such as ticket state machines, review gates, and graph edges
- richer transport semantics such as HTTP-specific handler contracts or MCP workflow helpers
- attached artifact families such as runtime log captures, session transcripts, benchmark bundles, or generated outputs
- specialized query behavior, ranking, audit trails, or replay and journaling models

The design rule is simple: foundational mechanics stay shared; domain meaning stays layered.

## Bootstrap Comparison Matrix

| Domain | Foundational store mechanics | Domain or extension layer | Transport expectation | Validation anchor | Reuse signal |
| --- | --- | --- | --- | --- | --- |
| `ticket-api` | Domain store over shared `memory_api` entity storage with built-in schema registration and workspace-aware open paths | Workflow states, dependency graph rules, board coordination, and transport-specific review flows | CLI, MCP, and HTTP are all core for this domain and must stay in parity | `tools/cli/ticket-cli/tests/contracts_schema_validation.rs` plus transport parity work | Richest precedent for shared foundation plus domain workflow layering |
| `spec-api` | Small shared-store bootstrap with embedded schema registration and minimal wrapper logic | Section handling, traceability rules, and spec review semantics | CLI and MCP are core; HTTP remains required where exposed | `crates/spec-api/tests/schema_test.rs` | Closest example of the smallest viable foundational profile |
| `rule-api` | Shared-store bootstrap with multi-schema registration and standard workspace behavior | Generated-target workflows, rule search semantics, and repo-specific operator helpers | CLI and MCP are core; HTTP is extension-only if introduced later | `crates/rule-api/src/default_schema.rs` and rule-store tests | Best example of multiple built-in schemas over the same foundation |
| `test-api` | Durable evidence store over shared memory-store mechanics | Validation, benchmark, and compliance semantics layered onto the same durable base | CLI and MCP are core; HTTP is required only if the domain adds it | `memory-api/crates/test-api/src/lib.rs` and transport validation work | Precedent that generic store mechanics can support evidence-heavy domains |
| `log-api` | Durable runtime-log metadata and index store over shared foundational patterns | Runtime capture locators, active-log workflows, and query semantics for log artifacts | CLI and MCP are core; HTTP is required only if the domain adds it | runtime-session and log-store validation work | Precedent for a foundational store plus attached artifact content |
| `session-api` | Foundational durable metadata, indexes, and workspace-aware store mechanics still apply where session records are stored | Session planning, capture layout, transcript artifacts, and hook-driven workflows are extension behavior | CLI and MCP are core; HTTP is required only if the domain adds it | `crates/session-api/src/store_tests.rs` and capture-hook E2E tests | Proof that attached artifacts do not remove a domain from the shared foundational profile |

## Template Boundary Decision

### First template target

The first canonical bootstrap template targets the core foundational profile for a fully operational durable store, not a toy single-transport slice.

### Extension handling rule

The first template must leave explicit hooks for extension profiles so later domains can add workflow engines, attached artifacts, or specialized transports without reworking the base scaffolding.

### Handwritten vs generated boundary

- Handwritten, domain-owned surfaces:
  - entity schemas or equivalent domain-shape rules
  - domain query semantics and specialized write invariants
  - extension-profile behavior such as workflow rules, attached artifact routing, or custom operators
  - cross-store vocabulary and evidence expectations beyond the shared minimum contract
- Candidate generated surfaces:
  - crate skeleton and manifest wiring
  - default schema or registry module
  - store open/create shell using shared `memory-api` primitives
  - CLI and MCP stubs, plus HTTP stub wiring where the selected profile requires HTTP
  - baseline CRUD, schema-validation, workspace-resolution, and transport-parity tests

## Required Validation

- `cargo test --manifest-path memory-api/Cargo.toml -p spec-api`
- `cargo test --manifest-path memory-api/Cargo.toml -p ticket-cli --test contracts_schema_validation`
- `cargo test --manifest-path memory-api/Cargo.toml -p session-api`
- Focused workspace-resolution validation against the `memory-api` resolver path so new stores inherit nested-workspace behavior rather than re-implementing it.
- At least one bootstrap fixture or template test must prove create, get, update, delete, schema validation, workspace-root discovery, and claimed transport parity out of the box.

## Related Implementation Tickets

- [79dd2d35 [workflow-policy][memory-store] Research and define minimal domain-store bootstrap policy](C:/Users/linus/git/graph_app/context-engine/.ticket/tickets/79dd2d35-267b-4395-8316-0761df45f3c5/ticket.toml)
- [e268a1e8 [memory-api][bootstrap] Implement core-profile minimal-store fixture and template smoke path](C:/Users/linus/git/graph_app/context-engine/.ticket/tickets/e268a1e8-3f3a-433f-b4a0-d58c590b8d29/ticket.toml)
- [39239e48 Transport-layer workspace-resolution parity (tracker)](C:/Users/linus/git/graph_app/context-engine/memory-api/.ticket/tickets/39239e48-828a-41d8-a697-9cf02e980da9/ticket.toml)
- [66fae806 [scaffold] Rule-generated store bootstrap instructions and slash command skill](C:/Users/linus/git/graph_app/context-engine/.ticket/tickets/66fae806-203d-4235-9151-4272eb0bb603/ticket.toml)

## Background Knowledge References

- `memory-api/crates/ticket-api/src/model/default_schema.rs`
- `memory-api/crates/spec-api/src/default_schema.rs`
- `memory-api/crates/rule-api/src/default_schema.rs`
- `memory-api/crates/session-api/src/store.rs`
- `memory-api/crates/memory-api/src/workspace.rs`
- `memory-api/tools/http/ticket-http/src/serve/handlers/schema.rs`

## Legacy Content (Preserved)

The original scaffold and slash-skill idea remains valid, but it now targets the core bootstrap profile plus extension hooks rather than a narrow entity-store-only template.

## Acceptance criteria

- The repository has an explicit core profile for a fully operational foundational store.
- Extension profiles clearly own domain-specific workflows, attached artifacts, and richer query behavior without redefining the shared base.
- `test`, `log`, and `session` are treated as foundational-store-based domains where applicable rather than as exclusions from the bootstrap policy.
- Required bootstrap validation includes schema validation, CRUD, nested-workspace resolution, and claimed transport parity.
- Follow-up scaffold automation is described as a consumer of the policy, not the policy itself.

## Traceability

- [79dd2d35 [workflow-policy][memory-store] Research and define minimal domain-store bootstrap policy](C:/Users/linus/git/graph_app/context-engine/.ticket/tickets/79dd2d35-267b-4395-8316-0761df45f3c5/ticket.toml)
- [e268a1e8 [memory-api][bootstrap] Implement core-profile minimal-store fixture and template smoke path](C:/Users/linus/git/graph_app/context-engine/.ticket/tickets/e268a1e8-3f3a-433f-b4a0-d58c590b8d29/ticket.toml)
- [39239e48 Transport-layer workspace-resolution parity (tracker)](C:/Users/linus/git/graph_app/context-engine/memory-api/.ticket/tickets/39239e48-828a-41d8-a697-9cf02e980da9/ticket.toml)
- [66fae806 [scaffold] Rule-generated store bootstrap instructions and slash command skill](C:/Users/linus/git/graph_app/context-engine/.ticket/tickets/66fae806-203d-4235-9151-4272eb0bb603/ticket.toml)
- [23e81ad8 [rule+skill] Rule-store sources for domain-store scaffolding instructions](C:/Users/linus/git/graph_app/context-engine/.ticket/tickets/23e81ad8-b67c-49af-97b5-f90f8bb0ae2c/ticket.toml)
- [07d4b1b0 [skill] One-prompt domain-store scaffold slash command flow](C:/Users/linus/git/graph_app/context-engine/.ticket/tickets/07d4b1b0-bc20-4ba7-98d4-ed09365f0437/ticket.toml)

## Validation

- Focused package tests for `spec-api`, `ticket-cli`, and `session-api` remain green.
- Future scaffold generation work must add a template or fixture test that proves CRUD, schema validation, nested workspace discovery, and claimed transport parity without domain-specific patching.
