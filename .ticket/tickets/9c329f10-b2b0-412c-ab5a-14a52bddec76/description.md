# Goal
Triage `dead_code` compiler warnings in the `viewer-api` submodule (split off from parent `9347c9f8` mechanical pass). Largest concentration.

# Scope (host `cargo check --workspace` counts)
- ~109 `dead_code` warnings.
- Backend: `viewer-api/viewer-ctl/src/config.rs` (1).
- Frontend (`viewer-api-dioxus`):
  - `graph3d/data.rs` (70 — dominant)
  - `effects/wgpu_overlay/element_types.rs` (15)
  - `graph3d/mod.rs` (6)
  - `graph3d/theme.rs` (5)
  - `components/theme_settings/model.rs` (5)
  - `store/theme.rs` (4)
  - `effects/wgpu_overlay/settings.rs`, `components/theme_settings/preview.rs`, `store/session.rs` (1 each)

# Approach
- Much of the Dioxus/wgpu frontend dead_code is likely intentional scaffolding for in-progress 3D/theming features. Prefer `#[allow(dead_code)]` with rationale over deletion unless clearly abandoned.

# Validation
- `cargo check -p viewer-ctl -p viewer-api-dioxus` clean of dead_code for touched files.
- Browser smoke check for viewer-api frontend if any rendering-adjacent code changes.

# Notes
- Submodule commit: changes land in `viewer-api`, then update pointer in root repo.
- Mechanical warnings already resolved by parent `9347c9f8`.