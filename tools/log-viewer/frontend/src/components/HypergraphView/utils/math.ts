/**
 * Math utilities for 3D hypergraph projection and interaction.
 */
import type { Vec3 } from '../../Scene3D/math3d';

/**
 * Project a world position to screen coordinates.
 */
export function worldToScreen(
    worldPos: Vec3,
    viewProj: Float32Array,
    cw: number,
    ch: number,
): { x: number; y: number; z: number; visible: boolean } {
    const vp = viewProj;
    const cx = vp[0]! * worldPos[0] + vp[4]! * worldPos[1] + vp[8]! * worldPos[2] + vp[12]!;
    const cy = vp[1]! * worldPos[0] + vp[5]! * worldPos[1] + vp[9]! * worldPos[2] + vp[13]!;
    const cz = vp[2]! * worldPos[0] + vp[6]! * worldPos[1] + vp[10]! * worldPos[2] + vp[14]!;
    const cw2 = vp[3]! * worldPos[0] + vp[7]! * worldPos[1] + vp[11]! * worldPos[2] + vp[15]!;

    if (cw2 <= 0.001) return { x: -9999, y: -9999, z: 1, visible: false };

    const ndcX = cx / cw2;
    const ndcY = cy / cw2;
    const ndcZ = cz / cw2;

    const sx = (ndcX * 0.5 + 0.5) * cw;
    const sy = (1 - (ndcY * 0.5 + 0.5)) * ch;

    return { x: sx, y: sy, z: ndcZ, visible: ndcZ >= 0 && ndcZ <= 1 };
}

/**
 * Pixels-per-world-unit at a given world position.
 *
 * Uses the Euclidean distance from the camera to the point and the known
 * vertical FOV. This is completely independent of camera orientation —
 * a node at a given distance from the camera always has the same on-screen
 * scale regardless of which direction the camera faces.
 */
const HALF_FOV_TAN = Math.tan(Math.PI / 8); // tan(fov/2) where fov = PI/4

export function worldScaleAtDepth(
    camPos: Vec3,
    worldPos: Vec3,
    ch: number,
): number {
    const dx = worldPos[0] - camPos[0];
    const dy = worldPos[1] - camPos[1];
    const dz = worldPos[2] - camPos[2];
    const dist = Math.sqrt(dx * dx + dy * dy + dz * dz);
    if (dist < 0.001) return ch; // prevent division by zero
    return ch / (2 * dist * HALF_FOV_TAN);
}

/**
 * Ray-sphere intersection test.
 * Returns the distance along the ray to the first intersection, or null if no hit.
 */
export function raySphere(
    ro: Vec3,
    rd: Vec3,
    center: Vec3,
    radius: number,
): number | null {
    const oc: Vec3 = [ro[0] - center[0], ro[1] - center[1], ro[2] - center[2]];
    const a = rd[0] * rd[0] + rd[1] * rd[1] + rd[2] * rd[2];
    const b = 2 * (oc[0] * rd[0] + oc[1] * rd[1] + oc[2] * rd[2]);
    const c = oc[0] * oc[0] + oc[1] * oc[1] + oc[2] * oc[2] - radius * radius;
    const disc = b * b - 4 * a * c;
    if (disc < 0) return null;
    const t = (-b - Math.sqrt(disc)) / (2 * a);
    return t > 0 ? t : null;
}
