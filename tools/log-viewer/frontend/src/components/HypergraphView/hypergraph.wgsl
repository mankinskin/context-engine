// ── Hypergraph 3D View – shaders ──

struct Camera {
    viewProj : mat4x4<f32>,
    eye      : vec4<f32>,
    time     : vec4<f32>,   // x=time, y=0, z=0, w=0
};

@group(0) @binding(0) var<uniform> cam : Camera;

// ══════════════════════════════════════════════════════
//  NODE RENDERING  (instanced billboard impostor spheres)
// ══════════════════════════════════════════════════════

struct NodeInstance {
    @location(2) center : vec3<f32>,    // world position
    @location(3) radius : f32,          // sphere radius
    @location(4) color  : vec4<f32>,    // base color + alpha
    @location(5) flags  : vec4<f32>,    // x=selected, y=hovered, z=isAtom, w=0
};

struct NodeVsOut {
    @builtin(position) pos   : vec4<f32>,
    @location(0) uv          : vec2<f32>,
    @location(1) worldCenter : vec3<f32>,
    @location(2) radius      : f32,
    @location(3) color       : vec4<f32>,
    @location(4) flags       : vec4<f32>,
};

@vertex
fn vs_node(
    @location(0) quadPos : vec2<f32>,   // −1..1 billboard quad
    inst : NodeInstance,
) -> NodeVsOut {
    // Build billboard in view space
    let right = normalize(vec3(cam.viewProj[0][0], cam.viewProj[1][0], cam.viewProj[2][0]));
    let up    = normalize(vec3(cam.viewProj[0][1], cam.viewProj[1][1], cam.viewProj[2][1]));

    let expand = 1.3;  // padding for AA edge
    let worldPos = inst.center
        + right * quadPos.x * inst.radius * expand
        + up    * quadPos.y * inst.radius * expand;

    var out: NodeVsOut;
    out.pos         = cam.viewProj * vec4(worldPos, 1.0);
    out.uv          = quadPos;
    out.worldCenter = inst.center;
    out.radius      = inst.radius;
    out.color       = inst.color;
    out.flags       = inst.flags;
    return out;
}

@fragment
fn fs_node(in: NodeVsOut) -> @location(0) vec4<f32> {
    let d = length(in.uv);
    if (d > 1.0) { discard; }

    // Sphere normal from billboard UV
    let z = sqrt(max(1.0 - d * d, 0.0));
    let right = normalize(vec3(cam.viewProj[0][0], cam.viewProj[1][0], cam.viewProj[2][0]));
    let up    = normalize(vec3(cam.viewProj[0][1], cam.viewProj[1][1], cam.viewProj[2][1]));
    let fwd   = normalize(cross(right, up));
    let N = normalize(right * in.uv.x + up * in.uv.y + fwd * z);

    let L = normalize(vec3(0.4, 0.8, 0.3));
    let V = normalize(cam.eye.xyz - in.worldCenter);
    let H = normalize(L + V);

    let ambient  = 0.18;
    let diffuse  = max(dot(N, L), 0.0) * 0.55;
    let spec     = pow(max(dot(N, H), 0.0), 40.0) * 0.35;
    let rim      = pow(1.0 - max(dot(N, V), 0.0), 3.0) * 0.15;
    let fresnel  = pow(1.0 - max(dot(N, V), 0.0), 4.0) * 0.25;

    var base = in.color.rgb;

    // Selected: glow ring
    if (in.flags.x > 0.5) {
        base = mix(base, vec3(1.0, 0.9, 0.4), 0.25);
        let ring = smoothstep(0.7, 0.85, d) * smoothstep(1.0, 0.92, d);
        let glow = ring * 0.6 * (0.7 + 0.3 * sin(cam.time.x * 3.0));
        let lit = ambient + diffuse + spec + rim + fresnel;
        return vec4(base * lit + vec3(glow * 0.8, glow * 0.6, glow * 0.1) + vec3(spec * 0.15), 1.0);
    }

    // Hovered: brightening
    if (in.flags.y > 0.5) {
        base = mix(base, vec3(1.0), 0.15);
    }

    let lit = ambient + diffuse + spec + rim;
    let aa = 1.0 - smoothstep(0.92, 1.0, d);
    return vec4((base * lit + vec3(spec * 0.12)) * aa, aa);
}


// ══════════════════════════════════════════════════════
//  EDGE RENDERING  (instanced line segments as thin quads)
// ══════════════════════════════════════════════════════

struct EdgeInstance {
    @location(2) posA   : vec3<f32>,    // start point
    @location(3) posB_x : f32,
    @location(4) posB_yz_color : vec4<f32>,  // yz = posB.yz, zw = color.rg
    @location(5) color_ba_flags : vec4<f32>, // xy = color.ba, z = flags, w = patternIdx
};

struct EdgeVsOut {
    @builtin(position) pos : vec4<f32>,
    @location(0) color     : vec4<f32>,
    @location(1) edgeUV    : vec2<f32>,
    @location(2) flags     : f32,
};

@vertex
fn vs_edge(
    @location(0) quadPos : vec2<f32>,
    @location(6) posA    : vec3<f32>,
    @location(7) posB    : vec3<f32>,
    @location(8) color   : vec4<f32>,
    @location(9) flags   : f32,  // x=highlighted
) -> EdgeVsOut {
    let midA = posA;
    let midB = posB;
    let dir = midB - midA;
    let pos01 = quadPos.x * 0.5 + 0.5;  // 0..1 along line
    let center = mix(midA, midB, pos01);

    let viewDir = normalize(cam.eye.xyz - center);
    let lineDir = normalize(dir);
    let side = normalize(cross(lineDir, viewDir));

    let halfWidth = select(0.015, 0.035, flags > 0.5);
    let worldPos = center + side * quadPos.y * halfWidth;

    var out: EdgeVsOut;
    out.pos   = cam.viewProj * vec4(worldPos, 1.0);
    out.color = color;
    out.edgeUV = quadPos;
    out.flags = flags;
    return out;
}

@fragment
fn fs_edge(in: EdgeVsOut) -> @location(0) vec4<f32> {
    let alpha = 1.0 - smoothstep(0.6, 1.0, abs(in.edgeUV.y));
    var col = in.color.rgb;
    var a = in.color.a * alpha;

    // Highlighted edges are brighter
    if (in.flags > 0.5) {
        col = mix(col, vec3(1.0), 0.3);
        a *= 1.4;
    }

    // Fade at endpoints
    let endFade = smoothstep(0.0, 0.08, 0.5 - abs(in.edgeUV.x));
    a *= endFade;

    return vec4(col * a, a);
}


// ══════════════════════════════════════════════════════
//  PARTICLE EFFECTS  (angelic beams + glitter on nodes)
// ══════════════════════════════════════════════════════

struct ParticleInstance {
    @location(2) center   : vec3<f32>,  // world position of particle
    @location(3) size     : f32,        // billboard radius / beam half-width
    @location(4) color    : vec4<f32>,  // RGBA
    @location(5) params   : vec4<f32>,  // x=kind(0=beam,1=glitter), y=tLife, z=hue, w=spawnT
};

struct ParticleVsOut {
    @builtin(position) pos : vec4<f32>,
    @location(0) uv        : vec2<f32>,
    @location(1) color     : vec4<f32>,
    @location(2) params    : vec4<f32>,
};

@vertex
fn vs_particle(
    @location(0) quadPos : vec2<f32>,
    inst : ParticleInstance,
) -> ParticleVsOut {
    let right = normalize(vec3(cam.viewProj[0][0], cam.viewProj[1][0], cam.viewProj[2][0]));
    let up    = normalize(vec3(cam.viewProj[0][1], cam.viewProj[1][1], cam.viewProj[2][1]));

    var worldPos: vec3<f32>;
    let kind = inst.params.x;

    if (kind < 0.5) {
        // Angelic beam: tall thin vertical billboard (world-up, not camera-up)
        let half_w = inst.size * 0.04;
        let half_h = inst.size * 0.6;
        worldPos = inst.center
            + right * quadPos.x * half_w
            + vec3(0.0, 1.0, 0.0) * quadPos.y * half_h;
    } else {
        // Glitter: small camera-facing billboard
        let r = inst.size * 0.06;
        worldPos = inst.center
            + right * quadPos.x * r
            + up    * quadPos.y * r;
    }

    var out: ParticleVsOut;
    out.pos    = cam.viewProj * vec4(worldPos, 1.0);
    out.uv     = quadPos;
    out.color  = inst.color;
    out.params = inst.params;
    return out;
}

@fragment
fn fs_particle(in: ParticleVsOut) -> @location(0) vec4<f32> {
    let kind   = in.params.x;
    let t_life = in.params.y;
    let hue    = in.params.z;
    let spawnT = in.params.w;

    if (kind < 0.5) {
        // ── Angelic beam: double-pointed crystalline shard ──
        let dx = in.uv.x;
        let dy = in.uv.y;

        let t   = (dy + 1.0) * 0.5;
        let mid = abs(t - 0.5) * 2.0;
        let shard_width = (1.0 - mid * mid) * 0.22;

        let hx = abs(dx) / max(shard_width, 0.005);
        let edge = smoothstep(1.2, 0.8, hx);

        let core = exp(-dx * dx / max(shard_width * shard_width * 0.1, 0.0005));
        let h_falloff = edge * (0.25 + 0.75 * core);

        let v_fade = 1.0 - mid * mid;
        let bright = h_falloff * v_fade * t_life * 1.6;

        if (bright < 0.003) { discard; }

        let center_col = vec3(1.6, 1.5, 1.3);
        let edge_col   = vec3(1.1, 0.85, 0.4);
        let ray_col = mix(edge_col, center_col, core * 0.95);

        let col = ray_col * bright * 0.6;
        let a   = min(bright * 0.4, 1.0);
        return vec4(col * a, a);
    } else {
        // ── Angelic glitter: twinkling sparkle ──
        let d = length(in.uv);
        let dot_mask = smoothstep(1.0, 0.15, d);

        let twinkle = 0.6 + 0.4 * sin(cam.time.x * 12.0 + spawnT * 7.3);
        let bright = t_life * dot_mask * twinkle * 1.4;

        if (bright < 0.008) { discard; }

        let warm = vec3(1.3, 1.15, 0.85);
        let cool = vec3(0.85, 0.90, 1.25);
        let phase = fract(hue * 3.7 + cam.time.x * 0.5);
        let glitter_col = mix(warm, cool, smoothstep(0.3, 0.7, phase));

        let col = glitter_col * bright;
        let a   = min(bright * 0.9, 1.0);
        return vec4(col * a, a);
    }
}


// ══════════════════════════════════════════════════════
//  LABEL TEXT  (reserved for future: SDF text rendering)
// ══════════════════════════════════════════════════════
