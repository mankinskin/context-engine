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

// ---- element kind constants for effect preview containers -------------------
const KIND_FX_SPARK   : f32 = 8.0;
const KIND_FX_EMBER   : f32 = 9.0;
const KIND_FX_BEAM    : f32 = 10.0;
const KIND_FX_GLITTER : f32 = 11.0;

// ---- index ranges per particle type (must match TypeScript) -----------------
const SPARK_END   : u32 = 96u;
const EMBER_END   : u32 = 288u;
const RAY_END     : u32 = 544u;
const GLITTER_END : u32 = 640u;

// ---- uniforms (192 bytes = 48 × f32) ----------------------------------------
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
    crt_scanlines_h  : f32,    // horizontal scanlines (+grid) intensity 0.0–1.0
    crt_scanlines_v  : f32,    // vertical scanlines (+grid) intensity 0.0–1.0
    crt_edge_shadow  : f32,    // edge/border shadow intensity 0.0–1.0
    crt_flicker      : f32,    // torch flicker intensity 0.0–1.0
    cursor_style     : f32,    // 0 = default, 1 = metal, 2 = glass
    smoke_intensity  : f32,    // background smoke brightness 0.0–1.0
    smoke_speed      : f32,    // smoke animation speed multiplier 0.0–5.0
    smoke_warm_scale : f32,    // UV scale for warm smoke layers 0.0–2.0
    smoke_cool_scale : f32,    // UV scale for cool wisp layer 0.0–2.0
    smoke_moss_scale : f32,    // UV scale for moss-tone blending 0.0–2.0
    grain_intensity  : f32,    // grain brightness/amplitude 0.0–1.0
    grain_coarseness : f32,    // grain frequency scale 0.0–1.0
    grain_size       : f32,    // grain pixel block size (1–8 px, normalized 0.0–1.0)
    vignette_str     : f32,    // edge vignette darkening 0.0–1.0
    underglow_str    : f32,    // warm bottom underglow 0.0–1.0
    spark_speed      : f32,    // metal spark speed multiplier 0.0–3.0
    ember_speed      : f32,    // ember/ash speed multiplier 0.0–3.0
    beam_speed       : f32,    // angelic beam speed multiplier 0.0–3.0
    glitter_speed    : f32,    // glitter speed multiplier 0.0–3.0
    beam_height      : f32,    // beam quad height multiplier (default 35.0)
    beam_count       : f32,    // max active beams (0 = all slots)
    beam_drift       : f32,    // beam upward drift distance multiplier 0.0–3.0
    scroll_dx        : f32,    // scroll delta X this frame (pixels, screen space)
    scroll_dy        : f32,    // scroll delta Y this frame (pixels, screen space)
    spark_count      : f32,    // max active sparks (fraction of slots, 0–2)
    spark_size       : f32,    // spark size multiplier 0.0–3.0
    ember_count      : f32,    // max active embers (fraction of slots, 0–2)
    ember_size       : f32,    // ember size multiplier 0.0–3.0
    glitter_count    : f32,    // max active glitter (fraction of slots, 0–2)
    glitter_size     : f32,    // glitter size multiplier 0.0–3.0
    cinder_size      : f32,    // cinder border glow size multiplier 0.0–3.0
    _pad0            : f32,    // padding to 48 f32 (192 bytes)
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
