# Genesis Engine — Test Strategy (planned, not yet implemented)

**Date:** 2026-11-27. **Evidence base:** `research/notes/quality-ci-gpu.md`, `research/notes/geospatial.md`, `GENESIS_ENGINE_SPEC.md` §47–48. This document defines the intended layered strategy. No tests exist yet because no product code exists yet. All thresholds below are **project policy to be calibrated with fixtures**, not claimed standards.

## 1. Layered strategy

| Layer | What it covers | Planned approach | Evidence / caveat |
| --- | --- | --- | --- |
| Unit | CRS conversion, geometry ops, schema validation, deterministic generation, ID/hash behaviour, job state transitions, cache/index behaviour | Rust unit tests in each crate; golden vectors for axis/unit conversion (Y↔Z, degrees↔radians, scale) and named pinned RNG (ChaCha8/12). | Rand book warns seeding alone ≠ reproducibility; `StdRng`/`SmallRng` are non-portable. `geo` 0.33.1 has known far-from-origin issues fixed only on unreleased `main` → add regression fixtures. |
| Property / fuzz | Invalid polygons, degenerate geometry, coordinate edge cases (antimeridian, ±180, large eastings), malformed files, parser robustness, serialization round trips | `proptest`/`cargo fuzz`; corpus of malformed OSM multipolygons; fuzz hostcall buffers for plugins. | Antimeridian behaviour differs across RFC 7946, PROJ and PostGIS; define canonical internal policy first. |
| Integration | Geospatial import (PBF/GeoJSON/GeoPackage/FlatGeobuf/COG), external worker adapters (COLMAP, Open3D, Blender), World IR persistence, selective regeneration, export pipelines, plugin boundaries | Fixture-based tests with small redistributable data; spawn real workers where licences permit; assert normalised progress events and cancellation. | Reconstruction tools expose no documented cancellation API → process supervision + checkpoint resume is the only portable pattern. |
| End-to-end | Create project → import sample region → generate a supported layer → inspect → validate → export → reopen in a target tool | One narrow vertical slice first (spec §50); same architecture as later CLI/headless paths. | Do not start before research/architecture approval. |
| Cross-platform | Windows x64/ARM64, macOS ARM64 (plus Intel), Linux x64/ARM64 | GitHub Actions native runners; macOS for Metal/signing; Linux for high-volume tests; GPU lane release-only. | Standard runners: 2–4 vCPU, 7–16 GB RAM, 14 GB SSD — fixtures must fit. macOS ARM64 has no static UDID / nested virtualisation. |
| Performance | Increasing feature counts, large polygons/road graphs, peak RAM, geometry throughput, export time, viewport frame time, cache hit rate, chunk load/unload | Benchmark harness with seeded fixtures; report ranges, p50/p95/max, never a single number. | All current performance ranges are **[E] estimates**, not measurements. |

## 2. Validation gates (spec §26 dimensions)

1. **Format gate:** `glTF-Validator` exit code 0 on GLB/glTF exports; archive its JSON report (Apache-2.0, verified tool behaviour).
2. **Structural gate:** zero-area faces = 0; report non-manifold edges/vertices, self-intersections, normal consistency, UV/texture distortion (PyMeshLab filters; offline sidecar, not core runtime).
3. **Geometric gate:** sampled point-cloud-to-mesh deviation / Hausdorff with p50/p95/max and sampling method; ICP `fitness`/`inlier_rmse` or COLMAP reprojection error. Internal consistency only — never a substitute for geographic accuracy.
4. **Geographic gate:** only with independent checkpoints: ASPRS Ed2 V2 RMSEH/RMSEV (NVA acceptance, VVA report-only)/RMSE3D, ≥30 checkpoints, ≥2× product accuracy, full residual table. Otherwise state explicitly "no geographic accuracy claim".
5. **Budget gate:** triangle/material/texture/chunk/memory limits checked against a named target profile; values are project policy, not standards claims.
6. **Provenance gate:** every metric record stores reference dataset, CRS/datum, method, tool version, parameters, seed and limitations.

**Naming trap:** the Rust crate `trimesh` 0.5.2 is an ODE physics crate, **not** Python `trimesh` mesh QA. Use explicit crate identity in tickets.

## 3. Fixture policy

Small, deterministic, redistributable only. Prefer Microsoft GlobalMLBuildingFootprints (CDLA Permissive 2.0) or Google Open Buildings elected under CC BY 4.0 small cut-outs, with licence text and election recorded. **Do not** bundle raw OSM extracts, legacy ODbL datasets, Cesium ion/Google Maps content, proprietary imagery or large model weights. Pin dataset index snapshots and record checksums. Benchmark study area: a bounded ~500 m–1 km section of Ikeja, Lagos — **exact boundary to be chosen and recorded by the owner** before any comparative benchmark.

## 4. Security and licence checks in CI

`cargo audit` / `cargo deny` (to be verified), lockfiles committed, builds with `--locked`, GitHub Actions pinned to SHAs, secret scanning + push protection, dependency licence audit (must include COLMAP/Open3D/MeshLab/PyMeshLab/trimesh/GDAL/GEOS), `gdal --license` audit of shipped binaries, adversarial plugin fixtures (infinite loop, huge `memory.grow`, empty writes, invalid WIT imports), zip-slip/decompression-bomb fixtures on all three OSes.

## 5. Definition of done

Per spec §48: scope matches ticket; acceptance criteria pass; tests added and pass; format/lint checks pass; type/build checks pass; error paths handled; security/licence implications reviewed; docs updated; no unrelated changes; independent reviewer approves; evidence recorded; `progress.md` and ticket status updated; commit created per policy. `verified` never means "code written".
