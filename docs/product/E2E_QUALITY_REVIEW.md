# Product-level E2E and asset-quality strategy

This review is about the **product journey**, not unit correctness. We drive the shipped surfaces (headless CLI and React Studio shell) the way a user would, then judge the generated assets against a quality rubric.

## Journey under test

1. Create a project with an explicit CRS/datum.
2. Import a lawful boundary and OSM features with real provenance (not defaults).
3. Generate geometry, validate, inspect the report, export GLB.
4. Inspect the exported GLB header/meshes/extras. GLB re-import and provenance comparison are covered by `crates/genesis-core/tests/vertical_slice.rs`, not by the product harness.
5. Open the built Studio shell, inspect hierarchy/inspector/viewport, select and transform a node, and use keyboard navigation.
6. Exercise adverse journeys actually run: missing input file and unsupported profile. Other adverse cases (denied sources, corrupt IR, seed change, symlinked destinations, export before validation) are covered by product-code tests, not by this product journey; export after a valid report succeeds by design.

## Quality rubric (what "good" means here)

- **Geometric validity**: no zero-area faces, no NaN/Inf, consistent +Z winding for upward surfaces, bounded triangle counts, stable IDs.
- **Source fidelity**: mesh footprints and heights derived from the imported OSM/boundary data, CRS/frame origin matching the source area; provenance and licence preserved end to end.
- **Honest evidence**: `measured` only for source-declared measurements; inferred geometry labelled; no survey or photoreal claim.
- **Usability**: a user can complete the journey with documented commands, get legible errors, and see the scene in Studio without hidden state.
- **Limitations to surface**: procedural primitives are schematic, not photographic reconstructions. "Looks like the map" means footprint/height agreement with the imported OSM source — not Google Maps imagery, which is a hard-denied reconstruction input (D-028).

## Evidence to collect

- Full command transcript and JSON envelopes from a fresh project.
- Per-mesh metrics (vertices, triangles, bounds, evidence, provenance).
- Validation report metrics and exported-GLB structure; re-import comparison is Core-test coverage (`vertical_slice.rs`), not product-harness evidence.
- Studio screenshots and interaction results.
- Adverse-journey outcomes and whether errors are actionable.
- A written verdict per rubric dimension: PASS / PARTIAL / FAIL, with reasons.

No claim of visual or survey quality may be made from a synthetic 3×3 DEM or primitive OSM fixture. The verdict must separate "product works" from "assets are production-quality".
