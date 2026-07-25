Phase E. Reframe context-engine as an instantiated example of a target environment: it retains only the context-stack (context-api/insert/read/search/trace) plus its own generated artifacts, and consumes the workflow tooling as an installed dependency rather than vendoring it.

## Consumption model (reviewed 2026-07-25)
- context-engine depends on each tool's **domain crate lib** (for in-process/library use) AND **invokes the installed transport bins** (mcp/cli/http) as external tools.
- Transport bins are selected via features per the domain-crate contract (`0da6894c`); context-engine enables the transports it actually invokes.

## Scope
- Remove workflow tooling from context-engine as vendored submodules; declare `workflow-tools` (domain crates + selected transport bins) as an installed dependency.
- Keep context-stack and context-engine's own artifact stores (.ticket/.spec/.rule/...).
- Ensure the workflow tools/artifacts remain usable after installation.
- Update install scripts (install-tools.sh, install-deps.sh) to install the workflow-tools dependency and the transport bins.

## Acceptance criteria
- context-engine builds/operates with workflow-tools installed (domain crate libs + transport bins).
- Only context-stack + context-engine artifacts remain in-repo.
- Documented install step reproduces a working environment from a clean checkout.

## Dependencies
- Blocked by umbrella creation, artifact migration, and skill packaging.