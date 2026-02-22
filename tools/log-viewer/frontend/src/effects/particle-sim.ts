// particle-sim.ts — Shared 3D particle simulation for angelic beams & glitter
//
// Extracted from HypergraphView so other 3D views can reuse the same
// particle system.  These functions are pure — no side effects, no globals.

/** Particle state for 3D scenes with beam and glitter effects. */
export interface Particle3D {
    x: number; y: number; z: number;
    vx: number; vy: number; vz: number;
    size: number;
    life: number;
    maxLife: number;
    hue: number;
    /** 0 = angelic beam, 1 = glitter */
    kind: number;
    spawnT: number;
}

/** Instance floats per particle: center(3) + size(1) + color(4) + params(4) */
export const PARTICLE_INSTANCE_FLOATS = 12;

/**
 * Spawn an angelic beam particle rising from a sphere surface.
 *
 * @param particles  Live particle array (mutated in place)
 * @param cx, cy, cz  Sphere centre
 * @param radius      Sphere radius
 * @param time        Current time (seconds)
 * @param maxBeams    Cap on simultaneous beams
 */
export function spawnBeam(
    particles: Particle3D[],
    cx: number, cy: number, cz: number,
    radius: number, time: number, maxBeams: number,
): void {
    if (particles.filter(p => p.kind === 0).length >= maxBeams) return;
    const angle = Math.random() * Math.PI * 2;
    const r = radius * (0.8 + Math.random() * 0.4);
    const p: Particle3D = {
        x: cx + Math.cos(angle) * r,
        y: cy,
        z: cz + Math.sin(angle) * r,
        vx: (Math.random() - 0.5) * 0.15,
        vy: 1.2 + Math.random() * 0.8,
        vz: (Math.random() - 0.5) * 0.15,
        size: 0.6 + Math.random() * 1.0,
        life: 2.0 + Math.random() * 2.0,
        maxLife: 0,
        hue: 0.08 + Math.random() * 0.06,
        kind: 0,
        spawnT: time,
    };
    p.maxLife = p.life;
    particles.push(p);
}

/**
 * Spawn a glitter sparkle drifting along a sphere surface.
 *
 * @param particles  Live particle array (mutated in place)
 * @param cx, cy, cz  Sphere centre
 * @param radius      Sphere radius
 * @param time        Current time (seconds)
 * @param maxGlitter  Cap on simultaneous glitter particles
 */
export function spawnGlitter(
    particles: Particle3D[],
    cx: number, cy: number, cz: number,
    radius: number, time: number, maxGlitter: number,
): void {
    if (particles.filter(p => p.kind === 1).length >= maxGlitter) return;
    const angle = Math.random() * Math.PI * 2;
    const phi = (Math.random() - 0.5) * Math.PI;
    const r = radius * (0.9 + Math.random() * 0.3);
    const tangX = -Math.sin(angle);
    const tangZ = Math.cos(angle);
    const dir = Math.random() > 0.5 ? 1 : -1;
    const p: Particle3D = {
        x: cx + Math.cos(angle) * Math.cos(phi) * r,
        y: cy + Math.sin(phi) * r,
        z: cz + Math.sin(angle) * Math.cos(phi) * r,
        vx: tangX * dir * (0.3 + Math.random() * 0.5) + (Math.random() - 0.5) * 0.1,
        vy: (Math.random() - 0.5) * 0.3,
        vz: tangZ * dir * (0.3 + Math.random() * 0.5) + (Math.random() - 0.5) * 0.1,
        size: 0.4 + Math.random() * 0.8,
        life: 0.8 + Math.random() * 1.5,
        maxLife: 0,
        hue: Math.random(),
        kind: 1,
        spawnT: time,
    };
    p.maxLife = p.life;
    particles.push(p);
}

/**
 * Simulate all active 3D particles.
 * Removes dead particles and updates positions.
 */
export function updateParticles3D(particles: Particle3D[], dt: number, time: number): void {
    for (let i = particles.length - 1; i >= 0; i--) {
        const p = particles[i]!;
        p.life -= dt;
        if (p.life <= 0) {
            particles.splice(i, 1);
            continue;
        }
        if (p.kind === 0) {
            // Beam: rise upward with gentle sway
            const sway = Math.sin(time * 1.5 + p.spawnT * 7.0) * 0.08;
            p.vx = p.vx * (1 - 0.5 * dt) + sway * dt;
            p.vz = p.vz * (1 - 0.5 * dt);
            p.vy *= (1 - 0.2 * dt);
        } else {
            // Glitter: drift along surface with sparkle sway
            const sway = Math.sin(time * 4.0 + p.spawnT * 13.0) * 0.15;
            p.vx = p.vx * (1 - 3 * dt) + sway * dt;
            p.vy = p.vy * (1 - 3 * dt) - 0.05 * dt;
            p.vz = p.vz * (1 - 3 * dt);
        }
        p.x += p.vx * dt;
        p.y += p.vy * dt;
        p.z += p.vz * dt;
    }
}

/**
 * Write particle data into a Float32Array for GPU upload.
 * Layout: center(3) + size(1) + color(4) + params(4) per particle.
 *
 * @returns number of particles written
 */
export function fillParticleBuffer(
    particles: Particle3D[],
    buf: Float32Array,
    maxCount: number,
): number {
    const count = Math.min(particles.length, maxCount);
    for (let i = 0; i < count; i++) {
        const p = particles[i]!;
        const off = i * PARTICLE_INSTANCE_FLOATS;
        buf[off + 0] = p.x;
        buf[off + 1] = p.y;
        buf[off + 2] = p.z;
        buf[off + 3] = p.size;
        // color (white — actual coloring done in shader)
        buf[off + 4] = 1.0;
        buf[off + 5] = 1.0;
        buf[off + 6] = 1.0;
        buf[off + 7] = 1.0;
        // params: kind, tLife, hue, spawnT
        buf[off + 8] = p.kind;
        buf[off + 9] = p.life / p.maxLife;
        buf[off + 10] = p.hue;
        buf[off + 11] = p.spawnT;
    }
    return count;
}
