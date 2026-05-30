## TypeScript Type Generation

- Do not hand-edit generated files under `packages/context-types/src/generated/`.
- Generate types from Rust `ts-rs` exports using `viewer-ctl gen-types` (or `cargo make gen-types`).
- PowerShell variant also available at `scripts/generate-types.ps1`.
- For context-api type exports, maintain feature-gated generation patterns (`ts-gen`) where required.