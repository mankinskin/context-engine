/**
 * NodeLayer - DOM node rendering with visualization state classes.
 */
import type { LayoutNode } from '../layout';
import { nodeWidthClass } from '../utils/nodeStyles';
import { getNodeVizClasses, type VisualizationState } from '../hooks/useVisualizationState';

export interface NodeLayerProps {
    nodes: LayoutNode[];
    maxWidth: number;
    selectedIdx: number;
    hoverIdx: number;
    vizState: VisualizationState;
}

export function NodeLayer({ nodes, maxWidth, selectedIdx, hoverIdx, vizState }: NodeLayerProps) {
    return (
        <>
            {nodes.map(n => {
                const isSel = n.index === selectedIdx;
                const isHov = n.index === hoverIdx;
                const levelClass = nodeWidthClass(n.width, maxWidth);
                const vizClasses = getNodeVizClasses(n.index, vizState);

                return (
                    <div
                        key={n.index}
                        class={`log-entry hg-node ${levelClass} ${isSel ? 'selected' : ''} ${isHov ? 'span-highlighted' : ''} ${n.isAtom ? 'hg-atom' : 'hg-compound'} ${vizClasses}`}
                        data-node-idx={n.index}
                    >
                        <div class="hg-node-content">
                            <span class={`level-badge ${levelClass}`}>{n.isAtom ? 'ATOM' : `W${n.width}`}</span>
                            <span class="hg-node-label">{n.label}</span>
                            <span class="hg-node-idx">#{n.index}</span>
                        </div>
                    </div>
                );
            })}
        </>
    );
}
