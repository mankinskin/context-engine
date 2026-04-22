# spec-viewer: theme settings

The spec-viewer's theme settings panel **MUST** conform to the canonical spec
[`viewer-api/theme-settings`](../viewer-api/theme-settings) — same layout, same 17 sections,
same controls, same persistence keys.

## Viewer-specific overrides

| Field | Value |
|---|---|
| GPU master toggle default | **OFF** (`viewer-api-gpu-enabled` defaults to `"false"`) |
| Default `EffectSettings` | `DEFAULT_EFFECT_SETTINGS_OFF` (all toggles false, glass/blur 0) |
| Presets | inherited from `viewer-api`'s shared preset list (no spec-viewer-specific presets) |

## Rationale

The spec-viewer is a **read-first** viewer for browsing the spec hierarchy. Animated
WebGPU effects (smoke, particles, CRT scanlines) would distract from the rendered Markdown
content, so they ship disabled by default. Users who want the full visual treatment can
enable the master toggle in the GPU Rendering section.

## Implementation pointers

- Panel lives in `tools/viewer/viewer-api/frontend/dioxus/src/components/theme_settings.rs`
  (shared component, mounted by the spec-viewer's settings page).
- Store factory: `ThemeStore::new()` in
  `tools/viewer/viewer-api/frontend/dioxus/src/store/theme.rs` with
  `gpu_enabled: false` default.
- CSS: `tools/viewer/spec-viewer/frontend/dioxus/public/viewer-api.css` — kept in sync with
  the shared copy under `tools/viewer/viewer-api/frontend/dioxus/public/`.
