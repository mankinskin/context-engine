// Re-export preact and signals to ensure single instance across consuming packages
export * from 'preact';
export * from 'preact/hooks';
export * as signals from '@preact/signals';
export { signal, computed, effect, batch } from '@preact/signals';
export type { Signal, ReadonlySignal } from '@preact/signals';

// Re-export common components
export * from './components/TreeView';
export * from './components/Spinner';
export * from './components/TabBar';
export * from './components/Icons';
export * from './components/Header';
export * from './components/Sidebar';
export * from './components/Layout';
export * from './components/CodeViewer';

// Re-export session utilities
export * from './session';
