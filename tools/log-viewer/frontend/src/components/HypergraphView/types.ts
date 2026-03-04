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
