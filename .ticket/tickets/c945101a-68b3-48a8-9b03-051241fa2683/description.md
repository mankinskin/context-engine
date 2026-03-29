# Integration: Context Hypergraph Visualization in SVO World

## Problem

The context-engine hypergraph (atoms, sequences, search results) must be visualized as a 3D graph in the SVO ray-marched world. Graph nodes are voxel clusters in the SVO, and edges are voxel lines connecting them.

## Architecture: Hypergraph as SVO Voxel Structures

### Node Rendering

Each hypergraph node is a small cluster of colored voxels in the SVO:

```rust
#[derive(Component)]
pub struct HypergraphNode {
    pub atom_id: String,
    pub node_type: NodeType, // atom, sequence, group
    pub label: String,
}

fn spawn_graph_node(
    voxel_world: &mut VoxelWorld,
    position: IVec3,
    node_type: NodeType,
    palette: &ThemePalette,
) {
    let material = match node_type {
        NodeType::Atom => VoxelMaterial::PalettePrimary,
        NodeType::Sequence => VoxelMaterial::PaletteSecondary,
        NodeType::Group => VoxelMaterial::PaletteHighlight,
    };
    // 3x3x3 cube of voxels for visibility
    voxel_world.apply_sdf_brush(position.as_vec3(), 1.5, material);
}
```

### Edge Rendering

Edges between graph nodes are drawn as voxel lines:
```rust
fn draw_graph_edge(voxel_world: &mut VoxelWorld, from: IVec3, to: IVec3, palette: &ThemePalette) {
    for pos in voxel_line(from, to) {
        voxel_world.set_voxel(pos, VoxelMaterial::Custom(palette.voxel_secondary));
    }
}
```

### Layout

Graph layout computed on CPU using force-directed or hierarchical algorithm, then positions mapped to SVO coordinates.

### API Integration

Data from context-api:
- Workspace operations for graph traversal
- Search results visualized as highlighted subgraphs
- Insert operations reflected in real-time

### Interaction

- Hover on a graph node voxel cluster → tooltip WorldPanel appears (T10)
- Click on node → detail panel with atom content
- Search highlighting: matching nodes glow (bright `color_data`)

## Dependencies
- T7 (physics): VoxelWorld API for creating/destroying voxel clusters
- T10 (3D UI): WorldPanel tooltips for node details
- T5 (theme): Node colors from palette
- T6 (3D scene): Voxels rendered by ray marching

## Acceptance Criteria
1. Hypergraph nodes appear as colored voxel clusters in the SVO
2. Edges are visible as voxel lines between nodes
3. Different node types have distinct colors from the palette
4. Search results highlight matching nodes
5. Graph updates (insert, delete) reflect in the SVO within one frame
6. Node detail shows on click (WorldPanel tooltip)
