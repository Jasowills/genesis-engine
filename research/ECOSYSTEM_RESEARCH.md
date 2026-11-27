# Genesis Engine — Ecosystem Research

**Research date:** 2026-11-27. **Evidence base:** `research/notes/*.md` (six briefs complete and read: `ai-ml`, `competitive-data-rights`, `desktop-viewport-formats`, `plugin-security`, `procedural`, `reconstruction`). Two briefs (geospatial; quality/CI/GPU) were in progress at the time of writing and their rows will be appended after verification. Every recommendation below is **evidence-based but not owner-approved** — this document does not make architecture decisions.

## 1. Method

Primary sources only for material claims: official docs, official repositories, LICENSE/COPYRIGHT files, release notes and advisories, standards specifications, papers. Access date 2026-11-27 throughout. Conflicts are logged in `research/RESEARCH_REGISTER.md` §4. Unverified claims are marked and are not used to justify a recommendation.

## 2. Capability-by-capability recommendations

Field set per spec §20: Capability | Candidate | Primary source | Licence | Status | Platform | Integration | Strengths | Limitations | Genesis fit | Recommendation | Confidence | Follow-up. To keep this document readable, URLs are consolidated in the note files; the columns below carry the decision-relevant evidence.

| Capability | Candidate | Licence / status | Genesis fit | Recommendation (evidence only) | Complexity | Confidence |
| --- | --- | --- | --- | --- | --- | --- |
| Desktop shell | Tauri 2.12.7 | Apache-2.0/MIT; active; Win/macOS/Linux | High | **Wrap** — Rust core matches, small bundles, only verified updater on all three OSes. Webview GPU variance needs PoC. Electron 45.5.0 is the consistent-Chromium alternative but has no Linux built-in updater. | Medium | High |
| Studio UI | React 19.3.1 + TypeScript 7.0.2 | MIT / Apache-2.0; active | High | **Adopt** for hierarchy/inspector panels; verify TS 7 tooling compatibility. DOM ARIA panels give accessibility; canvas needs keyboard-numeric alternatives. | Low–medium | High (versions), Medium (TS7 tooling) |
| Viewport renderer | three.js r187 (+ R3F 9.8.2 / drei 10.7.9) | MIT; very active | High | **Wrap** as leading web viewport: `TransformControls`, `BatchedMesh` `batchId`, `LOD` hysteresis, GLTF/KTX2/Draco/PLY/PCD loaders. Babylon.js 9.38.0 (Apache-2.0, documented `GizmoManager` + `snapDistance`) is second. Bevy 0.20.1 / wgpu 30.0.2 + egui are higher-cost alternates. **Separate decision from shell.** | Medium | High |
| Internal World IR | Purpose-built versioned IR (proposed) | Project licence TBD | High | **Build** a thin IR with adapters: glTF is explicitly not an authoring/streaming format; 3D Tiles is streaming-oriented; USD brings a heavy C++ dependency and a modified trademark licence. IR must own stable IDs, provenance, confidence, CRS/units/up-axis, seeds, validation state, migrations. | High initially | High for need, Low for design maturity |
| Interchange / export | glTF 2.0 / GLB | Khronos; ratified extensions | High | **Adopt** as first interchange target (PBR, KTX2, Draco/meshopt, gaussian splatting extension, XMP). Keep source IR uncompressed. | Low | High |
| Streaming / baked output | 3D Tiles 1.1 (OGC 22-025r4) | OGC community standard | Medium–high | **Adapt** as output target and precision/streaming reference; not authoring. | Medium–high | High |
| Authoring/composition research | OpenUSD v26.11.1 | Tomorrow Open Source Technology License 1.0 (trademark modification) | Medium | **Investigate, do not adopt yet**: layers/variants/references are valuable; legal review + heavy C++ dependency. | High | High for capabilities |
| OSM → 3D reference | OSM2World | MIT per LICENSE (conflict: OSM Wiki says LGPL); active | High | **Wrap as optional converter / coverage baseline**; resolve licence conflict before vendoring; Java/JVM stack. | Medium | High (with conflict noted) |
| OSM road surfaces | osm2streets | Apache-2.0; Rust; API unstable | High | **Evaluate for wrap/fork**: strongest Rust-native centreline→surface/lane implementation; turning movements, crosswalks and edit preservation are *planned*, not implemented. | Medium | High |
| Road topology reference | Lanelet2 | BSD-3; active; C++/Python | Medium | **Study** for lane graph/consistent edit model; explicitly not a raw OSM parser. | Medium | High |
| Terrain / LiDAR bridge | PDAL, Open3D | BSD / MIT; active | High | **Wrap** as worker libraries (point-cloud ingest, fusion, registration, meshing). Keep permissive code in-process if desired. | Low–medium | Medium–high |
| SfM / MVS backbone | COLMAP 4.2.1 | BSD-3; active | High | **Wrap as external worker** — de-facto pose/SfM standard, full CLI, no UV texture generation in core; pair with texturing stage. | Medium | High |
| Textured mesh stage | OpenMVS 2.4.0 / Meshroom / ODM | AGPL / MPL / AGPL | Medium–high | **Wrap behind process boundary only**; legal review required for AGPL + OpenMVS's research-only IBFS component. | Medium | High (licence), Medium (fit) |
| Photogrammetry (aerial) | OpenDroneMap | AGPL-3.0; active; Docker/CLI/REST | Medium | **Defer pending legal review**; excellent georeferenced DEM/orthophoto/textured mesh output. | Medium | High |
| Neural reconstruction | Nerfstudio (Apache-2.0, stale release), gsplat (Apache-2.0, active) | Permissive but CUDA-only | Low–medium | **Defer**: novel-view/splat branch only; no editable mesh guarantee; maintenance signal on Nerfstudio is slow. | High | High |
| Splat → editable mesh | SuGaR, 2DGS | Inria/MPII non-commercial research only | Low | **Reject for product use**; study methods only. Conversion is lossy (view-dependent radiance, transparency, fine detail) and watertightness remains an open problem. | High | High |
| AI runtime baseline | ONNX Runtime 1.32.0 | MIT; Production/Stable; broad EP matrix; cancellation API | High | **Wrap via C ABI adapter** as default local runtime. Candle (MIT/Apache) is a Rust-native optional adapter where an exact model implementation exists; Burn is monitor-only. | Medium | High (docs), Medium (perf unbenchmarked) |
| AI models (commercially clean) | SAM ViT-B (Apache-2.0), YOLOX (Apache-2.0), Depth Anything V2 **Small** (Apache-2.0), LaMa (Apache-2.0 code; verify weights) | See note | High | **Adopt selectively** for segmentation, detection, relative depth, inpainting. Avoid Ultralytics YOLO (AGPL/paid), DA-V2 Base/Large/Giant (CC-BY-NC), DUSt3R, Hunyuan3D 2, VGGT non-commercial checkpoint. | High | High |
| Deterministic generation | Rust `rand` + `rand_chacha` (ChaCha8/12) | MIT/Apache-2.0 | High | **Build** named pinned RNG + recorded seeds + golden vectors; `StdRng`/`SmallRng` are explicitly non-portable. No mature incremental-rebuild contract found in the ecosystem. | Low | High |
| Plugin runtime | Wasmtime 51.0.0 + WASI 0.2 components + WIT ABI | Apache-2.0 WITH LLVM-exception; active, monthly majors | High | **Wrap** as leading untrusted-plugin sandbox: capability-based deny-by-default, typed ABI, Tier-1 Win/macOS/Linux. Require ≥51.0.0 (recent Windows escape and host-memory CVEs); epochs + async timeout, explicit resource caps; no `inherit_env`/`inherit_stdio`. | High | High |
| Native workers | Separate OS processes (Windows Job Objects, Linux seccomp, Apple sandbox) | Platform docs | High | **Build** a cross-platform worker-supervision abstraction; argv-only execution, scrubbed env, per-job CPU/memory/time caps, kill tree. seccomp alone is not a sandbox. | High | Medium (macOS thin) |
| Supply chain | Cargo.lock, `--locked`, pinned Actions SHAs, attestations, SBOM | Tool-supported | High | **Adopt**; provenance gives integrity, not trustworthiness. Enable secret scanning + push protection. | Low–medium | High |
| Attribution/provenance | Genesis-owned provenance profile on IR + export | n/a | High | **Build** — no standard vocabulary found; must survive GLB/USD/3D Tiles round-trips and export to game engines. | Medium | High for need |

## 3. Reuse vs build — summary

- **Reuse/wrap (mature, permissive, CLI/library-first):** Tauri, React, three.js, glTF loaders, COLMAP, Open3D, PDAL, ONNX Runtime, Wasmtime, KTX2/Basis, Draco/meshopt, OSM2World (conditional), osm2streets (conditional).
- **Wrap behind process boundary (copyleft):** OpenMVS, ODM, Blender/MeshLab, AliceVision/Meshroom/OpenMVG (MPL), Mapillary-based Python helpers.
- **Adapt (standards/concepts, not code):** 3D Tiles, USD composition ideas, CGA/Geometry Nodes/Houdini PDG patterns, Lanelet2 topology, ASAM OpenDRIVE, WAI-ARIA.
- **Build (no mature equivalent that meets Genesis requirements):** thin versioned World IR + provenance/confidence model; job orchestration with cancellation/progress/checkpoints; validation gates and quality profiles; selective/incremental regeneration policy; Studio↔engine command/event contracts; plugin manifest/signing/rollback; lawful-source adapter boundary; benchmark harness.
- **Reject for product use:** Google Maps/Street View/Photorealistic 3D Tiles and Cesium ion content as reconstruction inputs (terms prohibit derivation/offline/AI training); INRIA 3DGS/SuGaR/2DGS (non-commercial); research-only AI weights; FBX SDK until licence verified.

## 4. Integration complexity and sequencing (evidence-derived, not approved)

1. **Low:** project skeleton, lockfiles, CRS/precision golden tests, glTF mapping, provenance schema draft, accessibility baseline.
2. **Medium:** Tauri + React + three.js capability PoC; OSM/footprint import; World IR prototype; COLMAP/Open3D worker adapters; ONNX Runtime adapter; Wasmtime plugin PoC.
3. **High:** selective regeneration contract; 3D Tiles generation; splat ingest/validation policy; USD investigation; native worker sandbox profiles per OS; AGPL/legal reviews.
4. **Unknown until PoC:** WebGPU availability in each webview; large-scene frame/resource budgets; wandel through incremental edits surviving source updates; component overhead.

## 5. Evidence quality

High-confidence areas: licences and release states verified from LICENSE files/API; format capabilities from standards; desktop/viewport versions from official registries; security advisories from upstream. Medium: performance claims (mostly vendor figures, no independent benchmark run); platform support for some tools. Low/unverified: global OSM completeness; gen-map source; FBX SDK licence; material recognition models; incremental regeneration contracts; macOS sandbox specifics; cross-version determinism of proprietary node systems.

## 6. Geospatial rows (R-01, appended)

| Capability | Candidate | Licence / status | Genesis fit | Recommendation (evidence only) | Complexity | Confidence |
| --- | --- | --- | --- | --- | --- | --- |
| In-process geometry | `geo` 0.33.1 + `geo-types` 0.7.21 | MIT/Apache-2.0; active (2026-04-20) | High | **Integrate** as default Rust geometry layer; add far-from-origin regression fixtures (known precision fixes are unreleased on `main`). | Low | High (release), Medium (precision) |
| Format conversion bus | `geozero` 0.15.1 | MIT/Apache-2.0; active | High | **Integrate** for zero-copy GeoJSON/WKB/WKT/GDAL/GEOS/FlatGeobuf/GeoParquet conversion; CRS/epoch stays in GE metadata, not geometry. | Low | High |
| CRS / datum engine | PROJ 9.9.0 + `proj` 0.32.0 | MIT; EPSG v13.102; active | High | **Integrate**; store WKT2/PROJJSON **plus** epoch and vertical datum; ship offline grids for target regions; wrapper covers only a subset of the C API. | Medium | High |
| Topology repair | GEOS 3.15.1 + `geos` 0.19 bindings | LGPL-2.1; active; wrapper MIT | High | **Wrap dynamically** behind a `GeometryOps` trait; `geo` first, GEOS for coverage cleaning; avoid static linking (LGPL obligations). | Medium | High |
| Broad interop / raster | GDAL 3.13.3 + `gdal` 0.19.0 | MIT-style core; binary deps vary; active | High | **Wrap** as optional interop adapter with a small GE trait; audit shipped binary licences; one dataset per thread. | Medium–high | High |
| Large-scale / multi-user | PostGIS 3.7 on PostgreSQL | GPLv2 server; ordinary networked client use does not trigger GPL | Medium | **Adapt as optional server mode**; keep MIT core as network client; encode SRID + epoch + height datum explicitly. | Medium | High |
| Spatial index | `rstar` 0.14.0 | MIT/Apache-2.0; active; MSRV 1.90 | High | **Integrate** for tile-local R*-tree; use FlatGeobuf/GiST for on-disk/global scale. | Low | High |
| Geodesy | `geographiclib-rs` 0.2.7 | MIT; active | Medium | **Integrate** for geodesic distance/area; cross-validate vs PROJ `geod`; Rust port is a subset. | Low | Medium–high |
| Vector formats | GeoJSON RFC 7946, GeoPackage 1.4, FlatGeobuf, GeoParquet 2.0 | IETF/OGC/BSD-2/Apache-2.0; GeoParquet is *incubating*, not final OGC | High | **Adapt**: GeoJSON for interchange (WGS84/CRS84, Z=ellipsoidal height); GeoPackage as portable bundle/cache; FlatGeobuf for streaming vector tiles; GeoParquet only for large attribute-heavy input (requires Rust 1.95). | Medium | High |
| Raster / elevation | GeoTIFF 1.1 + COG 1.0, `geotiff` 0.1.0 | OGC royalty-free (no patent grant); crate MIT but minimal | High | **Integrate via GDAL** for COG/range/warp; `geotiff` only for small trusted TIFFs; store source resolution + vertical datum + accuracy in project metadata. | Medium | High |
| Terrain streaming | quantized-mesh 1.0 | **Licence unverified** (no LICENSE in repo) | Medium | **Adapt later**; verify licence before deriving code. | Medium | Medium |
| OSM ingestion | `osmpbf` 0.3.8, libosmium (BSL-1.0), osm2pgsql (GPL-2.0) | Permissive / GPL | High | **Integrate** pure-Rust PBF primitive + own way/relation assembly; keep osm2pgsql external; never use the OSM editing API for bulk reads. | Medium | High |

## 7. Quality / CI / GPU rows (R-08, appended)

| Capability | Candidate | Evidence | Genesis fit | Recommendation (evidence only) | Complexity | Confidence |
| --- | --- | --- | --- | --- | --- | --- |
| Geographic accuracy claims | ASPRS Ed2 V2 (2024): RMSEH/V/3D, ≥30 independent checkpoints ≥2× product accuracy, NVA for acceptance, VVA report-only | Standard | High | **Adopt as the only basis for survey/reference claims**; otherwise report "no geographic accuracy claim". Image similarity is explicit non-evidence. | Medium | High |
| Quality metadata | ISO 19157-1:2023 (no thresholds defined) | Standard (paid) | Medium | **Map** Genesis metric records to 19157-style measures; do not claim conformance without full review. | Low | Medium |
| Mesh/geometry QA | PyMeshLab filters, Open3D metrics, trimesh 5.2.2 (Python), glTF-Validator (Apache-2.0) | Tool APIs verified | High | **Wrap as offline QA sidecar/CI**, not core runtime; gate zero-area faces, non-manifold edges, self-intersections, normal consistency, UV distortion; archive validator JSON. Note Rust `trimesh` 0.5.2 is an ODE physics crate, not mesh QA. | Medium | High for APIs, Low for licences |
| Platform matrix | Windows x64/ARM64 (DX12/Vulkan/CPU); macOS ARM64-first, Metal, **no CUDA**, Vulkan-on-Metal uncertain; Linux x64/ARM64 strongest for Vulkan/CUDA/ROCm | Official docs | High | **Plan matrix accordingly**; CPU fallback mandatory; expose capability manifest; do not promise unverified backends. | High | High (medium for macOS Vulkan) |
| GPU abstraction | wgpu v30 (Apache-2.0/MIT, MSRV 1.87) | Official docs | Medium | **Prototype** for portable viewport/compute; reconstruction-kernel parity unverified; prefer native Metal on macOS. | High | High for API, Low for fit |
| CI cost | Public standard runners free; private $0.006 Linux / $0.010 Windows / $0.062 macOS per min; T4 GPU $0.052/$0.102 (Team/Enterprise only) | GitHub docs | High | Keep high-volume tests on Linux; macOS for Metal/signing; GPU jobs release-only. | Low | High |
| Signing cost | Apple $99/yr; Windows Artifact Signing ~$9.99/mo, OV $150–300/yr, EV $400+/yr | Official docs | High | **Owner approval required before purchase**; design entitlements/key custody early. | Medium | High |
| Performance budgets | All time/RAM/disk ranges are **[E] estimates**, not measurements; COLMAP SIFT GPU memory ≈ `4m² + 1024m` bytes | Anchors + estimates | High | Preflight must output **ranges + confidence**, never a single promise; calibrate with fixtures. | Medium | Low (needs measurement) |

## 8. Research phase status

All eight briefs are complete, read and consolidated. Remaining work is Phase 4 onward: resolve the five logged conflicts in `RESEARCH_REGISTER.md` §4 before writing the corresponding ADRs, then architecture, PRD, roadmap, tickets and the owner decision package. No product implementation has started.
