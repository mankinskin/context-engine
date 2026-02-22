// Compact 3D math library for WebGPU (column-major matrices)

export type Vec3 = [number, number, number];
export type Mat4 = Float32Array; // 16 elements, column-major

export function vec3Sub(a: Vec3, b: Vec3): Vec3 {
    return [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
}

export function vec3Add(a: Vec3, b: Vec3): Vec3 {
    return [a[0] + b[0], a[1] + b[1], a[2] + b[2]];
}

export function vec3Scale(v: Vec3, s: number): Vec3 {
    return [v[0] * s, v[1] * s, v[2] * s];
}

export function vec3Dot(a: Vec3, b: Vec3): number {
    return a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
}

export function vec3Cross(a: Vec3, b: Vec3): Vec3 {
    return [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ];
}

export function vec3Normalize(v: Vec3): Vec3 {
    const len = Math.sqrt(v[0] * v[0] + v[1] * v[1] + v[2] * v[2]);
    if (len < 1e-8) return [0, 0, 0];
    return [v[0] / len, v[1] / len, v[2] / len];
}

// ── Matrices (column-major, WebGPU conventions) ──

export function mat4Identity(): Mat4 {
    const m = new Float32Array(16);
    m[0] = m[5] = m[10] = m[15] = 1;
    return m;
}

/** Perspective projection with Z ∈ [0,1] (WebGPU clip space) */
export function mat4Perspective(fovY: number, aspect: number, near: number, far: number): Mat4 {
    const m = new Float32Array(16);
    const f = 1.0 / Math.tan(fovY / 2);
    m[0] = f / aspect;
    m[5] = f;
    m[10] = far / (near - far);
    m[11] = -1;
    m[14] = (near * far) / (near - far);
    return m;
}

export function mat4LookAt(eye: Vec3, target: Vec3, up: Vec3): Mat4 {
    const f = vec3Normalize(vec3Sub(target, eye));
    const s = vec3Normalize(vec3Cross(f, up));
    const u = vec3Cross(s, f);

    const m = new Float32Array(16);
    m[0] = s[0];  m[1] = u[0];  m[2] = -f[0]; m[3] = 0;
    m[4] = s[1];  m[5] = u[1];  m[6] = -f[1]; m[7] = 0;
    m[8] = s[2];  m[9] = u[2];  m[10] = -f[2]; m[11] = 0;
    m[12] = -vec3Dot(s, eye);
    m[13] = -vec3Dot(u, eye);
    m[14] = vec3Dot(f, eye);
    m[15] = 1;
    return m;
}

export function mat4Multiply(a: Mat4, b: Mat4): Mat4 {
    const m = new Float32Array(16);
    for (let c = 0; c < 4; c++) {
        for (let r = 0; r < 4; r++) {
            let sum = 0;
            for (let k = 0; k < 4; k++) {
                sum += a[k * 4 + r] * b[c * 4 + k];
            }
            m[c * 4 + r] = sum;
        }
    }
    return m;
}

export function mat4Translate(v: Vec3): Mat4 {
    const m = mat4Identity();
    m[12] = v[0]; m[13] = v[1]; m[14] = v[2];
    return m;
}

export function mat4ScaleMat(v: Vec3): Mat4 {
    const m = new Float32Array(16);
    m[0] = v[0]; m[5] = v[1]; m[10] = v[2]; m[15] = 1;
    return m;
}

export function mat4RotateY(angle: number): Mat4 {
    const m = mat4Identity();
    const c = Math.cos(angle);
    const s = Math.sin(angle);
    m[0] = c;  m[2] = s;
    m[8] = -s; m[10] = c;
    return m;
}

/** Full 4×4 matrix inverse via cofactors (returns null if singular) */
export function mat4Inverse(m: Mat4): Mat4 | null {
    const inv = new Float32Array(16);

    inv[0]  =  m[5]*m[10]*m[15] - m[5]*m[11]*m[14] - m[9]*m[6]*m[15]
             + m[9]*m[7]*m[14]  + m[13]*m[6]*m[11]  - m[13]*m[7]*m[10];
    inv[4]  = -m[4]*m[10]*m[15] + m[4]*m[11]*m[14]  + m[8]*m[6]*m[15]
             - m[8]*m[7]*m[14]  - m[12]*m[6]*m[11]  + m[12]*m[7]*m[10];
    inv[8]  =  m[4]*m[9]*m[15]  - m[4]*m[11]*m[13]  - m[8]*m[5]*m[15]
             + m[8]*m[7]*m[13]  + m[12]*m[5]*m[11]  - m[12]*m[7]*m[9];
    inv[12] = -m[4]*m[9]*m[14]  + m[4]*m[10]*m[13]  + m[8]*m[5]*m[14]
             - m[8]*m[6]*m[13]  - m[12]*m[5]*m[10]  + m[12]*m[6]*m[9];

    inv[1]  = -m[1]*m[10]*m[15] + m[1]*m[11]*m[14]  + m[9]*m[2]*m[15]
             - m[9]*m[3]*m[14]  - m[13]*m[2]*m[11]  + m[13]*m[3]*m[10];
    inv[5]  =  m[0]*m[10]*m[15] - m[0]*m[11]*m[14]  - m[8]*m[2]*m[15]
             + m[8]*m[3]*m[14]  + m[12]*m[2]*m[11]  - m[12]*m[3]*m[10];
    inv[9]  = -m[0]*m[9]*m[15]  + m[0]*m[11]*m[13]  + m[8]*m[1]*m[15]
             - m[8]*m[3]*m[13]  - m[12]*m[1]*m[11]  + m[12]*m[3]*m[9];
    inv[13] =  m[0]*m[9]*m[14]  - m[0]*m[10]*m[13]  - m[8]*m[1]*m[14]
             + m[8]*m[2]*m[13]  + m[12]*m[1]*m[10]  - m[12]*m[2]*m[9];

    inv[2]  =  m[1]*m[6]*m[15]  - m[1]*m[7]*m[14]   - m[5]*m[2]*m[15]
             + m[5]*m[3]*m[14]  + m[13]*m[2]*m[7]   - m[13]*m[3]*m[6];
    inv[6]  = -m[0]*m[6]*m[15]  + m[0]*m[7]*m[14]   + m[4]*m[2]*m[15]
             - m[4]*m[3]*m[14]  - m[12]*m[2]*m[7]   + m[12]*m[3]*m[6];
    inv[10] =  m[0]*m[5]*m[15]  - m[0]*m[7]*m[13]   - m[4]*m[1]*m[15]
             + m[4]*m[3]*m[13]  + m[12]*m[1]*m[7]   - m[12]*m[3]*m[5];
    inv[14] = -m[0]*m[5]*m[14]  + m[0]*m[6]*m[13]   + m[4]*m[1]*m[14]
             - m[4]*m[2]*m[13]  - m[12]*m[1]*m[6]   + m[12]*m[2]*m[5];

    inv[3]  = -m[1]*m[6]*m[11]  + m[1]*m[7]*m[10]   + m[5]*m[2]*m[11]
             - m[5]*m[3]*m[10]  - m[9]*m[2]*m[7]    + m[9]*m[3]*m[6];
    inv[7]  =  m[0]*m[6]*m[11]  - m[0]*m[7]*m[10]   - m[4]*m[2]*m[11]
             + m[4]*m[3]*m[10]  + m[8]*m[2]*m[7]    - m[8]*m[3]*m[6];
    inv[11] = -m[0]*m[5]*m[11]  + m[0]*m[7]*m[9]    + m[4]*m[1]*m[11]
             - m[4]*m[3]*m[9]   - m[8]*m[1]*m[7]    + m[8]*m[3]*m[5];
    inv[15] =  m[0]*m[5]*m[10]  - m[0]*m[6]*m[9]    - m[4]*m[1]*m[10]
             + m[4]*m[2]*m[9]   + m[8]*m[1]*m[6]    - m[8]*m[2]*m[5];

    let det = m[0] * inv[0] + m[1] * inv[4] + m[2] * inv[8] + m[3] * inv[12];
    if (Math.abs(det) < 1e-10) return null;

    det = 1.0 / det;
    for (let i = 0; i < 16; i++) inv[i] *= det;
    return inv;
}

// ── Transforms ──

/** Transform a point (w=1) by a 4×4 matrix, with perspective divide */
export function mat4TransformPoint(m: Mat4, p: Vec3): Vec3 {
    const w = m[3] * p[0] + m[7] * p[1] + m[11] * p[2] + m[15];
    return [
        (m[0] * p[0] + m[4] * p[1] + m[8]  * p[2] + m[12]) / w,
        (m[1] * p[0] + m[5] * p[1] + m[9]  * p[2] + m[13]) / w,
        (m[2] * p[0] + m[6] * p[1] + m[10] * p[2] + m[14]) / w,
    ];
}

/** Transform a direction (w=0) by a 4×4 matrix */
export function mat4TransformDir(m: Mat4, d: Vec3): Vec3 {
    return [
        m[0] * d[0] + m[4] * d[1] + m[8]  * d[2],
        m[1] * d[0] + m[5] * d[1] + m[9]  * d[2],
        m[2] * d[0] + m[6] * d[1] + m[10] * d[2],
    ];
}

// ── Ray casting ──

export interface Ray { origin: Vec3; direction: Vec3 }

function mat4TransformVec4(
    m: Mat4, v: [number, number, number, number],
): [number, number, number, number] {
    return [
        m[0]*v[0] + m[4]*v[1] + m[8]*v[2]  + m[12]*v[3],
        m[1]*v[0] + m[5]*v[1] + m[9]*v[2]  + m[13]*v[3],
        m[2]*v[0] + m[6]*v[1] + m[10]*v[2] + m[14]*v[3],
        m[3]*v[0] + m[7]*v[1] + m[11]*v[2] + m[15]*v[3],
    ];
}

/** Cast a ray from camera through a screen pixel */
export function screenToRay(
    sx: number, sy: number,
    w: number, h: number,
    invViewProj: Mat4,
): Ray {
    const nx = (2 * sx / w) - 1;
    const ny = 1 - (2 * sy / h);

    const near4 = mat4TransformVec4(invViewProj, [nx, ny, 0, 1]);
    const far4  = mat4TransformVec4(invViewProj, [nx, ny, 1, 1]);
    const near: Vec3 = [near4[0] / near4[3], near4[1] / near4[3], near4[2] / near4[3]];
    const far:  Vec3 = [far4[0]  / far4[3],  far4[1]  / far4[3],  far4[2]  / far4[3]];

    return { origin: near, direction: vec3Normalize(vec3Sub(far, near)) };
}

/** Slab method for ray–AABB intersection. Returns t or null. */
export function rayAABBIntersect(ray: Ray, bmin: Vec3, bmax: Vec3): number | null {
    let tmin = -Infinity;
    let tmax = Infinity;

    for (let i = 0; i < 3; i++) {
        if (Math.abs(ray.direction[i]) < 1e-8) {
            if (ray.origin[i] < bmin[i] || ray.origin[i] > bmax[i]) return null;
        } else {
            let t1 = (bmin[i] - ray.origin[i]) / ray.direction[i];
            let t2 = (bmax[i] - ray.origin[i]) / ray.direction[i];
            if (t1 > t2) { const tmp = t1; t1 = t2; t2 = tmp; }
            tmin = Math.max(tmin, t1);
            tmax = Math.min(tmax, t2);
            if (tmin > tmax) return null;
        }
    }

    if (tmax < 0) return null;
    return tmin >= 0 ? tmin : tmax;
}

/** Ray–horizontal-plane intersection at given Y */
export function rayPlaneIntersect(ray: Ray, planeY: number): Vec3 | null {
    if (Math.abs(ray.direction[1]) < 1e-8) return null;
    const t = (planeY - ray.origin[1]) / ray.direction[1];
    if (t < 0) return null;
    return vec3Add(ray.origin, vec3Scale(ray.direction, t));
}
