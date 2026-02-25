/**
 * SearchStatePanel - Floating panel for navigating algorithm steps.
 *
 * When path_id groups exist, displays each group as a collapsible section.
 * Selecting a step reconstructs the path graph up to that point.
 * Falls back to a flat list for events without path_id.
 */
import { useRef, useEffect, useState } from 'preact/hooks';
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

/**
 * A single path group section.
 */
function PathGroupSection({ group }: { group: PathGroup }) {
    const isActive = activePathId.value === group.pathId;
    const currentStep = activePathStep.value;
    const listRef = useRef<HTMLDivElement>(null);
    const [collapsed, setCollapsed] = useState(false);

    // Auto-scroll to active item
    useEffect(() => {
        if (listRef.current && isActive && currentStep >= 0) {
            const activeEl = listRef.current.querySelector('.ssp-item.active');
            if (activeEl) {
                activeEl.scrollIntoView({ behavior: 'smooth', block: 'nearest' });
            }
        }
    }, [isActive, currentStep]);

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
                    <div ref={listRef} class="ssp-group-list">
                        {group.events.map((ev, idx) => (
                            <div
                                key={idx}
                                class={`ssp-item ${currentStep === idx ? 'active' : ''}`}
                                onClick={() => handleStepClick(idx)}
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
    const listRef = useRef<HTMLDivElement>(null);

    useEffect(() => {
        if (listRef.current && currentStep >= 0) {
            const activeEl = listRef.current.querySelector('.ssp-item.active');
            if (activeEl) {
                activeEl.scrollIntoView({ behavior: 'smooth', block: 'nearest' });
            }
        }
    }, [currentStep]);

    const handlePrev = () => {
        if (currentStep > 0) setActiveSearchStep(currentStep - 1);
    };

    const handleNext = () => {
        if (currentStep < states.length - 1) setActiveSearchStep(currentStep + 1);
    };

    return (
        <>
            <div ref={listRef} class="ssp-list">
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
