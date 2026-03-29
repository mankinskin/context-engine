# Integration: Ticket Editor in SVO Ray-Marched World

## Problem

The ticket-api CRUD and graph operations must be accessible through a 3D interface in the context-editor. Ticket nodes are displayed as **world-space glass panels** (analytical SDFs in the ray marching loop) connected by visible edges in the voxel world. Users interact with tickets through the glass panel UI.

## Architecture: Tickets as World-Space Glass SDFs

### Ticket Node Display

Each ticket is a `WorldPanel` entity (from T10) positioned in 3D space:

```rust
#[derive(Component)]
pub struct TicketNode {
    pub ticket_id: String,
    pub title: String,
    pub state: TicketState,
    pub priority: Priority,
}

fn spawn_ticket_node(
    commands: &mut Commands,
    ticket: &Ticket,
    position: Vec3,
    palette: &ThemePalette,
) {
    commands.spawn(WorldPanelBundle {
        panel: WorldPanel {
            size: Vec2::new(3.0, 2.0),
            corner_radius: 0.2,
            ior: 1.3,
            tint: state_color(ticket.state, palette),
            blur_roughness: 0.3,
            text_content: ticket.title.clone(),
            billboard: true,
        },
        glass: GlassPanel { .. },
        transform: Transform::from_translation(position),
        ..default()
    })
    .insert(TicketNode {
        ticket_id: ticket.id.clone(),
        title: ticket.title.clone(),
        state: ticket.state,
        priority: ticket.priority,
    });
}
```

### Ticket Graph Edges

Dependency edges between tickets are rendered as voxel lines in the SVO:

```rust
fn draw_ticket_edge(
    voxel_world: &mut VoxelWorld,
    from: Vec3,
    to: Vec3,
    palette: &ThemePalette,
) {
    // Bresenham 3D line in voxel space
    for pos in voxel_line(from, to) {
        voxel_world.set_voxel(pos, VoxelMaterial::Custom(palette.voxel_secondary));
    }
}
```

### API Integration

Ticket data is fetched from ticket-api and synchronized into ECS:

```rust
fn ticket_sync_system(
    mut commands: Commands,
    ticket_api: Res<TicketApiClient>,
    existing: Query<(Entity, &TicketNode)>,
) {
    // Fetch ticket list from API
    // Diff against existing entities
    // Spawn new, despawn removed, update changed
}
```

### Interaction

Clicking on a ticket glass panel opens a detail editor (Dioxus component):
- View/edit title, description, state, priority
- Transition state (with valid state machine transitions)
- View/edit dependency edges

## Dependencies
- T10 (3D UI): WorldPanel system for glass SDF display
- T7 (physics): VoxelWorld for drawing edge lines
- T5 (theme): State-dependent colors from palette
- T9 (bridge): Hit testing for panel interaction

## Acceptance Criteria
1. Tickets from ticket-api appear as glass panels in the 3D world
2. Ticket state is reflected in panel tint color
3. Dependency edges are visible as voxel lines between panels
4. Clicking a ticket panel opens a detail editor
5. State transitions through the editor respect the state machine
6. Adding/removing tickets updates the 3D scene within one frame
