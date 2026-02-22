// fragment.wgsl — per-element border-glow + aurora background
//
// Bindings
//   binding 0 (uniform)          : Uniforms  — time, viewport size, element count
//   binding 1 (read-only-storage): ElemRect[] — DOM element rects uploaded each frame
//
// Two effects are composited:
//   1. Aurora background — full-screen animated fbm noise
//   2. Per-element border glow — each element's inner border is highlighted
//      with an animated colour derived from its selector index ("element id")

// ---- uniforms / storage ---------------------------------------------------

struct Uniforms {
    time          : f32,
    width         : f32,
    height        : f32,
    element_count : f32,
}

// Each element: rect(x,y,w,h) + hue + 3 padding f32 = 32 bytes (aligned)
struct ElemRect {
    rect : vec4f,   // x, y, w, h  (screen-space pixels, y=0 at top-left)
    hue  : f32,
    _p0  : f32,
    _p1  : f32,
    _p2  : f32,
}

@group(0) @binding(0) var<uniform>       u     : Uniforms;
@group(0) @binding(1) var<storage, read> elems : array<ElemRect>;

// ---- colour helper --------------------------------------------------------

// Converts a hue value in [0, 1] to an RGB colour (saturation=1, value=1).
fn hue_to_rgb(h: f32) -> vec3f {
    let h6 = h * 6.0;
    let r  = abs(h6 - 3.0) - 1.0;
    let g  = 2.0 - abs(h6 - 2.0);
    let b  = 2.0 - abs(h6 - 4.0);
    return clamp(vec3f(r, g, b), vec3f(0.0), vec3f(1.0));
}

// ---- noise helpers (aurora background) ------------------------------------

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
    for (var i = 0; i < 5; i++) {
        val  += amp * smooth_noise(p * freq);
        amp  *= 0.5;
        freq *= 2.0;
    }
    return val;
}

// ---- main fragment --------------------------------------------------------

@fragment
fn fs_main(@builtin(position) pos: vec4f) -> @location(0) vec4f {
    let px = pos.xy;
    let uv = px / vec2f(u.width, u.height);
    let t  = u.time * 0.35;

    // --- Aurora background -------------------------------------------------
    var p  = uv * 3.5 + vec2f(t * 0.25, 0.0);
    let n1 = fbm(p);
    let n2 = fbm(p + vec2f(0.0, t * 0.08) + vec2f(n1 * 1.8));
    let n3 = fbm(p + vec2f(n2 * 1.4, 0.0) - vec2f(0.0, t * 0.06));
    let band      = smoothstep(0.25, 0.80, n3) * (1.0 - uv.y * 0.9);
    let intensity = band * 0.18;
    let c1        = vec3f(0.10, 0.42, 0.50);
    let c2        = vec3f(0.18, 0.35, 0.58);
    let c3        = vec3f(0.38, 0.25, 0.52);
    let aurora_rgb = mix(mix(c1, c2, n2), c3, n1 * 0.6);
    var out = vec4f(aurora_rgb * intensity, intensity * 0.55);

    // --- Per-element border glow -------------------------------------------
    // For each DOM element, pixels inside its bounding rect receive an
    // animated coloured glow that is brightest at the inner edge and fades
    // inward.  The glow colour is derived from the element's selector index
    // (its "element id"), giving each UI region a unique stable colour.
    //
    // Complexity: O(pixels × element_count).  With MAX_ELEMENTS = 32 this
    // is well within GPU budget — modern hardware runs thousands of shader
    // invocations in parallel.  Raise MAX_ELEMENTS cautiously on complex UIs.
    let count = u32(u.element_count);
    for (var i = 0u; i < count; i++) {
        let e  = elems[i];
        let r  = e.rect;
        let ex = r.x;
        let ey = r.y;
        let ew = r.z;
        let eh = r.w;

        // Only shade pixels inside this element's rect
        if px.x >= ex && px.x < ex + ew && px.y >= ey && px.y < ey + eh {
            // Inward distance to the nearest edge
            let dx   = min(px.x - ex, ex + ew - px.x);
            let dy   = min(px.y - ey, ey + eh - px.y);
            let dist = min(dx, dy);

            // Glow profile: rises from 0 at the edge, peaks around 3 px
            // inside, then fades out by 16 px inward.
            let glow = smoothstep(0.0, 2.0, dist) * smoothstep(16.0, 4.0, dist);

            // Slowly-drifting hue + per-element animated pulse
            let hue   = fract(e.hue + u.time * 0.04);
            let pulse = 0.55 + 0.45 * sin(u.time * 1.8 + e.hue * 6.28318);
            let rgb   = hue_to_rgb(hue);
            let alpha = glow * 0.42 * pulse;

            // Additive blend so element glows stack with the aurora
            out = out + vec4f(rgb * alpha, alpha);
        }
    }

    return clamp(out, vec4f(0.0), vec4f(1.0));
}
