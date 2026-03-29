# Impl: Code file viewer — syntax highlighting, glass panels via Bevy render pass

## Problem

The context-editor needs a code viewing panel for displaying source files with syntax highlighting, integrated into the 3D glass panel system (rendered by Bevy's glass render node), useful for navigating from log entries or graph nodes to source code.

## Scope

### Code Display (`src/editor/code/viewer.rs`)
- Display source file content with line numbers
- Monospace font rendering via DOM (browser text rendering)
- Scrollable content within glass panel

### Syntax Highlighting (`src/editor/code/highlight.rs`)
- Token-based highlighting for common languages (Rust, TypeScript, TOML, Markdown)
- Colors from `ThemePalette` Bevy resource (keyword, string, comment, number, type, function)
- Implemented as CSS classes on `<span>` elements within `<pre>` block
- Lightweight: regex-based tokenizer (not full parser)

### File Navigation (`src/editor/code/nav.rs`)
- Open file by path (from log entry reference or context graph link)
- Jump to specific line number
- Highlight active line

### Glass Panel Integration
- Code viewer rendered in glass panel (positioned via Taffy-Bevy bridge → `LayoutRects` resource)
- Tab system for multiple open files
- Close tab / switch tabs
- Glass refraction produced by Bevy glass render node against 3D scene background

## Integration Points
- **viewer-api**: source file resolution (filesystem-backed, path validation)
- **Bevy ECS**: glass panels via `LayoutRects` resource, theme via `ThemePalette` resource
- **T3 (glass)**: code panel rendered by glass render node
- **T5 (themes)**: syntax colors from `ThemePalette`
- **T9 (Taffy-Bevy bridge)**: panel layout → Bevy resource → GPU

## Files to Create
| File | Purpose |
|------|---------|
| `src/editor/code/mod.rs` | Code editor module |
| `src/editor/code/viewer.rs` | Code file display |
| `src/editor/code/highlight.rs` | Syntax highlighting |
| `src/editor/code/nav.rs` | File navigation + line jump |

## Acceptance Criteria
1. Source files display with line numbers and monospace font
2. Syntax highlighting colors Rust keywords, strings, comments distinctly
3. Line jump scrolls to and highlights the target line
4. Multiple files open as tabs in the same glass panel
5. All file content rendered via `set_text_content` (XSS prevention)
6. Theme switch updates syntax colors immediately (reads `ThemePalette` resource)
