<!-- aligned-structure:v2 -->

# Motivation

Repository command execution is currently discovered ad hoc: Cargo binary targets, executable scripts, managed services (viewer-ctl/install-ctl), VS Code extensions, and git hooks each have their own listing mechanism or none at all. `tools/install/artifacts.toml` already exists as a runtime-read registry for `install-ctl` (foundation ticket [f3c2b8a9 install-ctl: Generalize viewer-ctl into a general install/ctl for tool binaries and extensions](.ticket/tickets/f3c2b8a9-1d2e-4c3f-9b8a-0a1b2c3d4e5f/ticket.toml)), but it has no schema versioning, no safety classification, no lifecycle-capability declaration, no hook coverage, and no generated human-readable catalog. Agents and `install-ctl` need one canonical, versioned source of truth for every repository-delivered command surface instead of re-deriving it per tool.

# Dependent expectation

If this spec is implemented, dependents can rely on the following:

- `tools/install/artifacts.toml` is the single canonical, versioned, committed registry for every repository-delivered Cargo binary target, executable script, managed service, VS Code extension, and git hook. No second registry file is introduced.
- Every registry entry declares a stable `id`, a `source path`, a command template or invocation metadata sufficient to run it, an `owner`, a `category`, an explicit `lifecycle` capability set (the operations that entry supports: e.g. `install`, `start`, `stop`, `restart`, `inspect`), and a `safety` classification of exactly `safe` or `approval-required` — no third safety level exists.
- A generator renders the registry into a committed root [COMMANDS.md](COMMANDS.md) Markdown catalog listing every entry and its declared actions; the generator is deterministic, so re-running it against an unchanged registry produces byte-identical output.
- A freshness check proves the committed `COMMANDS.md` is exactly what the generator currently produces from `tools/install/artifacts.toml`, so registry and catalog cannot silently drift apart.
- Every entry classified `safe` is bounded and read-only (for example: a read-only CLI query or catalog inspection); every other capability — install, start, stop, restart, uninstall, or any state-mutating action — requires `approval-required` classification regardless of entry category.
- Hooks are registered in the same schema as other entries but expose only the `inspect` lifecycle capability in this contract's scope. No `install`, `enable`, or `disable` lifecycle operation is defined for hooks by this contract.
- The registry is repository-scoped only: it enumerates artifacts that exist in this repository's source tree. Discovery of external, non-repository tools is explicitly out of scope and is not implied by any registry entry or generated catalog output.
- `install-ctl` (ticket [c7becdaa install-ctl: Manage registry tools and services across install lifecycle](.ticket/tickets/c7becdaa-6939-4ab9-a8a5-29fbf8921584/ticket.toml)) and Terminal Command Agent's catalog-driven planning (ticket [fdd059ed Drive Terminal Command Agent planning from generated command catalog](.ticket/tickets/fdd059ed-69e4-4328-9167-ea4986aee788/ticket.toml)) both consume this same registry and generated catalog rather than each maintaining a parallel listing.

## Scope

- Extend `tools/install/artifacts.toml` in place with the versioned schema and full entry coverage described above.
- Generate and commit root `COMMANDS.md` from the registry, with a freshness check that fails when the committed file no longer matches generator output.
- Cover Cargo binary targets, executable scripts, managed services, VS Code extensions, and hooks with the required per-entry metadata.
- Classify every entry's every declared action as `safe` (bounded, read-only) or `approval-required` (everything else).
- Register hooks as discoverable/inspectable registry entries only.

## Non-goals

- No external (non-repository) tool discovery of any kind.
- No hook lifecycle operations beyond `inspect` (no install/enable/disable/uninstall for hooks) in this contract.
- No change to the Terminal Command Agent's existing fixed VS Code client tool allowlist — runtime schemas remain the source of truth for client tool invocation; the generated catalog becomes the source of truth for *repository command* selection only, and only once ticket [fdd059ed](.ticket/tickets/fdd059ed-69e4-4328-9167-ea4986aee788/ticket.toml) is implemented.
- No third safety level and no per-entry custom safety taxonomy — exactly `safe` and `approval-required`.

# Guards

No `ValidationSpec` guard ids are registered yet — this spec-authoring session has no `test-mcp`/`test-cli` write access, so guards cannot be materialized as real test-store entities from here. The following guards are planned and must be registered as real `ValidationSpec` ids before this spec can be marked `verified`:

- `val-registry-schema-parse` — parses and validates `tools/install/artifacts.toml`: every entry has a unique `id`, and all required metadata fields (`source path`, command template/invocation metadata, `owner`, `category`, `lifecycle`, `safety`) are present.
- `val-registry-catalog-render` — renders the catalog from the registry and asserts every registry entry and its declared actions appear in the rendered output.
- `val-registry-catalog-freshness` — regenerates `COMMANDS.md` from the current registry and asserts byte-identical equality with the committed file.
- `val-registry-safety-bound` — asserts every entry classified `safe` exposes only bounded read-only actions, and every other action (install/start/stop/restart/uninstall) is classified `approval-required`.
- `val-registry-hook-inspect-only` — asserts every hook entry exposes only the `inspect` lifecycle capability and no install/enable/disable capability.

# Positions

- `partial` — `tools/install/artifacts.toml`: exists today as a runtime-read registry for `install-ctl` covering Cargo binaries and services, but has no schema version marker, no `safety` classification, no `lifecycle` capability set, and no hook or extension coverage yet.
- `not-implemented` — versioned schema fields (`owner`, `category`, `lifecycle`, `safety`) on registry entries.
- `not-implemented` — generated root `COMMANDS.md` catalog and its generator.
- `not-implemented` — freshness validation proving the committed catalog matches generator output.
- `not-implemented` — hook registry entries and their `inspect`-only lifecycle exposure.
- `not-implemented` — `install-ctl` lifecycle integration against the extended registry (owned by ticket [c7becdaa](.ticket/tickets/c7becdaa-6939-4ab9-a8a5-29fbf8921584/ticket.toml)).
- `not-implemented` — Terminal Command Agent catalog-driven planning (owned by ticket [fdd059ed](.ticket/tickets/fdd059ed-69e4-4328-9167-ea4986aee788/ticket.toml)); the agent's existing fixed VS Code client tool allowlist is unaffected by this spec.

# Governing-rule requirement

No governing PolicyRule for this spec's component (`tooling`) was found in the rule store at authoring time (`rule.exe search`/`rule.exe get` returned no match for a tooling/registry-introduction rule). This spec-authoring session has no `rule-mcp` write access, so a governing PolicyRule cannot be created here. A PolicyRule that introduces this spec by computed readiness (per the `rule-introduces-spec` mechanism in [spec-system.instructions.md](.agents/instructions/spec/spec-system.instructions.md)) must be authored and linked here before this spec can be considered fully governed; until then this position is `not-implemented`.

# Acceptance Criteria

1. `tools/install/artifacts.toml` parses under the extended schema; every entry has a unique `id` and all required fields (`source path`, command template/invocation metadata, `owner`, `category`, `lifecycle`, `safety`).
2. The generated `COMMANDS.md` catalog lists every registry entry and every action declared in that entry's `lifecycle` set.
3. Regenerating `COMMANDS.md` from the current registry produces output byte-identical to the committed file (freshness check passes on an unmodified registry, fails on a modified one).
4. Every entry whose `safety` is `safe` exposes only bounded read-only actions; every entry with any other action (install, start, stop, restart, uninstall) is classified `approval-required`. No entry has a `safety` value other than `safe` or `approval-required`.
5. Every hook entry in the registry declares only the `inspect` lifecycle capability; no hook entry declares install, enable, or disable.
6. The registry contains no entry describing a tool that is not part of this repository's source tree (no external-tool discovery entries).

# Traceability

- [495125df Canonical executable and hook registry for command execution](.ticket/tickets/495125df-257d-4a56-84cb-784ea822a1d7/ticket.toml) (epic)
- [f52cc8e5 Define executable and hook registry schema with Markdown catalog generation](.ticket/tickets/f52cc8e5-9faf-4a41-9c5b-ad7c2a381dd9/ticket.toml) (primary implementation ticket for this contract)
- [c7becdaa install-ctl: Manage registry tools and services across install lifecycle](.ticket/tickets/c7becdaa-6939-4ab9-a8a5-29fbf8921584/ticket.toml) (dependent)
- [fdd059ed Drive Terminal Command Agent planning from generated command catalog](.ticket/tickets/fdd059ed-69e4-4328-9167-ea4986aee788/ticket.toml) (dependent)
- [15234799 Register repository hooks in command registry](.ticket/tickets/15234799-d540-4e49-9bf2-4514b768cb79/ticket.toml) (dependent)
- [f3c2b8a9 install-ctl: Generalize viewer-ctl into a general install/ctl for tool binaries and extensions](.ticket/tickets/f3c2b8a9-1d2e-4c3f-9b8a-0a1b2c3d4e5f/ticket.toml) (foundation ticket owning `tools/install/artifacts.toml` today)
