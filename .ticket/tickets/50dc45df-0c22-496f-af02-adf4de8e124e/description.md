# Waypoint 6: Specification health policy and validation hook

## Governing specifications

- `.spec/specs/b4475214-e14e-4926-b853-b2553444e36f/body.md`
- `.spec/specs/89360ad7-d638-49e7-85ba-21839fa99851/body.md`

## Owned implementation surface

- `workflow-tools/spec/crates/spec-api/src/manifest.rs` and `store.rs` health reporting
- `workflow-tools/spec/src/cli/commands/query.rs`
- `.spec/health-policy.toml`
- `workflow-tools/test/crates/test-api/` validation-link exposure
- `.github/hooks/hooks.json` and `tools/agent-hooks`

Implement structural health findings and policy-controlled PostToolUse enforcement after W6.1, W6.2, W6.3, and W6.4 deliver their data. Health stays diagnostic and exits successfully with findings; only the hook blocks policy-selected errors. This ticket does not create/migrate manifests, infer relationships, implement the annotation macro, or change ticket lifecycle.

## Acceptance criteria

1. Health reports stable severity/category/policy for v2/version/migration, component/criterion/edge/template/annotation integrity, hierarchy, body/TOML link parity, examples, parent navigation, and prefix registry; `violation` remains distinct from `migration_notice`.
2. `.spec/health-policy.toml` has `policy_version = 1`, validates mappings and versioned expiry/rationale-backed allowlist identities, and keeps the existing three unrelated `9f0b9e30` entries as the only baseline exemptions.
3. PostToolUse validates changed roots plus impacted ancestors, fails closed on validator failure, blocks only policy errors, and leaves explicit `spec health --all` as the complete nonblocking diagnostic command.
4. Validation evidence exposes criterion links and newest-result semantics without a first-class criterion index or implied fulfillment.

## Focused validation

- `cargo test -p spec-api`
- focused test-api tests
- `./target/debug/spec.exe --workspace . health --all`
- hook fixtures for parity/hierarchy/examples/navigation/prefix policy, allowed notices, blocked violations, and validator failure

## Done condition

Health deterministically diagnoses all v2 structural contracts and the hook enforces only the configured policy.