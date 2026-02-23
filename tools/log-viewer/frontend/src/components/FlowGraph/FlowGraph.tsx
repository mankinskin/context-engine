import { useEffect, useRef, useMemo, useState } from 'preact/hooks';
import { signal } from '@preact/signals';
import cytoscape from 'cytoscape';
import dagre from 'cytoscape-dagre';
import { entries, currentFile, selectEntry, setTab } from '../../store';
import type { LogEntry } from '../../types';

// Register dagre layout
cytoscape.use(dagre);

/** Expose the live Cytoscape instance so WgpuOverlay can read node positions. */
export const cytoscapeInstance = signal<cytoscape.Core | null>(null);

interface GraphNode {
  id: string;
  entry: LogEntry;
  type: 'event' | 'span_enter' | 'span_exit';
}

interface GraphEdge {
  source: string;
  target: string;
  type: 'sequence' | 'span';
}

/** Node dimensions for layout and HTML overlays */
const NODE_WIDTH = 340;
const NODE_HEIGHT = 72;

function buildGraph(logEntries: LogEntry[]): { nodes: GraphNode[]; edges: GraphEdge[] } {
  const nodes: GraphNode[] = [];
  const edges: GraphEdge[] = [];
  const spanStack: { name: string; nodeId: string }[] = [];
  
  let prevNodeId: string | null = null;
  
  for (const entry of logEntries) {
    const nodeId = `node-${entry.line_number}`;
    
    nodes.push({
      id: nodeId,
      entry,
      type: entry.event_type as 'event' | 'span_enter' | 'span_exit'
    });
    
    // Add sequence edge from previous node
    if (prevNodeId) {
      edges.push({ source: prevNodeId, target: nodeId, type: 'sequence' });
    }
    
    // Handle span relationships
    if (entry.event_type === 'span_enter' && entry.span_name) {
      spanStack.push({ name: entry.span_name, nodeId });
    } else if (entry.event_type === 'span_exit' && entry.span_name) {
      // Find matching span enter
      const spanIdx = spanStack.findLastIndex(s => s.name === entry.span_name);
      if (spanIdx >= 0) {
        const spanEntry = spanStack[spanIdx];
        if (spanEntry) {
          edges.push({ source: spanEntry.nodeId, target: nodeId, type: 'span' });
        }
        spanStack.splice(spanIdx, 1);
      }
    }
    
    prevNodeId = nodeId;
  }
  
  return { nodes, edges };
}

export function FlowGraph() {
  const containerRef = useRef<HTMLDivElement>(null);
  const overlayRef = useRef<HTMLDivElement>(null);
  const cyRef = useRef<cytoscape.Core | null>(null);
  
  const logEntries = entries.value;
  const file = currentFile.value;

  // Compute graph structure
  const { nodes: allNodes, edges: allEdges } = useMemo(
    () => buildGraph(logEntries),
    [logEntries]
  );

  const maxNodes = 200;
  const limitedNodes = useMemo(() => allNodes.slice(0, maxNodes), [allNodes]);
  const limitedNodeIds = useMemo(
    () => new Set(limitedNodes.map(n => n.id)),
    [limitedNodes]
  );
  const limitedEdges = useMemo(
    () => allEdges.filter(e => limitedNodeIds.has(e.source) && limitedNodeIds.has(e.target)),
    [allEdges, limitedNodeIds]
  );

  // Node positions from Cytoscape layout — set once after layout completes
  const [positions, setPositions] = useState<Map<string, { x: number; y: number }>>(new Map());

  useEffect(() => {
    if (!containerRef.current || limitedNodes.length === 0) return;
    
    if (cyRef.current) {
      cyRef.current.destroy();
    }
    
    const cy = cytoscape({
      container: containerRef.current,
      elements: [
        ...limitedNodes.map(node => ({
          data: { 
            id: node.id,
            type: node.type,
            level: node.entry.level,
          }
        })),
        ...limitedEdges.map((edge, i) => ({
          data: { 
            id: `edge-${i}`, 
            source: edge.source, 
            target: edge.target,
            type: edge.type
          }
        }))
      ],
      style: [
        {
          selector: 'node',
          style: {
            // Invisible nodes — just for layout; HTML overlays render on top
            'background-opacity': 0,
            'border-width': 0,
            'label': '',
            'width': NODE_WIDTH,
            'height': NODE_HEIGHT,
          }
        },
        {
          selector: 'edge',
          style: {
            'width': 2,
            'line-color': '#555',
            'target-arrow-color': '#555',
            'target-arrow-shape': 'triangle',
            'curve-style': 'bezier'
          }
        },
        {
          selector: 'edge[type="span"]',
          style: {
            'line-color': '#27ae60',
            'target-arrow-color': '#27ae60',
            'line-style': 'dashed'
          }
        }
      ],
      layout: {
        name: 'dagre',
        rankDir: 'TB',
        nodeSep: 30,
        rankSep: 50,
        edgeSep: 10,
        fit: false,
        padding: 30
      } as any
    });

    // Sync the HTML overlay container transform to match Cytoscape pan/zoom
    const syncOverlay = () => {
      if (!overlayRef.current) return;
      const pan = cy.pan();
      const zoom = cy.zoom();
      overlayRef.current.style.transform =
        `translate(${pan.x}px, ${pan.y}px) scale(${zoom})`;
    };
    
    // After layout, set up initial view then read model positions
    cy.one('layoutstop', () => {
      // Set view first so syncOverlay captures the final pan/zoom
      cy.zoom(1.2);
      cy.pan({ x: 50, y: 50 });
      const first = cy.nodes().first();
      if (first.length) cy.center(first);

      const pos = new Map<string, { x: number; y: number }>();
      cy.nodes().forEach(node => {
        const p = node.position();
        pos.set(node.id(), { x: p.x, y: p.y });
      });
      setPositions(pos);

      // Sync after a microtask so the re-render from setPositions has completed
      queueMicrotask(syncOverlay);
    });
    
    // Keep overlay in sync on every pan/zoom change
    cy.on('viewport', syncOverlay);
    
    cyRef.current = cy;
    cytoscapeInstance.value = cy;
    
    return () => {
      if (cyRef.current) {
        cyRef.current.destroy();
        cyRef.current = null;
        cytoscapeInstance.value = null;
      }
    };
  }, [logEntries, file]);

  if (!file) {
    return (
      <div class="flow-graph empty">
        <div class="placeholder-message">
          <span class="placeholder-icon">🔀</span>
          <p>Select a log file to view flow graph</p>
        </div>
      </div>
    );
  }

  if (logEntries.length === 0) {
    return (
      <div class="flow-graph empty">
        <div class="placeholder-message">
          <span class="placeholder-icon">📊</span>
          <p>No entries to visualize</p>
        </div>
      </div>
    );
  }

  const handleNodeClick = (entry: LogEntry) => {
    selectEntry(entry);
    setTab('logs');
  };

  return (
    <div class="flow-graph">
      <div class="flow-header">
        <span>Flow Graph</span>
        {logEntries.length > 200 && (
          <span class="flow-warning">Warning: Showing first 200 of {logEntries.length} entries</span>
        )}
        <div class="flow-legend">
          <span class="legend-item"><span class="dot event"></span> Event</span>
          <span class="legend-item"><span class="dot span-enter"></span> Span Enter</span>
          <span class="legend-item"><span class="dot span-exit"></span> Span Exit</span>
        </div>
      </div>
      <div class="flow-viewport">
        {/* Cytoscape container — edges only (nodes are invisible) */}
        <div class="flow-cy-container" ref={containerRef}></div>
        {/* HTML node overlays — positioned using Cytoscape layout coordinates */}
        <div class="flow-nodes-overlay" ref={overlayRef}>
          {limitedNodes.map(node => {
            const pos = positions.get(node.id);
            if (!pos) return null;
            const level = node.entry.level.toLowerCase();
            const typeClass = node.type === 'span_enter' ? 'span_enter'
                            : node.type === 'span_exit'  ? 'span_exit'
                            : 'event';
            const typeLabel = node.type === 'span_enter' ? 'ENTER'
                            : node.type === 'span_exit'  ? 'EXIT'
                            : 'EVENT';
            const msg = node.entry.message.length > 80
              ? node.entry.message.slice(0, 80) + '…'
              : node.entry.message;
            const time = node.entry.timestamp?.split('T')[1]?.slice(0, 12) || '';

            return (
              <div
                key={node.id}
                class={`flow-node level-${level} type-${typeClass}`}
                style={{
                  left: `${pos.x - NODE_WIDTH / 2}px`,
                  top:  `${pos.y - NODE_HEIGHT / 2}px`,
                  width:  `${NODE_WIDTH}px`,
                  height: `${NODE_HEIGHT}px`,
                }}
                onClick={() => handleNodeClick(node.entry)}
              >
                <div class="flow-node-row1">
                  <span class={`level-badge ${level}`}>{node.entry.level}</span>
                  <span class={`type-badge ${typeClass}`}>{typeLabel}</span>
                  {node.entry.span_name && (
                    <span class="span-name">{node.entry.span_name}</span>
                  )}
                  <span class="entry-meta">#{node.entry.line_number}</span>
                </div>
                <div class="flow-node-message">{msg}</div>
                {time && <div class="flow-node-time">{time}</div>}
              </div>
            );
          })}
        </div>
      </div>
    </div>
  );
}
