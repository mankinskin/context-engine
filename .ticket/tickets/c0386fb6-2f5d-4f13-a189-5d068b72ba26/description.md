# Impl: Documentation editor — markdown editing, doc-viewer API integration, glass panels via Bevy

## Problem

The context-editor needs a documentation editing panel integrated into the 3D world that connects to doc-viewer endpoints for reading/searching documentation and provides inline markdown editing. Panels are rendered as glass surfaces by the Bevy glass render pass.

## Scope

### Backend Integration (`src/editor/docs/api.rs`)
- HTTP client for doc-viewer API endpoints:
  - `GET /api/docs` — list documents
  - `GET /api/docs/{path}` — read document content
  - `GET /api/search?q=...` — full-text search across docs
  - `PUT /api/docs/{path}` — update document (if supported)

### Document Browser (`src/editor/docs/browser.rs`)
- Tree view of documentation files (folders + files)
- Search bar for full-text search across all docs
- Click document → opens content in editor panel

### Markdown Viewer/Editor (`src/editor/docs/markdown.rs`)
- Read mode: render markdown to DOM elements (pulldown-cmark)
- Edit mode: textarea with monospace font for raw markdown editing
- Toggle between read/edit modes
- Syntax highlighting for code blocks (using theme palette from Bevy `ThemePalette` resource)
- Uses `set_text_content` for rendered text (XSS safety)

### Glass Panel Integration
- Document browser as sidebar glass panel (positioned via Taffy-Bevy bridge)
- Markdown editor as main content glass panel
- Both panels registered in `LayoutRects` Bevy resource → glass shader renders refraction

## Integration Points
- **doc-viewer API**: document CRUD + search
- **Bevy ECS**: glass panels as `LayoutRects` entries, theme as `ThemePalette` resource
- **T3 (glass)**: all panels rendered by glass render node
- **T9 (Taffy-Bevy bridge)**: panel layout computation → Bevy resource → GPU
- **T5 (themes)**: code block and text colors from `ThemePalette` resource

## Files to Create
| File | Purpose |
|------|---------|
| `src/editor/docs/mod.rs` | Documentation editor module |
| `src/editor/docs/api.rs` | doc-viewer HTTP client |
| `src/editor/docs/browser.rs` | Document tree browser |
| `src/editor/docs/markdown.rs` | Markdown viewer/editor |

## Acceptance Criteria
1. Document tree loads from doc-viewer API
2. Click document shows markdown content rendered to DOM
3. Full-text search returns results with highlighted matches
4. Edit mode provides raw markdown textarea
5. Code blocks display with syntax-aware coloring from `ThemePalette`
6. All user-supplied text rendered via `set_text_content` (no innerHTML)
