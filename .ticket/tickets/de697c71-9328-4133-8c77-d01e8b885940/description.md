# Impl: Open parameter manipulation UI — Bevy resources for physics, lighting, colors, and shaders

## Problem

The context-editor needs a flexible parameter manipulation panel that lets users tweak all simulation and rendering parameters in real time. All parameters are **Bevy resources** that rendering systems, physics, and shaders read each frame.

## Architecture: Parameters as Bevy Resources

Every tweakable value is stored in a Bevy resource. The parameter panel (Dioxus UI) writes to these resources via a shared channel, and Bevy systems read them to update GPU uniforms, physics config, and lighting:

```
Dioxus slider → shared channel → Bevy resource → Bevy system → GPU uniform / Rapier config
```

## Scope

### Parameter Registry (`src/ui/params.rs`)
- Central registry of all tweakable parameters
- `ParamDef { name, group, min, max, step, default, current }` for numeric params
- `ColorParam { name, group, current_rgba }` for color params
- `BoolParam`, `EnumParam` for toggles and selections
- Groups: "Physics", "Lighting", "Glass", "Particles", "Environment", "Camera"

### Parameter Panel Component (`src/ui/param_panel.rs`)
- Dioxus component: collapsible groups with sliders/color pickers/toggles
- Glass panel background (uses `use_glass_panel` hook from T9)
- Live preview: parameter change immediately writes to Bevy resource → GPU uniform updates next frame
- Reset to default button per parameter and per group

### Parameter → Bevy Resource Bindings
| Group | Parameters | Bevy Resource |
|-------|-----------|---------------|
| Physics | gravity, damping, friction, wind_strength, wind_dir | `RapierConfiguration`, `WindConfig` |
| Lighting | sun_dir, sun_color, ambient, specular_power | `DirectionalLight` entity, `AmbientLight` resource |
| Glass | refraction_intensity, chromatic_strength, corner_radius, edge_glow | `GlassConfig` resource → glass render node uniforms |
| Particles | spawn_rate, lifetime, speed, size, type_weights | `ParticleConfig` resource → compute shader uniforms |
| Environment | fog_density, fog_color, sky_zenith, sky_horizon, day_speed | `EnvironmentConfig` resource → environment shader uniforms |
| Camera | fov, near_clip, far_clip, orbit_speed, zoom_speed | Camera entity `Projection` + `CameraMode` component |

### Persistence
- Parameter state saved to localStorage
- Import/export as JSON for sharing presets

## Files to Create
| File | Purpose |
|------|---------|
| `src/ui/params.rs` | Parameter registry + definitions |
| `src/ui/param_panel.rs` | Dioxus parameter panel component |

## Acceptance Criteria
1. Slider changes immediately visible in GPU rendering (< 1 frame latency via Bevy resource)
2. Color picker updates `ThemePalette` Bevy resource in real time
3. Physics parameters affect Rapier simulation when changed (`RapierConfiguration`)
4. All parameter groups collapsible/expandable
5. Reset to default restores original values
6. Parameters persist across page reloads
7. At least 20 parameters across 6 groups defined
