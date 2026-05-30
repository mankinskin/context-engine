## Shared Frontend Package Usage

- Shared TS UI/style primitives currently live under `tools/viewer/log-viewer/frontend/viewer-api-frontend/`.
- Shared Dioxus viewer primitives and test helpers live under `memory-viewers/viewer-api/viewer-api/frontend/dioxus/`.
- Place cross-viewer reusable components in the shared package, not copied per tool.
- Keep tool-specific behavior in each tool frontend and shared behavior in viewer-api frontend.