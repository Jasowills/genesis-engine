# Genesis Engine — Competitive Analysis

**Research date:** 2026-11-27. Evidence base: `research/notes/competitive-data-rights.md`, `research/notes/reconstruction.md`, `research/notes/procedural.md`, `research/notes/ai-ml.md` (all primary-source cited, access date 2026-11-27). This document compares tools against *real Genesis workflows*, not feature checklists. It records evidence and implications; it does not approve an architecture.

## 1. Landscape by workflow stage

| Genesis workflow stage | Closest existing solutions | What they actually do | Where they stop short of Genesis |
| --- | --- | --- | --- |
| Select area + inspect sources | Overture Maps Explorer, OSM/Overpass, Google Earth Engine catalogues, Cesium ion | Global conflated data with per-source licence fields; Earth Engine catalogues. | They are data products/viewers, not reconstruction projects with a reproducible job record, correction loop and confidence model. |
| Reconstruct from imagery | COLMAP (BSD-3), AliceVision/Meshroom (MPL-2.0), OpenDroneMap (AGPL-3.0), MicMac (CeCILL-B), Nerfstudio/gsplat (Apache-2.0 but CUDA-only) | SfM/MVS → dense → mesh; ODM adds georeferenced DEM/orthophoto; splat pipelines render novel views. | No provenance-aware world IR, no per-object sourced/inferred labels, no target-engine quality gate, no cancellation API (verified evidence gap), splats are not editable meshes. |
| Build from OSM data | OSM2World (MIT per LICENSE; **conflict:** OSM Wiki says LGPL), map3d/Mapforge (MIT), osm2threejs (MIT, very new), gen-map (source/licence unverifiable), Blosm (GPL, Blender-only) | 250+ OSM tags, roofs, roads, street furniture; several export GLB. | OSM-derived outputs inherit ODbL duties; none advertises per-asset provenance or validation; OSM2World CLI/API described as unstable and elevation handling fragile; no selective regeneration contract. |
| Height / facade evidence | Microsoft GlobalML footprints (CDLA Permissive 2.0, partial measured height), Google Open Buildings v3 + 2.5D (CC BY 4.0 or ODbL; height evidence, model private), Li et al. 2023 / shadow / SAR research | Footprints at global scale; some height with confidence/vintage. | Google's own documentation warns confidence is uncalibrated and height accuracy weaker in the Global South; no tool combines these into one editable world with sourced-vs-inferred flags. |
| Procedural rules | CityEngine/CGA + PRT SDK (proprietary; redistribution restrictions), Houdini/HDA/PDG (proprietary), Blender Geometry Nodes (GPL) | Mature rule/grammar authoring, instancing, deterministic node graphs. | Licensing and embedding cost make them unsuitable as the required core; baked data is not guaranteed portable across versions (Blender); no verified mature incremental-rebuild contract. |
| AI-assisted extraction | SAM (Apache-2.0 weights), YOLOX (Apache-2.0), Depth Anything V2 Small (Apache-2.0), VGGT (AUP + gated commercial checkpoint), Ultralytics YOLO (AGPL/paid), Hunyuan3D 2 (territory exclusions) | Strong segmentation/depth/geometry research. | Code and weight licences diverge; most "impressive" 3D generators are research-only or CUDA/Linux-only; no mature AI adapter contract with model identity, checksum, licence and cancellation. |
| Edit / finish / export | Blender (GPL tool, output not covered by GPL), Open3D (MIT), meshlab/pymeshlab (GPL), glTF/GLB, USD, 3D Tiles | Capable mesh cleanup, UV/bake, export. | Round-trip metadata/provenance preservation is not a documented feature of these pipelines; external GPL tools must stay behind a process boundary. |
| Hosted reality capture | ArcGIS Reality, Bentley iTwin (Windows-only), RealityScan ($1,250/seat above revenue threshold), Cesium ion, Google Maps/Photorealistic 3D Tiles | Excellent visual/survey output, cloud scaling. | Proprietary; terms restrict offline use, derivation and AI training (Google ToS §3.2.3(a)–(c); Cesium ion ToS §§2.2.2, 2.2.4(xiii), 2.2.5). These are hard rights blockers for Genesis's core workflow. |

## 2. Real-workflow comparison (what a user actually tries to do)

1. **"Reconstruct my neighbourhood from data I am allowed to use, entirely offline."**
   - Hosted tools fail on rights or offline terms; OSM2World/map3d can do OSM extrusion offline but cannot fuse Microsoft/Google footprints, terrain, licensed imagery or validation. Genesis's differentiator is *lawful multi-source fusion with provenance*, not raw extrusion.
2. **"Show me which parts are measured, estimated and guessed."**
   - No reviewed open tool exposes per-asset `sourced / derived-with-confidence / inferred-by-rule` with model/tool identity and licence. This is the strongest evidence-backed gap.
3. **"Change one building and regenerate only that part."**
   - Meshroom has node cache invalidation; Blender has bake nodes (not portable across versions); osm2streets lists edit preservation as *planned*. No mature, verified incremental contract exists in the surveyed ecosystem — Genesis must design it, and that design is a research/design risk, not a solved problem.
4. **"Export a game-ready asset for my target engine."**
   - glTF/GLB exporters are common; collision, LOD chains, chunk budgets and provenance retention are not. This is where the "provenance-first game-ready export" positioning is defensible.
5. **"Benchmark against alternatives on identical data."**
   - Existing projects publish quality claims on their own datasets; none provides a reproducible same-region, same-input comparison with explicit ground-truth gaps. A benchmark-first Genesis study is itself a differentiator.

## 3. Differentiation claims that survive evidence review

| Claim | Evidence | Caveat |
| --- | --- | --- |
| Provenance as a first-class artefact | No reviewed open map-to-3D tool advertises per-asset source/licence/inference records; commercial tools (GeoCity Forge, Scope City) market provenance, showing demand. | Must be implemented, not just documented. |
| Lawful multi-source fusion | Overture conflates OSM + Microsoft + Google footprints but is a data product; no open tool reviewed combines sources with one validation/correction loop. | ODbL share-alike still attaches to Overture buildings/base/transportation themes. |
| Honest uncertainty | Google Open Buildings 2.5D warns its confidence is uncalibrated; OSM docs say `lanes` may be a minimum; a footprint implies no height/roof/facade. | Requires a schema that stores these distinctions. |
| Local-first with explicit lawful-source boundaries | Google Maps and Cesium ion terms prohibit offline/derivative/AI-training use; an open engine with a "lawful source" adapter boundary is a structural advantage. | Only if enforced in code, not merely policy text. |
| Editable game-ready output vs splat vendors | Splat pipelines produce view-dependent radiance without topology/UVs; conversion to mesh is lossy and licence-restricted (SuGaR/2DGS non-commercial). | Genesis must not promise splat fidelity in a conventional mesh. |

## 4. Benchmark design (spec Part X) — proposed, not approved

**Study area:** a small bounded section of Ikeja, Lagos, ~500 m–1 km across. **The exact boundary must be selected and recorded by the owner** (spec §49). Without a fixed, recorded boundary and frozen input revisions, no comparison is objective.

**Candidates (same boundary, same frozen inputs only):** Genesis prototype; OSM2World; map3d/Mapforge; osm2threejs; a photogrammetry pipeline only where lawful source imagery with matching coverage exists. Do not compare outputs derived from different source data as if they were equivalent.

**Measures:** geographic alignment; road coverage/connectivity; building footprint coverage; height error only where reference measurements exist; geometry validity; visual quality at matched viewpoints; texture coverage/resolution; export compatibility; runtime and peak memory; asset/triangle/file counts; effort for editing and selective regeneration.

**Ground truth limitation:** unless independent survey or LiDAR reference data is supplied for the study area, the benchmark can establish completeness and structural quality only — **not real-world accuracy**. This limitation must appear in any published result.

## 5. What the research could not establish (do not promote to fact)

- Global OSM building completeness rates; exact CityEngine pricing/current OS matrix; gen-map source licence; exact 2DGS licence text; True for Microsoft road-detection code release status.
- Independent comparisons of converted splat meshes vs native photogrammetric meshes (only paper claims found).
- Cancellation/timeouts for any reconstruction tool (no documented API found).
- Whether a specific rendered GLB is legally a Produced Work or part of an ODbL Derivative Database — needs legal review, not just licence reading.
- Vendor EULAs for Esri/Bentley/SideFX/RealityScan concerning generated assets and embedding.

## 6. Consequence for scope

The competitive evidence argues for a first vertical slice that is **provenance-complete but capability-narrow**: one small OSM/footprint-based area, roads + simple buildings, World IR, viewport inspection, validation and GLB export. Advanced photogrammetry, AI facade reconstruction, cloud workers and full-city pipelines are not justified until the foundational architecture is verified. This is a recommendation for the owner's MVP decision (D-013), not a decision made here.
