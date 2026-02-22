// ── Scene3D shaders: Blinn-Phong lit cubes + grid floor ──

struct Uniforms {
    viewProj  : mat4x4<f32>,   // 0
    model     : mat4x4<f32>,   // 64
    color     : vec4<f32>,     // 128
    lightDir  : vec4<f32>,     // 144
    cameraPos : vec4<f32>,     // 160
    flags     : vec4<f32>,     // 176  (x=isGround, y=isHovered, z=time, w=isDragged)
};

@group(0) @binding(0) var<uniform> u: Uniforms;

struct VsOut {
    @builtin(position) pos      : vec4<f32>,
    @location(0)       normal   : vec3<f32>,
    @location(1)       worldPos : vec3<f32>,
};

// ── vertex ──
@vertex
fn vs_main(
    @location(0) position : vec3<f32>,
    @location(1) normal   : vec3<f32>,
) -> VsOut {
    var out: VsOut;
    let wp = u.model * vec4(position, 1.0);
    out.pos      = u.viewProj * wp;
    out.normal   = normalize((u.model * vec4(normal, 0.0)).xyz);
    out.worldPos = wp.xyz;
    return out;
}

// ── fragment ──
@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    let N = normalize(in.normal);
    let L = normalize(u.lightDir.xyz);
    let V = normalize(u.cameraPos.xyz - in.worldPos);
    let H = normalize(L + V);

    // ── ground plane with anti-aliased grid ──
    if (u.flags.x > 0.5) {
        let gx = abs(fract(in.worldPos.x + 0.5) - 0.5) / fwidth(in.worldPos.x);
        let gz = abs(fract(in.worldPos.z + 0.5) - 0.5) / fwidth(in.worldPos.z);
        let line = 1.0 - min(min(gx, gz), 1.0);

        // fade grid at distance
        let dist = length(in.worldPos.xz);
        let fade = 1.0 - smoothstep(6.0, 14.0, dist);

        let base = vec3(0.07, 0.07, 0.09);
        let grid = vec3(0.22, 0.24, 0.30);

        // highlight axis lines
        let axX = 1.0 - min(abs(in.worldPos.z) / fwidth(in.worldPos.z), 1.0);
        let axZ = 1.0 - min(abs(in.worldPos.x) / fwidth(in.worldPos.x), 1.0);
        let axis = max(axX, axZ);

        var col = mix(base, grid, line * fade * 0.6);
        col = mix(col, vec3(0.32, 0.36, 0.48), axis * fade * 0.45);
        return vec4(col, 1.0);
    }

    // ── object shading ──
    let ambient  = 0.14;
    let diffuse  = max(dot(N, L), 0.0);
    let specular = pow(max(dot(N, H), 0.0), 48.0);
    let rim      = pow(1.0 - max(dot(N, V), 0.0), 3.0);

    var base = u.color.rgb;

    // hover highlight
    if (u.flags.y > 0.5) {
        base = mix(base, vec3(1.0), 0.18);
        let lit = ambient + diffuse * 0.65 + specular * 0.40 + rim * 0.35;
        return vec4(base * lit + vec3(specular * 0.22), 1.0);
    }

    // dragged: subtle brightening
    if (u.flags.w > 0.5) {
        base = mix(base, vec3(1.0), 0.08);
    }

    let lit = ambient + diffuse * 0.65 + specular * 0.28 + rim * 0.08;
    return vec4(base * lit + vec3(specular * 0.12), 1.0);
}
