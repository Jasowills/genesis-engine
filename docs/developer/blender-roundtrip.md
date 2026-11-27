# Blender GLB round-trip (GE-0029)

The round-trip uses Blender strictly as an **external process**. Genesis Core contains no Blender code, links no GPL library, and never executes Blender at runtime; the probe script lives in `tools/blender_roundtrip.py` and is run manually or in a provisioning-enabled CI lane.

## Pinned evidence

- Blender **4.5.17 LTS** (build `2f5ec98689b8`, 2026-11-24), installed locally via Homebrew cask (free, no purchase).
- Input: Core-generated GLB from the frozen Ikeja fixture (`genesis project/boundary/features/generate/validate/export`).
- Probe: `blender --background --python tools/blender_roundtrip.py -- input.glb output.glb report.json`.
- Measured: 2 meshes before and after, object names preserved, `genesis` extras retained on mesh data with identical values, `metadata_identical: true`, `metadata_loss: []`.
- Core re-import of `fixtures/ikeja-blender-roundtrip.glb`: version 2.0, 2 meshes, 14 triangles, provenance attribution `© OpenStreetMap contributors`, licence `ODbL-1.0`, generator `procedural-v0.1`.

## Known limitations / metadata loss

No loss was observed for `genesis` extras in this pinned version. Blender may rename objects or merge materials across versions; custom-property preservation depends on glTF `extras` support and on extras landing on mesh data rather than objects. The fixture carries no materials, so material round-trip is **unverified**. Component overhead and non-macOS platforms are not measured here. The generated GLB is output data, not Blender GPL code.

## Verification

`cargo test --workspace --locked` includes two round-trip tests: Core re-import of the Blender-exported GLB with provenance assertions, and validation of the pinned Blender report (`metadata_identical`, no metadata loss, documented limitations).
