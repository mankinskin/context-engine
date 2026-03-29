# Integration: Code Viewer in SVO Ray-Marched World

## Problem

Source code must be viewable through glass panel interfaces in the 3D SVO world. Code files are displayed as **glass panels** with syntax-highlighted text content, positioned near related hypergraph nodes or ticket panels.

## Architecture: Code as Glass SDFs

### Code Panel Display

Each code file/snippet is a `WorldPanel` with clear glass and monospaced text:

```rust
#[derive(Component)]
pub struct CodePanel {
    pub file_path: String,
    pub language: String,
    pub content: String,
    pub line_offset: usize,
    pub scroll_offset: f32,
}

fn spawn_code_panel(
    commands: &mut Commands,
    file: &CodeFile,
    position: Vec3,
) {
    commands.spawn(WorldPanelBundle {
        panel: WorldPanel {
            size: Vec2::new(5.0, 6.0), // tall for code
            corner_radius: 0.1,
            ior: 1.15,
            tint: Color::rgba(0.05, 0.05, 0.1, 0.1), // dark tint for code
            blur_roughness: 0.05, // nearly clear
            text_content: file.content.clone(),
            billboard: false,
        },
        ..default()
    })
    .insert(CodePanel {
        file_path: file.path.clone(),
        language: file.language.clone(),
        content: file.content.clone(),
        line_offset: 0,
        scroll_offset: 0.0,
    });
}
```

### Features
- Syntax highlighting via pre-computed ANSI colors mapped to palette
- Line numbers rendered on left margin
- Scrollable via mouse wheel (UV offset on text texture)
- Linked to hypergraph nodes: code panel anchored near its related graph node

### Interaction
- Scroll through code with mouse wheel
- Click to position cursor (future: editing)
- Search highlights matching lines

## Dependencies
- T10 (3D UI): WorldPanel system for glass SDF display
- T5 (theme): Code colors from palette
- T14 (context graph): Spatial proximity to related graph nodes
- T9 (bridge): Hit testing for scroll/click events

## Acceptance Criteria
1. Code files display as glass panels with readable monospaced text
2. Syntax highlighting is visible on the panel surface
3. Scrolling works for long files
4. Code panels positioned near related graph nodes
5. Panels integrate visually with the SVO ray-marched scene
6. Line numbers visible on left margin
