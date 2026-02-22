// noise.wgsl — procedural noise, RNG, and colour helpers
//
// Shared utility functions concatenated between types.wgsl and the
// pipeline-specific shader file.  No bindings declared here.

// ---- colour helpers ---------------------------------------------------------

fn hue_to_rgb(h: f32) -> vec3f {
    let h6 = fract(h) * 6.0;
    let r  = abs(h6 - 3.0) - 1.0;
    let g  = 2.0 - abs(h6 - 2.0);
    let b  = 2.0 - abs(h6 - 4.0);
    return clamp(vec3f(r, g, b), vec3f(0.0), vec3f(1.0));
}

// Cinder palette: 0..1 → ember / gold / ash / vine cycle
fn cinder_rgb(t: f32) -> vec3f {
    let ember = vec3f(0.85, 0.30, 0.08);
    let gold  = vec3f(0.80, 0.55, 0.12);
    let ash   = vec3f(0.35, 0.32, 0.28);
    let vine  = vec3f(0.18, 0.45, 0.15);
    let s = fract(t);
    if s < 0.25 { return mix(ember, gold, s * 4.0); }
    if s < 0.50 { return mix(gold, ash, (s - 0.25) * 4.0); }
    if s < 0.75 { return mix(ash, vine, (s - 0.50) * 4.0); }
    return mix(vine, ember, (s - 0.75) * 4.0);
}

// Kind-aware palette for element borders/glows
fn kind_ember(kind: u32, hue: f32) -> vec3f {
    if kind == 1u { return vec3f(0.80, 0.15, 0.05); }   // error: deep red
    if kind == 2u { return vec3f(0.75, 0.50, 0.10); }   // warn: bonfire gold
    if kind == 3u { return vec3f(0.30, 0.35, 0.42); }   // info: stone blue
    if kind == 4u { return vec3f(0.25, 0.24, 0.22); }   // debug: dim ash
    if kind == 5u { return vec3f(0.20, 0.50, 0.18); }   // span: vine green
    if kind == 6u { return vec3f(0.90, 0.40, 0.10); }   // selected: bright ember
    if kind == 7u { return vec3f(0.75, 0.08, 0.05); }   // panic: blood red
    return vec3f(0.22, 0.20, 0.18);                      // structural: dark iron
}

// ---- hash / noise -----------------------------------------------------------

fn hash2(p: vec2f) -> f32 {
    return fract(sin(dot(p, vec2f(127.1, 311.7))) * 43758.5453);
}

fn smooth_noise(p: vec2f) -> f32 {
    let i  = floor(p);
    let f  = fract(p);
    let uv = f * f * (3.0 - 2.0 * f);
    return mix(
        mix(hash2(i),                   hash2(i + vec2f(1.0, 0.0)), uv.x),
        mix(hash2(i + vec2f(0.0, 1.0)), hash2(i + vec2f(1.0, 1.0)), uv.x),
        uv.y
    );
}

fn fbm(p_in: vec2f) -> f32 {
    var val  = 0.0;
    var amp  = 0.5;
    var freq = 1.0;
    var p    = p_in;
    for (var i = 0; i < 3; i++) {
        val  += amp * smooth_noise(p * freq);
        amp  *= 0.5;
        freq *= 2.0;
    }
    return val;
}

// ---- pseudorandom number generator (PCG) ------------------------------------

fn pcg_hash(input: u32) -> u32 {
    var state = input * 747796405u + 2891336453u;
    let word  = ((state >> ((state >> 28u) + 4u)) ^ state) * 277803737u;
    return (word >> 22u) ^ word;
}

fn rand_f(seed: u32) -> f32 {
    return f32(pcg_hash(seed)) / 4294967295.0;
}

fn rand2(seed: u32) -> vec2f {
    return vec2f(rand_f(seed), rand_f(seed + 1u));
}
