## Frontend Rules

- Reuse shared UI primitives/styles from `tools/viewer/viewer-api/frontend/ts` where possible.
- Keep viewer-specific features modular; avoid duplicating shared components.
- Prefer explicit loading/error/empty states for all async data views.
- Keep theme/effects integration centralized so log-viewer and ticket-viewer can share behavior.
- Preserve keyboard navigation and visible focus states for interactive controls.
- Verify responsive behavior for desktop, tablet, and narrow mobile widths.