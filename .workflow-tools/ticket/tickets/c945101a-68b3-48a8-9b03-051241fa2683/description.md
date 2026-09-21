# Context Graph 3D: Hypergraph Nodes as Voxel Clusters Generating Splats

## Problem

The context-engine hypergraph is visualized in 3D. Each graph node becomes a voxel cluster in the SVO, and the splat generation pipeline produces splats from these voxels — nodes appear as soft, volumetric shapes. Edges are voxel lines (like ticket edges in T12) that also generate splats.

## Architecture

### Node → Voxel Cluster

Each hypergraph node is represented as a small voxel cluster (sphere, cube, or custom shape based on node type) placed at a force-directed layout position:

```rust
#[derive(Component)]
pub struct GraphNode3D {
    pub node_id: NodeId,
    pub node_type: NodeType,
    pub cluster_shape: ClusterShape,
}

pub enum ClusterShape {
    Sphere { radius_voxels: u32 },
    Cube { half_extent_voxels: u32 },
    Custom(Vec<IVec3>),  // explicit voxel offsets
}

fn spawn_graph_node(
    node: &HyperNode,
    pos: Vec3,
    svo: &mut VoxelWorld,
) {
    let shape = shape_for_type(node.node_type);
    let color = color_for_type(node.node_type);
    let material = MaterialDef {
        base_color: color,
        roughness: 0.3,   // slightly glossy — PBR gives subtle view-dependent highlights
        metallic: 0.0,
    };

    // Write voxels for this node's cluster
    for offset in shape.voxel_offsets() {
        let voxel_pos = world_to_svo(pos) + offset;
        svo.set_voxel(voxel_pos, color, material);
    }
}
```

### Voxel Splat Visual Quality

Since graph nodes are voxel clusters, the splat generation pipeline (T6) converts them to splats automatically:
- Half-extent from voxel size → soft spherical blobs
- PBR material parameters from MaterialDef → subtle view-dependent shading
- LOD reduces splat count for distant nodes

The result: graph nodes appear as soft, volumetric, slightly glossy shapes floating in 3D space.

### Edge Visualization

```rust
fn draw_graph_edges(
    edges: &[HyperEdge],
    nodes: &Query<(&GraphNode3D, &Transform)>,
    svo: &mut VoxelWorld,
) {
    for edge in edges {
        let from = node_position(nodes, edge.from);
        let to = node_position(nodes, edge.to);
        let material = edge_material(edge.edge_type);
        svo.draw_voxel_line(from, to, material);
    }
}
```

Edge voxels generate splats too — connections appear as soft glowing lines. Edge type determines PBR material properties:
- **Sequence edges**: diffuse, muted color
- **Dependency edges**: metallic PBR (specular highlights when viewed at glancing angles)
- **Hyperedges**: bright, high-opacity splats

### Interactive Labels

Each node can have a small glass panel label (T10 WorldPanel) floating above it, showing node ID or summary text. These labels are billboarded (face camera).

### Camera Navigation

- Click node → select, show details in side panel
- Double-click → zoom camera to node (smooth lerp)
- Scroll → zoom in/out on graph
- Drag → orbit around selected node or graph center

### Graph Updates

When the context-engine hypergraph changes:
1. New voxels written to SVO for added/modified nodes
2. Removed node voxels cleared from SVO
3. SVO marked dirty → double-buffered upload (T7) → splats regenerated next frame
4. Visual update is automatic through the pipeline

## Dependencies
- T7 (physics+world): VoxelWorld / SVO storage for node clusters and edge lines
- T6 (3D scene): splat generation from node/edge voxels
- T5 (theme): MaterialDef → PBR material parameters for node/edge materials
- T10 (3D UI): Glass panel labels above nodes
- context-api: Hypergraph data source

## Acceptance Criteria
1. Each hypergraph node rendered as a voxel cluster → splats
2. Node type determines shape, color, and PBR material
3. Edges as voxel lines → splats with type-dependent SH
4. Force-directed 3D layout positions nodes
5. Click/double-click interaction for selection and zoom
6. Node labels as billboarded glass panels
7. Graph mutations update SVO → splats regenerated via double buffer
8. LOD reduces distant node splat count
