# Product E2E and asset-quality verdict (2026-11-27)

This is a **product** review: the generated assets and the user journey were judged, not just the code. Strategy is in `E2E_QUALITY_REVIEW.md`; raw evidence is in `evidence/`.

## How the product was driven

- Built the shipped CLI and ran `project → boundary → features → generate → validate → export → scene` on a fresh Ikeja project with caller-supplied CRS and OSM provenance.
- Measured every mesh (vertices, triangles, finiteness, degeneracy, winding, bounds, evidence, provenance) and the exported GLB (header, meshes, triangles, extras) with `tools/product_e2e.py`.
- Built the React Studio shell, injected a real Core-generated scene snapshot via the dev-only `?e2e=1` hook, and drove it in Chrome: viewed the 3D viewport, clicked nodes, checked the inspector, edited a numeric transform, and used keyboard navigation.
- Ran adverse journeys against the real binary (missing input, unsupported profile, export after validation).

## Verdicts

| Dimension | Verdict | Evidence and reasoning |
| --- | --- | --- |
| Geometric validity | **PASS** | 2 meshes, 14 triangles, 0 degenerate faces, 0 downward-facing faces, all vertices finite; GLB re-import preserves 2 meshes / 14 triangles and all `genesis` extras. Validation report: 0 missing provenance, 0 zero-area faces, 0 non-manifold edges, 0 orientation conflicts. |
| Source fidelity | **PASS** | Building mesh bounds match the OSM polygon footprint **exactly** in the local ENU frame (east/north error 0.0 m). Road surface stays within the configured 8 m width of its OSM centreline (max deviation ~3.7 m, expected from strip offsets). Frame origin is now shared between World IR and procedural geometry. |
| Coordinate/precision honesty | **PASS** | Project CRS `EGM96`, WGS84 ECEF→ENU local frame derived from the imported boundary bbox; source-linked origin recorded per mesh; no large-coordinate f32 storage. |
| Provenance and evidence | **PASS** | Exported extras retain `osm:fixture:ikeja`, `© OpenStreetMap contributors`, `ODbL-1.0`, seed 42, generator `procedural-v0.1`, derivation chain and stable IDs. Building evidence is `measured` because the OSM fixture declares a height; road is `inferred_by_rule`. No survey or photoreal claim. |
| User journey (CLI + Studio) | **PASS** | All user-facing commands produced versioned JSON with canonical command names; Studio loaded the generated scene in WebGL2, selected nodes, showed provenance/evidence/derivation chain, accepted numeric transform previews, and supported mouse and keyboard selection. Errors for missing input and unsupported profile were structured and actionable (`cli_error`, exit 1). |
| Visual/asset quality | **PARTIAL — schematic, not map-like** | The viewport shows a flat grey building prism and a flat diagonal road strip on a dark background. No textures, facades, roofs detail, markings, vegetation, terrain relief or map imagery. This is the approved procedural primitive slice (GE-0015/0028), not a photogrammetric reconstruction. It is geometrically faithful to the imported OSM footprint but **does not look like the referenced maps** and is not production-ready visual quality. |
| Benchmark/ground truth | **NOT CLAIMED** | Google Maps / Street View / Photorealistic 3D Tiles are hard-denied reconstruction inputs (D-028) and no independent ground truth exists for the Ikeja fixture. Comparisons are against the lawful OSM source only. |

## Product bug found during this review and fixed

The CLI recorded the local frame from the boundary bbox centre while procedural geometry used its own feature-min origin. The same OSM data therefore produced two different local frames. Both now use the project frame (`generate_*_with_origin`), with product-level evidence showing exact footprint agreement.

## Gaps a principal product engineer would flag next

1. **Visual fidelity** — to look like maps we need roof/facade detail, textures/materials, road markings and real terrain; none is in the approved slice. Adding these is new scope with new tickets and review.
2. **Scale/readability** — the single-building screenshot fills the viewport; camera framing and multi-object navigation need UX work.
3. **Ground-truth benchmark** — a lawful, attributed reference dataset is required before any "accuracy" or "quality versus reality" claim.
4. **Engine/viewport** — WebGL2 rendering works in Chrome here; Tauri packaging and WebView2/WKWebView/WebKitGTK parity remain unverified (ADR-0003 PoC gates).

Bottom line: **the product works end to end and produces valid, source-faithful, provenance-complete assets; the assets are deliberately schematic and must not be presented as photorealistic or map-identical.**
