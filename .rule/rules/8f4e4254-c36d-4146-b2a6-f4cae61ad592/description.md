## Shared Frontend Package Usage

- Shared UI and style primitives live under `tools/viewer/viewer-api/frontend/ts/`.
- Place cross-viewer reusable components in the shared package, not copied per tool.
- Keep tool-specific behavior in each tool frontend and shared behavior in viewer-api frontend.