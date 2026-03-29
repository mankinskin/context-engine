# Integration: Document Editor in SVO Ray-Marched World

## Problem

Documentation from the doc-viewer system must be browsable and editable through glass panel interfaces in the 3D SVO world. Documents are displayed as **frosted glass panels** (high `blur_roughness` SDFs) with text content rendered on the surface.

## Architecture: Docs as Frosted Glass SDFs

### Document Panel Display

Each document page is a `WorldPanel` with frosted glass appearance:

```rust
#[derive(Component)]
pub struct DocPanel {
    pub doc_id: String,
    pub doc_type: DocType,  // agent_doc, crate_doc
    pub content: String,
    pub scroll_offset: f32,
}

fn spawn_doc_panel(
    commands: &mut Commands,
    doc: &Document,
    position: Vec3,
) {
    commands.spawn(WorldPanelBundle {
        panel: WorldPanel {
            size: Vec2::new(4.0, 5.0), // larger for reading
            corner_radius: 0.15,
            ior: 1.2,
            tint: Color::rgba(0.9, 0.95, 1.0, 0.15), // subtle blue
            blur_roughness: 0.7, // frosted for readability
            text_content: doc.content.clone(),
            billboard: false, // can be oriented in world
        },
        ..default()
    })
    .insert(DocPanel {
        doc_id: doc.id.clone(),
        doc_type: doc.doc_type,
        content: doc.content.clone(),
        scroll_offset: 0.0,
    });
}
```

### API Integration

Documents fetched from doc-viewer MCP tools or viewer-api:
- `list` for browsing available docs
- `search` for finding docs by content
- `create`/`update` for editing

### Interaction

- Scroll: mouse wheel when hovering over doc panel (shifts text UV)
- Edit: click to focus, then stream keystrokes to Dioxus editor overlay
- Search: Dioxus search bar filters and highlights matching doc panels

## Dependencies
- T10 (3D UI): WorldPanel system for glass SDF display
- T5 (theme): Document panel tint from palette
- T9 (bridge): Hit testing and Dioxus event routing

## Acceptance Criteria
1. Documents from doc-viewer are displayed as frosted glass panels
2. Text is readable on the frosted glass surface
3. Scrolling works on doc panels (mouse wheel)
4. Document search highlights matching panels
5. Document edits persist through the API
6. Panels integrate visually with the SVO ray-marched scene
