/**
 * ControlsHUD - Mouse/keyboard controls hint overlay.
 */

export function ControlsHUD() {
    return (
        <div class="hypergraph-hud">
            <span>Left drag: Move nodes</span>
            <span>Right / Left empty: Orbit</span>
            <span>Middle / Shift+Left: Pan</span>
            <span>Scroll: Zoom</span>
            <span>Click node: Select &amp; Focus</span>
        </div>
    );
}
