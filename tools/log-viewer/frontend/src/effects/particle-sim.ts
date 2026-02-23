// particle-sim.ts — Shared particle simulation for angelic beams & glitter
//
// Works in both 2D and 3D contexts.  In 3D mode, coordinates are world-space
// positions.  In 2D mode, they can be screen-space pixel coordinates.
// All functions are pure — no side effects, no globals.
//
// Effect settings (speed, drift, count, etc.) are passed explicitly so
// callers can wire them from any settings source.

/** Particle state for beam and glitter effects (works in 2D or 3D). */
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

/** Effect settings passed to the particle simulation functions. */
export interface ParticleEffectSettings {
    /** Beam animation speed multiplier (0–3). Default 1. */
    beamSpeed: number;
    /** Beam upward drift distance multiplier (0–3). Default 1. */
    beamDrift: number;
    /** Max simultaneous beams. 0 = unlimited. */
    beamCount: number;
    /** Beam visual height multiplier (10–100). Default 35. */
    beamHeight: number;
    /** Glitter animation speed multiplier (0–3). Default 1. */
    glitterSpeed: number;
}

/** Default effect settings matching the theme store defaults. */
export const DEFAULT_PARTICLE_SETTINGS: ParticleEffectSettings = {
    beamSpeed: 1,
    beamDrift: 1,
    beamCount: 256,
    beamHeight: 35,
    glitterSpeed: 1,
};

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
    radius: number, time: number,
    settings: ParticleEffectSettings = DEFAULT_PARTICLE_SETTINGS,
): void {
    const maxBeams = settings.beamCount || 256;
    if (particles.filter(p => p.kind === 0).length >= maxBeams) return;
    const spd = Math.max(settings.beamSpeed, 0.01);
    const drift = settings.beamDrift;
    const angle = Math.random() * Math.PI * 2;
    const r = radius * (0.8 + Math.random() * 0.4);
    const p: Particle3D = {
        x: cx + Math.cos(angle) * r,
        y: cy,
        z: cz + Math.sin(angle) * r,
        vx: (Math.random() - 0.5) * 0.15 * spd,
        vy: (1.2 + Math.random() * 0.8) * drift * spd,
        vz: (Math.random() - 0.5) * 0.15 * spd,
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
    settings: ParticleEffectSettings = DEFAULT_PARTICLE_SETTINGS,
): void {
    if (particles.filter(p => p.kind === 1).length >= maxGlitter) return;
    const spd = Math.max(settings.glitterSpeed, 0.01);
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
        vx: (tangX * dir * (0.3 + Math.random() * 0.5) + (Math.random() - 0.5) * 0.1) * spd,
        vy: (Math.random() - 0.5) * 0.3 * spd,
        vz: (tangZ * dir * (0.3 + Math.random() * 0.5) + (Math.random() - 0.5) * 0.1) * spd,
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
export function updateParticles3D(
    particles: Particle3D[], dt: number, time: number,
    settings: ParticleEffectSettings = DEFAULT_PARTICLE_SETTINGS,
): void {
    for (let i = particles.length - 1; i >= 0; i--) {
        const p = particles[i]!;
        // Scale dt by per-kind speed so particles animate faster/slower
        const spd = p.kind === 0
            ? Math.max(settings.beamSpeed, 0.01)
            : Math.max(settings.glitterSpeed, 0.01);
        const sdt = dt * spd;
        p.life -= sdt;
        if (p.life <= 0) {
            particles.splice(i, 1);
            continue;
        }
        if (p.kind === 0) {
            // Beam: rise upward with gentle sway
            const sway = Math.sin(time * 1.5 + p.spawnT * 7.0) * 0.08;
            p.vx = p.vx * (1 - 0.5 * sdt) + sway * sdt;
            p.vz = p.vz * (1 - 0.5 * sdt);
            p.vy *= (1 - 0.2 * sdt);
        } else {
            // Glitter: drift along surface with sparkle sway
            const sway = Math.sin(time * 4.0 + p.spawnT * 13.0) * 0.15;
            p.vx = p.vx * (1 - 3 * sdt) + sway * sdt;
            p.vy = p.vy * (1 - 3 * sdt) - 0.05 * sdt;
            p.vz = p.vz * (1 - 3 * sdt);
        }
        p.x += p.vx * sdt;
        p.y += p.vy * sdt;
        p.z += p.vz * sdt;
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
