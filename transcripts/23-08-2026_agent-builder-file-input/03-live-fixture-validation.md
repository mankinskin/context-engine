# 03: Executable Fixture Validation

## Outcome

Create `workflow-tools/agent-builder/test-fixtures/age-lookup/` as an isolated checked-in fixture. It contains `agent-templates/`, agent-builder configuration, an attached file describing a fictional person, and `.ticket/` data containing that person's age in prose. Add `workflow-tools/agent-builder/tests/age_lookup_e2e.rs`; its live CLI test asks, "How old is the person?", parses the exact template-required response `{"age": <integer>}`, and verifies the returned age.

## Evidence

- [memory-api/crates/memory-fixtures/src/lib.rs](../../memory-api/crates/memory-fixtures/src/lib.rs) provides fixture materialization into a temporary workspace.
- [workflow-tools/agent-builder/src/main.rs](../../workflow-tools/agent-builder/src/main.rs) already authenticates through environment variables.

## Non-goal

Do not treat a live provider test as the only verification layer, record credentials in fixtures, or build a general regression suite for every model/provider.

## Validation Method

Run `cargo test --manifest-path workflow-tools/agent-builder/Cargo.toml` for offline coverage. The ignored live test must fail fast unless `COPILOT_API_KEY` and `OPENAI_API_KEY` are both set, matching the current CLI's documented `copilot::Client::from_env()` inputs. With both values available, run `cargo test --manifest-path workflow-tools/agent-builder/Cargo.toml --test age_lookup_e2e -- --ignored` and require exactly `{"age": <fixture-age>}`.