# Blender GLB round-trip (GE-0029)

The round-trip uses Blender strictly as an **external process**. Genesis Core contains no Blender code, links no GPL library, and never executes Blender at runtime; the probe script lives in `tools/blender_roundtrip.py` and is run manually or in a provisioning-enabled CI lane.

## Pinned evidence

- Blender **4.5.17 LTS** (build `2f5ec98689b8`, 2026-11-24), installed locally via Homebrew cask (free, no purchase).
- Input: Core-generated GLB from the frozen Ikeja fixture (`genesis project/boundary/features/generate/validate/export`).
- Probe: `blender --background --python tools/blender_roundtrip.py -- input.glb output.glb report.json`.
- Measured: 2 meshes before and after, object names preserved, `genesis` extras retained on mesh data with identical values, `metadata_identical: true`, `metadata_loss: []`.
- Core re-import of `fixtures/ikeja-blender-roundtrip.glb`: version 2.0, 2 meshes, 14 triangles, provenance attribution `© OpenStreetMap contributors`, licence `ODbL-1.0`, generator `procedural-v0.1`.

## Known limitations / metadata loss

No loss was observed for `genesis` extras **content** in this pinned version (array-valued extras are preserved as arrays in the report). Two evidence limitations are recorded: (1) Blender truncated the 67-character Core node/mesh names to 63 characters in the exported GLB, even though the before/after in-memory names matched — hierarchical identity is tracked by `genesis.asset_id`, not by node name; (2) the fixture has a flat hierarchy (`parent: null`) and no materials, so deeper hierarchy and material round-trip are **unverified**. Blender may rename objects or merge materials across versions; custom-property preservation depends on glTF `extras` support and on extras landing on mesh data rather than objects. Component overhead and non-macOS platforms are not measured here. The generated GLB is output data, not Blender GPL code.

## Verification

`cargo test --workspace --locked` includes two round-trip tests: Core re-import of the Blender-exported GLB with provenance assertions, and validation of the pinned Blender report (`metadata_identical`, no metadata loss, documented limitations).
