Reviewer approved PILOT-TICKET (61ce77f9) on substance but gated closure on a convention fix.

The new pilot crate `memory-api/crates/ticket/Cargo.toml` still declares its `memory-fixtures` dependency as a path dep:

    memory-fixtures = { path = "../memory-fixtures" }

while every other rewired consumer (spec-api dev-deps, ticket-api dev-deps, memory-matrix normal dep) now resolves memory-fixtures from the git remote:

    memory-fixtures = { git = "https://github.com/mankinskin/memory-fixtures", branch = "main" }

This was left as a path dep intentionally to keep the just-validated pilot green during the migration. Now that the pilot verdict has landed (approved), align the pilot crate's `memory-fixtures` dep to the same git pin for convention consistency, then re-run the pilot validation:

    cargo build -p ticket
    cargo build -p ticket --features cli,mcp,http
    cargo test -p ticket --features cli,mcp,http --test reference_proof

## Acceptance Criteria
- `memory-api/crates/ticket/Cargo.toml` resolves `memory-fixtures` from the git remote (`branch = "main"`), matching the other consumers.
- Slim build, all-feature build, and reference_proof (6 passed) remain green after the change.
- Once green, PILOT-TICKET (61ce77f9) is closed to done.