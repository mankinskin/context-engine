# Impl: Color theme and palette system — Bevy resource + GPU uniforms + live switching

## Problem

The context-editor needs a versatile theme system stored as a **Bevy resource** that streams palette colors to the GPU as uniforms, supports live theme switching without pipeline recreation, and provides presets for different editor moods.

## Architecture: Bevy Resource

The `ThemePalette` is a **Bevy resource** that all rendering systems read:
- Glass render node reads palette for tint/glow colors
- Particle system reads palette for particle colors
- Scene pass reads palette for fog/sky/ambient colors
- A Bevy system uploads the palette to GPU each frame when dirty

## Scope

### Palette Uniform Buffer
- 24+ color slots as `vec4<f32>` (matches existing palette.wgsl layout)
- Uploaded once per frame via Bevy system when `ThemePalette` resource is changed
- Theme switch = resource update → buffer upload in same frame, no shader recompilation

### Theme Presets
- Dark (default): deep blues, muted grays, cyan accents
- Paper: warm beige, brown, sepia tones
- Neon: vibrant purples, electric blues, hot pink accents
- Forest: deep greens, earth tones, golden highlights
- Custom: user-defined via parameter sliders

### Shader Integration
- `shaders/palette.wgsl` — shared palette data structure (#include pattern)
- All shaders (glass, particles, scene, UI) read palette colors from uniform
- Glass tint, particle glow, background fog all derive from active theme

### Bevy API (`src/gpu/theme.rs`)
```rust
#[derive(Resource)]
struct ThemePalette {
    colors: [Vec4; 24],
    dirty: bool,
    preset: ThemePreset,
}
```
- `set_theme(preset)`, `set_color(slot, color)`
- Theme persistence via localStorage (web-sys)
- Dioxus state integration: theme signal writes to Bevy resource via shared channel

### CSS Variable Bridge
- Theme colors also exported as CSS variables for DOM text elements
- `--text-color`, `--accent-color`, `--bg-color` etc.
- Ensures DOM overlay text matches GPU-rendered UI aesthetic

## Reuse from Existing Code
- Port palette.wgsl from `log-viewer/frontend/src/effects/palette.wgsl`
- Port theme preset patterns from `log-viewer/frontend-leptos/src/theme.rs`
- Port color hex conversion utilities from `viewer-api/frontend/src/utils/`

## Files to Create
| File | Purpose |
|------|---------|
| `shaders/palette.wgsl` | Shared palette uniform struct |
| `src/gpu/theme.rs` | `ThemePalette` Bevy resource + presets |

## Acceptance Criteria
1. At least 4 theme presets with distinct visual identities
2. Theme switch updates all GPU shaders in the same frame (no flicker)
3. DOM overlay CSS variables sync with GPU palette colors
4. Custom color slots editable at runtime
5. Theme persists across page reloads via localStorage
6. Particle, glass, and background shaders all read from the same palette Bevy resource
