/**
 * DecompositionManager — imperatively reparents child DOM nodes into
 * decomposition rows under their expanded parent element.
 *
 * Manages expand/collapse state and provides information about which
 * nodes are currently reparented so that the node positioner can skip
 * 3D transforms for them and the edge builder can hide internal edges.
 */
import type { GraphLayout } from '../layout';
import { getDecompositionPatterns } from '../layout';
import { ROW_COLORS } from '../gpu/constants';
import { edgePairKey } from '../utils/math';

// ── Types ──

export interface ExpandedNodeState {
    parentEl: HTMLDivElement;
    container: HTMLDivElement;
    children: { el: HTMLDivElement }[];
}

// ── Manager ──

export class DecompositionManager {
    private expandedNodes = new Map<number, ExpandedNodeState>();
    private nodeElMap = new Map<number, HTMLDivElement>();
    private lastExpandedKeyStr = '';

    constructor(
        private layout: GraphLayout,
        private nodeLayer: HTMLDivElement,
        private onSelectNode?: (idx: number) => void,
    ) {
        this.refreshNodeElMap();
    }

    // ── Public API ──

    /** Get the map of node index → DOM element. */
    getNodeElMap(): Map<number, HTMLDivElement> {
        return this.nodeElMap;
    }

    /** Get the set of currently-expanded node indices. */
    getExpandedNodes(): Map<number, ExpandedNodeState> {
        return this.expandedNodes;
    }

    /** Get indices of nodes currently inside a decomposition row. */
    getReparentedSet(): Set<number> {
        const set = new Set<number>();
        for (const [, state] of this.expandedNodes) {
            for (const { el } of state.children) {
                const idx = el.getAttribute('data-node-idx');
                if (idx != null) set.add(Number(idx));
            }
        }
        return set;
    }

    /** Get mapping: reparented child → expanded parent */
    getChildParentMap(): Map<number, number> {
        const map = new Map<number, number>();
        for (const [expIdx, state] of this.expandedNodes) {
            for (const { el } of state.children) {
                const idx = el.getAttribute('data-node-idx');
                if (idx != null) map.set(Number(idx), expIdx);
            }
        }
        return map;
    }

    /** Edge pair keys that should be hidden (parent↔child inside decomp). */
    getHiddenDecompEdgeKeys(): Set<number> {
        const keys = new Set<number>();
        for (const [expIdx, state] of this.expandedNodes) {
            for (const { el } of state.children) {
                const idx = el.getAttribute('data-node-idx');
                if (idx != null) {
                    const ci = Number(idx);
                    keys.add(edgePairKey(expIdx, ci));
                    keys.add(edgePairKey(ci, expIdx));
                }
            }
        }
        return keys;
    }

    /**
     * Synchronise the set of expanded nodes with the desired set.
     * Collapses removed, expands added, reorders if changed.
     */
    update(desiredExpanded: Set<number>): void {
        const desiredKeyStr = [...desiredExpanded].sort((a, b) => a - b).join(',');
        if (desiredKeyStr === this.lastExpandedKeyStr) return;

        // Collapse nodes no longer desired
        for (const idx of [...this.expandedNodes.keys()]) {
            if (!desiredExpanded.has(idx)) this.collapseNode(idx);
        }
        // Expand desired nodes that aren't already expanded
        for (const idx of desiredExpanded) {
            if (!this.expandedNodes.has(idx)) this.expandNode(idx);
        }
        this.reorderNodeLayer();
        this.lastExpandedKeyStr = desiredKeyStr;
    }

    /** Collapse everything and restore DOM order. */
    collapseAll(): void {
        const hadExpanded = this.expandedNodes.size > 0;
        for (const idx of [...this.expandedNodes.keys()]) {
            this.collapseNode(idx);
        }
        if (hadExpanded) this.reorderNodeLayer();
    }

    /** Re-scan the DOM for `data-node-idx` attributes. */
    refreshNodeElMap(): void {
        this.nodeElMap.clear();
        const divs = this.nodeLayer.children;
        for (let i = 0; i < divs.length; i++) {
            const el = divs[i] as HTMLDivElement;
            const idx = el.getAttribute('data-node-idx');
            if (idx != null) this.nodeElMap.set(Number(idx), el);
        }
        // Also include elements inside any expanded decomp container
        for (const state of this.expandedNodes.values()) {
            const nested = state.container.querySelectorAll<HTMLDivElement>('[data-node-idx]');
            for (const el of nested) {
                const idx = el.getAttribute('data-node-idx');
                if (idx != null) this.nodeElMap.set(Number(idx), el);
            }
        }
    }

    // ── Internal ──

    private collapseNode(idx: number): void {
        const state = this.expandedNodes.get(idx);
        if (!state) return;

        for (const { el } of state.children) {
            el.classList.remove('hg-decomp-child');
            el.style.flex = '';
            this.nodeLayer.appendChild(el);
        }
        state.container.remove();

        const ep = state.parentEl as any;
        if (ep.__parentDown) state.parentEl.removeEventListener('mousedown', ep.__parentDown);
        if (ep.__parentUp) state.parentEl.removeEventListener('mouseup', ep.__parentUp);
        state.parentEl.classList.remove('hg-expanded');

        this.expandedNodes.delete(idx);
    }

    private expandNode(idx: number): void {
        if (this.expandedNodes.has(idx)) return;

        const node = this.layout.nodeMap.get(idx);
        if (!node || node.isAtom) return;

        const patterns = getDecompositionPatterns(this.layout, idx);
        if (patterns.length === 0) return;

        // Refresh map in case Preact re-rendered
        this.refreshNodeElMap();

        const parentEl = this.nodeElMap.get(idx);
        if (!parentEl) return;

        parentEl.classList.add('hg-expanded');

        // Create decomposition container
        const container = document.createElement('div');
        container.className = 'decomp-patterns';

        const children: { el: HTMLDivElement }[] = [];

        for (let pi = 0; pi < patterns.length; pi++) {
            const pat = patterns[pi]!;
            const row = document.createElement('div');
            row.className = 'decomp-row';
            row.style.background = ROW_COLORS[pi % ROW_COLORS.length]!;

            const label = document.createElement('span');
            label.className = 'decomp-row-label';
            label.textContent = `P${pat.patternIdx}`;
            row.appendChild(label);

            const tokens = document.createElement('div');
            tokens.className = 'decomp-tokens';

            for (const child of pat.children) {
                const childEl = this.nodeElMap.get(child.index);
                if (!childEl) continue;

                children.push({ el: childEl });
                childEl.classList.add('hg-decomp-child');
                childEl.style.flex = `${child.fraction}`;
                tokens.appendChild(childEl);
            }

            row.appendChild(tokens);
            container.appendChild(row);
        }

        // DOM event handlers on parentEl
        const onSelectNode = this.onSelectNode;
        const onParentMouseDown = (e: MouseEvent) => {
            if (e.button !== 0) return;
            const childTarget = (e.target as HTMLElement).closest('.hg-decomp-child');
            if (childTarget) {
                const cIdx = childTarget.getAttribute('data-node-idx');
                if (cIdx != null && onSelectNode) {
                    onSelectNode(Number(cIdx));
                }
            }
            e.stopPropagation();
        };
        const onParentMouseUp = (e: MouseEvent) => {
            e.stopPropagation();
        };
        parentEl.addEventListener('mousedown', onParentMouseDown);
        parentEl.addEventListener('mouseup', onParentMouseUp);
        (parentEl as any).__parentDown = onParentMouseDown;
        (parentEl as any).__parentUp = onParentMouseUp;

        parentEl.appendChild(container);
        this.expandedNodes.set(idx, { parentEl, container, children });
    }

    private reorderNodeLayer(): void {
        const elByIdx = new Map<number, HTMLDivElement>();
        const divs = this.nodeLayer.children;
        for (let i = 0; i < divs.length; i++) {
            const el = divs[i] as HTMLDivElement;
            const idx = el.getAttribute('data-node-idx');
            if (idx != null) elByIdx.set(Number(idx), el);
        }
        for (const n of this.layout.nodes) {
            const el = elByIdx.get(n.index);
            if (el) this.nodeLayer.appendChild(el);
        }
    }
}
