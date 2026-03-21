// GraphStub: placeholder panel for the hypergraph dependency view.
// Replaced by the real implementation in Wave 2 Track G (2772fe5d).

import { JSX } from 'preact';
import { openTicketId } from '../store';

export function GraphStub(): JSX.Element {
  const id = openTicketId.value;

  return (
    <div class="graph-stub">
      <div class="graph-stub__header">Dependency Graph</div>
      <div class="graph-stub__body">
        {id ? (
          <p class="graph-stub__note">
            Graph view coming in Wave 2 Track G.
            <br />
            <small>Ticket: {id.slice(0, 8)}…</small>
          </p>
        ) : (
          <p class="graph-stub__note">
            Select a ticket to explore its dependency graph.
          </p>
        )}
      </div>
    </div>
  );
}
