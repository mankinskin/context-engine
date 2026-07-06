# Goal
Triage `dead_code` compiler warnings in the `memory-viewers` submodule (split off from parent `9347c9f8` mechanical pass).

# Scope (host `cargo check --workspace` counts)
- ~38 `dead_code` warnings.
- ticket-viewer frontend (`ticket-viewer-dioxus`):
  - `types.rs` (18)
  - `sse.rs` (3)
  - `api/backend.rs` (1)
- spec-viewer frontend (`spec-viewer-dioxus`):
  - `types.rs` (10)
  - `api.rs` (4)
  - `routes.rs`, `store.rs` (1 each)

# Approach
- Frontend `types.rs` dead_code is often unused response fields / DTO variants. Prefer `#[allow(dead_code)]` on intentional API-mirror types over deletion; delete only clearly obsolete items.

# Validation
- `cargo check -p ticket-viewer-dioxus -p spec-viewer-dioxus` clean of dead_code for touched files.
- Playwright/browser smoke if any behavior-adjacent change.

# Notes
- Submodule commit: changes land in `memory-viewers`, then update pointer in root repo.
- Mechanical warnings already resolved by parent `9347c9f8` (5 spec-viewer files fixed).