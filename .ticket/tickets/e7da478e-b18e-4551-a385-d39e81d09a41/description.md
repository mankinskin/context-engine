# Epic: context-editor — Unified 3D World Editor for context-engine

## Vision

A single-binary, GPU-accelerated tool that merges the log-viewer, doc-viewer,
and ticket-viewer into an immersive 3D workspace rendered entirely in the
browser via WebGPU/WASM.

## Technology Stack

| Layer | Technology | Role |
|-------|-----------|------|
| **UI logic** | Dioxus (web) | Component tree, events, accessibility |
| **Layout** | Taffy | Pixel-accurate flexbox for UI panel sizing |
| **ECS runtime** | Bevy | Entity-Component-System, render graph orchestration |
| **Physics** | bevy_rapier3d | Collision and character movement on SVO-derived geometry |
| **World structure** | Sparse Voxel Octree (SVO) | Compact octree in GPU storage buffer — structure & physics |
| **Visual rendering** | Procedural Gaussian Splatting (3DGS) | Photorealistic visuals generated from SVO on GPU |
| **Projection** | EWA Splatting | Anti-aliased 3D→2D Gaussian projection via covariance matrices |
| **Sorting** | GPU Radix Sort | Parallel depth+tile sorting of millions of splats |
| **Compositing** | Tiled Forward+ Renderer | 16×16 tile-based rasterization with front-to-back alpha blending |
| **Glass/UI** | Analytical SDF + Mipmap Blur | Snell's refraction, chromatic aberration, pseudo-caustics, frosted glass |
| **CPU↔GPU sync** | Double Buffering (Ping-Pong) | Lock-free CPU writes while GPU reads previous frame |
| **Build** | Trunk | Compile to WASM, bundle assets for browser |
| **API** | context-api, ticket-api | Data access for workspaces, search, tickets |

## Rendering Pipeline (per frame)

```
1. CPU (Bevy ECS / WASM)
   ├─ Drain Dioxus events, recompute Taffy layout
   ├─ Update SVO dirty regions → write to BACK buffer
   └─ Swap double buffers (Front ↔ Back)

2. GPU Compute Phase
   ├─ Particle simulation (SVO collision via query_svo_distance)
   ├─ Gaussian Generator: for each occupied voxel → emit 1-N Gaussians
   │   ├─ LOD: coarse voxel → 1 large fuzzy Gaussian
   │   └─ Leaf voxel → many small sharp Gaussians
   ├─ EWA Projection: Σ' = J·W·Σ·Wᵀ·Jᵀ (3D covariance → 2D ellipse)
   ├─ Key construction: tile_id (20 bit) | depth (12 bit) → u32
   └─ Radix Sort (8 passes × 4-bit): histogram → prefix-sum → scatter

3. GPU Fragment Phase (Tiled Forward+)
   ├─ Per-pixel: determine tile_idx from screen coords
   ├─ Glass SDF check (analytical):
   │   ├─ If inside glass → Snell refraction bends lookup coords
   │   ├─ Chromatic aberration: R/G/B sampled at slightly offset UVs
   │   ├─ Pseudo-caustics: fwidth(distortion) → brightness boost
   │   └─ Frosted blur: textureSampleLevel at mipmap LOD from roughness
   ├─ Loop sorted Gaussians for this tile (front-to-back):
   │   ├─ EWA power: -0.5 · dᵀ · V · d
   │   ├─ SH color evaluation (view-dependent material)
   │   ├─ Alpha blend: weight = α · remaining_alpha
   │   └─ Early-out when remaining_alpha < 0.01
   └─ Output final pixel color
```

## Why SVO + Gaussian Splatting?

| Concern | SVO alone | SVO + 3DGS hybrid |
|---------|-----------|-------------------|
| Visual quality | Hard voxel edges | Soft, photorealistic splats derived from voxels |
| VRAM usage | 8 bytes/node | Same octree + Gaussians generated on-the-fly (no storage) |
| LOD | Octree depth cutoff | Automatic: large fuzzy Gaussian at distance, sharp near camera |
| Lighting | SDF soft shadows | Spherical Harmonics → view-dependent color + soft light scatter |
| Glass interaction | Voxels refract OK | Gaussians have alpha/extent → organic refraction through glass |
| Physics | Direct SVO query | SVO is authoritative; Gaussians are visual-only |
| Sort cost | N/A (ray march) | O(N log N) GPU radix sort, amortized < 1ms for 1M splats |

## Why Double Buffering?

Without ping-pong buffers, the GPU must wait for WASM to finish uploading SVO changes (stall), or WASM must wait for the GPU frame to complete. Double buffering decouples them:
- Frame N: GPU reads FRONT buffer, WASM writes to BACK buffer
- Frame N+1: Swap → GPU immediately renders new data at 120 FPS
- Bind groups are pre-built for both buffers; swap is a pointer flip

## Why Bevy?

Bevy provides the ECS runtime, plugin system, and render graph infrastructure.
World geometry is NOT rendered via Bevy's built-in PBR pipeline — all rendering
flows through custom compute + fragment passes (Gaussian generator → radix sort
→ tiled rasterizer). Bevy's value is system scheduling, resource management,
and the `bevy_rapier3d` physics plugin.

## Phases

| Phase | Tickets | Milestone |
|-------|---------|-----------|
| 0 – Scaffold | T1 | Crate compiles to WASM, Bevy + Dioxus run |
| 1 – Render infra | T2, T6 | SVO + Gaussian generator + tiled renderer on canvas |
| 2 – Visuals | T3, T4, T5 | Liquid glass with caustics, particles as Gaussians, theme |
| 3 – World sim | T7, T8 | Physics via Rapier on SVO, character movement |
| 4 – UI bridge | T9, T10 | Dioxus layout → 3D glass SDFs, world-space panels |
| 5 – Integrations | T12–T15 | Ticket, doc, code, graph editors in 3D world |
| 6 – World editor | T16 | Voxel painting with SDF brushes, live Gaussian regeneration |
| 7 – Tuning | T11 | Runtime parameter UI for all rendering/physics knobs |

## Acceptance Criteria (Epic-Level)

1. Single `trunk serve` command launches the full editor in a browser
2. 3D world rendered via procedural Gaussian splatting from SVO at ≥ 60 FPS (1080p)
3. Tiled forward+ renderer handles ≥ 1M Gaussians at < 10ms sort + rasterize
4. Liquid glass UI panels refract Gaussians with chromatic aberration and caustics
5. Frosted glass uses mipmap blur (textureSampleLevel), not per-pixel Gaussian blur
6. Double-buffered SVO upload: no stalls when editing voxels mid-frame
7. SVO drives Rapier physics; Gaussians are visual-only
8. Spherical Harmonics on Gaussians produce view-dependent material appearance
9. All existing viewer backends (ticket, doc, log, context) accessible from within
10. Total WASM bundle < 15 MB gzipped
