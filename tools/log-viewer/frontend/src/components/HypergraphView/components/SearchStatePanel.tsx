/**
 * SearchStatePanel - Floating panel for navigating algorithm steps.
 */
import { useRef, useEffect } from 'preact/hooks';
import { searchStates, activeSearchStep, setActiveSearchStep } from '../../../store';

/**
 * Convert transition kind to display name.
 */
function getTransitionName(state: (typeof searchStates.value)[0]): string {
    const kind = state.transition?.kind ?? 'unknown';
    return kind
        .split('_')
        .map((w: string) => w.charAt(0).toUpperCase() + w.slice(1))
        .join(' ');
}

/**
 * Get CSS class for transition phase.
 */
function phaseClass(state: (typeof searchStates.value)[0]): string {
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

export function SearchStatePanel() {
    const states = searchStates.value;
    const currentStep = activeSearchStep.value;
    const listRef = useRef<HTMLDivElement>(null);

    // Auto-scroll to active item when step changes
    useEffect(() => {
        if (listRef.current && currentStep >= 0) {
            const activeEl = listRef.current.querySelector('.ssp-item.active');
            if (activeEl) {
                activeEl.scrollIntoView({ behavior: 'smooth', block: 'nearest' });
            }
        }
    }, [currentStep]);

    // Don't render if no search states
    if (states.length === 0) return null;

    const handlePrev = () => {
        const newStep = currentStep <= 0 ? 0 : currentStep - 1;
        setActiveSearchStep(newStep);
    };

    const handleNext = () => {
        const newStep = currentStep >= states.length - 1 ? states.length - 1 : currentStep + 1;
        setActiveSearchStep(newStep);
    };

    const handleItemClick = (step: number) => {
        setActiveSearchStep(step);
    };

    return (
        <div class="search-state-panel">
            <div class="ssp-header">
                <span class="ssp-title">Operation Steps</span>
                <span class="ssp-count">{states.length} steps</span>
            </div>
            <div ref={listRef} class="ssp-list">
                {states.map((state, idx) => (
                    <div
                        key={state.step}
                        class={`ssp-item ${currentStep === idx ? 'active' : ''}`}
                        onClick={() => handleItemClick(idx)}
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
        </div>
    );
}
