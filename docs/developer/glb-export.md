# GLB interchange (GE-0019)

`export_glb(meshes, world, sources, report)` writes a glTF 2.0 binary with:

- POSITION float32 VEC3 accessors and uint32 scalar indices, with real computed `min`/`max`.
- Node/mesh `extras.genesis` carrying asset ID, generator version, seed, evidence, World IR source object IDs, local-frame origin and full provenance.
- A separate machine-readable attribution sidecar from `attribution_sidecar`.
- Explicit warnings for materials, textures, LOD, collision, instancing and float64 omission.

The extension allowlist is **empty** in this slice; no `extensionsUsed` is emitted when empty (an empty array is invalid glTF). The lawful-source gate and structural export gate run before writing. `import_glb` re-reads the header, chunks, accessors and Genesis extras so metadata survival is tested, not assumed.

## Format gate

The official Khronos `gltf-validator` (npm 2.0.0-dev.3.10) runs against a generated fixture in CI on Ubuntu and via `prototype/gltf-fixture/validate.mjs`. Current result: **0 errors, 0 warnings, 0 infos**. Earlier drafts failed with `EMPTY_ENTITY` and `BUFFER_VIEW_TARGET_MISSING`; the fixes are recorded in `prototype/gltf-fixture/RESULTS.md`.

## Limits

No materials, UVs, textures, animation, LOD, collision, engine plugins, USD or 3D Tiles. A structurally valid GLB does not prove visual or geographic correctness; those remain separate validation dimensions.
