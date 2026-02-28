/**
 * SearchStatePanel - Floating panel for navigating algorithm steps.
 *
 * When path_id groups exist, displays each group as a collapsible section.
 * Within each path group, consecutive steps on the same node are collapsed
 * into a single row that can be expanded to show internal sub-steps.
 * Selecting a step reconstructs the path graph up to that point.
 * Falls back to a flat list for events without path_id.
 */
import { useState, useMemo, useEffect } from 'preact/hooks';
import {
    searchStates,
    activeSearchStep,
    setActiveSearchStep,
    pathGroups,
    activePathId,
    activePathStep,
    setActivePathId,
    setActivePathStep,
    type PathGroup,
} from '../../../store';
import type { GraphOpEvent } from '../../../types/generated/GraphOpEvent';

/**
 * Convert transition kind to display name.
 */
function getTransitionName(state: { transition?: { kind?: string } }): string {
    const kind = state.transition?.kind ?? 'unknown';
    return kind
        .split('_')
        .map((w: string) => w.charAt(0).toUpperCase() + w.slice(1))
        .join(' ');
}

/**
 * Get CSS class for transition phase.
 */
function phaseClass(state: { transition?: { kind?: string } }): string {
    const kind = state.transition?.kind ?? 'unknown';
    return `phase-${kind.toLowerCase().replace(/_/g, '')}`;
}

/**
 * Get operation type badge emoji.
 */
function opTypeBadge(opType: string): string {
    if (opType === 'search') return '🔍';
    if (opType === 'insert') return '+';
    return '📖';
}

/**
 * Get path transition display name (for the path-specific column).
 */
function pathTransitionName(pt: { kind?: string } | null | undefined): string {
    if (!pt?.kind) return '';
    return pt.kind
        .split('_')
        .map((w: string) => w.charAt(0).toUpperCase() + w.slice(1))
        .join(' ');
}

// ── Node grouping ──────────────────────────────────────────────────────

/**
 * A group of consecutive events that share the same `location.selected_node`.
 */
interface NodeGroup {
    /** The shared node index, or null when events have no selected node. */
    nodeIndex: number | null;
    /** The events in this group (references into the parent PathGroup.events). */
    events: GraphOpEvent[];
    /** Indices of these events within the parent PathGroup.events array. */
    stepIndices: number[];
}

/**
 * Extract the node that an event should be grouped under.
 *
 * - `visit_parent` steps are grouped with the parent being visited (`to`).
 * - `visit_child` steps are grouped with the child being visited (`to`).
 * - All other transitions use `location.selected_node`.
 */
function selectedNode(ev: GraphOpEvent): number | null {
    const t = ev.transition;
    if (!t) return ev.location?.selected_node ?? null;
    switch (t.kind) {
        case 'visit_parent':
        case 'visit_child':
            return t.to;
        default:
            return ev.location?.selected_node ?? null;
    }
}

/**
 * Group a flat event list into consecutive runs sharing the same selected_node.
 */
function groupByNode(events: GraphOpEvent[]): NodeGroup[] {
    if (events.length === 0) return [];
    const groups: NodeGroup[] = [];
    const first = events[0]!;
    let cur: NodeGroup = { nodeIndex: selectedNode(first), events: [first], stepIndices: [0] };
    for (let i = 1; i < events.length; i++) {
        const ev = events[i]!;
        const node = selectedNode(ev);
        if (node === cur.nodeIndex) {
            cur.events.push(ev);
            cur.stepIndices.push(i);
        } else {
            groups.push(cur);
            cur = { nodeIndex: node, events: [ev], stepIndices: [i] };
        }
    }
    groups.push(cur);
    return groups;
}

// ── Render a single event row ──────────────────────────────────────────

function EventItem({
    ev,
    isActive,
    onClick,
    indented,
}: {
    ev: GraphOpEvent;
    isActive: boolean;
    onClick: () => void;
    indented?: boolean;
}) {
    return (
        <div
            class={`ssp-item ${isActive ? 'active' : ''} ${indented ? 'ssp-item-indented' : ''}`}
            onClick={onClick}
        >
            <span class="ssp-step">
                {opTypeBadge(ev.op_type)}
                {ev.step}
            </span>
            <div class="ssp-content">
                <div class={`ssp-phase ${phaseClass(ev)}`}>
                    {getTransitionName(ev)}
                </div>
                <div class="ssp-path-trans">
                    ↳ {pathTransitionName(ev.path_transition)}
                </div>
                <div class="ssp-desc">{ev.description}</div>
            </div>
        </div>
    );
}

// ── Node group row (collapsed / expanded) ──────────────────────────────

function NodeGroupRow({
    group,
    currentStep,
    onStepClick,
    expandedNodes,
    onToggle,
}: {
    group: NodeGroup;
    currentStep: number;
    onStepClick: (idx: number) => void;
    expandedNodes: Set<number>;
    onToggle: (firstIdx: number) => void;
}) {
    const firstIdx = group.stepIndices[0]!;
    const containsActive = group.stepIndices.includes(currentStep);

    // Single-event groups render directly — no collapse needed
    if (group.events.length === 1) {
        return (
            <EventItem
                ev={group.events[0]!}
                isActive={currentStep === firstIdx}
                onClick={() => onStepClick(firstIdx)}
            />
        );
    }

    const isExpanded = expandedNodes.has(firstIdx) || containsActive;
    const firstEv = group.events[0]!;
    const lastEv = group.events[group.events.length - 1]!;

    return (
        <div class={`ssp-node-group ${containsActive ? 'active-node-group' : ''}`}>
            <div
                class="ssp-node-group-header"
                onClick={() => onToggle(firstIdx)}
            >
                <span class="ssp-node-group-chevron">{isExpanded ? '▾' : '▸'}</span>
                <span class="ssp-node-group-label">
                    node {group.nodeIndex ?? '?'}
                </span>
                <span class="ssp-node-group-range">
                    {firstEv.step}–{lastEv.step}
                </span>
                <span class="ssp-node-group-count">{group.events.length}</span>
            </div>
            {isExpanded && (
                <div class="ssp-node-group-items">
                    {group.events.map((ev, i) => {
                        const idx = group.stepIndices[i]!;
                        return (
                            <EventItem
                                key={idx}
                                ev={ev}
                                isActive={currentStep === idx}
                                onClick={() => onStepClick(idx)}
                                indented
                            />
                        );
                    })}
                </div>
            )}
        </div>
    );
}

// ── Path group section ─────────────────────────────────────────────────

/**
 * A single path group section.
 */
function PathGroupSection({ group }: { group: PathGroup }) {
    const isActive = activePathId.value === group.pathId;
    const currentStep = activePathStep.value;
    const [collapsed, setCollapsed] = useState(false);
    const [expandedNodes, setExpandedNodes] = useState<Set<number>>(new Set());

    const nodeGroups = useMemo(() => groupByNode(group.events), [group.events]);

    const handleGroupClick = () => {
        if (isActive) {
            setCollapsed(!collapsed);
        } else {
            setActivePathId(group.pathId);
            setCollapsed(false);
        }
    };

    const handleStepClick = (stepIdx: number) => {
        if (!isActive) {
            setActivePathId(group.pathId);
        }
        setActivePathStep(stepIdx);
        // Also sync the global step so the main event view stays in sync
        const globalIdx = group.globalIndices[stepIdx];
        if (globalIdx != null) {
            setActiveSearchStep(globalIdx);
        }
    };

    const handleToggleNodeGroup = (firstIdx: number) => {
        setExpandedNodes(prev => {
            const next = new Set(prev);
            if (next.has(firstIdx)) {
                next.delete(firstIdx);
            } else {
                next.add(firstIdx);
            }
            return next;
        });
    };

    const handlePrev = () => {
        if (currentStep > 0) {
            handleStepClick(currentStep - 1);
        }
    };

    const handleNext = () => {
        if (currentStep < group.events.length - 1) {
            handleStepClick(currentStep + 1);
        }
    };

    return (
        <div class={`ssp-path-group ${isActive ? 'active-group' : ''}`}>
            <div class="ssp-group-header" onClick={handleGroupClick}>
                <span class={`ssp-group-chevron ${collapsed && isActive ? 'collapsed' : ''}`}>
                    {isActive ? (collapsed ? '▶' : '▼') : '▷'}
                </span>
                <span class="ssp-group-id" title={group.pathId}>
                    {group.pathId.length > 20 ? group.pathId.slice(0, 18) + '…' : group.pathId}
                </span>
                <span class="ssp-group-count">{group.events.length}</span>
            </div>

            {isActive && !collapsed && (
                <>
                    <div
                        class="ssp-group-list"
                    >
                        {nodeGroups.map((ng, gi) => (
                            <NodeGroupRow
                                key={gi}
                                group={ng}
                                currentStep={currentStep}
                                onStepClick={handleStepClick}
                                expandedNodes={expandedNodes}
                                onToggle={handleToggleNodeGroup}
                            />
                        ))}
                    </div>
                    <div class="ssp-controls">
                        <button class="ssp-btn" onClick={handlePrev} disabled={currentStep <= 0}>
                            ← Prev
                        </button>
                        <span class="ssp-position">
                            {currentStep >= 0 ? currentStep + 1 : '—'} / {group.events.length}
                        </span>
                        <button class="ssp-btn" onClick={handleNext} disabled={currentStep >= group.events.length - 1}>
                            Next →
                        </button>
                    </div>
                </>
            )}
        </div>
    );
}

/**
 * Flat list (no path groups) — original behavior for events without path_id.
 */
function FlatStepList() {
    const states = searchStates.value;
    const currentStep = activeSearchStep.value;

    const handlePrev = () => {
        if (currentStep > 0) setActiveSearchStep(currentStep - 1);
    };

    const handleNext = () => {
        if (currentStep < states.length - 1) setActiveSearchStep(currentStep + 1);
    };

    return (
        <>
            <div
                class="ssp-list"
            >
                {states.map((state, idx) => (
                    <div
                        key={state.step}
                        class={`ssp-item ${currentStep === idx ? 'active' : ''}`}
                        onClick={() => setActiveSearchStep(idx)}
                    >
                        <span class="ssp-step">
                            {opTypeBadge(state.op_type)}
                            {state.step}
                        </span>
                        <div class="ssp-content">
                            <div class={`ssp-phase ${phaseClass(state)}`}>{getTransitionName(state)}</div>
                            <div class="ssp-desc">{state.description}</div>
                        </div>
                    </div>
                ))}
            </div>
            <div class="ssp-controls">
                <button class="ssp-btn" onClick={handlePrev} disabled={currentStep <= 0}>
                    ← Prev
                </button>
                <span class="ssp-position">
                    {currentStep >= 0 ? currentStep + 1 : '—'} / {states.length}
                </span>
                <button class="ssp-btn" onClick={handleNext} disabled={currentStep >= states.length - 1}>
                    Next →
                </button>
            </div>
        </>
    );
}

export function SearchStatePanel() {
    const states = searchStates.value;
    const groups = pathGroups.value;

    // Don't render if no search states
    if (states.length === 0) return null;

    const hasGroups = groups.length > 0;
    const totalSteps = states.length;
    const groupCount = groups.length;

    // Window-level Up/Down navigation for search steps
    useEffect(() => {
        function handleKeyDown(e: KeyboardEvent) {
            const tag = (e.target as HTMLElement)?.tagName;
            if (tag === 'INPUT' || tag === 'TEXTAREA' || tag === 'SELECT') return;
            // Don't intercept if focus is on a list that handles its own arrows
            const target = e.target as HTMLElement;
            if (target.closest('.file-list, .log-entries')) return;

            if (e.key !== 'ArrowUp' && e.key !== 'ArrowDown') return;
            e.preventDefault();

            const delta = e.key === 'ArrowDown' ? 1 : -1;

            if (hasGroups) {
                // Navigate within the active path group
                const activeGroup = groups.find(g => g.pathId === activePathId.value);
                if (!activeGroup) {
                    // No active group yet — activate the first one
                    if (groups.length > 0) {
                        setActivePathId(groups[0]!.pathId);
                        setActivePathStep(0);
                        const gi = groups[0]!.globalIndices[0];
                        if (gi != null) setActiveSearchStep(gi);
                    }
                    return;
                }
                const cur = activePathStep.value;
                const next = Math.max(0, Math.min(activeGroup.events.length - 1, cur + delta));
                if (next !== cur) {
                    setActivePathStep(next);
                    const gi = activeGroup.globalIndices[next];
                    if (gi != null) setActiveSearchStep(gi);
                }
            } else {
                // Flat list navigation
                const cur = activeSearchStep.value;
                const next = Math.max(0, Math.min(states.length - 1, cur + delta));
                if (next !== cur) {
                    setActiveSearchStep(next);
                }
            }
        }

        window.addEventListener('keydown', handleKeyDown);
        return () => window.removeEventListener('keydown', handleKeyDown);
    }, [states, groups, hasGroups]);

    // Auto-scroll the active item into view
    useEffect(() => {
        const panel = document.querySelector('.search-state-panel');
        if (!panel) return;
        const activeEl = panel.querySelector('.ssp-item.active');
        if (activeEl) {
            activeEl.scrollIntoView({ behavior: 'smooth', block: 'nearest' });
        }
    }, [activeSearchStep.value, activePathStep.value]);

    return (
        <div class="search-state-panel">
            <div class="ssp-header">
                <span class="ssp-title">
                    {hasGroups ? 'Search Paths' : 'Operation Steps'}
                </span>
                <span class="ssp-count">
                    {hasGroups
                        ? `${groupCount} path${groupCount !== 1 ? 's' : ''} · ${totalSteps} steps`
                        : `${totalSteps} steps`
                    }
                </span>
            </div>

            {hasGroups ? (
                <div class="ssp-groups-container">
                    {groups.map(group => (
                        <PathGroupSection key={group.pathId} group={group} />
                    ))}
                </div>
            ) : (
                <FlatStepList />
            )}
        </div>
    );
}
