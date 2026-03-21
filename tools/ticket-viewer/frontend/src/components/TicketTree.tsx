// TicketTree: workspace-aware ticket list rendered as a tree via viewer-api's
// TreeView.  Supports filtering by state and a text search over title/id.

import { JSX } from 'preact';
import { TreeView, type TreeNode } from '@context-engine/viewer-api-frontend';
import {
  activeTab,
  authToken,
  detailError,
  detailLoading,
  filteredTickets,
  globalError,
  openTicketDescription,
  openTicketDetail,
  openTicketId,
  selectedWorkspace,
  ticketsLoading,
  treeFilter,
  treeStateFilter,
} from '../store';
import { getTicket, getTicketDescription } from '../api';
import type { TicketSummary } from '../types';

const STATE_BADGE_COLORS: Record<string, string> = {
  open: '#4a9eff',
  'in-progress': '#f0a500',
  review: '#9b7fe8',
  validating: '#63b3ed',
  validated: '#48bb78',
  done: '#68d391',
  blocked: '#fc8181',
  cancelled: '#a0aec0',
};

function stateBadge(state: string | null): string | undefined {
  if (!state) return undefined;
  return state;
}

/** Convert flat ticket list to tree nodes grouped by state. */
function buildTree(tickets: TicketSummary[]): TreeNode<TicketSummary>[] {
  // Group by state.
  const groups = new Map<string, TicketSummary[]>();
  for (const t of tickets) {
    const key = t.state ?? 'unknown';
    const list = groups.get(key) ?? [];
    list.push(t);
    groups.set(key, list);
  }

  const stateOrder = [
    'open', 'in-progress', 'review', 'validating', 'validated',
    'done', 'blocked', 'cancelled', 'unknown',
  ];

  const orderedKeys = [
    ...stateOrder.filter((s) => groups.has(s)),
    ...[...groups.keys()].filter((k) => !stateOrder.includes(k)),
  ];

  return orderedKeys.map((state) => {
    const group = groups.get(state)!;
    return {
      id: `group:${state}`,
      label: state,
      icon: 'folder',
      badge: group.length,
      children: group.map((t) => ({
        id: t.id,
        label: t.title ?? t.id.slice(0, 8),
        icon: 'doc',
        badge: stateBadge(t.state),
        data: t,
        tooltip: t.id,
      })),
    };
  });
}

async function openTicket(id: string) {
  if (openTicketId.value === id) return;
  openTicketId.value = id;
  openTicketDetail.value = null;
  openTicketDescription.value = null;
  detailError.value = null;
  detailLoading.value = true;
  activeTab.value = 'description';
  const ws = selectedWorkspace.value;
  const token = authToken.value || undefined;

  try {
    const [detailResp, descResp] = await Promise.all([
      getTicket(id, ws, token),
      getTicketDescription(id, ws, token),
    ]);
    openTicketDetail.value = detailResp.ticket;
    openTicketDescription.value = descResp.description;
  } catch (e) {
    detailError.value = String(e);
    globalError.value = String(e);
  } finally {
    detailLoading.value = false;
  }
}

export function TicketTree(): JSX.Element {
  const isLoading = ticketsLoading.value;
  const list = filteredTickets.value;
  const nodes = buildTree(list);

  return (
    <div class="ticket-tree">
      {/* Search + filter toolbar */}
      <div class="ticket-tree__toolbar">
        <input
          type="search"
          class="ticket-tree__search"
          placeholder="Search tickets…"
          value={treeFilter.value}
          onInput={(e) =>
            (treeFilter.value = (e.target as HTMLInputElement).value)
          }
        />
        <select
          class="ticket-tree__state-filter"
          value={treeStateFilter.value}
          onChange={(e) =>
            (treeStateFilter.value = (e.target as HTMLSelectElement).value)
          }
        >
          <option value="">All states</option>
          {Object.keys(STATE_BADGE_COLORS).map((s) => (
            <option key={s} value={s}>
              {s}
            </option>
          ))}
        </select>
      </div>

      {isLoading && <div class="ticket-tree__loading">Loading…</div>}

      {!isLoading && list.length === 0 && (
        <div class="ticket-tree__empty">
          {selectedWorkspace.value
            ? 'No tickets match the current filter.'
            : 'Select a workspace to begin.'}
        </div>
      )}

      {!isLoading && list.length > 0 && (
        <TreeView
          nodes={nodes}
          selectedId={openTicketId.value ?? undefined}
          defaultExpanded={nodes.map((n) => n.id)}
          onSelect={(node: TreeNode<TicketSummary>) => {
            // Only leaf nodes (tickets) are selectable; group folders are ignored.
            if (!node.id.startsWith('group:')) {
              void openTicket(node.id);
            }
          }}
        />
      )}
    </div>
  );
}
