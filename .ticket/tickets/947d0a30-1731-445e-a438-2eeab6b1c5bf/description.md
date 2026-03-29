# Style: Theme Palette Resource Driving SVO Materials and Glass Tints

## Problem

All visual elements — SVO voxel colors, glass panel tints, particle colors, lighting — must be driven by a single `ThemePalette` Bevy resource so the entire scene can be re-themed at runtime.

## Architecture: Palette → SVO Materials

### ThemePalette Resource

```rust
#[derive(Resource, Clone)]
pub struct ThemePalette {
    // Voxel materials
    pub voxel_primary: Color,     // main world surface
    pub voxel_secondary: Color,   // accent surfaces
    pub voxel_terrain: Color,     // ground/floor
    pub voxel_highlight: Color,   // selected/hovered voxels

    // Glass
    pub glass_tint: Color,        // default UI panel tint
    pub glass_frosted_tint: Color,// frosted panel tint
    pub glass_accent: Color,      // highlighted panel tint

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
```

### Palette → SVO Color Data

When the palette changes, voxel `color_data` fields that reference palette slots are updated:

```rust
#[derive(Copy, Clone)]
pub enum VoxelMaterial {
    PalettePrimary,
    PaletteSecondary,
    PaletteTerrain,
    PaletteHighlight,
    Custom(Color),
}

impl VoxelMaterial {
    pub fn resolve(&self, palette: &ThemePalette) -> u32 {
        let color = match self {
            Self::PalettePrimary => palette.voxel_primary,
            Self::PaletteSecondary => palette.voxel_secondary,
            Self::PaletteTerrain => palette.voxel_terrain,
            Self::PaletteHighlight => palette.voxel_highlight,
            Self::Custom(c) => *c,
        };
        pack_rgba(color)
    }
}
```

### Theme Change System

When the palette changes, re-pack all palette-referenced voxels:

```rust
fn theme_update_system(
    palette: Res<ThemePalette>,
    mut voxel_world: ResMut<VoxelWorld>,
) {
    if palette.is_changed() {
        // Re-resolve palette-referenced voxels
        for (idx, mat) in voxel_world.material_refs.iter() {
            let new_color = mat.resolve(&palette);
            if voxel_world.nodes[*idx].color_data != new_color {
                voxel_world.nodes[*idx].color_data = new_color;
                voxel_world.mark_dirty_node(*idx);
            }
        }
        // Glass panels re-read palette via their tint field → handled by glass_panel_uniform_system
        // Lights re-read palette via GlobalUniforms → handled by light_uniform_system
    }
}
```

### Bevy ECS Integration

Palette is a Bevy `Resource` inserted at app startup and modifiable at runtime:

```rust
app.insert_resource(ThemePalette::dark_default())
   .add_systems(Update, theme_update_system);
```

Preset themes:
- `ThemePalette::dark_default()` — dark voxels, cyan glass, warm lighting
- `ThemePalette::light_default()` — bright voxels, subtle glass, cool lighting
- `ThemePalette::high_contrast()` — accessibility theme

## Scope

### Rust: Theme (`src/theme.rs`)
- `ThemePalette` resource with color fields
- `VoxelMaterial` enum with palette resolution
- `theme_update_system` (re-color voxels on palette change)
- Preset constructors (dark, light, high-contrast)

### Rust: Integration
- Glass tints read from palette in `glass_panel_uniform_system`
- Light colors read from palette in `light_uniform_system`
- Particle colors read from palette in `emitter_system`

## Dependencies
- T1 (scaffold): Bevy App with resource registration
- T6 (3D scene): Voxel color_data field drives ray march output
- T3 (liquid glass): Glass tints come from palette

## Acceptance Criteria
1. `ThemePalette` resource exists and is accessible from any Bevy system
2. Changing `palette.voxel_primary` at runtime updates all primary voxels in < 1 frame
3. Glass panel tints reflect palette changes
4. Light colors reflect palette changes
5. At least 2 preset themes (dark, light) with distinct visual appearance
6. Palette change triggers minimal dirty-region upload (only changed voxels)
