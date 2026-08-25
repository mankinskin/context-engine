## Problem

No public workflow-tools installation story is currently provable. The context-engine workspace uses local Cargo `[patch]` overrides and the existing contract-reference project has local source access, so neither can establish that an external consumer can bootstrap, resolve, install, invoke, and persist workflow-tool data through public interfaces.

## Clean-Environment Scenario

1. Start in a fresh temporary directory or checkout with no Cargo `[patch]` overrides, no vendored workflow tooling, and no copied context-engine workflow sources.
2. Bootstrap `workflow-skill` using the documented public installation command.
3. Resolve the minimal consumer's public version-pinned Cargo workflow dependency with no local path dependency or `[patch]` override.
4. Install the selected workflow transport into a caller-controlled location and invoke the installed transport.
5. Create one ticket record and one spec record in consumer-owned stores, then read both records back and verify their declared identifiers and expected fields.

## Validation

| Command | Owning-spec criterion | Required evidence |
| --- | --- | --- |
| `cargo metadata --format-version 1 --no-deps` | MEC-1 | Workflow-tool dependencies report external source identities with no local path or patch override. |
| `bash fixtures/minimal-consumer/run-tutorial.sh` | MEC-2, MEC-4 | The clean tutorial completes and creates then reads back the declared ticket and spec records from consumer-owned stores. |
| `cargo build --manifest-path fixtures/minimal-consumer/Cargo.toml` | MEC-3 | The minimal consumer builds against public dependencies. |

## Non-Goals

- No viewer or browser tests.
- No multi-domain transport coverage.
- No artifact-store migration.
- No context-engine cutover.

## Implementation Plan

1. Add the isolated `workflow-tools/fixtures/minimal-consumer/` consumer and its tutorial without reusing the existing contract-reference project.
2. Implement the clean-environment bootstrap, public dependency resolution, installed transport invocation, and ticket/spec record read-back described by owning spec `bc639ab3`.
3. Run the validation commands above and record outcomes against MEC-1 through MEC-4.
