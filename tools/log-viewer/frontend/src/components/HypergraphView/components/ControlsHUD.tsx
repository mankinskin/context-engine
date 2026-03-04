/**
 * ControlsHUD - Mouse/keyboard controls hint overlay.
 */
import { autoLayoutEnabled } from '../../../store';

export function ControlsHUD() {
    const active = autoLayoutEnabled.value;
    return (
        <div class="hypergraph-hud">
            <span>Left drag: Move nodes</span>
            <span>Right / Left empty: Orbit</span>
            <span>Middle / Shift+Left: Pan</span>
            <span>Scroll: Zoom</span>
            <span>Click node: Select &amp; Focus</span>
            <button
                class={`hg-btn hg-toggle ${active ? 'hg-toggle-on' : ''}`}
                onClick={() => { autoLayoutEnabled.value = !active; }}
                title="When enabled, clicking a node reflows the layout around it. When disabled, nodes can be freely dragged."
            >
                {active ? '📐 Layout ON' : '📐 Layout OFF'}
            </button>
        </div>
    );
}
