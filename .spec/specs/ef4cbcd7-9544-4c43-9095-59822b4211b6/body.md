<!-- aligned-structure:v2 -->

# Specification Query And Link Resolution CLI

## Target Code Location

[workflow-tools/spec/src/cli/args.rs](workflow-tools/spec/src/cli/args.rs) declares CLI arguments; [workflow-tools/spec/src/cli/commands/crud.rs](workflow-tools/spec/src/cli/commands/crud.rs) implements `cmd_get`; [workflow-tools/spec/src/cli/commands/refs.rs](workflow-tools/spec/src/cli/commands/refs.rs) implements code-reference output; [workflow-tools/spec/src/cli/commands/query.rs](workflow-tools/spec/src/cli/commands/query.rs) exposes `spec health`; [workflow-tools/spec/src/cli/commands/validate_links.rs](workflow-tools/spec/src/cli/commands/validate_links.rs) resolves current ticket links.

## Naming Conventions

Use `spec dump <id>` for the complete projection, `spec links <id>` for resolved links, and `spec health` for diagnostic findings. This child owns `query-spec-dump`, `query-resolved-links`, and `query-health-findings`; all are read-only.

## Requester Input

> The cli should be able to output the data about each spec id and also resolve and list all of its links from the toml file.

## Reading Order

1. [55d8f2eb Specification Store Contract](.spec/specs/55d8f2eb-70f1-4b90-8c8f-e50d5e311d48/body.md) - persisted manifest and edges provider; its separate `spec migrate`/`spec_migrate_*` operation owns explicit mutation.
2. [ad0685f5 Directed Contract Edge](.spec/specs/ad0685f5-cb35-4c61-b1dc-f69232521e25/body.md) - structured edge provider.
3. [workflow-tools/spec/src/cli/commands/crud.rs](workflow-tools/spec/src/cli/commands/crud.rs) - current get output.
4. [workflow-tools/spec/src/cli/commands/refs.rs](workflow-tools/spec/src/cli/commands/refs.rs) - current code-ref-only output.

## Responsibility

If implemented, a CLI or MCP caller can obtain a complete structured projection for one spec id and resolved TOML-backed spec, code, ticket, and document links without scraping Markdown.

## Interfaces And Dependencies

`spec dump <id> --json` returns id, timestamps, fields, code refs, criteria, evidence, observations, provider-owned edges, sections, and body for one component spec. `spec links <id> --json` returns normalized link kind, source field, target `{kind, repo_relative_ref, optional_locator}`, resolution result, and failure detail. `spec health --json` returns diagnostic findings with stable severity and category/policy; it does not globally fail merely because findings exist.

## Behavior

- `query-spec-dump`: emit all persisted data for an unambiguous spec id, including structured extras and `code_refs`.
- `query-resolved-links`: enumerate and resolve every TOML-backed spec, code, ticket, document, and component-edge link; compare kinds explicitly and do not infer body-only links as structured data.
- `query-health-findings`: emit the structured diagnostic report, preserving distinct `violation` and `migration_notice` categories for hook policy evaluation.

## Boundaries And Failure Cases

The commands are read-only and do not claim body parity. Health findings are
diagnostic results, not a global CLI rejection; configured PostToolUse policy is
the only blocking decision. Unknown/ambiguous id, unparseable field, missing
store, dangling target, and cross-workspace target return a typed resolution
failure while preserving the source record.
This child never exposes or aliases `spec migrate`, `spec_migrate_*`, `spec move`,
scan, or open as a mutation path.

## Provider/Consumer Contract

Consumes [55d8f2eb Specification Store Contract](.spec/specs/55d8f2eb-70f1-4b90-8c8f-e50d5e311d48/body.md) `store-persists-artifacts` and [ad0685f5 Directed Contract Edge](.spec/specs/ad0685f5-cb35-4c61-b1dc-f69232521e25/body.md) `edge-persisted-typed-model`; provides query evidence to reviewers and MCP callers.

## Examples

`spec dump f1b8f01a --json` returns the parent component spec fields and its hierarchy. `spec links f1b8f01a --json` reports a resolved Health Check component spec link and `SpecStore::health_all` code link, while a dangling document link appears with `resolution = "missing"`. `spec health --json` returns a `migration_notice` separately from a `violation`, leaving blocking to the PostToolUse hook.

## Evidence

Position: `partial`; `get --full` emits manifest/body and `refs` emits only code refs. Planned command tests cover every link kind, resolved/missing targets, and matching MCP projection.

## Scope

Owns read-only query projection and resolution, not persistence, health policy,
or the separate migration interface.