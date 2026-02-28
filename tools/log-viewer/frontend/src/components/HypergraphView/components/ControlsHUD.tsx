/**
 * ControlsHUD - Mouse/keyboard controls hint overlay.
 */
import { selectHighlightMode } from '../../../store';

export function ControlsHUD() {
    const active = selectHighlightMode.value;
    return (
        <div class="hypergraph-hud">
            <span>Left drag: Move nodes</span>
            <span>Right / Left empty: Orbit</span>
            <span>Middle / Shift+Left: Pan</span>
            <span>Scroll: Zoom</span>
            <span>Click node: Select &amp; Focus</span>
            <button
                class={`hg-btn hg-toggle ${active ? 'hg-toggle-on' : ''}`}
                onClick={() => { selectHighlightMode.value = !active; }}
                title="When enabled, clicking a node highlights its neighborhood and reflows the layout. When disabled, clicking only pans the camera."
            >
                {active ? '🔗 Highlight ON' : '🔗 Highlight OFF'}
            </button>
        </div>
    );
}
