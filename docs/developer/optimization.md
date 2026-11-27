# LOD, collision and delivery compression (GE-0030)

Source World IR and geometry remain uncompressed; everything here is delivery-time.

- `generate_lod_chain(mesh, target)` produces three deterministic **screening** LODs from a named target profile (`DesktopPreview` cap 12 triangles, `DesktopGameReady` cap 64). Each level keeps a fixed stride of source triangles, remaps indices to compact vertices, preserves the geodetic frame/seed/provenance, and is checked with `verify_mesh` plus a budget assertion. This is not visually faithful simplification.
- `build_collision_proxy(mesh, tolerance)` produces a padded AABB and measures coverage over all vertices and triangle centroids. A shrunk proxy reports coverage below 1.0; non-finite samples are rejected. Coverage is a geometric enclosure metric, not a physics-engine guarantee.
- `compression_allowlist()` records codec licences (Draco Apache-2.0, meshopt MIT, KTX2/Basis Apache-2.0/BSD-3 sources). `compression_is_allowed` gates declaration on a permissive codec licence. **No codec encoder/decoder is bundled or invoked** by Core.

## Scope limit

Budgets are named policy thresholds, not measured frame-time or memory results. No texture is actually transcoded, no Draco/meshopt binary is distributed, and no target-engine optimisation is performed (GE-0031). Precision regressions cover stable IDs, frame origin, seed and provenance only.

## Verification

`cargo test --workspace --locked` includes three optimization tests: budget/frame/determinism across LOD levels, collision coverage with a deliberately incomplete proxy, and licence-aware codec allowlisting.
