The repository ships graph renderers of its own: the ticket CLI/MCP `topgraph` and `subgraph` commands, the session workflow renderers `session_workflow_render_mermaid` and `session_workflow_render_terminal`, and the ticket-viewer's graph view.

The reported labelling and legend problems occur very often, suggesting the gaps are not limited to hand-written diagrams in agent responses. Audit each renderer against the convention defined by the sibling ticket "Define mermaid graph rendering conventions for agent responses".

For each renderer, report whether:

- the node label carries the ticket type;
- node sizing accommodates the label;
- a legend is emitted; and
- edge direction is documented.

Implementation of renderer fixes is explicitly out of scope. This ticket produces findings and follow-up tickets only.