import { useEffect, useRef } from 'preact/hooks';
import cytoscape from 'cytoscape';
import { entries, currentFile, selectEntry, setTab } from '../../store';
import type { LogEntry } from '../../types';

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
        edges.push({ source: spanStack[spanIdx].nodeId, target: nodeId, type: 'span' });
        spanStack.splice(spanIdx, 1);
      }
    }
    
    prevNodeId = nodeId;
  }
  
  return { nodes, edges };
}

function getLevelColor(level: string): string {
  switch (level.toUpperCase()) {
    case 'ERROR': return '#e74c3c';
    case 'WARN': return '#f39c12';
    case 'INFO': return '#3498db';
    case 'DEBUG': return '#9b59b6';
    case 'TRACE': return '#95a5a6';
    default: return '#7f8c8d';
  }
}

export function FlowGraph() {
  const containerRef = useRef<HTMLDivElement>(null);
  const cyRef = useRef<cytoscape.Core | null>(null);
  
  const logEntries = entries.value;
  const file = currentFile.value;

  useEffect(() => {
    if (!containerRef.current || logEntries.length === 0) return;
    
    const { nodes, edges } = buildGraph(logEntries);
    
    // Limit nodes for performance
    const maxNodes = 200;
    const limitedNodes = nodes.slice(0, maxNodes);
    const limitedNodeIds = new Set(limitedNodes.map(n => n.id));
    const limitedEdges = edges.filter(e => 
      limitedNodeIds.has(e.source) && limitedNodeIds.has(e.target)
    );
    
    if (cyRef.current) {
      cyRef.current.destroy();
    }
    
    const cy = cytoscape({
      container: containerRef.current,
      elements: [
        ...limitedNodes.map(node => ({
          data: { 
            id: node.id, 
            label: node.entry.span_name || node.entry.message.slice(0, 30),
            entry: node.entry,
            type: node.type,
            level: node.entry.level
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
            'background-color': (ele: any) => getLevelColor(ele.data('level')),
            'label': 'data(label)',
            'font-size': '10px',
            'text-wrap': 'ellipsis',
            'text-max-width': '100px',
            'color': '#fff',
            'text-outline-color': '#000',
            'text-outline-width': 1,
            'width': 40,
            'height': 40,
            'shape': 'ellipse'
          }
        },
        {
          selector: 'node[type="span_enter"]',
          style: {
            'border-width': 2,
            'border-color': '#fff'
          }
        },
        {
          selector: 'node[type="span_exit"]',
          style: {
            'border-width': 2,
            'border-style': 'dashed',
            'border-color': '#fff'
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
        },
        {
          selector: ':selected',
          style: {
            'border-width': 4,
            'border-color': '#f1c40f'
          }
        }
      ],
      layout: {
        name: 'breadthfirst',
        directed: true,
        spacingFactor: 1.5,
        avoidOverlap: true
      }
    });
    
    // Handle node clicks
    cy.on('tap', 'node', (evt) => {
      const entry = evt.target.data('entry') as LogEntry;
      if (entry) {
        selectEntry(entry);
        setTab('logs');
      }
    });
    
    cyRef.current = cy;
    
    return () => {
      if (cyRef.current) {
        cyRef.current.destroy();
        cyRef.current = null;
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

  return (
    <div class="flow-graph">
      <div class="flow-header">
        <span>Flow Graph</span>
        {logEntries.length > 200 && (
          <span class="flow-warning">⚠️ Showing first 200 of {logEntries.length} entries</span>
        )}
        <div class="flow-legend">
          <span class="legend-item"><span class="dot event"></span> Event</span>
          <span class="legend-item"><span class="dot span-enter"></span> Span Enter</span>
          <span class="legend-item"><span class="dot span-exit"></span> Span Exit</span>
        </div>
      </div>
      <div class="flow-container" ref={containerRef}></div>
    </div>
  );
}
