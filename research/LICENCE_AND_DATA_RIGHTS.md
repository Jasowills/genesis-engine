# Genesis Engine — Licence and Data Rights

**Research date:** 2026-11-27. **Evidence base:** `research/notes/competitive-data-rights.md`, `research/notes/ai-ml.md`, `research/notes/reconstruction.md`, `research/notes/procedural.md`. Every claim here is traceable to those primary-source records. **Not legal advice.** This is an engineering risk record, not a licence selection (D-014 remains open pending owner approval).

## 1. The three questions that must never be collapsed

| Question | Governs | Examples found |
| --- | --- | --- |
| Tool / source-code licence | Whether Genesis can vendor, link, modify, redistribute the software | CesiumJS + cesium-unreal Apache-2.0; OSM2World MIT (conflict: OSM Wiki says LGPL); Open3D MIT; COLMAP BSD-3; gsplat/Nerfstudio Apache-2.0; AliceVision/Meshroom/OpenMVG MPL-2.0; OpenMVS/ODM/Open3DSG AGPL-3.0; Blender/MeshLab GPL; Houdini/Esri/Bentley/RealityScan proprietary |
| Dataset licence | Geometry, tags, derived databases | OSM, Microsoft legacy footprints and roads ODbL; Microsoft GlobalML + Overture places CDLA Permissive 2.0 (Foursquare places Apache-2.0 with NOTICE); Google Open Buildings CC BY 4.0 **or** ODbL (user elects); Mapillary imagery generally CC BY-SA |
| Model / weight licence | Inference artefacts, commercial use, attribution | SAM Apache-2.0 weights; Depth Anything V2 Small Apache-2.0 but Base/Large/Giant CC-BY-NC; Ultralytics YOLO AGPL-3.0 or paid; DUSt3R CC BY-NC-SA; Hunyuan3D 2 bespoke territory/MAU/AUP terms; VGGT original checkpoint non-commercial, commercial checkpoint gated; SA-1B research-only |

An open tool does **not** make its inputs or model weights open. No usable public building/facade model weights were found for the footprint datasets — the datasets publish derived polygons only.

## 2. Hard rights blockers for the core workflow

| Source | Primary evidence | Restriction |
| --- | --- | --- |
| Google Maps Platform / Photorealistic 3D Tiles / Street View | `https://cloud.google.com/maps-platform/terms` §3.2.3(a)–(c); `https://developers.google.com/maps/documentation/tile/policies` | No scraping, no caching beyond permission, **no creating content from Google Maps Content** (explicitly: no 3D buildings from 45° imagery, no Street View-derived indexes), and **no use to train/test/validate/fine-tune AI models**. Map Tiles policies prohibit extracting/tracing 3D objects from Photorealistic 3D Tiles. → **Reject as reconstruction input** (D-new, owner to record). |
| Cesium ion content | `https://cesium.com/legal/terms-of-service/` (major update 2025-08-20); global 3D content ToS | No offline storage/redistribution except limited Clips; logo required; no AI training without express permission (§2.2.4(xiii)); no competing apps (§2.2.5). CesiumJS/3D Tiles *software* is Apache-2.0 and fine to wrap. → **Reject ion-hosted content as input**; use open standards with Genesis-owned content. |
| Non-commercial reconstruction research | INRIA/MPII 3DGS licence (also SuGaR, 2DGS): research/evaluation only, commercial use needs written consent | Do not bundle or derive a commercial product from these implementations. |
| Research-only AI weights | DUSt3R CC BY-NC-SA; DA-V2 Base/Large/Giant CC-BY-NC; Hunyuan3D 2 territorial exclusions + 1M MAU + AUP + training ban; Ultralytics YOLO AGPL or paid | Default MVP must use Apache/MIT/BSD-clean models only (SAM, YOLOX, DA-V2 Small). |
| AGPL tools | OpenMVS, ODM, Open3DSG | May be invoked as separate external processes; legal review required before bundling/linking. OpenMVS additionally bundles IBFS marked "research purposes only". |

## 3. Obligations that shape the World IR and export contracts

1. **ODbL share-alike** applies to a *Derivative Database* publicly used (§4.4), not merely to a rendered Produced Work (§4.5(b)); if a Produced Work is made from a Derivative Database and publicly used, share-alike is triggered (§4.4(c)); §4.6 requires offering the whole derivative database or a machine-readable alteration file. OSMF guidance (updated 2026-09-10) covers databases, ML models and computer games/simulations. **Engineering consequence:** keep structured derived data separable from rendered assets, and carry attribution + licence offer through export. The legal boundary for a generated GLB is unverified and needs review.
2. **Attribution must survive export** — `© OpenStreetMap contributors` (plus Overture where used), licence URL, election for dual-licensed Google data, dataset vintage, confidence thresholds, and per-source fields. Overture `sources/license` fields must be preserved per record.
3. **Code/weight notices** — Apache-2.0 and BSD-3 require retaining notices; Hunyuan3D requires a specific NOTICE and non-endorsement statement; CDLA Permissive 2.0 §2.1 requires making the agreement text available with shared Data, while §3.1 leaves Results (including ML models) unrestricted.
4. **Imagery is not bundled** — Microsoft and Google release derived footprints, not the source imagery. Any image fixture needs its own licence. Mapillary imagery is generally CC BY-SA and needs per-dataset verification.
5. **Secrets** — never place tokens, keys or credentials in source, prompts, logs, tickets or commits.

## 3a. Geospatial licence obligations (R-01)

- **OSM ODbL 1.0** (verified primary text): §4.2 notices when publicly conveying a database; §4.3 notice for public use of a Produced Work; §4.4 share-alike for a publicly used Derivative Database; §4.6 **must offer recipients a machine-readable copy of the whole derivative database or a file containing all alterations**; §4.5 internal use is exempt and creating a Produced Work does not itself create a Derivative Database. This is the highest-risk obligation for distributable Genesis project files. The legal boundary for generated scene files needs review.
- **OSMF attribution safe harbour** (guidelines edited 2026-09-10): databases → attribution + ODbL text/link in metadata/README; games/simulations → splash screen, in-game view, credits or menu; "OpenStreetMap" linked to `openstreetmap.org/copyright` satisfies both credit and licence-link requirements.
- **OSMF API policy:** the editing API is not for read-only/bulk use; heavy users must use planet.osm/bulk mirrors; max 2 download threads, valid User-Agent, access can be withdrawn. Use local extracts and diff replay.
- **GDAL:** MIT-style core source licence with some BSD files, but **shipped binaries may include LGPL/GPL/proprietary drivers** — `gdal --license` must be audited for the exact build on each OS.
- **PROJ:** X/MIT; PROJ-data grids are permissive but some carry CC BY-SA — check `copyright_and_licenses.csv` for grids shipped.
- **GEOS:** LGPL-2.1 — dynamic linking preferred; static linking (including `geos` crate `static` feature bundling 3.14.1) triggers relink/source-offer obligations.
- **PostGIS:** GPLv2 server; official FAQ confirms ordinary network client use does **not** require the client to be GPL — only distributing a modified PostGIS does.
- **osm2pgsql GPL-2.0 / libosmium BSL-1.0 / osmpbf MIT-or-Apache:** keep OSM→GIS conversion tools external to the MIT/Apache core.
- **Standards:** OGC COG 1.0 royalty-free to implement but requires retained notices, modification notice, no patent grant; GeoJSON RFC 7946 IETF standard; GeoParquet is an *incubating* OGC standard (spec repo Apache-2.0, site CC BY 4.0).
- **quantized-mesh and 3D Tiles repository licences are UNVERIFIED** (no LICENSE file in repo roots, checked 2026-11-27); implementing from the OGC standards is a different question from deriving code.

## 4. Required provenance record (evidence-derived, not an approved schema)

Source identity + publisher + URL + version; licence elected + URI; exact attribution string; vintage/temporal scope; spatial scope and filters; full derivation chain with tool versions; **observed vs inferred flags** (footprint known / height measured / estimated / roof inferred / facade observed / procedural / AI-generated); model identity, version, checksum, licence, inference settings, device; derivative-database status; UI/display obligations that survive export; third-party overlay terms; timestamp, source checksum and content hash of the asset bundle.

## 5. comparatively safe small fixtures (subject to legal review)

- Microsoft GlobalMLBuildingFootprints — CDLA Permissive 2.0; pin a dated index snapshot; keep the licence text; note the README "CLDA" typo and 1.4B vs 999M inconsistency.
- Google Open Buildings v3 / 2.5D elected under **CC BY 4.0** — record the election; small cut-outs only; do not bundle imagery.
- Overture `places` records with `source=Foursquare` — Apache-2.0 with NOTICE; non-Foursquare places CDLA. **Do not** bundle ODbL buildings/base/transportation unless prepared to share alike.
- OSM2World MIT (pending LGPL conflict resolution) as code reference only.
- Permissive libraries: Open3D MIT, COLMAP BSD-3, gsplat/Nerfstudio Apache-2.0, PDAL BSD.

**Do not bundle as unattributed fixtures:** raw OSM extracts, legacy MS ODbL country datasets, Cesium ion output, Google Maps/Street View/3D Tiles content, proprietary vendor samples, or large model weights.

## 6. Open decisions this creates

- **D-014** Core and plugin licence (Apache-2.0 candidate with express patent grant; MIT alternative) — owner approval required after dependency review.
- New: owner acknowledgement that Google Maps/Cesium ion content cannot be used as reconstruction input; record as a hard source constraint.
- New: what to do about ODbL-derived structured exports (licence offer + machine-readable alteration file) — legal review.
- New: licence for the plugin SDK/examples vs documentation vs fixtures vs generated sample assets vs model weights — treated as separate questions.

## 7. Unverified / thin evidence

gen-map source and licence (linked repo 404s); exact vendor EULAs (Esri, Bentley, SideFX, Epic); OSM2World default-style licence; exact 2DGS LICENSE.md text; MicMac binary-specific licence; WFC/GEOS/Shapely/Rand licence files; whether a rendered GLB is a Derivative Database in a given jurisdiction; Mapillary per-dataset licence versions; quantized-mesh/3D Tiles repo licences; `geopackage` Rust crate licence; exact GDAL binary dependency mix per OS; Rust `geotiff` maturity.
