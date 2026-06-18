## Frontend Rules

- Reuse shared TS UI primitives/styles from `tools/viewer/log-viewer/frontend/viewer-api-frontend` where applicable.
- Reuse shared Dioxus viewer primitives and test helpers from `viewer-api/viewer-api/frontend/dioxus` for WASM viewers.
- Keep viewer-specific features modular; avoid duplicating shared components.
- Prefer explicit loading/error/empty states for all async data views.
- Keep theme/effects integration centralized so log-viewer and ticket-viewer can share behavior.
- Preserve keyboard navigation and visible focus states for interactive controls.
- Verify responsive behavior for desktop, tablet, and narrow mobile widths.