# Procedural road and building generation (GE-0015)

`generate_road_surfaces` and `generate_building_volumes` convert imported OSM features into simple local-frame meshes. This is the first visible reconstruction capability; no LOD, collision, facade, bridge or sidewalk generation is included.

## Determinism

The RNG is **ChaCha8**, seeded from `(recorded seed, stable object key)` via SHA-256; `StdRng`/`SmallRng` are deliberately not used because their values may change between releases. The generator version (`procedural-v0.1`) and resolved seed are recorded on every `GeneratedGeometry`. Same features + seed produce byte-identical output.

## Geometry

- Coordinates are local metres (east, north, up) relative to the first feature, never global float32 positions.
- Buildings: extruded prisms. Numeric `height` → `measured`; `building:levels` → `derived_with_confidence` at 3 m/floor; otherwise an inferred default chosen deterministically from 6/9/12 m.
- Roads: quad strips around the centreline with a recorded width. Per-vertex averaged (miter) normals let adjacent segments share edges, so bends do not leave gaps — simple junction handling.
- `verify_mesh` checks finite vertices, triangle index alignment and in-range indices.

## Provenance

Every generated asset carries the source descriptor's licence election, attribution, version/vintage and derivation chain, plus source feature IDs and the recorded seed/algorithm. The lawful-source gate is applied to both generators.

## Verification

`cargo test --workspace --locked` includes four procedural tests: determinism with identical output, evidence tiers and named-RNG recording, provenance plus mesh validity and golden vertex counts, and gate/invalid-request handling. Forty-three Core tests pass in total.
