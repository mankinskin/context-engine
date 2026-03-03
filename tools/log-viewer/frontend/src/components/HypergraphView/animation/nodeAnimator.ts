/**
 * Node lerp animation — smoothly animates node positions toward their targets.
 */
import type { LayoutNode } from '../layout';

/**
 * Advance all nodes toward their target positions using exponential decay.
 *
 * @param nodes — layout nodes to animate (mutated in-place)
 * @param dt — frame delta-time in seconds
 * @param lerpSpeed — exponential decay rate (higher = snappier, default 12)
 */
export function animateNodes(nodes: LayoutNode[], dt: number, lerpSpeed = 12): void {
    const lerpFactor = 1 - Math.exp(-lerpSpeed * dt);
    for (const n of nodes) {
        n.x += (n.tx - n.x) * lerpFactor;
        n.y += (n.ty - n.y) * lerpFactor;
        n.z += (n.tz - n.z) * lerpFactor;
    }
}
