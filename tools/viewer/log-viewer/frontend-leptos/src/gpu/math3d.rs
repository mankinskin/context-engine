/// Compact 3D math library for WebGPU — port of `Scene3D/math3d.ts`.
///
/// All matrices are column-major `[f32; 16]`, matching WebGPU / WGSL conventions.

pub type Vec3 = [f32; 3];
pub type Mat4 = [f32; 16];

// ── Vec3 ──────────────────────────────────────────────────────────────────────

pub fn vec3_sub(a: Vec3, b: Vec3) -> Vec3 {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

pub fn vec3_add(a: Vec3, b: Vec3) -> Vec3 {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

pub fn vec3_scale(v: Vec3, s: f32) -> Vec3 {
    [v[0] * s, v[1] * s, v[2] * s]
}

pub fn vec3_dot(a: Vec3, b: Vec3) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

pub fn vec3_cross(a: Vec3, b: Vec3) -> Vec3 {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

pub fn vec3_normalize(v: Vec3) -> Vec3 {
    let len = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    if len < 1e-8 {
        return [0.0, 0.0, 0.0];
    }
    [v[0] / len, v[1] / len, v[2] / len]
}

pub fn vec3_length(v: Vec3) -> f32 {
    (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
}

// ── Mat4 ──────────────────────────────────────────────────────────────────────

pub fn mat4_identity() -> Mat4 {
    let mut m = [0f32; 16];
    m[0] = 1.0;
    m[5] = 1.0;
    m[10] = 1.0;
    m[15] = 1.0;
    m
}

/// Perspective projection with Z ∈ [0,1] (WebGPU clip space).
pub fn mat4_perspective(fov_y: f32, aspect: f32, near: f32, far: f32) -> Mat4 {
    let mut m = [0f32; 16];
    let f = 1.0 / (fov_y / 2.0).tan();
    m[0] = f / aspect;
    m[5] = f;
    m[10] = far / (near - far);
    m[11] = -1.0;
    m[14] = (near * far) / (near - far);
    m
}

pub fn mat4_look_at(eye: Vec3, target: Vec3, up: Vec3) -> Mat4 {
    let f = vec3_normalize(vec3_sub(target, eye));
    let s = vec3_normalize(vec3_cross(f, up));
    let u = vec3_cross(s, f);

    let mut m = [0f32; 16];
    m[0] = s[0];
    m[1] = u[0];
    m[2] = -f[0];
    m[3] = 0.0;
    m[4] = s[1];
    m[5] = u[1];
    m[6] = -f[1];
    m[7] = 0.0;
    m[8] = s[2];
    m[9] = u[2];
    m[10] = -f[2];
    m[11] = 0.0;
    m[12] = -vec3_dot(s, eye);
    m[13] = -vec3_dot(u, eye);
    m[14] = vec3_dot(f, eye);
    m[15] = 1.0;
    m
}

/// Column-major matrix multiply: returns `a * b`.
pub fn mat4_multiply(a: Mat4, b: Mat4) -> Mat4 {
    let mut m = [0f32; 16];
    for c in 0..4 {
        for r in 0..4 {
            let mut sum = 0.0f32;
            for k in 0..4 {
                sum += a[k * 4 + r] * b[c * 4 + k];
            }
            m[c * 4 + r] = sum;
        }
    }
    m
}

pub fn mat4_translate(v: Vec3) -> Mat4 {
    let mut m = mat4_identity();
    m[12] = v[0];
    m[13] = v[1];
    m[14] = v[2];
    m
}

pub fn mat4_scale(v: Vec3) -> Mat4 {
    let mut m = [0f32; 16];
    m[0] = v[0];
    m[5] = v[1];
    m[10] = v[2];
    m[15] = 1.0;
    m
}

pub fn mat4_rotate_y(angle: f32) -> Mat4 {
    let mut m = mat4_identity();
    let c = angle.cos();
    let s = angle.sin();
    m[0] = c;
    m[2] = s;
    m[8] = -s;
    m[10] = c;
    m
}

/// Full 4×4 matrix inverse via cofactors. Returns `None` if singular.
pub fn mat4_inverse(m: Mat4) -> Option<Mat4> {
    let mut inv = [0f32; 16];
    inv[0] = m[5] * m[10] * m[15] - m[5] * m[11] * m[14] - m[9] * m[6] * m[15]
        + m[9] * m[7] * m[14] + m[13] * m[6] * m[11] - m[13] * m[7] * m[10];
    inv[4] = -m[4] * m[10] * m[15] + m[4] * m[11] * m[14] + m[8] * m[6] * m[15]
        - m[8] * m[7] * m[14] - m[12] * m[6] * m[11] + m[12] * m[7] * m[10];
    inv[8] = m[4] * m[9] * m[15] - m[4] * m[11] * m[13] - m[8] * m[5] * m[15]
        + m[8] * m[7] * m[13] + m[12] * m[5] * m[11] - m[12] * m[7] * m[9];
    inv[12] = -m[4] * m[9] * m[14] + m[4] * m[10] * m[13] + m[8] * m[5] * m[14]
        - m[8] * m[6] * m[13] - m[12] * m[5] * m[10] + m[12] * m[6] * m[9];
    inv[1] = -m[1] * m[10] * m[15] + m[1] * m[11] * m[14] + m[9] * m[2] * m[15]
        - m[9] * m[3] * m[14] - m[13] * m[2] * m[11] + m[13] * m[3] * m[10];
    inv[5] = m[0] * m[10] * m[15] - m[0] * m[11] * m[14] - m[8] * m[2] * m[15]
        + m[8] * m[3] * m[14] + m[12] * m[2] * m[11] - m[12] * m[3] * m[10];
    inv[9] = -m[0] * m[9] * m[15] + m[0] * m[11] * m[13] + m[8] * m[1] * m[15]
        - m[8] * m[3] * m[13] - m[12] * m[1] * m[11] + m[12] * m[3] * m[9];
    inv[13] = m[0] * m[9] * m[14] - m[0] * m[10] * m[13] - m[8] * m[1] * m[14]
        + m[8] * m[2] * m[13] + m[12] * m[1] * m[10] - m[12] * m[2] * m[9];
    inv[2] = m[1] * m[6] * m[15] - m[1] * m[7] * m[14] - m[5] * m[2] * m[15]
        + m[5] * m[3] * m[14] + m[13] * m[2] * m[7] - m[13] * m[3] * m[6];
    inv[6] = -m[0] * m[6] * m[15] + m[0] * m[7] * m[14] + m[4] * m[2] * m[15]
        - m[4] * m[3] * m[14] - m[12] * m[2] * m[7] + m[12] * m[3] * m[6];
    inv[10] = m[0] * m[5] * m[15] - m[0] * m[7] * m[13] - m[4] * m[1] * m[15]
        + m[4] * m[3] * m[13] + m[12] * m[1] * m[7] - m[12] * m[3] * m[5];
    inv[14] = -m[0] * m[5] * m[14] + m[0] * m[6] * m[13] + m[4] * m[1] * m[14]
        - m[4] * m[2] * m[13] - m[12] * m[1] * m[6] + m[12] * m[2] * m[5];
    inv[3] = -m[1] * m[6] * m[11] + m[1] * m[7] * m[10] + m[5] * m[2] * m[11]
        - m[5] * m[3] * m[10] - m[9] * m[2] * m[7] + m[9] * m[3] * m[6];
    inv[7] = m[0] * m[6] * m[11] - m[0] * m[7] * m[10] - m[4] * m[2] * m[11]
        + m[4] * m[3] * m[10] + m[8] * m[2] * m[7] - m[8] * m[3] * m[6];
    inv[11] = -m[0] * m[5] * m[11] + m[0] * m[7] * m[9] + m[4] * m[1] * m[11]
        - m[4] * m[3] * m[9] - m[8] * m[1] * m[7] + m[8] * m[3] * m[5];
    inv[15] = m[0] * m[5] * m[10] - m[0] * m[6] * m[9] - m[4] * m[1] * m[10]
        + m[4] * m[2] * m[9] + m[8] * m[1] * m[6] - m[8] * m[2] * m[5];

    let det = m[0] * inv[0] + m[1] * inv[4] + m[2] * inv[8] + m[3] * inv[12];
    if det.abs() < 1e-10 {
        return None;
    }
    let inv_det = 1.0 / det;
    for v in inv.iter_mut() {
        *v *= inv_det;
    }
    Some(inv)
}

// ── Transform helpers ──────────────────────────────────────────────────────────

/// Transform a point (w=1) by a 4×4 matrix, with perspective divide.
pub fn mat4_transform_point(m: Mat4, p: Vec3) -> Vec3 {
    let w = m[3] * p[0] + m[7] * p[1] + m[11] * p[2] + m[15];
    [
        (m[0] * p[0] + m[4] * p[1] + m[8] * p[2] + m[12]) / w,
        (m[1] * p[0] + m[5] * p[1] + m[9] * p[2] + m[13]) / w,
        (m[2] * p[0] + m[6] * p[1] + m[10] * p[2] + m[14]) / w,
    ]
}

/// Transform a direction (w=0) by a 4×4 matrix.
pub fn mat4_transform_dir(m: Mat4, d: Vec3) -> Vec3 {
    [
        m[0] * d[0] + m[4] * d[1] + m[8] * d[2],
        m[1] * d[0] + m[5] * d[1] + m[9] * d[2],
        m[2] * d[0] + m[6] * d[1] + m[10] * d[2],
    ]
}

// ── Ray casting ────────────────────────────────────────────────────────────────

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

fn mat4_transform_vec4(m: Mat4, v: [f32; 4]) -> [f32; 4] {
    [
        m[0] * v[0] + m[4] * v[1] + m[8] * v[2] + m[12] * v[3],
        m[1] * v[0] + m[5] * v[1] + m[9] * v[2] + m[13] * v[3],
        m[2] * v[0] + m[6] * v[1] + m[10] * v[2] + m[14] * v[3],
        m[3] * v[0] + m[7] * v[1] + m[11] * v[2] + m[15] * v[3],
    ]
}

/// Cast a view ray through a screen pixel (NDC, WebGPU conventions).
pub fn screen_to_ray(sx: f32, sy: f32, w: f32, h: f32, inv_view_proj: Mat4) -> Ray {
    let nx = (2.0 * sx / w) - 1.0;
    let ny = 1.0 - (2.0 * sy / h);

    let near4 = mat4_transform_vec4(inv_view_proj, [nx, ny, 0.0, 1.0]);
    let far4 = mat4_transform_vec4(inv_view_proj, [nx, ny, 1.0, 1.0]);
    let near: Vec3 = [
        near4[0] / near4[3],
        near4[1] / near4[3],
        near4[2] / near4[3],
    ];
    let far: Vec3 = [
        far4[0] / far4[3],
        far4[1] / far4[3],
        far4[2] / far4[3],
    ];

    Ray {
        origin: near,
        direction: vec3_normalize(vec3_sub(far, near)),
    }
}

/// Slab method for ray–AABB intersection. Returns t or `None`.
pub fn ray_aabb_intersect(ray: &Ray, bmin: Vec3, bmax: Vec3) -> Option<f32> {
    let mut tmin = f32::NEG_INFINITY;
    let mut tmax = f32::INFINITY;

    for i in 0..3 {
        if ray.direction[i].abs() < 1e-8 {
            if ray.origin[i] < bmin[i] || ray.origin[i] > bmax[i] {
                return None;
            }
        } else {
            let mut t1 = (bmin[i] - ray.origin[i]) / ray.direction[i];
            let mut t2 = (bmax[i] - ray.origin[i]) / ray.direction[i];
            if t1 > t2 {
                std::mem::swap(&mut t1, &mut t2);
            }
            tmin = tmin.max(t1);
            tmax = tmax.min(t2);
            if tmin > tmax {
                return None;
            }
        }
    }

    if tmax < 0.0 {
        return None;
    }
    Some(if tmin >= 0.0 { tmin } else { tmax })
}

/// Ray–horizontal-plane intersection at given Y.
pub fn ray_plane_intersect(ray: &Ray, plane_y: f32) -> Option<Vec3> {
    if ray.direction[1].abs() < 1e-8 {
        return None;
    }
    let t = (plane_y - ray.origin[1]) / ray.direction[1];
    if t < 0.0 {
        return None;
    }
    Some(vec3_add(ray.origin, vec3_scale(ray.direction, t)))
}

/// Project a world position to screen coordinates.
/// Returns `(sx, sy, ndc_z, visible)`.
pub fn world_to_screen(world: Vec3, view_proj: Mat4, cw: f32, ch: f32) -> (f32, f32, f32, bool) {
    let vp = view_proj;
    let cx = vp[0] * world[0] + vp[4] * world[1] + vp[8] * world[2] + vp[12];
    let cy = vp[1] * world[0] + vp[5] * world[1] + vp[9] * world[2] + vp[13];
    let cz = vp[2] * world[0] + vp[6] * world[1] + vp[10] * world[2] + vp[14];
    let cw2 = vp[3] * world[0] + vp[7] * world[1] + vp[11] * world[2] + vp[15];

    if cw2 <= 0.001 {
        return (-9999.0, -9999.0, 1.0, false);
    }

    let ndc_x = cx / cw2;
    let ndc_y = cy / cw2;
    let ndc_z = cz / cw2;

    let sx = (ndc_x * 0.5 + 0.5) * cw;
    let sy = (1.0 - (ndc_y * 0.5 + 0.5)) * ch;

    (sx, sy, ndc_z, ndc_z >= 0.0 && ndc_z <= 1.0)
}

/// Pixels-per-world-unit scale for a point at `world_pos` seen from `eye`.
/// Uses fov_y = PI/4 (matching the camera in HypergraphView).
pub fn world_scale_at_depth(eye: Vec3, world_pos: Vec3, canvas_h: f32) -> f32 {
    // tan(PI/8) = tan(fov_y/2) for fov_y = PI/4
    const HALF_FOV_TAN: f32 = 0.41421356;
    let d = vec3_sub(world_pos, eye);
    let dist = vec3_length(d).max(0.01);
    canvas_h / (2.0 * dist * HALF_FOV_TAN)
}
