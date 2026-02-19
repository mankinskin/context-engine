import { useState, useEffect } from 'preact/hooks';
import { 
  filteredEntries, 
  showRaw, 
  searchQuery, 
  selectedEntry, 
  selectEntry,
  isLoading,
  currentFile 
} from '../../store';
import { LogEntryRow } from './LogEntryRow';
import { ChevronDown, ChevronRight } from '../Icons';

// Minimal folder icon
function FolderIcon({ size = 32, color = 'currentColor' }: { size?: number; color?: string }) {
  return (
    <svg width={size} height={size} viewBox="0 0 32 32" fill="none" style={{ opacity: 0.5 }}>
      <path d="M4 8C4 6.9 4.9 6 6 6H12L14 9H26C27.1 9 28 9.9 28 11V24C28 25.1 27.1 26 26 26H6C4.9 26 4 25.1 4 24V8Z" stroke={color} stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
    </svg>
  );
}

export function LogViewer() {
  const [expandAll, setExpandAll] = useState<boolean>(false);
  const [expandedEntries, setExpandedEntries] = useState<Set<number>>(new Set());
  const [headerScrollLeft, setHeaderScrollLeft] = useState(0);
  const [maxHeaderWidth, setMaxHeaderWidth] = useState(0);
  const [headerColWidth, _setHeaderColWidth] = useState(500);
  
  // Refs for header cells to sync scroll
  const headerCellRefs: { current: HTMLDivElement[] } = { current: [] };
  const scrollbarRef = { current: null as HTMLDivElement | null };
  
  // Calculate max header content width
  useEffect(() => {
    let maxWidth = 0;
    headerCellRefs.current.forEach(cell => {
      if (cell) {
        maxWidth = Math.max(maxWidth, cell.scrollWidth);
      }
    });
    setMaxHeaderWidth(maxWidth);
  }, [filteredEntries.value, expandedEntries]);
  
  // Sync all header cells when scrollbar moves
  const handleScrollbarChange = (scrollLeft: number) => {
    setHeaderScrollLeft(scrollLeft);
  };
  
  // Handle wheel scroll on header column
  const handleHeaderWheel = (e: WheelEvent) => {
    // Use deltaX for trackpad horizontal scroll, or deltaY with shift for mouse wheel
    const delta = e.deltaX !== 0 ? e.deltaX : (e.shiftKey ? e.deltaY : 0);
    if (delta === 0) return;
    
    e.preventDefault();
    setHeaderScrollLeft(prev => {
      const maxScroll = Math.max(0, maxHeaderWidth - headerColWidth);
      return Math.max(0, Math.min(maxScroll, prev + delta));
    });
    
    // Sync the scrollbar element if it exists
    if (scrollbarRef.current) {
      const maxScroll = Math.max(0, maxHeaderWidth - headerColWidth);
      const newScroll = Math.max(0, Math.min(maxScroll, headerScrollLeft + delta));
      scrollbarRef.current.scrollLeft = newScroll;
    }
  };
  
  const toggleExpanded = (lineNumber: number) => {
    setExpandedEntries(prev => {
      const next = new Set(prev);
      if (next.has(lineNumber)) {
        next.delete(lineNumber);
      } else {
        next.add(lineNumber);
      }
      return next;
    });
  };
  
  const registerHeaderCell = (index: number) => (el: HTMLDivElement | null) => {
    if (el) {
      headerCellRefs.current[index] = el;
    }
  };

  if (!currentFile.value) {
    return (
      <div class="log-viewer empty">
        <div class="placeholder-message">
          <FolderIcon size={32} />
          <p>Select a log file to view</p>
        </div>
      </div>
    );
  }

  if (isLoading.value) {
    return (
      <div class="log-viewer loading">
        <div class="spinner"></div>
        <p>Loading...</p>
      </div>
    );
  }

  if (filteredEntries.value.length === 0) {
    return (
      <div class="log-viewer empty">
        <div class="placeholder-message">
          <span class="placeholder-icon">🔍</span>
          <p>No entries match the current filters</p>
        </div>
      </div>
    );
  }

  const scrollRange = Math.max(0, maxHeaderWidth - headerColWidth);

  return (
    <div class="log-viewer">
      <div class="log-viewer-toolbar">
        <span class="toolbar-count">{filteredEntries.value.length} entries</span>
        <button class="expand-toggle" onClick={() => setExpandAll(!expandAll)} title={expandAll ? "Collapse all" : "Expand all"}>
          {expandAll ? <ChevronDown size={12} /> : <ChevronRight size={12} />}
        </button>
      </div>
      {/* Header column scrollbar */}
      {scrollRange > 0 && (
        <div class="header-scrollbar-container" style={{ width: `${headerColWidth}px` }}>
          <div 
            class="header-scrollbar"
            ref={(el) => { scrollbarRef.current = el; }}
            onScroll={(e) => handleScrollbarChange((e.target as HTMLDivElement).scrollLeft)}
          >
            <div class="header-scrollbar-content" style={{ width: `${maxHeaderWidth}px` }} />
          </div>
        </div>
      )}
      <div class="log-entries">
        {filteredEntries.value.map((entry, index) => (
          <LogEntryRow
            key={entry.line_number}
            entry={entry}
            showRaw={showRaw.value}
            searchQuery={searchQuery.value}
            isSelected={selectedEntry.value?.line_number === entry.line_number}
            onSelect={() => selectEntry(entry)}
            expandAll={expandAll}
            isExpanded={expandedEntries.has(entry.line_number)}
            onToggleExpand={() => toggleExpanded(entry.line_number)}
            headerCellRef={registerHeaderCell(index)}
            headerScrollLeft={headerScrollLeft}
            headerColWidth={headerColWidth}
            onHeaderWheel={handleHeaderWheel}
          />
        ))}
      </div>
    </div>
  );
}