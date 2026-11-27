# Procedural road and building generation (GE-0015)

`generate_road_surfaces` and `generate_building_volumes` convert imported OSM features into simple local-frame meshes. This is the first visible reconstruction capability; no LOD, collision, facade, bridge or sidewalk generation is included.

## Determinism

The RNG is **ChaCha8**, seeded from `(recorded seed, stable object key)` via SHA-256; `StdRng`/`SmallRng` are deliberately not used because their values may change between releases. The generator version (`procedural-v0.1`) and resolved seed are recorded on every `GeneratedGeometry`. Same features + seed produce byte-identical output; asset IDs include seed and generator version.

## Geometry

- Coordinates are local metres (east, north, up) relative to a deterministic frame origin — the lexicographically smallest coordinate across all features — independent of input ordering and ring winding.
- Buildings: extruded prisms. Clockwise rings are reversed and rotated to a canonical start, so winding cannot produce inside-out walls or a downward roof cap. Numeric `height` → `measured`; `building:levels` → `derived_with_confidence` at 3 m/floor; otherwise an inferred default chosen deterministically from 6/9/12 m. A height tag with units that cannot be parsed is never claimed as measured.
- Roads: quad strips around the centreline with a recorded width. Per-vertex averaged (miter) normals let adjacent segments of the **same way** share edges. Joins between **separate ways** are explicitly deferred (recorded in `parameters["junction"]`) and are not claimed as merged geometry.
- `verify_mesh` checks finite vertices, triangle index alignment, in-range indices and degenerate (repeated-index) triangles.

## Provenance

Every generated asset carries the source descriptor's licence election, attribution, version/vintage and derivation chain, plus source feature IDs, World IR `source_object_ids`, the recorded local-frame origin, seed and algorithm. The lawful-source gate is applied to both generators.

## Verification

`cargo test --workspace --locked` includes nine procedural tests: determinism, winding normalisation, stable origin/identity, evidence tiers, provenance and golden vertex counts, `verify_mesh` negative cases, levels/unit handling, cross-feature junction limitation, and gate/invalid requests. Forty-eight Core tests pass in total.
