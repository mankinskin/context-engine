/**
 * ControlsHUD - Mouse/keyboard controls hint overlay.
 */
import { autoLayoutEnabled } from '../../../store';
import {
    nestingEnabled,
    duplicateModeEnabled,
    parentDepth,
    childDepth,
    setNestingEnabled,
    setDuplicateMode,
    setParentDepth,
    setChildDepth,
} from '../hooks';

export function ControlsHUD() {
    const layoutActive = autoLayoutEnabled.value;
    const nestingActive = nestingEnabled.value;
    const duplicateActive = duplicateModeEnabled.value;
    const pDepth = parentDepth.value;
    const cDepth = childDepth.value;

    return (
        <div class="hypergraph-hud">
            <span>Left drag: Move nodes</span>
            <span>Right / Left empty: Orbit</span>
            <span>Middle / Shift+Left: Pan</span>
            <span>Scroll: Zoom</span>
            <span>Click node: Select &amp; Focus</span>
            
            <div class="hg-hud-divider" />
            
            <button
                class={`hg-btn hg-toggle ${layoutActive ? 'hg-toggle-on' : ''}`}
                onClick={() => { autoLayoutEnabled.value = !layoutActive; }}
                title="When enabled, clicking a node reflows the layout around it."
            >
                {layoutActive ? '📐 Layout ON' : '📐 Layout OFF'}
            </button>
            
            <button
                class={`hg-btn hg-toggle ${nestingActive ? 'hg-toggle-on' : ''}`}
                onClick={() => setNestingEnabled(!nestingActive)}
                title="When enabled, shows parent shells and child duplicates around selected node."
            >
                {nestingActive ? '🪺 Nesting ON' : '🪺 Nesting OFF'}
            </button>
            
            {nestingActive && (
                <>
                    <button
                        class={`hg-btn hg-toggle ${duplicateActive ? 'hg-toggle-on' : ''}`}
                        onClick={() => setDuplicateMode(!duplicateActive)}
                        title="When enabled, child nodes are duplicated inside parent. When disabled, children are moved."
                    >
                        {duplicateActive ? '📋 Duplicate' : '↗️ Reparent'}
                    </button>
                    
                    <div class="hg-depth-control">
                        <label title="Number of parent shell levels to show">
                            Parents: 
                            <input
                                type="range"
                                min="1"
                                max="5"
                                value={pDepth}
                                onInput={(e) => setParentDepth(parseInt((e.target as HTMLInputElement).value))}
                            />
                            <span class="hg-depth-value">{pDepth}</span>
                        </label>
                    </div>
                    
                    <div class="hg-depth-control">
                        <label title="Number of child levels to show inside expanded node">
                            Children:
                            <input
                                type="range"
                                min="1"
                                max="3"
                                value={cDepth}
                                onInput={(e) => setChildDepth(parseInt((e.target as HTMLInputElement).value))}
                            />
                            <span class="hg-depth-value">{cDepth}</span>
                        </label>
                    </div>
                </>
            )}
        </div>
    );
}
