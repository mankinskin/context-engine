# Style: Theme Palette Driving SVO Materials, Gaussian SH, and Glass Tints

## Problem

All visual elements — SVO voxel colors, Gaussian Spherical Harmonics coefficients, glass panel tints, particle colors, lighting — must be driven by a single `ThemePalette` Bevy resource for runtime re-theming.

## Architecture: Palette → SH Coefficients → Gaussian Color

### ThemePalette Resource

```rust
#[derive(Resource, Clone)]
pub struct ThemePalette {
    // Voxel materials (stored in SVO, drive Gaussian generation)
    pub voxel_primary: MaterialDef,
    pub voxel_secondary: MaterialDef,
    pub voxel_terrain: MaterialDef,
    pub voxel_highlight: MaterialDef,

    // Glass
    pub glass_tint: Color,
    pub glass_frosted_tint: Color,
    pub glass_accent: Color,

    // Particles
    pub particle_primary: Color,
    pub particle_secondary: Color,

    // Lighting
    pub ambient_color: Color,
    pub key_light_color: Color,
    pub fill_light_color: Color,

    // UI text
    pub text_primary: Color,
    pub text_secondary: Color,
}

#[derive(Clone)]
pub struct MaterialDef {
    pub base_color: Color,
    pub roughness: f32,
    pub metallic: f32,
}
```

### Palette → Spherical Harmonics

When the Gaussian generator creates splats from SVO voxels, it reads the voxel's `color_data` and the palette's material properties to produce SH coefficients:

- **Diffuse materials** (roughness ≈ 1.0): SH band 0 only (view-independent color)
- **Glossy materials** (roughness < 0.5): SH bands 0–2 (view-dependent highlights)
- **Metallic materials**: SH coefficients tinted by base color (colored reflections)

```wgsl
fn material_to_sh(color: vec3f, roughness: f32, metallic: f32) -> array<f32, 48> {
    var sh: array<f32, 48>;
    // Band 0 (DC): base color
    sh[0] = color.r * 0.282;
    sh[1] = color.g * 0.282;
    sh[2] = color.b * 0.282;
    // Higher bands: strength inversely proportional to roughness
    let spec_scale = (1.0 - roughness) * 0.5;
    // Band 1-3 coefficients for view-dependent appearance...
    return sh;
}
```

### Theme Change System

```rust
fn theme_update_system(
    palette: Res<ThemePalette>,
    mut voxel_world: ResMut<VoxelWorld>,
) {
    if palette.is_changed() {
        for (idx, mat) in voxel_world.material_refs.iter() {
            let new_color = mat.resolve(&palette);
            if voxel_world.nodes[*idx].color_data != new_color {
                voxel_world.nodes[*idx].color_data = new_color;
                voxel_world.mark_dirty_node(*idx);
            }
        }
        // Dirty SVO → re-upload → Gaussian generator re-reads colors → new SH on next frame
        // Glass panels re-read palette tints via glass_panel_uniform_system
    }
}
```

A palette change causes: dirty SVO upload → Gaussian generator re-emits with new SH → tiled renderer shows new colors. One frame latency via double buffering.

### Presets

```rust
impl ThemePalette {
    pub fn dark_default() -> Self { /* dark voxels, cyan glass, warm key light */ }
    pub fn light_default() -> Self { /* bright voxels, subtle glass, cool lighting */ }
    pub fn high_contrast() -> Self { /* accessibility theme */ }
}
```

## Dependencies
- T1 (scaffold): Bevy App with resource registration
- T6 (3D scene): Gaussian generator reads color_data + roughness from SVO
- T3 (liquid glass): Glass tints come from palette

## Acceptance Criteria
1. `ThemePalette` resource accessible from any Bevy system
2. Changing `palette.voxel_primary` updates all primary voxels → new Gaussians with new SH
3. Glossy materials show view-dependent highlights via SH bands 1–3
4. Glass panel tints reflect palette changes
5. At least 2 preset themes (dark, light) with distinct visual appearance
6. Theme change propagates within 1 frame (via double-buffered SVO)
