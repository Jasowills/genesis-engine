# Genesis Engine — Formats and Standards

**Research date:** 2026-11-27. Evidence base: `research/notes/desktop-viewport-formats.md`, `research/notes/procedural.md` and `research/notes/geospatial.md` (all primary-source cited, access date 2026-11-27). Versions below are the ones verified on the research date, not recommendations.

## 1. Three format roles must be separated

| Role | Evidence-backed choice | Why |
| --- | --- | --- |
| Authoring / internal World IR | **Purpose-built versioned IR** (proposed, not yet designed) | glTF is explicitly a *transmission* format, not an authoring or streaming format; 3D Tiles is streaming-oriented, not authoring-oriented. The IR must carry stable IDs, provenance, source/inferred flags, validation results, CRS + units + up-axis, generation seeds/rules, composition hints and a migration path. None of the surveyed formats covers all of these. |
| Runtime interchange / game export | **glTF 2.0 / GLB** | Ratified extensions: `KHR_gaussian_splatting`, `KHR_texture_basisu`, `KHR_draco_mesh_compression`, `EXT_meshopt_compression`, `KHR_materials_*`, `EXT_mesh_gpu_instancing`, `KHR_xmp_json_ld`. Native loaders in three.js r187, Babylon 9.38.0, Bevy, Cesium. Not an authoring or streaming format. |
| Large-scene streaming / baked output | **3D Tiles 1.1 (OGC 22-025r4)** | HLOD with `geometricError` in metres and SSE refinement, implicit tiling, `REPLACE`/`ADD`, metadata at tileset/tile/content/feature level; glTF is the primary tile format; local frames are Z-up/metres with explicit Y→Z rotation. Good output target; authoring-hostile. |
| Optional authoring/composition investigation | **OpenUSD v26.11.1** | Layers, references, variants, payloads, value resolution, `PointInstancer`, stage metrics/up-axis, `UsdVol ParticleField3DGaussianSplat`, UsdPreviewSurface 2.6. Licence is the **Tomorrow Open Source Technology License 1.0** (Apache-2.0 with modified trademark clause, §6) — legal review required. Heavy C++ dependency. |

Other formats: OBJ/MTL as a strict-subset fallback import; FBX **licence unverified** (Autodesk SDK page unreadable) — do not shortlist; PLY/PCD at the point-cloud ingest edge (LAS/LAZ/E57 licences unverified); splat ingest via `splat-transform` 3.12.0 (PLY/SOG/SPZ/KSPLAT/.splat, LOD chunking, GLB output) — do not treat splats as editable meshes.

## 2. Coordinate, scale and precision

- **glTF:** +Y up, metres, radians, **float32** vertex positions (core 2.0.1). **Derived:** float32 ULP is 0.5 m at Earth-radius 6.37e6 m and 1 m at 1e7 m — this is why georeferenced scenes jitter. glTF 2.1 adds `float64` component-type enum values but vertex attributes still require the draft `KHR_accessor_float64`; Khronos contributors state WebGL/WebGPU lack f64 buffers/shaders.
- **3D Tiles:** +Z up locally, metres, radians, WGS84 ECEF often global; per-tile 4x4 transforms and explicit glTF rotation.
- **USD:** configurable stage up-axis and stage metrics.
- **Engineering consequence:** authoritative coordinates high precision, rendering relative to a floating origin / local tile frames. This belongs in the World IR from day one — it cannot be bolted on later. Requires origin-rebasing invariants across picking, undo, physics and metadata, plus golden regression tests.

## 3. Materials, textures, compression

- Material baseline: glTF metallic-roughness + ratified `KHR_materials_*` (anisotropy, clearcoat, dispersion, emissive_strength, ior, iridescence, retroreflection, sheen, specular, transmission, unlit, variants, volume). MaterialX (Apache-2.0, unverified at recheck) for node graphs if USD path is chosen.
- Textures: KTX2/Basis Universal v2.5 (Apache-2.0; ETC1S, UASTC LDR/HDR, XUASTC) with `KHR_texture_basisu`.
- Mesh: Draco 1.5.9 (Apache-2.0) and meshopt v1.3 / gltfpack (MIT). **Compression is for delivery, not authoring** — keep IR uncompressed to preserve precision and provenance.

## 4. Metadata preservation

- glTF `extras` is the documented app-specific mechanism but has **no shared schema** → vendor drift; `KHR_xmp_json_ld` is ratified.
- 3D Tiles 3D Metadata spec defines schemas/classes/enums and per-vertex/texel/feature metadata with semantics.
- No standard provenance/confidence vocabulary was found. Genesis needs its own metadata profile recording source, licence, observed-vs-inferred flags, validation version, generation seed, CRS and content hashes, with round-trip tests across GLB/USD/3D Tiles.

## 4a. Geospatial standards and internal CRS (R-01)

- **Internal representation:** authoritative geodetic coordinates + CRS identifier/PROJJSON + coordinate epoch + **vertical datum name and unit** in the project file; convert to a local ENU/tangent-plane frame for mesh maths and rendering. PROJ `+proj=topocentric` (EPSG:9836/9837), glTF `CESIUM_RTC` and 3D Tiles `RTC_CENTER` are the documented precedents. Switching origins must be consistent across geometry, terrain, picking, undo and metadata.
- **GeoJSON (RFC 7946):** WGS84/CRS84, lon/lat order, third coordinate = **ellipsoidal** height, ~2–5 m different from mean sea level; no CRS member; antimeridian-crossing geometries SHOULD be cut; bbox east < west when crossing. Six decimal degrees ≈ 10 cm is precision, **not** uncertainty.
- **GeoPackage 1.4 (OGC 12-128r19):** portable SQLite bundle with vector, raster, tiles, R-Tree; coordinate-epoch extension is a GDAL convention pending standardisation.
- **FlatGeobuf:** packed Hilbert R-tree, streaming write/append, HTTP range bbox filtering; cannot do random writes; geozero adapter is read-only.
- **GeoParquet 2.0:** PROJJSON CRS, decimal-year `epoch`, `edges` with geodesic interpretations; **incubating** OGC standard, not final; Rust `geoparquet`/`geoarrow` 0.10 require Rust 1.95.
- **GeoTIFF 1.1 + COG 1.0 (OGC 21-026):** tiled IFDs, overviews, GeoKeys, prescribed IFD layout for range requests; HTTP Range + CORS required; BigTIFF above 4 GB; OGC licence has no patent grant. Vertical datum is a convention, not guaranteed.
- **Shapefile:** import-only; 2 GB component limit, 10-char field names, no topology, `.prj` is an optional sidecar.
- **Antimeridian:** RFC 7946 cut convention, PROJ `+over`/`+lon_wrap`, PostGIS geography has no dateline/pole conception. Define an explicit cut-or-unwrap policy with a test corpus.
- **Terrain streaming:** quantized-mesh 1.0 is a candidate but its repository licence is unverified; do not derive code until clarified.
- **Rust libraries:** `geo` 0.33.1, `geozero` 0.15.1, `proj` 0.32.0, `geos` 11.3.1 (dynamic), `rstar` 0.14.0 (MSRV 1.90), `geographiclib-rs` 0.2.7, `gdal` 0.19.0 — all permissive wrappers; GEOS itself LGPL-2.1; GDAL binary dependency mix must be audited.

## 5. Road / procedural standards

- **ASAM OpenDRIVE 1.9.0** (released 2026-05-19) — free-to-download standard for road geometry, lanes, roadmarks, signals and junctions; `.xodr`/`.xodrz`; trademark considerations. Candidate validated export target; it is a static simulation description, not an OSM converter.
- **Simple 3D Buildings** (OSM community spec) — `height`, `min_height`, `building:levels`, `roof:shape/height/angle/orientation/material`, `building:part`; explicitly limited; levels have invalid values in the database; no facade-height tag.
- **WAI-ARIA 1.2 / APG** — conformance baseline for Studio DOM panels (tree/grid semantics, keyboard paths). No primary source found defining accessible 3D viewport interactions → keyboard-numeric transforms must be first-class.

## 6. Desktop/viewport separation (evidence, not decision)

| Decision | Evidence-supported shortlist | Caveat |
| --- | --- | --- |
| Desktop shell | Tauri 2.12.7 (Apache-2.0/MIT, Rust core, small bundles, updater verified on Win/macOS/Linux with mandatory signature verification) | Webview differs per OS (WebView2/WKWebView/WebKitGTK) → GPU capability PoC required. Electron 45.5.0 is the alternative for consistent Chromium but has no Linux built-in updater. |
| Viewport renderer | three.js r187 (MIT; TransformControls, BatchedMesh `batchId`, LOD with hysteresis, GLTF/KTX2/Draco/PLY/PCD/USD loaders) + React/TS via R3F 9.8.2 + drei 10.7.9 | Babylon.js 9.38.0 (Apache-2.0, documented `GizmoManager` + `snapDistance`) is the second candidate. Bevy 0.20.1 / wgpu 30.0.2 + egui 0.37.0 are higher-cost alternates. These are **two separate decisions** per spec §17. |
| Accessibility | ARIA tree/grid panels + keyboard-numeric transform path | Canvas accessibility has no standard pattern in the reviewed sources. |

## 7. Precision/streaming proof-of-concept requirements (to be tickets)

1. Floating-origin test at ~1e7 m coordinates in each shortlisted viewport.
2. Load/unload stress test for GPU resource leaks (three.js r187 and Babylon 9.38.0 both show active disposal fixes).
3. World IR → GLB → re-import metadata round-trip; World IR → 3D Tiles → reload.
4. Axis/unit conversion golden fixtures (Y↔Z, degrees↔radians, scale).
5. WebGL2/WebGPU capability probe in all three target webviews.

## 8. Unverified / thin evidence

FBX SDK licence; LAS/LAZ/E57 licence text; MaterialX exact crate licence (GitHub API 403); cesium-native release version (API 403); Slint/Qt licences; Babylon Native embedding; Bevy gizmo/snapping maturity; quantized-mesh and 3D Tiles repository licences; `geopackage` Rust crate version/licence; `geo` far-from-origin fixes (unreleased); no independent performance benchmarks were run. Geospatial detail has been appended in §4a.
