import { activeDoc, activeTabId, openTabs, openCrateDoc, openCategoryPage } from '../store';
import { ChevronRightIcon, DocumentIcon, CrateIcon, FolderIcon, HomeIcon } from '@context-engine/viewer-api-frontend';

interface BreadcrumbPart {
  label: string;
  type: 'home' | 'root' | 'category' | 'crate' | 'module' | 'doc';
  onClick?: () => void;
}

function parseBreadcrumbs(filename: string | null, title: string, docType: string): BreadcrumbPart[] {
  if (!filename) return [];
  
  const parts: BreadcrumbPart[] = [];
  
  // Handle category pages themselves
  if (filename === 'page:home') {
    parts.push({ label: 'Home', type: 'home' });
    return parts;
  }
  
  if (filename === 'page:agent-docs') {
    parts.push({
      label: 'Home',
      type: 'home',
      onClick: () => openCategoryPage('page:home'),
    });
    parts.push({ label: 'Agent Docs', type: 'root' });
    return parts;
  }
  
  if (filename === 'page:crate-docs') {
    parts.push({
      label: 'Home',
      type: 'home',
      onClick: () => openCategoryPage('page:home'),
    });
    parts.push({ label: 'Crate Docs', type: 'root' });
    return parts;
  }
  
  // Always start with Home
  parts.push({
    label: 'Home',
    type: 'home',
    onClick: () => openCategoryPage('page:home'),
  });
  
  if (filename.startsWith('crate:')) {
    // Crate documentation: crate:crateName or crate:crateName:module/path
    parts.push({
      label: 'Crate Docs',
      type: 'root',
      onClick: () => openCategoryPage('page:crate-docs'),
    });
    
    const colonParts = filename.split(':');
    const crateName = colonParts[1];
    const modulePath = colonParts.slice(2).join(':');
    
    // Crate name - clicking navigates to crate root (index.yaml)
    parts.push({
      label: crateName,
      type: 'crate',
      onClick: modulePath ? () => openCrateDoc(crateName) : undefined,
    });
    
    if (modulePath) {
      // Split module path by / and add each as a breadcrumb
      const modules = modulePath.split('/');
      modules.forEach((mod, idx) => {
        const isLast = idx === modules.length - 1;
        // Build partial path up to this module
        const partialPath = modules.slice(0, idx + 1).join('/');
        
        parts.push({
          label: mod,
          type: isLast ? 'doc' : 'module',
          // Only add onClick for non-last items (intermediate modules)
          onClick: isLast ? undefined : () => openCrateDoc(crateName, partialPath),
        });
      });
    }
  } else {
    // Agent documentation
    parts.push({
      label: 'Agent Docs',
      type: 'root',
      onClick: () => openCategoryPage('page:agent-docs'),
    });
    
    // Add category from doc_type
    if (docType && docType !== 'crate') {
      const categoryLabel = formatCategoryName(docType);
      parts.push({ label: categoryLabel, type: 'category' });
    }
    
    // Add document title (current, not clickable)
    parts.push({ label: title, type: 'doc' });
  }
  
  return parts;
}

function formatCategoryName(name: string): string {
  return name
    .split('-')
    .map(word => word.charAt(0).toUpperCase() + word.slice(1))
    .join(' ');
}

function getIcon(type: BreadcrumbPart['type']) {
  switch (type) {
    case 'home':
      return <HomeIcon size={12} />;
    case 'root':
      return <FolderIcon size={12} />;
    case 'crate':
      return <CrateIcon size={12} />;
    case 'category':
    case 'module':
      return <FolderIcon size={12} />;
    case 'doc':
      return <DocumentIcon size={12} />;
    default:
      return null;
  }
}

export function Breadcrumbs() {
  const doc = activeDoc.value;
  const activeId = activeTabId.value;
  const activeTab = openTabs.value.find(t => t.filename === activeId);
  
  if (!activeId || !activeTab) {
    return (
      <div class="breadcrumbs">
        <span class="breadcrumb-empty">Select a document to view</span>
      </div>
    );
  }
  
  const parts = parseBreadcrumbs(
    activeId,
    activeTab.title,
    doc?.doc_type ?? ''
  );
  
  return (
    <div class="breadcrumbs">
      <nav class="breadcrumb-nav" aria-label="Breadcrumb">
        {parts.map((part, idx) => (
          <span key={idx} class="breadcrumb-item">
            {idx > 0 && (
              <span class="breadcrumb-separator">
                <ChevronRightIcon size={12} />
              </span>
            )}
            {part.onClick ? (
              <button
                type="button"
                class="breadcrumb-label clickable"
                title={part.label}
                onClick={part.onClick}
              >
                <span class="breadcrumb-icon">{getIcon(part.type)}</span>
                {part.label}
              </button>
            ) : (
              <span 
                class={`breadcrumb-label current`}
                title={part.label}
              >
                <span class="breadcrumb-icon">{getIcon(part.type)}</span>
                {part.label}
              </span>
            )}
          </span>
        ))}
      </nav>
      {activeTab.isLoading && (
        <span class="breadcrumb-loading">Loading...</span>
      )}
    </div>
  );
}
