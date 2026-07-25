Phase E. Reframe context-engine as an instantiated example of a target environment: it retains only the context-stack (context-api/insert/read/search/trace) plus its own generated artifacts, and consumes the workflow tooling as an installed dependency rather than vendoring it.

## Scope
- Remove workflow tooling from context-engine as vendored submodules; declare `workflow-tools` as an installed dependency.
- Keep context-stack and context-engine's own artifact stores (.ticket/.spec/.rule/...).
- Ensure the workflow tools/artifacts remain usable in context-engine after installation of workflow-tools.
- Update install scripts (install-tools.sh, install-deps.sh) to install the workflow-tools dependency.

## Acceptance criteria
- context-engine builds/operates with workflow-tools installed as a dependency.
- Only context-stack + context-engine artifacts remain in-repo.
- Documented install step reproduces a working environment from a clean checkout.

## Dependencies
- Blocked by umbrella creation, artifact migration, and skill packaging.