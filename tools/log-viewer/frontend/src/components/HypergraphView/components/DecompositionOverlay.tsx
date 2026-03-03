/**
 * DecompositionOverlay — renders decomposition patterns for a selected
 * compound node as horizontal rows of proportionally-sized child tokens.
 *
 * Each row represents one decomposition pattern (one valid way to split
 * the parent string into sub-tokens). Children within a row are sized
 * proportionally to their width relative to the parent width, so every
 * row has the same total visual width.
 *
 * No two patterns share border offsets, making each row an independent
 * partition of the parent's full width.
 */
import type { GraphLayout, DecompositionPattern } from '../layout';
import { getDecompositionPatterns } from '../layout';
import { nodeWidthClass } from '../utils/nodeStyles';
import { getNodeVizClasses, type VisualizationState } from '../hooks/useVisualizationState';

export interface DecompositionOverlayProps {
    parentIdx: number;
    layout: GraphLayout;
    vizState: VisualizationState;
    onFocusNode: (nodeIndex: number) => void;
}

/** Palette of subtle row-separator colors so patterns are visually distinct. */
const ROW_COLORS = [
    'rgba(80, 140, 200, 0.12)',
    'rgba(200, 120, 80, 0.12)',
    'rgba(100, 180, 100, 0.12)',
    'rgba(160, 120, 200, 0.12)',
    'rgba(200, 180, 80, 0.12)',
    'rgba(80, 200, 180, 0.12)',
];

export function DecompositionOverlay({
    parentIdx,
    layout,
    vizState,
    onFocusNode,
}: DecompositionOverlayProps) {
    const parent = layout.nodeMap.get(parentIdx);
    if (!parent || parent.isAtom) return null;

    const patterns = getDecompositionPatterns(layout, parentIdx);
    if (patterns.length === 0) return null;

    return (
        <div class="decomp-overlay">
            <div class="decomp-header">
                <span class="decomp-parent-label">{parent.label}</span>
                <span class="decomp-parent-width">W{parent.width}</span>
                <span class="decomp-pattern-count">
                    {patterns.length} pattern{patterns.length !== 1 ? 's' : ''}
                </span>
            </div>
            <div class="decomp-patterns">
                {patterns.map((pat, pi) => (
                    <div
                        key={pat.patternIdx}
                        class="decomp-row"
                        style={{ background: ROW_COLORS[pi % ROW_COLORS.length] }}
                    >
                        <span class="decomp-row-label">P{pat.patternIdx}</span>
                        <div class="decomp-tokens">
                            {pat.children.map(child => {
                                const levelClass = nodeWidthClass(child.width, layout.maxWidth);
                                const vizClasses = getNodeVizClasses(child.index, vizState);
                                return (
                                    <div
                                        key={`${pat.patternIdx}-${child.index}`}
                                        class={`decomp-token ${levelClass} ${vizClasses}`}
                                        style={{ flex: `${child.fraction}` }}
                                        onClick={() => onFocusNode(child.index)}
                                        title={`#${child.index} "${child.label}" (W${child.width})`}
                                    >
                                        <span class="decomp-token-label">{child.label}</span>
                                        <span class="decomp-token-idx">#{child.index}</span>
                                    </div>
                                );
                            })}
                        </div>
                    </div>
                ))}
            </div>
        </div>
    );
}
