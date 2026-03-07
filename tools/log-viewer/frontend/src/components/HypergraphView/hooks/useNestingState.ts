/**
 * useNestingState — manages nesting view settings with localStorage persistence.
 *
 * Provides settings state and a NestingState object that tracks the current
 * selection, shell layout, and duplicate nodes.
 */
import { signal, computed } from '@preact/signals';
import { useMemo } from 'preact/hooks';
import type { GraphLayout } from '../layout';
import {
    type NestingSettings,
    type NestingState,
    type ShellNode,
    type DuplicateNode,
    DEFAULT_NESTING_SETTINGS,
} from '../types';

// ── LocalStorage Keys ──
const STORAGE_KEY = 'hg-nesting-settings';

// ── Signals for Settings ──
function loadSettings(): NestingSettings {
    try {
        const stored = localStorage.getItem(STORAGE_KEY);
        if (stored) {
            const parsed = JSON.parse(stored);
            return { ...DEFAULT_NESTING_SETTINGS, ...parsed };
        }
    } catch {
        // Ignore parse errors
    }
    return DEFAULT_NESTING_SETTINGS;
}

/** Global nesting settings signal */
export const nestingSettings = signal<NestingSettings>(loadSettings());

/** Persist settings to localStorage when they change */
if (typeof window !== 'undefined') {
    // Subscribe to changes and save
    nestingSettings.subscribe((settings) => {
        try {
            localStorage.setItem(STORAGE_KEY, JSON.stringify(settings));
        } catch {
            // Ignore storage errors
        }
    });
}

// ── Derived Signals ──
export const nestingEnabled = computed(() => nestingSettings.value.enabled);
export const duplicateModeEnabled = computed(() => nestingSettings.value.duplicateMode);
export const parentDepth = computed(() => nestingSettings.value.parentDepth);
export const childDepth = computed(() => nestingSettings.value.childDepth);

// ── Settings Mutators ──
export function setNestingEnabled(enabled: boolean): void {
    nestingSettings.value = { ...nestingSettings.value, enabled };
}

export function setDuplicateMode(duplicateMode: boolean): void {
    nestingSettings.value = { ...nestingSettings.value, duplicateMode };
}

export function setParentDepth(depth: number): void {
    const parentDepth = Math.max(1, Math.min(5, depth));
    nestingSettings.value = { ...nestingSettings.value, parentDepth };
}

export function setChildDepth(depth: number): void {
    const childDepth = Math.max(1, Math.min(3, depth));
    nestingSettings.value = { ...nestingSettings.value, childDepth };
}

// ── Shell Layout Computation ──

/**
 * Compute parent shell positions around the selected node.
 */
export function computeShellLayout(
    layout: GraphLayout,
    centerIdx: number,
    maxDepth: number,
): ShellNode[] {
    if (centerIdx < 0) return [];

    const shells: ShellNode[] = [];
    const visited = new Set<number>([centerIdx]);

    let currentLevel = [centerIdx];
    for (let level = 1; level <= maxDepth; level++) {
        const nextLevel: number[] = [];
        for (const idx of currentLevel) {
            const node = layout.nodeMap.get(idx);
            if (!node) continue;
            for (const parentIdx of node.parentIndices) {
                if (visited.has(parentIdx)) continue;
                visited.add(parentIdx);
                nextLevel.push(parentIdx);
            }
        }

        if (nextLevel.length === 0) break;

        // Distribute parents on arc above the center
        const arcSpan = Math.PI * 0.6; // 108 degrees
        const startAngle = Math.PI / 2 - arcSpan / 2;
        nextLevel.forEach((parentIdx, i) => {
            const t = nextLevel.length > 1 ? i / (nextLevel.length - 1) : 0.5;
            shells.push({
                nodeIdx: parentIdx,
                shellLevel: level,
                angle: startAngle + t * arcSpan,
                scale: 1 + level * 0.4, // 1.4, 1.8, 2.2, ...
            });
        });

        currentLevel = nextLevel;
    }

    return shells;
}

// ── Duplicate Node Computation ──

/**
 * Create duplicate nodes for children of the selected node.
 */
export function computeDuplicates(
    layout: GraphLayout,
    centerIdx: number,
    _maxDepth: number, // TODO: Support nested expansion with depth > 1
): DuplicateNode[] {
    if (centerIdx < 0) return [];

    const duplicates: DuplicateNode[] = [];
    const centerNode = layout.nodeMap.get(centerIdx);
    if (!centerNode) return [];

    // For now, just direct children (depth 1)
    // Future: support nested expansion with maxDepth > 1
    centerNode.childIndices.forEach((childIdx, slotIndex) => {
        duplicates.push({
            originalIdx: childIdx,
            duplicateId: `dup-${centerIdx}-${childIdx}`,
            parentIdx: centerIdx,
            slotIndex,
        });
    });

    return duplicates;
}

// ── Hook ──

export interface UseNestingStateResult {
    settings: NestingSettings;
    state: NestingState;
    setEnabled: (enabled: boolean) => void;
    setDuplicateMode: (mode: boolean) => void;
    setParentDepth: (depth: number) => void;
    setChildDepth: (depth: number) => void;
}

/**
 * Hook to manage nesting view state for the hypergraph visualization.
 */
export function useNestingState(
    layout: GraphLayout | null,
    selectedIdx: number,
): UseNestingStateResult {
    const settings = nestingSettings.value;

    // Compute shells and duplicates based on current selection
    const state = useMemo<NestingState>(() => {
        if (!layout || selectedIdx < 0 || !settings.enabled) {
            return {
                settings,
                selectedIdx: -1,
                shells: [],
                duplicates: [],
                duplicatedOriginals: new Set(),
            };
        }

        const shells = computeShellLayout(layout, selectedIdx, settings.parentDepth);
        const duplicates = settings.duplicateMode
            ? computeDuplicates(layout, selectedIdx, settings.childDepth)
            : [];

        const duplicatedOriginals = new Set(duplicates.map(d => d.originalIdx));

        return {
            settings,
            selectedIdx,
            shells,
            duplicates,
            duplicatedOriginals,
        };
    }, [layout, selectedIdx, settings]);

    return {
        settings,
        state,
        setEnabled: setNestingEnabled,
        setDuplicateMode: setDuplicateMode,
        setParentDepth: setParentDepth,
        setChildDepth: setChildDepth,
    };
}
