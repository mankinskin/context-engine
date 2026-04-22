# ticket-viewer: theme settings

The ticket-viewer's theme settings panel **MUST** conform to the canonical spec
[`viewer-api/theme-settings`](../viewer-api/theme-settings) — same layout, same 17 sections,
same controls, same persistence keys.

## Viewer-specific overrides

| Field | Value |
|---|---|
| GPU master toggle default | **OFF** (`viewer-api-gpu-enabled` defaults to `"false"`) |
| Default `EffectSettings` | `DEFAULT_EFFECT_SETTINGS_OFF` (all toggles false, glass/blur 0) |
| Presets | inherited from `viewer-api`'s shared preset list (no ticket-viewer-specific presets) |

## Rationale

The ticket-viewer is primarily used for browsing and editing tickets in lists/forms.
The full WebGPU effect set (smoke, particles, CRT) is intentionally opt-in so that the
viewer is fast, low-power, and unobtrusive on first load. The graph3d view continues to
use WebGPU regardless of the master toggle (it owns the canvas via `GPU_CANVAS_OWNER`).

## Implementation pointers

- Panel lives in `tools/viewer/viewer-api/frontend/dioxus/src/components/theme_settings.rs`
  (shared component).
- Store: `ThemeStore` in `tools/viewer/viewer-api/frontend/dioxus/src/store/theme.rs` with
  `gpu_enabled: false` default.
- CSS: `tools/viewer/ticket-viewer/frontend/dioxus/public/viewer-api.css` — kept in sync with
  the shared copy under `tools/viewer/viewer-api/frontend/dioxus/public/`.
- Canvas ownership arbitration with graph3d: `GPU_CANVAS_OWNER` thread-local in
  `tools/viewer/viewer-api/frontend/dioxus/src/effects/mod.rs`.
