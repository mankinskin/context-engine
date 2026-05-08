## Stack and Shared Dependencies

- Frontends in this repository use Preact + Vite + TypeScript.
- Prefer `@preact/signals` patterns for shared reactive state where already used.
- Reuse shared package primitives from `@context-engine/viewer-api-frontend` before adding tool-local duplicates.