# VFX: Liquid Glass as Analytical SDF in Ray Marching Loop

## Problem

UI panels must appear as frosted glass floating in 3D space. In the SVO ray marching architecture, glass is NOT a separate post-process pass — it is an **analytical SDF evaluated inside the same ray marching loop** as the voxel world. When a ray hits a glass SDF, Snell's law bends the ray before it continues into the SVO behind the panel.

## Architecture: SDF Glass in Unified Ray March

### Why Not Post-Process Glass?

Post-process glass (sample background, blur, tint) has fundamental limitations:
- Cannot handle overlapping glass panels (only one layer of refraction)
- Cannot bend the view of what's behind the glass (no physical refraction)
- Requires a separate render pass for the background texture
- No physically correct light behavior through glass

SDF glass in the ray marching loop gives us:
- **Physical refraction**: Snell's law changes ray direction at glass surface
- **Stacked panels**: Multiple glass layers refract cumulatively
- **Free soft edges**: SDF distance provides smooth anti-aliased boundaries
- **Zero extra passes**: Same shader, same ray, same pixel

### Glass SDF Evaluation

Each UI panel is a 3D box SDF with rounded corners:

```wgsl
struct GlassPanel {
    center: vec3f,    // world position
    half_size: vec3f, // width/height/depth extents
    corner_radius: f32,
    ior: f32,         // index of refraction (1.5 = glass)
    tint: vec4f,      // RGBA tint color
    blur_roughness: f32, // surface roughness for frosted glass
}

fn sdf_rounded_box(p: vec3f, panel: GlassPanel) -> f32 {
    let q = abs(p - panel.center) - panel.half_size + panel.corner_radius;
    return length(max(q, vec3f(0.0))) + min(max(q.x, max(q.y, q.z)), 0.0) - panel.corner_radius;
}
```

### Ray March Integration

The unified ray march loop checks both SVO voxels and glass SDFs at each step:

```wgsl
fn march_ray(origin: vec3f, dir: vec3f) -> vec4f {
    var ray_pos = origin;
    var ray_dir = dir;
    var accumulated_tint = vec4f(1.0);
    var t = 0.0;

    for (var i = 0u; i < MAX_STEPS; i++) {
        let p = origin + ray_dir * t;

        // 1. Check glass SDF (closest panel)
        let glass_dist = nearest_glass_sdf(p);

        // 2. Check SVO distance
        let svo_dist = query_svo_distance(p);

        // 3. Hit glass first?
        if glass_dist < HIT_THRESHOLD && glass_dist < svo_dist {
            let normal = glass_sdf_normal(p);
            // Snell's law: bend ray direction
            ray_dir = refract(ray_dir, normal, 1.0 / panel.ior);
            // Accumulate tint
            accumulated_tint *= panel.tint;
            t += HIT_THRESHOLD * 2.0; // step past surface
            continue;
        }

        // 4. Hit SVO?
        if svo_dist < HIT_THRESHOLD {
            let color = shade_voxel(p, ray_dir);
            return vec4f(color.rgb * accumulated_tint.rgb, color.a);
        }

        // 5. Step forward by minimum distance
        t += min(glass_dist, svo_dist);
        if t > MAX_DISTANCE { break; }
    }
    return vec4f(0.0); // sky/background
}
```

### Snell's Law Refraction

When a ray enters glass, its direction changes based on the index of refraction:

```wgsl
fn refract_ray(incident: vec3f, normal: vec3f, eta: f32) -> vec3f {
    let cos_i = dot(-incident, normal);
    let sin2_t = eta * eta * (1.0 - cos_i * cos_i);
    if sin2_t > 1.0 {
        return reflect(incident, normal); // total internal reflection
    }
    let cos_t = sqrt(1.0 - sin2_t);
    return eta * incident + (eta * cos_i - cos_t) * normal;
}
```

This creates the physical distortion effect: objects behind glass appear shifted, compressed near edges, and color-tinted.

### Frosted Glass (Roughness)

For frosted/matte glass, the refracted ray direction is slightly randomized:

```wgsl
// After computing refracted direction:
if panel.blur_roughness > 0.0 {
    // Perturb ray using blue noise or hash
    let noise = hash_vec3(p * 1000.0);
    ray_dir = normalize(ray_dir + noise * panel.blur_roughness * 0.1);
}
```

For high-quality frost, sample multiple refracted rays and average (expensive) or use a single-sample approximation with temporal accumulation.

### Bevy ECS Integration

Glass panels are ECS entities, NOT hard-coded shader data:

```rust
#[derive(Component)]
pub struct GlassPanel {
    pub ior: f32,           // default 1.5
    pub tint: Color,        // default white with alpha 0.1
    pub blur_roughness: f32, // 0.0 = clear, 1.0 = fully frosted
    pub corner_radius: f32,
}

// System: collect glass panels into GPU uniform buffer
fn glass_panel_uniform_system(
    query: Query<(&Transform, &GlassPanel, &LayoutRect)>,
    mut glass_buffer: ResMut<GlassPanelBuffer>,
) {
    glass_buffer.panels.clear();
    for (transform, panel, rect) in query.iter() {
        glass_buffer.panels.push(GlassPanelGpu {
            center: transform.translation,
            half_size: rect.half_size_3d(),
            corner_radius: panel.corner_radius,
            ior: panel.ior,
            tint: panel.tint.as_linear_rgba_f32().into(),
            blur_roughness: panel.blur_roughness,
        });
    }
}
```

### Lighting Through Glass

Glass panels cast colored, soft shadows:
- Shadow rays that pass through glass accumulate the panel's tint
- Shadow intensity is modulated by glass opacity
- This is automatic: the shadow ray in `soft_shadow()` also evaluates glass SDFs

### Performance

| Metric | Target |
|--------|--------|
| Glass SDF eval per step | < 0.01ms (analytical, no texture reads) |
| Max panels per frame | 16 (uniform buffer limit; expandable via storage buffer) |
| Refraction overhead | ~5% over non-glass rays (one `refract` + tint multiply) |

## Scope

### WGSL (in `ray_march.wgsl` — shared with T6)
- `GlassPanel` struct definition
- `sdf_rounded_box()` function
- `refract_ray()` using Snell's law
- `nearest_glass_sdf()` — evaluate all panels, return closest
- `glass_sdf_normal()` — gradient-based normal for refraction
- Integration into main `march_ray()` loop

### Rust: ECS
- `GlassPanel` component
- `GlassPanelBuffer` resource (uniform buffer for GPU)
- `glass_panel_uniform_system` (query → pack → upload)

### Rust: Render
- Glass uniform bind group layout (shared with ray march pipeline)
- Panel count pushed as push constant or uniform

## Dependencies
- T6 (3D scene): Ray marching pipeline and shader must exist — glass SDFs integrate into it
- T9 (bridge): Panel positions come from Taffy layout → 3D transform

## Acceptance Criteria
1. A glass panel SDF is visible in the ray-marched scene as a translucent rectangle
2. Objects behind the glass appear refracted (visibly shifted/distorted)
3. Tint color modulates what's seen through the glass
4. Two overlapping glass panels produce cumulative refraction and tinting
5. Frosted glass (roughness > 0) produces a blurred/scattered appearance
6. Glass panel casts a colored soft shadow on voxels behind it
7. No separate render pass — glass evaluation is inside `march_ray()` loop
8. Total internal reflection occurs at extreme viewing angles (Snell's law edge case)
