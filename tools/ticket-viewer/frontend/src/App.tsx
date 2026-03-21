// Root application component.
//
// Implements the tri-pane layout from wireframes-v0.1.md:
//   Left pane:   Workspace picker + ticket tree
//   Center pane: Tabbed description.md / ticket.toml viewer
//   Right pane:  Graph stub (Wave 2 Track G placeholder)

import { JSX } from 'preact';
import { useEffect } from 'preact/hooks';
import { Layout } from '@context-engine/viewer-api-frontend';
import { WorkspacePicker } from './components/WorkspacePicker';
import { TicketTree } from './components/TicketTree';
import { TicketContent } from './components/TicketContent';
import { GraphStub } from './components/GraphStub';
import {
  authToken,
  globalError,
  selectedWorkspace,
  workspaces,
  workspacesLoading,
  tickets,
  ticketsLoading,
  restoreWorkspaceState,
} from './store';
import { listWorkspaces, listTickets } from './api';

export function App(): JSX.Element {
  // Load workspace list on mount.
  useEffect(() => {
    async function load() {
      workspacesLoading.value = true;
      try {
        const resp = await listWorkspaces(authToken.value || undefined);
        workspaces.value = resp.workspaces;

        // Auto-select first workspace if none saved.
        if (!selectedWorkspace.value && resp.workspaces.length > 0) {
          const first = resp.workspaces[0].name;
          selectedWorkspace.value = first;
          restoreWorkspaceState(first);

          ticketsLoading.value = true;
          try {
            const ticketResp = await listTickets(
              first,
              {},
              authToken.value || undefined,
            );
            tickets.value = ticketResp.items;
          } finally {
            ticketsLoading.value = false;
          }
        }
      } catch (e) {
        globalError.value = String(e);
      } finally {
        workspacesLoading.value = false;
      }
    }
    void load();
  }, []);

  const error = globalError.value;

  return (
    <div class="ticket-viewer-app">
      {error && (
        <div class="global-error-banner" role="alert">
          <strong>Error:</strong> {error}
          <button
            class="global-error-banner__dismiss"
            onClick={() => (globalError.value = null)}
            aria-label="Dismiss error"
          >
            ×
          </button>
        </div>
      )}

      <Layout
        header={
          <header class="app-header">
            <span class="app-header__title">Ticket Viewer</span>
            <span class="app-header__workspace">
              {selectedWorkspace.value || ''}
            </span>
          </header>
        }
        sidebar={
          <div class="left-pane">
            <WorkspacePicker />
            <TicketTree />
          </div>
        }
      >
        {/* Center + right pane inside a split container */}
        <div class="center-right-split">
          <div class="center-pane">
            <TicketContent />
          </div>
          <div class="right-pane">
            <GraphStub />
          </div>
        </div>
      </Layout>
    </div>
  );
}
