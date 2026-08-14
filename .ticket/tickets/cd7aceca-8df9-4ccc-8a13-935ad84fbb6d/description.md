When an agent renders a ticket dependency or ordering graph in a response, the diagram must make labels, edge semantics, and reading direction immediately understandable.

The current output has four gaps:

1. Nodes are frequently too small to give ticket titles adequate room, so titles are cramped or truncated.
2. Ticket type is not visible. The label must show the type before the short id, for example `bug · 07a3eb2d`, as the only extra field to avoid clutter.
3. There is no legend distinguishing edge kinds such as `depends_on` and `linked`.
4. The reading direction is not stated, making arrows ambiguous.

Document a convention that defines node-label composition as `<type> · <short-id>` on the first line and the title on a following line via a line break, allowing the node to grow. The convention must require a legend block distinguishing edge kinds, an explicit statement of reading direction in or next to the diagram, and Mermaid sizing and spacing directives that prevent cramped nodes.

The convention must also choose and state whether arrows follow the stored `depends_on` direction (`blocked → blocker`) or the execution direction (`do-first → do-later`). Those directions are inverses, and silently mixing the two is the root of the ambiguity.

The likely home is a new file under `.agents/instructions/orchestration/`. Agent templates that emit diagrams should reference the shared convention rather than duplicating it.