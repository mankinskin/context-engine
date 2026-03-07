/**
 * Props interface for the HypergraphView core component.
 *
 * Decouples the rendering engine from log-viewer-specific signal dependencies
 * so that the core can eventually be extracted to viewer-api.
 */
import type { GraphOpEvent, VizPathGraph, SnapshotEdge } from '../../types/generated';
import type { HypergraphSnapshot } from '../../types';

/**
 * Data props accepted by HypergraphViewCore.
 * A thin wrapper reads signals and passes these as props.
 */
export interface HypergraphViewProps {
    /** The current graph snapshot to render */
    snapshot: HypergraphSnapshot | null;
    /** Current graph operation event (search/insert step) */
    currentEvent: GraphOpEvent | null;
    /** Active search path graph for edge highlighting */
    searchPath: VizPathGraph | null;
    /** Whether auto-layout is active (expand/contract around selected) */
    autoLayout: boolean;
    /** Snapshot edges for edge key computation */
    snapshotEdges: SnapshotEdge[] | null;
    /**
     * Opaque key that changes when the search step changes.
     * Used to trigger auto-focus on the primary node of a new step.
     * Typically `activeSearchStep.value + '/' + activePathStep.value`.
     */
    stepKey: string;
}

// ── Nesting View Types ──

/**
 * Settings for the hierarchical nesting view mode.
 */
export interface NestingSettings {
    /** Master toggle for nesting view */
    enabled: boolean;
    /** true = show duplicates, false = reparent/move nodes */
    duplicateMode: boolean;
    /** How many parent shell levels to show (1-5) */
    parentDepth: number;
    /** How many child levels to show inside expanded node (1-3) */
    childDepth: number;
}

/**
 * A parent node positioned on a concentric shell around the selected node.
 */
export interface ShellNode {
    /** Index of the node in the layout */
    nodeIdx: number;
    /** Shell level: 1 = direct parent, 2 = grandparent, etc. */
    shellLevel: number;
    /** Position on shell arc (radians, 0 = right, PI/2 = top) */
    angle: number;
    /** Visual scale multiplier (larger for deeper shells) */
    scale: number;
}

/**
 * A duplicate node rendered inside an expanded parent.
 */
export interface DuplicateNode {
    /** Index of the original node */
    originalIdx: number;
    /** Unique ID for DOM key */
    duplicateId: string;
    /** Index of the expanded parent containing this duplicate */
    parentIdx: number;
    /** Position in the row layout (0, 1, 2, ...) */
    slotIndex: number;
}

/**
 * Current state of the nesting view.
 */
export interface NestingState {
    /** Current nesting settings */
    settings: NestingSettings;
    /** Index of the currently selected/centered node (-1 if none) */
    selectedIdx: number;
    /** Parent nodes arranged in concentric shells */
    shells: ShellNode[];
    /** Child duplicates inside the expanded parent */
    duplicates: DuplicateNode[];
    /** Set of node indices that have duplicates (for dimming originals) */
    duplicatedOriginals: Set<number>;
}

/** Default nesting settings */
export const DEFAULT_NESTING_SETTINGS: NestingSettings = {
    enabled: true,
    duplicateMode: true,
    parentDepth: 2,
    childDepth: 1,
};
