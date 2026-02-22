// types.wgsl — shared struct definitions for all shader modules
//
// Concatenated after palette.wgsl and before noise.wgsl / pipeline files.
// Declares the palette uniform binding (shared across ALL pipelines).

// ---- palette uniform (binding 3, shared by compute + render) ----------------
@group(0) @binding(3) var<uniform> palette : ThemePalette;

// ---- particle kind constants ------------------------------------------------
const PK_METAL_SPARK : f32 = 0.0;
const PK_EMBER       : f32 = 1.0;
const PK_GOD_RAY     : f32 = 2.0;
const PK_GLITTER     : f32 = 3.0;

// ---- index ranges per particle type (must match TypeScript) -----------------
const SPARK_END   : u32 = 96u;
const EMBER_END   : u32 = 288u;
const RAY_END     : u32 = 416u;
const GLITTER_END : u32 = 512u;

// ---- uniforms (48 bytes = 12 × f32) ----------------------------------------
struct Uniforms {
    time             : f32,
    width            : f32,
    height           : f32,
    element_count    : f32,
    mouse_x          : f32,
    mouse_y          : f32,
    delta_time       : f32,
    hover_elem       : f32,
    hover_start_time : f32,
    selected_elem    : f32,    // index of selected element (-1 if none)
    _pad2            : f32,
    _pad3            : f32,
}

// ---- DOM element rectangle --------------------------------------------------
struct ElemRect {
    rect : vec4f,   // x, y, w, h
    hue  : f32,
    kind : f32,
    _p1  : f32,
    _p2  : f32,
}

// ---- particle state (48 bytes = 12 × f32) -----------------------------------
struct Particle {
    pos      : vec2f,
    vel      : vec2f,
    life     : f32,
    max_life : f32,
    hue      : f32,
    size     : f32,
    kind     : f32,     // PK_METAL_SPARK, PK_EMBER, PK_GOD_RAY
    spawn_t  : f32,     // absolute time when particle was spawned
    _p1      : f32,
    _p2      : f32,
}
