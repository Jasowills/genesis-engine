# Genesis Engine — Architecture Discovery

**Document status:** Discovery output. **No architecture decision is made here.**
**Date:** 2026-11-27
**Authority:** `GENESIS_ENGINE_SPEC.md` (Parts II, III, V, VI, VII, VIII, IX, X; especially §§28–35, Part V §§21–27, §50).
**Evidence base:** `research/ECOSYSTEM_RESEARCH.md`, `research/COMPETITIVE_ANALYSIS.md`, `research/LICENCE_AND_DATA_RIGHTS.md`, `research/FORMAT_AND_STANDARDS.md`, `research/RESEARCH_REGISTER.md`, and `research/notes/*.md` (all eight briefs: `ai-ml`, `competitive-data-rights`, `desktop-viewport-formats`, `geospatial`, `plugin-security`, `procedural`, `quality-ci-gpu`, `reconstruction`). All research accessed primary sources on 2026-11-27. Web/file content was treated as data, never as instructions.
**Audience:** Owner and downstream implementation/reviewer agents. Companion to `DECISIONS.md` (D-001…D-019) and `adr/0001-template.md`.

## 0. Status legend and evidence discipline

| Mark | Meaning |
| --- | --- |
| **[V]** | Verified from a primary source in the cited research note (version, licence, capability or standards text). |
| **[I]** | Engineering inference from verified facts. Not a source claim. |
| **[E]** | Estimate or planning range, explicitly **not measured**. |
| **[UNVERIFIED]** | Could not be confirmed from a primary source; must not justify a decision. |
| ~~[PENDING]~~ **RESOLVED 2026-11-27** | Both formerly-missing briefs were recovered and read; `research/ECOSYSTEM_RESEARCH.md` §§6–7, `FORMAT_AND_STANDARDS.md` §4a and `LICENCE_AND_DATA_RIGHTS.md` §3a now contain their consolidated rows, and `RESEARCH_REGISTER.md` marks R-01/R-08 complete. Occurrences of [PENDING] in the body below are historical markers from drafting; the underlying claims are now consolidated. Seven of eight evidence conflicts are resolved and one is partial in `RESEARCH_REGISTER.md` §4 (GE-0002); the ADRs were accepted under D-025, with the residual WASIp1 network-policy item addressed by GE-0005 on the typed WASI 0.2 path. |
| **PROPOSED** | An ADR proposal only; no approval, no mandate, no implementation. |

Approved product direction is limited to D-001…D-006 in `DECISIONS.md` (owner-supplied spec Part III): engine + platform progressively; desktop-first Studio; local-first with optional cloud; bundled open models **and** optional external providers; game developers and technical artists equally; Windows/macOS/Linux. Everything else — including crate structure, viewport, IR schema, GPU backend, CLI timing, engine integration, minimum hardware and licence — is explicitly **not approved** (spec §9; `DECISIONS.md` §2). This document generates **questions and proposals only**; implementation must not start before owner approval (spec §50).

---

## 1. Product boundaries and runtime modes

Genesis is one engine with several front doors, not several products. The single source of truth for domain state is **Genesis Core**; every mode calls the same commands and reads the same versioned contracts.

| Mode | Responsibility | Boundary rule | Status |
| --- | --- | --- | --- |
| **Genesis Studio** (desktop) | Project creation, geographic selection, layer/generation controls, viewport inspection, lightweight correction, validation reporting, export initiation, attribution display. | Owns presentation, input and interaction state **only**. It calls Core via explicit commands/events/versioned data contracts (spec §28, §8). It must not own CRS maths, geometry/provenance semantics, job state machines or validation rules. DOM panels use ARIA tree/grid; keyboard-numeric transforms are first-class because no accessible 3D canvas pattern was found [V: `desktop-viewport-formats.md` §3]. | Approved as direction (D-002); concrete stack PROPOSED (ADR-A02). |
| **Genesis Core** (Rust library) | World IR + schema versioning, geospatial semantics, job orchestration, storage/cache contracts, validation policy, export mapping, provenance and lawful-source enforcement. Rust-first (spec §29). | Public API is the stable seam used by Studio, CLI, headless, SDK and plugins. Non-Rust specialist processing is wrapped, never embedded as a mandatory dependency; every non-Rust worker must have documented reason, stable I/O contract, version detection, install/runtime requirements, cancellation/timeout where possible, progress, structured logs/error mapping, licence/security review, and integration fixtures (spec §29). Python must not become an accidental mandatory dependency. | Approved as direction; module layout open (D-007, D-010). |
| **CLI / headless** | Same commands as Studio with no UI: project create, import, generate, validate, export, job status/cancel, reproducibility report. | Must exercise the identical Core seams as Studio (spec §8 consequence 6; §50). Cannot depend on a running desktop shell or on viewport state. | Existence progressive; release timing open (D-008). |
| **SDK / integration API** | Documented Rust API + versioned data contracts for embedders, external tools and first-party adapters. | Semantic versioning of commands, events, IR schema, job records and AI/plugin contracts; no private Studio types in the public surface. | Progressive (spec §3, D-007). |
| **Optional cloud** | Remote compute, model delivery and (later) collaboration as **adapters/providers**, never a gate for basic use (spec §5 principles 5–6, §8). | Local/remote execution is a job parameter, not a different pipeline. Remote workers treated as hostile: authenticated channel, per-job isolation, attested binaries, audit of inputs/outputs [I from `plugin-security.md` §3; the full remote-worker threat model is UNVERIFIED]. | Open (D-015, D-016). |

**Hard boundary rule (non-negotiable in any proposal):** the UI must communicate with the engine through explicit commands, events and versioned data contracts; avoid embedding core business logic in React components (spec §28). The first vertical slice must use the same architecture later SDK/CLI/headless workflows will use (spec §8). If a capability cannot be driven headlessly, it is not yet a Core capability.

---

## 2. Component map: responsibilities and explicit seams

This is the candidate shape from spec §28, refined with research. Component names are logical, **not** a crate layout (spec §42 warns against scaffolding empty crates).

| # | Component | Responsibility | Owns (authoritative) | Explicit seam to other components | Evidence / status |
| --- | --- | --- | --- | --- | --- |
| C1 | **World IR + schema versioning** | Engine-agnostic, versioned description of worlds: stable world/layer/object/asset IDs, CRS + epoch + vertical datum + units + up-axis, local frame origin, geometry/material/collision/LOD references, hierarchy and dependencies, source feature IDs, licence/attribution, observed-vs-inferred flags, method/model/tool versions, confidence, seed/settings, content hashes, validation state, migrations. Structured metadata is split from binary geometry, textures, indexes and manifests — never one giant JSON with inline geometry (spec §30). | Domain identity and provenance semantics; schema version and migration rules. | **S1** in/out: all components read/write through versioned IR documents + binary payload references. Export adapters map IR→target; validation writes results back to IR; storage persists it. | Need [V: `FORMAT_AND_STANDARDS.md` §1–2]; concrete schema [PROPOSED, ADR-A01; D-010]. |
| C2 | **Geospatial adapters** | Import/clip/reproject/repair sources (OSM/PBF, footprints, DEM/COG, vector tiles); CRS/epoch/datum handling; antimeridian policy; tiled streaming; in-process geometry ops with GEOS fallback. | Source identity, CRS metadata, spatial filters, licence election of datasets. | **S2**: `SourceDescriptor` → normalised features + provenance record → C1/C3. TileSource abstraction over GDAL VSI/COG/FlatGeobuf/GeoParquet. | [V: `geospatial.md`] GDAL 3.13.3 (2026-08-18), `gdal` 0.19.0 / `gdal-sys` 0.12.0 / `gdal-src` 0.3.0+3.12.1 (**non-standard licence, manual review**), PROJ 9.9.0 / `proj` 0.32.0 / `proj-sys` 0.28.0, GEOS 3.15.1 LGPL-2.1 / `geos` 11.3.1 (static bundles GEOS 3.14.1), `geo` 0.33.1 + `geo-types` 0.7.21, `geozero` 0.15.1, `rstar` 0.14.0 (MSRV 1.90), `geographiclib-rs` 0.2.7, PostGIS 3.7.0, GeoPackage 1.4.0, FlatGeobuf crate 6.0.1, GeoParquet 2.0.0 (`geoparquet`/`geoarrow` 0.10.0, MSRV 1.95). **[PENDING]** consolidation of geospatial rows into aggregate docs. |
| C3 | **Reconstruction worker adapters** | Run staged external workers (SfM/pose → dense/fusion → mesh/texture → optional splat branch → validated mesh proxy) and internal procedural generators. Version-detect, supervise, normalise progress, map errors. | Worker capability matrix, per-stage checkpoints, tool identity/version and licence tier. | **S3**: worker descriptor + argv + input manifest → subprocess/pool → normalised events + output manifest → C1. | [V: `reconstruction.md`] COLMAP 4.2.1 BSD-3, OpenMVS 2.4.0 AGPL, OpenDroneMap AGPL-3.0, Meshroom/AliceVision MPL-2.0, Open3D 0.20.0 MIT, PDAL BSD, Blender GPL tool/process boundary. No documented cancellation API found [UNVERIFIED negative]; progress heterogeneous → normalisation needed [I]. OSM2World MIT per LICENSE **conflict** with OSM Wiki LGPL; `osm2streets` Apache-2.0, API unstable, edit preservation *planned*. |
| C4 | **AI adapter contract / runtime** | Swap local or external providers behind one contract: identity, model revision/checksum, code vs weight licence, typed I/O schema, device/EP priority, inference settings, cancellation, staged progress, canonical error codes, per-run evidence record. | Model/provider posture (commercial use, territory, attribution). | **S4**: capability request + typed tensors → adapter → typed results + provenance → C1/C5. Runtime selection (ORT/Candle/external) is an adapter detail. | [V: `ai-ml.md`] ONNX Runtime 1.32.0 MIT (2026-11-13), `RunOptions::SetTerminate()`, CoreML EP **preview**, DirectML EP sustained; Candle MIT/Apache-2.0; SAM ViT-B Apache-2.0 weights; YOLOX Apache-2.0; Depth Anything V2 **Small** Apache-2.0 (Base/Large/Giant CC-BY-NC-4.0); LaMa Apache-2.0 code / weight licence UNVERIFIED. |
| C5 | **Job orchestration & scheduling** | Explicit inspectable jobs: type/version, input refs + hashes, params + seed, resources, local/remote choice, dependencies, stages, progress, cancellation, retry, checkpoints, logs, output manifest, validation results, reproducibility metadata; selective regeneration and caching (spec §31). | Job graph state, cancellation tokens, resource budgets, cache keys. | **S5**: command → job record → S2/S3/S4/S7 dispatch → events → C6 (persist) and C10 (UI). | Spec §31; [I] no mature incremental-rebuild contract found in surveyed ecosystem (`COMPETITIVE_ANALYSIS.md` §2.3; `procedural.md` §4.2). |
| C6 | **Project storage & asset cache** | Separate project metadata, world/layer manifests, binary geometry, textures/materials, source refs or permitted local copies, intermediate artefacts, model caches, job records/logs, validation reports, exports. Content addressing/dedup measured, not assumed; eviction and user-visible disk controls; project portable, never only in a hidden global cache (spec §32). | Project layout and migration/atomic write policy. | **S6**: content-addressed blob store + manifests; C1 serialises through S6; C5 logs through S6. | Spec §32; [V] FlatGeobuf/COG/GeoPackage suitability in `geospatial.md` §3.5; [UNVERIFIED] exact project-file layout. |
| C7 | **Quality / validation** | Six dimensions from spec §26 (geographic, geometric, visual, structural, runtime readiness, evidence quality), quality profiles (Preview / Game-ready / High fidelity / Survey-reference, provisional), gated checks, metric records with reference data + limitations. | Validation rule sets and pass/fail gates; "no accuracy claim" default. | **S7**: IR + target profile → metric records → IR validation state + export gate. | **[PENDING]** consolidation: `quality-ci-gpu.md` gives ASPRS Ed2 V2 (2024) RMSE-only, ≥30 independent checkpoints ≥2× accurate, NVA/VVA split; ISO 19157-1:2023 defines no thresholds; glTF-Validator Apache-2.0; `trimesh` 5.2.2 licence UNVERIFIED; Rust `trimesh` 0.5.2 is an ODE wrapper, **not** mesh QA; PyMeshLab licence UNVERIFIED. |
| C8 | **Export & integration adapters** | Map IR → glTF 2.0/GLB first, then researched targets (Blender, Unreal, Unity, Godot, USD, 3D Tiles, OpenDRIVE). Preserve hierarchy, materials, collision/LOD, coordinate conversion, metadata/provenance; no support claim without automated/documented compatibility test (spec §27). | Extension allowlist and round-trip conformance fixtures. | **S8**: IR + profile → target bundle + validation report (C7) + attribution sidecar. | [V: `FORMAT_AND_STANDARDS.md`] glTF 2.0/2.0.1 with `KHR_gaussian_splatting` ratified; 3D Tiles 1.1 OGC 22-025r4; OpenUSD v26.11.1 under **Tomorrow Open Source Technology License 1.0** (Apache-2.0 with modified trademark §6; GitHub NOASSERTION); FBX SDK licence UNVERIFIED. |
| C9 | **Plugin SDK & runtime** | Manifest/semver, identity/publisher, host ABI version, declared capabilities, typed I/O, resource limits, cancellation, logs/progress, sandbox, signing/trust, dependency checks, uninstall/rollback. No implicit filesystem/network/process/credential access (spec §33). | Trust policy and capability grants. | **S9**: WIT world + bounded host functions → Wasmtime/WASI instance → events/results. | [V: `plugin-security.md`] Wasmtime 51.0.0 (2026-11-20), Apache-2.0 WITH LLVM-exception, WASI 0.2 stable preview (WASI 0.3 released 2026-06-11 but Wasmtime `p3` experimental/unstable), `wit-bindgen` 0.63.0. No standard manifest/install/rollback protocol found [UNVERIFIED]. |
| C10 | **Desktop shell + viewport** | Windowing, packaging/updater/signing, DOM hierarchy/inspector panels, 3D viewport, picking/selection/gizmos/numeric transforms, capability report. Renders IR-derived scene; never computes domain truth. | UI interaction state and GPU resources only. | **S10**: commands → C5/C1; versioned events/snapshots → viewport. | [V] Tauri 2.12.7 core / CLI 2.12.6 / bundler 2.10.6 (Apache-2.0/MIT) with verified Win/macOS/Linux updater; React 19.3.1 (MIT), TypeScript 7.0.2 (Apache-2.0); three.js r187 / 0.187.0 (MIT) + R3F 9.8.2 / drei 10.7.9; Babylon 9.38.0 (Apache-2.0) alternative. Webview variance WebView2/WKWebView/WebKitGTK; no accessible canvas pattern found. |
| C11 | **CLI (progressively)** | Headless command surface, machine-readable output, job control, batch/benchmark harness. | Same command contract as Studio. | **S11**: argv → Core commands → stdout/structured events + files; no UI dependency. | Spec §3/§8/§28; D-007/D-008 open. |
| C12 | **Lawful-source boundary & provenance service** (cross-cutting) | Enforce allowed inputs, reject rights-blocked sources, capture licence elections/attribution/vintage/confidence/derivation chain through IR and export. | Source allow/deny list and required provenance fields. | **S12**: consulted by C2/C3/C4/C8/C9 before any import, inference or export. | [V: `LICENCE_AND_DATA_RIGHTS.md`] OSM ODbL 1.0 §4.4/§4.6; Google Maps/Photorealistic 3D Tiles ToS §3.2.3(a)–(c) and Map Tiles policies prohibit derivation/caching/AI training; Cesium ion ToS (major update 2025-08-20) §§2.2.2, 2.2.3, 2.2.4(xiii), 2.2.5; WAIVER of these is not evidenced. |

### 2.1 Seam contract inventory (explicit boundaries)

- **S1 World IR:** versioned document + referenced binary payloads; migrations forward/backward; stable IDs and content hashes; no raw geometry in metadata JSON.
- **S2 Geospatial:** `SourceDescriptor{id,publisher,url,version,licence_elected,attribution,vintage,filters,checksum}` → normalised features + CRS/epoch/datum metadata; `TileSource` for COG/FlatGeobuf/GeoParquet/GDAL VSI; antimeridian cut/unwrap policy explicit.
- **S3 Worker:** argv-only, scrubbed env, version probe, input/output manifests, staged progress events, cancellation/timeout, structured errors; separate process for copyleft tools.
- **S4 AI:** typed named tensors + prompt schema, identity/checksum/licence, device/EP priority, cancellation (`SetTerminate` for ORT), canonical error codes, per-run evidence.
- **S5 Job:** versioned record with dependencies, resources, stages, retries, checkpoints, output manifest, reproducibility fields; bounded channels/queues (spec §46).
- **S6 Storage:** manifests + content-addressed blobs; atomic promotion; cache separate from portable project state.
- **S7 Validation:** metric record `{value, unit, reference_dataset, crs/datum, method, tool_version, params, seed, timestamp, limitations}`; gates per profile.
- **S8 Export:** target version + extension allowlist + metadata profile + round-trip fixture; attribution survives.
- **S9 Plugin:** versioned WIT world, deny-by-default capabilities, epoch interruption + wall-clock timeout, explicit memory/table/instance caps, bounded log/progress hostcalls, signed manifest + atomic activation (manifest design is **[I]**, no source-found standard).
- **S10 UI↔Core:** commands/events/versioned snapshots only; keyboard-numeric alternative for every viewport gizmo.
- **S11 CLI:** same S10 command schema, machine-readable output.
- **S12 Lawful source:** allow/deny gate + provenance schema across all import/inference/export paths.

---

## 3. Data-flow diagrams (ASCII)

### 3.1 Import → reconstruct → IR → validate → export

```
 [Source catalogue]                 [Lawful-source gate S12]
 OSM/PBF | MS GlobalML (CDLA 2.0) |   deny: Google Maps/Street View/3D Tiles content,
 OSM footprints (ODbL) | DEM/COG            Cesium ion content, non-commercial weights
 licensed user imagery            allow: recorded licence election + attribution + vintage
        |                                  |
        +----------> S2 geospatial adapters (GDAL 3.13.3 / PROJ 9.9.0 /
                       GEOS 3.15.1 dynamic / geo 0.33.1 / geozero 0.15.1)
                       clip - reproject - epoch/datum - repair - tile        [PENDING consolidation]
                                  |
                    normalised features + provenance record
                                  |
              +-------------------+--------------------+
              |                                        |
        S3 worker adapters                       S4 AI adapter
        (COLMAP 4.2.1 BSD-3;                     (ORT 1.32.0 MIT; SAM ViT-B;
         OpenMVS 2.4.0 AGPL as                    DA-V2 Small; YOLOX)
         external process only;                   typed I/O + licence/checksum
         Open3D 0.20.0 MIT;                       + cancellation
         procedural generators)                           |
              |                                        |
              +------------ staged artefacts ----------+
                                  |
                          S1 World IR (versioned)
        stable IDs | CRS+epoch+datum | ENU/local frame origin | sourced/inferred flags
        geometry refs | collision/LOD | seed/settings | hashes | validation state
                                  |
                    S6 storage: manifests + content-addressed blobs
                                  |
                          S7 quality/validation
        [PENDING] format gate (glTF-Validator), structural gate,
        geometric gate, geographic gate ONLY with independent checkpoints
        (ASPRS Ed2 V2: RMSE, >=30 checkpoints, NVA/VVA), budget + provenance gates
                                  |
                    S8 export adapters (glTF 2.0/GLB first)
                    metadata + attribution sidecar; 3D Tiles 1.1 / USD later
                                  |
                   target tool re-import (compatibility fixture)
```

### 3.2 Job execution with progress and cancellation

```
 Studio/CLI/SDK command
        |
        v
 S5 job graph: {type+version, input hashes, params+seed, resources,
                local|remote, deps, stages, retry, checkpoint policy}
        |
        +--> resource preflight [E, unmeasured]: RAM/VRAM/disk range, paid-cost consent
        |
        v
 scheduler picks adapter: S2 in-process | S3 process | S4 AI | S9 plugin | remote worker
        |
        +------------------- normalised events S5 --------------------+
        |  stage_started{id,expected_steps} | progress{stage,0..1,units}
        |  log{level,msg,redacted} | checkpoint{id,hash} | warning{code}
        |  error{canonical_code,provider_error,backend,model_revision}
        |  done{output_manifest,validation_ref} | cancelled{stage,reason}
        +--> S6 job record (append-only) --> S10 Studio progress UI / S11 CLI stdout
        |
   cancellation path:
        cancel request --> token --> S3: kill process tree (Job Objects/seccomp;
                                     seccomp alone is NOT a sandbox)
                                --> S4: ORT RunOptions::SetTerminate()
                                --> S9: epoch increment + tokio timeout
                                --> S2: cooperative check between tiles
        |
        v
 checkpoint resume / retry policy --> output manifest --> S7 validation --> S1 IR
```

### 3.3 Plugin invocation through WIT/WASI

```
 signed plugin manifest {id, publisher, plugin semver, host-ABI semver,
                         SHA-256 digest, capability grants, dependency pins}   [design is INFERENCE]
        |
        v
 install: verify digest/attestation --> inspect component imports (wasm-tools)
          --> reject unapproved capabilities --> immutable version dir
          --> atomic activation pointer (retain previous version for rollback)
        |
        v
 invoke: Core S9 host
          Engine + Store per instance + Linker (Wasmtime 51.0.0, Apache-2.0 WITH LLVM-exception)
          WASI 0.2 component, only required interfaces linked
          NO inherit_env, NO inherit_stdio, zero preopens, network explicitly denied
          StoreLimits: explicit memory/table/instance caps; WASI I/O + random caps
          epoch interruption + tokio::time::timeout (does NOT cover blocking hostcalls)
        |
        v
 typed WIT world: inputs (IR subdocument + bounded buffers)
        |
        v
 guest component --> bounded host functions: log(), progress(), read_input(), emit_result()
        |
        v
 results validate against schema --> write to S1 IR via Core
 errors map to canonical codes --> S5 job events --> S10 UI
 rollback: repoint activation to previous immutable version; quarantine failing digest
```

---

## 4. Failure modes, bottlenecks, hard-to-reverse choices, platform risks

Each item ties to evidence; none is an approved decision.

| Risk | Class | Evidence and consequence |
| --- | --- | --- |
| **Large-coordinate precision loss** | Failure + hard-to-reverse | glTF core POSITION is float32; derived ULP is ~0.5 m at Earth radius 6.37e6 m and ~1 m at 1e7 m. f64 vertex attributes need draft `KHR_accessor_float64`; glTF 2.1 enums do not lift vertex restrictions. `geo` 0.33.1 far-from-origin fixes are only in **unreleased** `main` (needle-triangle area 0, `closest_point` overflow, fixed `1e-4` snap radius); i_overlay panics fixed in 9.2 unreleased here. Must be designed into IR from day one; cannot be bolted on later [V + PENDING consolidation: `FORMAT_AND_STANDARDS.md` §2, `geospatial.md` §3.2]. |
| **Origin rebasing consistency** | Failure | Switching local origin must stay consistent across geometry, terrain, picking, undo/physics and metadata; otherwise silent offsets. Requires golden regression tests [V: `desktop-viewport-formats.md` §6; `geospatial.md` §3.2]. |
| **Antimeridian / CRS / epoch ambiguity** | Failure | RFC 7946 §3.1.9 only SHOULD-cut; bbox east<west; GeoJSON Z is ellipsoidal, ~2–5 m from mean sea level (§11.2); PROJ `+over` fails for many projections; PostGIS `geography` has "no conception of dateline or poles"; EPSG:4326 datum ensemble ~2 m; PROJ 9.9.0 warns on missing epoch. Needs explicit internal cut/unwrap + epoch/datum policy [V + PENDING: `geospatial.md` §§3.1, 3.3]. |
| **ODbL share-alike on structured exports** | Hard-to-reverse licence risk | ODbL §4.4 share-alike for publicly used Derivative Database; §4.4(c) triggered for Produced Work *from* a Derivative Database; §4.6 requires machine-readable whole DB or alteration file. Whether a generated GLB is a Produced Work or derivative is **[UNVERIFIED]** and needs legal review. Once exported publicly, unwind is expensive [V: `LICENCE_AND_DATA_RIGHTS.md` §3; `geospatial.md` §2.1]. |
| **Rights-blocked sources** | Hard blocker | Google Maps Platform ToS §3.2.3(a)–(c) prohibits creating content from Google Maps Content, deriving 3D buildings from 45° imagery, and training/testing/validating AI on it; Cesium ion ToS §§2.2.2, 2.2.4(xiii), 2.2.5 restrict offline use, AI training, competing apps. Must be enforced in code, not policy text [V]. |
| **Code vs weight licence divergence** | Licence risk | SAM weights Apache-2.0 but SA-1B dataset research-only; DA-V2 Small Apache-2.0 while Base/Large/Giant CC-BY-NC-4.0; Ultralytics YOLO AGPL-3.0 or paid; VGGT original checkpoint non-commercial with commercial checkpoint gated; Hunyuan3D 2 has territory exclusions + 1M MAU + AUP; LaMa weight licence UNVERIFIED. A single careless model swap changes commercial posture [V: `ai-ml.md` §3]. |
| **Copyleft worker leakage** | Licence / architecture | OpenMVS 2.4.0 and ODM AGPL-3.0; Meshroom/AliceVision MPL-2.0; Blender GPL tool (output not covered); osm2pgsql GPL-2.0; GEOS LGPL-2.1 (dynamic linking simpler; `geos` crate `static` bundles 3.14.1 and triggers relink obligations); GDAL binary builds may include LGPL/GPL/proprietary deps — audit `gdal --license`. Process boundary is the mitigation [V: `reconstruction.md` §5–6; `geospatial.md` §2.2]. |
| **No cancellation API in reconstruction tools** | Bottleneck / UX | No documented cancellation found for any surveyed tool [UNVERIFIED negative]; only process supervision + checkpoint resume evidenced. Progress formats are heterogeneous (stdout, node logs, viewer, W&B) → normalised progress adapter required [I: `reconstruction.md` §§3.4, 6]. |
| **Incremental regeneration is unsolved** | Bottleneck / hard-to-reverse | Meshroom node cache and Blender bake exist; Blender baked data not guaranteed cross-version; osm2streets edit preservation *planned*. No mature verified incremental contract found. Designing the IR dependency/IDs wrongly will make "regenerate one building" impossible without rework [V + I: `COMPETITIVE_ANALYSIS.md` §2.3; `procedural.md` §4.2]. |
| **Webview GPU variance** | Platform risk | Tauri uses WebView2/WKWebView/WebKitGTK; WebGL2/WebGPU availability differs; no verified f64 GPU buffers/shaders; Electron 45.5.0 gives consistent Chromium but has no Linux built-in updater. Requires capability PoC on all three OSes before choosing shell/viewport [V: `desktop-viewport-formats.md` §§2–4]. |
| **GPU/backend fragmentation** | Platform risk | CUDA 13.5 unsupported on macOS; ROCm 10.2 version-sensitive; MoltenVK deprecated by Khronos (KosmicKrisp recommended, maturity UNVERIFIED); Open3D SYCL experimental; wgpu v30 Apache-2.0/MIT MSRV 1.87 covers rendering, not proven for reconstruction kernels. GPU containers only credibly Linux/NVIDIA. Do not promise every backend on every OS (spec §35) [V + PENDING: `quality-ci-gpu.md` §2]. |
| **Wasmtime CVE density / capability defaults** | Security risk | 51.0.0 (2026-11-20) fixed host-affecting issues; Windows filesystem escape GHSA-f2pq-268w-886c (CVSS 7.5) affects <36.0.22, 37–48.0.9, 49.0–49.0.6, 50.0–50.0.2; host stream DoS GHSA-5m8g-xhq4-4h93 (CVSS 6.9) fixed in 51.0.0; fuel amplification CVE-2026-115390. `StoreLimitsBuilder::memory_size` has no default limit; WASI TCP default docs conflict → deny explicitly. Epoch ~10% overhead; fuel up to 2–3x in some measurements. Monthly majors + precompiled artifacts tied to exact version → recurring upgrade cost [V: `plugin-security.md` §§1, 4]. |
| **Plugin lifecycle has no standard** | Design gap | No primary source found for manifest, install transaction, dependency resolver or rollback protocol; component overhead percentage UNVERIFIED. Any proposed design is inference and needs a PoC before commitment [UNVERIFIED]. |
| **Malformed input / model files** | Security | ZIP-slip (CWE-22), decompression bombs (CWE-409), PyTorch pickle (`weights_only=True` default since 2.6 is necessary but insufficient), ONNX `external_data` path traversal (ONNX assurance case says reference implementation not production-safe on untrusted inputs; `base_dir=""` skips checks). Treat metadata as untrusted input to any AI agent (OWASP LLM01:2025) [V: `plugin-security.md` §3]. |
| **Unbounded resources / hidden fallbacks** | Reliability | Spec §46 forbids unbounded channels/queues/caches, silent fallback hiding data loss, panics for expected errors, hidden network access. ORT CoreML EP is **preview** with restrictive op tables; a CoreML→CPU fallback changes both performance and licence posture → must be recorded in the per-run evidence record [V + spec]. |
| **Unverifiable performance claims** | Bottleneck | All Genesis size/time/RAM ranges are **[E]** estimates anchored on GitHub runner specs and COLMAP's `4m²+1024m` SIFT matching formula; no independent benchmark was run. ASPRS accuracy needs ≥30 independent checkpoints ≥2× accurate; image similarity is not geographic accuracy [V + E: `quality-ci-gpu.md` §§1, 4]. |
| **Workspace/MSRV creep** | Delivery risk | `rstar` 0.14.0 MSRV 1.90; `geo` may rise to 1.90; `geoparquet`/`geoarrow` 0.10.0 MSRV 1.95; Tauri updater needs Rust ≥1.90. Choosing a low MSRV blocks geospatial format support; choosing 1.95 narrows contributor machines [V + PENDING]. |
| **Only one platform tested** | Verification gap | Host is Intel macOS only; Windows, Linux, ARM64, Apple Silicon and CUDA untested (`progress.md`, `ENVIRONMENT_REPORT.md`). All cross-platform claims are unverified locally. |

---

## 5. ADR proposals — all PROPOSED, none approved

Every proposal below follows the required fields. **Decision:** each ADR recommends a direction *for discussion*; it is not a decision. No code, crates, tickets or issues have been created.

### ADR-A01 — Thin versioned World IR with adapters vs USD-based IR

**Status:** PROPOSED. **Decision ID:** D-010 (open). **Owner approval needed:** **YES** — foundational, hard to reverse (spec §9: world interchange schema/versioning unapproved).

**Context.** Spec §30 requires an engine-agnostic, versioned IR with stable IDs, provenance, confidence, seeds, migrations, partial updates and export mapping; it forbids one giant inline-geometry JSON. glTF 2.0 is explicitly a **transmission** format, not authoring/streaming; 3D Tiles 1.1 (OGC 22-025r4) is streaming-oriented; neither carries provenance/confidence/variants. OpenUSD v26.11.1 has layers/references/variants/payloads and `PointInstancer` but comes with a heavy C++ dependency and the **Tomorrow Open Source Technology License 1.0** (Apache-2.0 with modified trademark §6; GitHub NOASSERTION) requiring legal review [V: `FORMAT_AND_STANDARDS.md` §§1, 4].

**Options considered.**
- **A — Thin purpose-built versioned IR + adapters (recommended proposal).** Small schema for identity, CRS/frame, provenance, confidence, dependencies, seeds, validation; binary geometry/textures by reference; adapters to GLB/3D Tiles/USD concepts.
- **B — USD-based IR.** Adopt OpenUSD stages/layers/variants as the internal representation; map metadata into USD attributes.
- **C — glTF/GLB as the internal format.** Keep one format for authoring and interchange.

**Recommendation (proposed):** A. Study USD *concepts* (composition, variants) but do not take the C++/licence dependency in the first slice; GLB is the first interchange export (spec §50 step 11; `COMPETITIVE_ANALYSIS.md` §6).

**Rationale.** A directly satisfies spec §30 and the provenance-first differentiator; it keeps the core Rust-first and licence-clean; B adds an unapproved trademark-modified licence and heavy dependency before the vertical slice validates the architecture; C is contradicted by the Khronos spec's own statement that glTF is not an authoring/streaming format and by lack of a shared metadata schema (`extras` is convention only) [V].

**Consequences.** Positive: stable IDs/migrations/partial regeneration are first-class; export mapping is explicit; no third-party IR licence. Negative: no ecosystem/tooling; schema design and round-trip tests are net-new work; risk of reinventing USD concepts.

**Reversibility.** Moderate: adding USD-style composition later is possible if the IR already stores relationships, but changing IR schema after projects exist requires migrations and fixture re-bakes — expensive. Mitigation: versioned schema, migration tests, small core.

**Evidence.** Spec §§16, 30; `FORMAT_AND_STANDARDS.md` §§1, 2; `notes/desktop-viewport-formats.md` §7; `ECOSYSTEM_RESEARCH.md` §2; `RESEARCH_REGISTER.md` §4 (USD/licence review not resolved).

### ADR-A02 — Tauri 2.12.7 + React 19.3.1/TypeScript 7.0.2 + three.js r187 vs alternatives

**Status:** PROPOSED. **Decision IDs:** D-011 (viewport), D-012 (GPU partly). **Owner approval needed:** **YES** — spec §17 says shell and viewport are separate unapproved decisions.

**Context.** Spec §§17, 28 require a desktop shell, React/TypeScript UI and a viewport, but explicitly leave the framework/renderer open. Research verified: Tauri core **2.12.7** / CLI **2.12.6** / bundler **2.10.6** (18 Nov 2026, Apache-2.0/MIT), updater verified on Windows/macOS/Linux with mandatory signatures (requires Rust ≥1.90); Electron **45.5.0** stable (MIT, Chromium 156) but **no Linux built-in updater**; Wails **3.1.0** (MIT, Go 1.26+). React **19.3.1** (MIT), TypeScript **7.0.2** (Apache-2.0); three.js **r187 / 0.187.0** (MIT, `TransformControls`, `BatchedMesh` batchId, LOD hysteresis, GLTF/KTX2/Draco/PLY/PCD loaders); Babylon.js **9.38.0** (Apache-2.0, documented `GizmoManager` + `snapDistance`); Bevy **0.20.1** / wgpu **30.0.2** / egui **0.37.0** are higher-cost alternates [V: `notes/desktop-viewport-formats.md` §§1–6].

**Options considered.**
- **A — Tauri + React/TS + three.js via R3F 9.8.2/drei 10.7.9 (recommended proposal).** Rust core aligns with Tauri; best editor primitive coverage; small bundles.
- **B — Electron + React/TS + three.js.** Consistent Chromium across OSes; larger binary; no Linux built-in updater.
- **C — Native-only (Bevy 0.20.1 / wgpu 30.0.2 + egui 0.37.0).** No webview variance, but no React/TS and significant editor-UX cost.
- **D — Tauri + Babylon.js 9.38.0.** Stronger built-in gizmos/snapping; smaller ecosystem.

**Recommendation (proposed):** A, gated on a PoC: WebGL2/WebGPU capability probe in all three webviews, floating-origin test at ~1e7 m, 100k-object picking/batching test, load/unload GPU-leak test, TypeScript 7 tooling check. Keep D as the fallback if gizmo/snap work dominates. Treat shell and viewport as two separate approvals.

**Rationale.** Tauri is the only shortlisted shell with a verified signed updater on all three target OSes and a native Rust bridge; three.js has the richest editor primitives and loaders. Electron's value is deterministic Chromium, but that must be weighed against bundle size and the Linux updater gap. No evidence supports a native-only Studio for the first slice [V + I].

**Consequences.** Positive: aligns with Rust-first core and React/TS UI. Negative: three OS webviews differ → GPU variance; canvas accessibility has no standard pattern → keyboard-numeric transforms must be built; TS 7 tooling compatibility unverified; viewport does not solve large-world streaming by itself.

**Reversibility.** Shell↔UI is moderately reversible if the Core seam (S10) holds and UI holds no domain logic. Viewport swap is costly if editor state or LOD policy leaks into renderer code — hence the seam requirement. Signing identities/key custody must be decided before public release (Apple Developer Program **$99/yr**; Windows Azure Artifact Signing ~**$9.99/mo**, OV **$150–300/yr**, EV **$400+/yr** [V: `quality-ci-gpu.md` §3]).

**Evidence.** Spec §§17, 28; `notes/desktop-viewport-formats.md` §§1–6; `notes/quality-ci-gpu.md` §2 (wgpu/backend matrix); `ECOSYSTEM_RESEARCH.md` §2.

### ADR-A03 — External-process worker model + normalised progress events

**Status:** PROPOSED. **Decision ID:** implementation architecture (affects packaging/licence posture). **Owner approval needed:** **YES** — process sandboxing and copyleft boundaries change distribution obligations.

**Context.** Spec §29 permits mature C++/Python/external tools behind stable adapters and lists mandatory worker requirements. Research shows mature reconstruction tools are CLI/library-first and process-oriented (COLMAP staged CLI, Meshroom node/renderfarm, ODM Docker/CLI/REST, Blender background mode); no cancellation API was found in any primary source [UNVERIFIED negative]; progress formats are heterogeneous; licences favour process separation (AGPL OpenMVS/ODM, MPL Meshroom, GPL blender) [V: `reconstruction.md` §§3.4, 5, 6].

**Options considered.**
- **A — Separate OS process per job + normalised event protocol (recommended proposal).** argv-only, scrubbed env, restricted cwd, per-job CPU/RAM/time caps, kill tree, version probe, staged progress/checkpoints, canonical errors.
- **B — In-process FFI libraries for permissive tools only.** Faster, but cannot host AGPL/MPL/GPL tools and couples crashes to Studio.
- **C — Containers/microVMs for every worker.** Strong isolation, heavy operational cost; GPU containers only credibly Linux/NVIDIA.

**Recommendation (proposed):** A for all specialist/non-Rust workers; permit B only for proven permissive libraries behind a reviewed adapter; defer C to optional cloud/headless workers. Normalise progress to `{stage, progress 0..1, units, checkpoint}` and cancellation to one token mapped per adapter (`SetTerminate` for ORT, kill-tree for processes, epoch+timeout for plugins).

**Rationale.** Process isolation is the only evidenced cross-tool pattern that simultaneously handles licensing, crashes, version detection and missing cancellation APIs; spec §29 already mandates the adapter requirements [V + I].

**Consequences.** Positive: blast-radius reduction, licence compliance, tool upgrade independence. Negative: IPC/copy overhead; per-OS sandbox profiles diverge (Windows Job Objects and Linux seccomp have primary docs; Apple sandbox specific docs are thin/UNVERIFIED); seccomp alone is **not** a sandbox; kill-tree behaviour needs tests.

**Reversibility.** Moderate-high at the seam: adapters can be replaced without changing the IR. Low if job stages/checkpoint formats are allowed to become tool-specific — enforce S3/S5 contracts.

**Evidence.** Spec §§29, 31; `reconstruction.md` §6; `plugin-security.md` §§2–3 (process sandbox limits); `ECOSYSTEM_RESEARCH.md` §3.

### ADR-A04 — ONNX Runtime 1.32.0 primary + swappable AI adapter contract

**Status:** PROPOSED. **Decision ID:** D-004 approves "open models + external providers" but **not** the runtime. **Owner approval needed:** **YES** — exact runtime and model distribution are unapproved (spec §9).

**Context.** Spec §18 mandates an AI adapter contract carrying model identity, version, checksum, I/O schema, device requirements, licence and attribution. Verified: ONNX Runtime **1.32.0** (released 2026-11-13, MIT, Production/Stable) has the broadest EP matrix and a documented cancellation API (`RunOptions::SetTerminate()`); CoreML EP is **preview** with restrictive op tables; DirectML EP is sustained; Candle is dual MIT/Apache-2.0 but hand-written per architecture; Burn (MIT/Apache-2.0) requires Rust 1.95 and is still changing. No independent cross-runtime benchmark was run; ORT's GitHub repo was unreachable in the research session (version/licence verified via PyPI/docs instead) [V + UNVERIFIED: `ai-ml.md` §1].

**Options considered.**
- **A — ORT 1.32.0 via C ABI behind the adapter (recommended proposal).** Broad hardware/EP support; cancellation; stable.
- **B — Candle as primary.** Rust-native, smaller binaries, no Python in production; model coverage per architecture, Windows linking issues documented.
- **C — Multi-runtime from day one.** Maximum flexibility, high integration/test cost.

**Recommendation (proposed):** A as default local runtime, with the full adapter contract (identity, SHA-256, code/weight licence, typed schemas, device/EP priority, settings, cancellation, canonical errors, per-run evidence). Candle stays an optional adapter where an exact model implementation exists; Burn/tract/LiteRT/ncnn are monitored. Default model shortlist (licence-clean): SAM ViT-B, YOLOX, Depth Anything V2 **Small**, LaMa (verify weights); avoid Ultralytics YOLO (AGPL-3.0/paid), DA-V2 Base/Large/Giant (CC-BY-NC-4.0), DUSt3R (CC BY-NC-SA), Hunyuan3D 2 (territory exclusions) [V].

**Rationale.** The spec forbids choosing a runtime merely because it is Rust; ORT's operator and EP coverage plus cancellation best fit a three-OS local-first Studio. The adapter keeps external providers and future runtimes swappable (D-004) without changing capabilities [V + I].

**Consequences.** Positive: commercially clean default path, explicit fallback/provenance logging. Negative: no first-class Rust API (C ABI/community crate), dynamic-shape models can lose EP acceleration, model conversion can be lossy; a CoreML→CPU fallback changes performance and licence posture and must be recorded.

**Reversibility.** High **if** no capability code imports ORT types directly. Migration means reimplementing adapters + re-running model parity fixtures, not rewriting the IR.

**Evidence.** Spec §§18, 29; `ai-ml.md` §§1–5; `ECOSYSTEM_RESEARCH.md` §2; `LICENCE_AND_DATA_RIGHTS.md` §1; `RESEARCH_REGISTER.md` §4 (repo availability conflict).

### ADR-A05 — Wasmtime/WASI 0.2 plugin sandbox

**Status:** PROPOSED. **Decision ID:** extends spec §33. **Owner approval needed:** **YES** — plugin SDK licence/trust policy is unapproved (D-014) and untrusted-code execution is a security decision.

**Context.** Spec §33 suggests Rust/WASM with Wasmtime and capability-based permissions, but requires validation. Verified: Wasmtime **51.0.0** (2026-11-20), Apache-2.0 WITH LLVM-exception, monthly majors (multiples of 12 are LTS for 24 months by policy; exact supported set UNVERIFIED); WASI 0.2 is a stable preview, WASI 0.3 released 2026-06-11 but Wasmtime `p3` is "experimental, unstable and incomplete"; Tier-1 includes x86_64 macOS/Windows/Linux, Tier-2 includes aarch64 macOS/Linux. Windows sandbox escape GHSA-f2pq-268w-886c (CVSS 7.5) fixed in 36.0.22/48.0.10/49.0.7/50.0.3; host stream DoS GHSA-5m8g-xhq4-4h93 fixed in 51.0.0; `wit-bindgen` **0.63.0**. Defaults close stdin, discard stdout/stderr, no env/args/preopens; TCP default docs conflict → deny explicitly. `StoreLimitsBuilder::memory_size` has **no default** [V: `plugin-security.md` §§1, 4].

**Options considered.**
- **A — Wasmtime 51.0.0+ / WASI 0.2 components / typed WIT world (recommended proposal).** Capability-based deny-by-default, typed ABI, cross-OS Tier-1.
- **B — Native dynamic libraries.** Highest performance; shares all host privileges, ABI-fragile, no memory/CPU limits; only for first-party signed accelerators [I].
- **C — Separate sandboxed OS process per plugin.** Strong blast-radius reduction; divergent per-OS policy and IPC cost; seccomp alone insufficient.
- **D — Defer plugins entirely past the vertical slice.**

**Recommendation (proposed):** A for untrusted third-party plugins, with ≥51.0.0 pinned (and patch monitoring), explicit WIT world, zero preopens/env/network, epoch interruption + `tokio::time::timeout` (never relying on epochs alone for blocking hostcalls), explicit Store/WASI caps, bounded log/progress hostcalls, SHA-256-pinned signed manifest and atomic activation with retained prior version. Combine with C for native/high-risk workers. Track WASI 0.3 but do not depend on it in v1.

**Rationale.** WASI has no ambient authority and Wasmtime provides the maturest Rust embedding with a typed component ABI; the CVE density is a patch-discipline requirement, not a reason to choose unsafe native loading [V + I].

**Consequences.** Positive: host cannot be trivially reached through ambient capabilities; language-neutral plugin ABI. Negative: recurring monthly-major upgrade work; precompiled artifacts tied to exact Wasmtime version; host functions are the weak point; manifest/signing/rollback are our design, not a sourced standard [UNVERIFIED]; component overhead percentage UNVERIFIED.

**Reversibility.** Moderate: WIT world versioning + semver-qualified imports allow evolution; changing host ABI forces plugin recompilation and a migration window.

**Evidence.** Spec §§33, 34; `plugin-security.md` §§1–4; `ECOSYSTEM_RESEARCH.md` §2; `RESEARCH_REGISTER.md` §4 (TCP default conflict).

### ADR-A06 — Lawful-source adapter boundary and provenance-first metadata

**Status:** PROPOSED. **Decision IDs:** new hard source constraint + D-014-adjacent legal review. **Owner approval needed:** **YES** — the owner must acknowledge blocked sources and whether OSM-derived databases may be published.

**Context.** Spec §§5, 12, 16, 26, 27 require evidence/provenance, observed-vs-inferred distinctions and respect for data rights. Research verified: OSM data ODbL 1.0 (§§4.2–4.6; attribution guidelines last edited 2026-09-10); Microsoft GlobalML footprints CDLA Permissive 2.0 (README typo "CLDA"; 1.4B intro vs 999M FAQ inconsistent); Google Open Buildings v3/2.5D electable CC BY 4.0 **or** ODbL; Overture buildings/base/transportation ODbL, places CDLA/Apache-2.0 with NOTICE; Google Maps Platform ToS §3.2.3(a)–(c) and Cesium ion ToS §§2.2.2/2.2.4(xiii)/2.2.5 prohibit the core derivation workflow [V]. No reviewed open map-to-3D tool advertises per-asset provenance [V: `COMPETITIVE_ANALYSIS.md` §3].

**Options considered.**
- **A — Enforceable allow/deny gate + provenance carried in IR and export (recommended proposal).** Each source declares licence elected, attribution, vintage, filters, checksum; C12 blocks denied sources before import/inference/export; observed vs inferred flags are mandatory fields.
- **B — Documentation-only policy.** Cheap, but terms violations would occur in code paths; contradicts spec §5 principle 15 and the differentiator analysis.
- **C — Allow all sources with user attestation.** Shifts legal risk to user; no machine-readable provenance; breaks export compliance.

**Recommendation (proposed):** A. Deny Google Maps/Street View/Photorealistic 3D Tiles content and Cesium ion-hosted content as reconstruction inputs; permit the open-standards software (CesiumJS/3D Tiles Apache-2.0) with Genesis-owned content. Record licence election per snapshot; keep structured derived data separable from rendered assets so ODbL obligations can be offered; require legal review before publishing any OSM-derived database or GLB [V + I].

**Rationale.** This is the evidence-backed product differentiator and the only way to satisfy "honest uncertainty" and "attribution survives export" simultaneously; no standard provenance vocabulary was found, so Genesis must own a profile [V: `FORMAT_AND_STANDARDS.md` §4].

**Consequences.** Positive: lawful multi-source fusion, reproducible source revisions, defensible export. Negative: blocks some high-fidelity commercial inputs; more bookkeeping; ODbL §4.6 offer/alteration-file design unresolved; legal boundary of a generated GLB **[UNVERIFIED]**.

**Reversibility.** Low once assets ship with provenance: schema changes require migration and re-export. Starting with a superset of fields is cheaper than adding later.

**Evidence.** Spec §§5, 12, 16, 18, 26, 27; `LICENCE_AND_DATA_RIGHTS.md` §§1–4; `notes/competitive-data-rights.md` §§1–3; `notes/geospatial.md` §2.1 **[PENDING]** consolidation.

### ADR-A07 — Floating-origin / local-frame precision strategy

**Status:** PROPOSED. **Decision IDs:** D-010-adjacent, D-012 partly. **Owner approval needed:** **YES** — foundational IR constraint; also requires a hardware/GPU promise decision (spec §35).

**Context.** Spec §§10, 30, 35 ask how precision loss far from origin is avoided. Verified: glTF core vertex positions are float32 (ULP ~0.5 m at Earth-radius scale, ~1 m at 1e7 m); f64 vertex attributes need draft `KHR_accessor_float64` and WebGL/WebGPU broadly lack f64 buffers/shaders; 3D Tiles uses per-tile transforms and `RTC_CENTER`, glTF has `CESIUM_RTC` (Stable); PROJ `+proj=topocentric` (since 8.0, EPSG:9836/9837) converts ECEF→ENU; `geo` 0.33.1 far-from-origin fixes are unreleased [V + PENDING: `FORMAT_AND_STANDARDS.md` §2; `geospatial.md` §§3.1–3.2].

**Options considered.**
- **A — Authoritative geodetic coordinates + per-tile/region ENU local frame + floating origin (recommended proposal).** High precision at rest; render/mesh maths relative to a centre.
- **B — Store everything in f64 global ECEF and transform at draw.** Simple conceptually; f32 GPU boundary still jitters; large transform matrices lose precision.
- **C — Wait for glTF 2.1 f64/`KHR_accessor_float64`.** Timing UNVERIFIED; would block the slice on a draft.

**Recommendation (proposed):** A. Store CRS + epoch + vertical datum + height unit and frame origin in the IR; do mesh maths and rendering in local ENU/tile frames; record rebasing invariants across picking, undo, physics and metadata; add golden fixtures at ~1e7 m, ±180 longitude and large projected eastings/northings. Do not design around glTF 2.1 f64.

**Rationale.** Industry-proven (3D Tiles/CESIUM_RTC) and the only option consistent with both the precision numbers and current GPU capability; spec §30 says the design must account for local-origin precision from the start [V + I].

**Consequences.** Positive: correct rendering at city/global scale; explicit datum/epoch model. Negative: origin switching is a cross-cutting invariant and a source of subtle bugs; tests must cover every consumer of coordinates.

**Reversibility.** Very low after projects exist — rebasing rules touch IR, jobs, exports and viewport; hence a P0 golden-test spike before the vertical slice.

**Evidence.** Spec §§10, 16, 30, 35; `FORMAT_AND_STANDARDS.md` §§2, 7; `notes/desktop-viewport-formats.md` §6; `notes/geospatial.md` §3.2 (PENDING consolidation).

### 5.1 Owner-approval summary

| ADR | Topic | Needs owner approval? | Why / linked decision |
| --- | --- | --- | --- |
| A01 | IR vs USD | **YES** | D-010 open; foundational (spec §9) |
| A02 | Shell + viewport + TS stack | **YES** | D-011 open, D-012 partly; two separate decisions (spec §17) |
| A03 | Worker process model | **YES** | Changes packaging, sandboxing and copyleft boundary |
| A04 | ORT primary + adapter | **YES** | D-004 approves strategy, not runtime/model list (spec §9) |
| A05 | Wasmtime/WASI sandbox | **YES** | Spec §33 candidate only; SDK licence/trust open (D-014) |
| A06 | Lawful-source boundary | **YES** | Hard rights blocker + legal review must be acknowledged |
| A07 | Floating-origin precision | **YES** | Foundational IR constraint + GPU promise (spec §35) |

**None is approved.** Each should become a real ADR under `adr/` only after the owner decides; conflicts in `RESEARCH_REGISTER.md` §4 must be resolved first where relevant (OSM2World licence, WASI TCP default, ORT repo availability, OSM2World CLI stability).

---

## 6. Spec Part V requirements → components, and first-slice scope

### 6.1 Requirement mapping

| Spec § | Requirement area | Primary components / seams | First vertical slice (spec §50) |
| --- | --- | --- | --- |
| §21 Project and world management | Project creation/templates, region/boundary, project CRS, source catalogue, layers/hierarchy, settings/target profiles, local layout/cache, migration compatibility, autosave/crash recovery, import/export/portability | C1, C6, C5, C10; S1/S6 | **Partial:** create project, local layout, small GeoJSON boundary, CRS validation, basic layer list. Templates, autosave/crash recovery, full migration story: deferred. |
| §22 Geographic selection and data discovery | Place-name search (permitted provider), coordinate entry, boundary draw/import, area/coverage, source availability/date, licence visibility, resolution/accuracy, preflight warnings | C2, C12, C10; S2/S12 | **Partial:** coordinate entry or imported small boundary, documented OSM extract, licence/date displayed, basic preflight. Geocoding provider and coverage estimates deferred. |
| §23 Generation controls | Selectable layers (terrain, buildings, roads, lanes, junctions, sidewalks, bridges, water, vegetation, furniture, markings, traffic, plugin-defined), dependencies, selective regeneration, expert overrides | C3, C5, C9; S3/S5 | **Narrow:** basic road network + simple building footprints/volumes from supported attributes (spec §50 step 5). Dependencies explained but not optimised; advanced layers deferred. |
| §24 Editing model | Studio corrections vs parameter changes vs regeneration vs DCC delegation; round-trip metadata | C1, C5, C10, C8; S1/S5/S10 | **Partial:** select/inspect a road/building; show provenance/method/confidence (step 8–9). Lightweight correction path and Blender round-trip deferred. |
| §25 Asset management | Stable IDs, hierarchy, geometry/material refs, textures, collision, LOD chains, instancing, dependencies, provenance/licensing, generation params, source revision, hashes, selective regen, overrides | C1, C6; S1/S6 | **Partial:** stable IDs and provenance fields in IR; no full collision/LOD/instancing/override system. |
| §26 Quality profiles | Preview / Game-ready / High fidelity / Survey-reference; six quality dimensions; validation metrics with references and limits | C7, C5; S7 | **Partial:** basic geometry + structural validation (step 10). No survey claim without independent checkpoints; budgets and full metric suite deferred **[PENDING]** consolidation. |
| §27 Export and integrations | Blender, Unreal, Unity, Godot, glTF/GLB, USD, 3D Tiles; versions, discovery, mapping, collision/LOD, coordinate conversion, metadata retention, automated fixtures | C8, C7, C12; S8/S7/S12 | **Narrow:** export GLB or another researched interchange format (step 11) and re-import/open in a verified target tool (step 12). Engine integrations not claimed until tested. |

### 6.2 Explicitly deferred out of the first vertical slice

Not in scope for the slice (spec §50 deliberately limits it; `COMPETITIVE_ANALYSIS.md` §6 agrees):

1. Photogrammetry/SfM/MVS workers (COLMAP, OpenMVS, ODM, Meshroom) and GPU compute pipelines.
2. AI facade reconstruction, segmentation/depth inference, external AI providers and model distribution.
3. Splat/NeRF ingest, splat→mesh conversion, or any claim of editable splat geometry (SuGaR/2DGS are non-commercial research only).
4. Full-city scale, large-area streaming, 3D Tiles generation, quantized-mesh terrain, USD authoring/composition.
5. Cloud workers, remote compute, collaboration and asset marketplace.
6. The full plugin SDK/runtime and third-party plugin registry (WIT/WASI PoC is a separate spike).
7. LOD chains, collision generation, material/texture compression pipelines (KTX2/Draco/meshopt), instancing and target-engine budgets.
8. Survey/reference accuracy claims; independent checkpoint ingestion and ASPRS reporting.
9. Autosave/crash recovery, project migration tooling, multi-user, advanced DCC round-trip and Blender integration.
10. Geocoding provider integration, coverage/cost estimates, paid-job consent flow, release signing/notarisation and auto-update rollout.
11. PostGIS server mode, GeoParquet 2.0 large-dataset path, and optional OS M→GIS tooling (osm2pgsql/libosmium).
12. A final core/plugin licence selection (D-014) and GitHub creation/issues (spec Phase 7/8 approval).

Benchmark-first rule (spec §49): the study area is a small bounded part of Ikeja, Lagos (~500 m–1 km); **the exact boundary must be selected and recorded by the owner**. Without a fixed boundary and frozen input revisions, comparisons are not objective; without independent ground truth, only completeness/structural quality may be claimed [V: `COMPETITIVE_ANALYSIS.md` §4].

---

## 7. Questions this raises for the owner

Maximum eight, focused. Each has why it matters, options, a recommendation (not a decision) and whether it blocks architecture.

**Q1. What are the platform boundaries and CLI timing — Studio + Core now, with CLI/SDK/headless in the first public release or later?** (D-007, D-008)
- *Why:* spec §8 requires the first slice to use the same architecture later headless workflows use; scope changes the seam design.
- *Options:* (a) Core+Studio now, CLI/SDK in early preview; (b) CLI/headless in first public release; (c) SDK deferred indefinitely.
- *Recommendation:* (a), with all domain logic behind Core seams from day one.
- *Blocks architecture:* **YES** (scope).

**Q2. Do you approve the thin versioned World IR direction (ADR-A01), and who owns schema migrations and stable IDs?** (D-010)
- *Why:* every component depends on S1; changing it after projects exist requires migrations.
- *Options:* (a) thin IR + adapters; (b) USD-based IR after legal review; (c) glTF as internal format.
- *Recommendation:* (a), with migrations and golden round-trip tests as P0.
- *Blocks architecture:* **YES** (foundational).

**Q3. Do you approve Tauri + React/TS + three.js as two separate decisions (ADR-A02), subject to the cross-platform PoC?** (D-011, partly D-012)
- *Why:* webview GPU variance, TS 7 tooling, accessibility and signing identities affect the Studio foundation.
- *Options:* (a) Tauri+three.js; (b) Electron+three.js; (c) Tauri+Babylon 9.38.0; (d) native Bevy/wgpu.
- *Recommendation:* (a), gated on WebGL2/WebGPU probes on all three OSes.
- *Blocks architecture:* **YES** (Studio foundation).

**Q4. Which game engine / DCC integration gets the first verified export fixture, and when?** (D-009)
- *Why:* spec §27 forbids claiming support without an automated/documented compatibility test; export mapping depends on the target.
- *Options:* (a) GLB only in slice, Blender first fixture after; (b) Unity/Unreal/Godot first; (c) 3D Tiles first.
- *Recommendation:* (a), then Blender as an external tool (Blender GPL tool, outputs not covered by GPL).
- *Blocks architecture:* **Partly** — blocks export milestone, not Core seams.

**Q5. What is the licence direction for core, plugin SDK, docs, fixtures and model weights, and are you prepared to publish an ODbL-compliant machine-readable derivative offer if OSM-derived structured data is shared?** (D-014 + new legal questions)
- *Why:* Apache-2.0 candidate with express patent grant vs MIT; ODbL §4.6 obligation; AGPL tools and non-commercial weights cannot be bundled.
- *Options:* (a) Apache-2.0 core + separate SDK/docs/fixture licences; (b) MIT core; (c) decide later (blocks publication).
- *Recommendation:* (a) after dependency audit, plus an explicit "no OSM-derived database publication without §4.6 offer" rule.
- *Blocks architecture:* **YES** for publication and source-boundary design (ADR-A06).

**Q6. Do you acknowledge that Google Maps/Street View/Photorealistic 3D Tiles content and Cesium ion-hosted content cannot be reconstruction inputs, and should any visual-only basemap integration be allowed?** (new hard source constraint)
- *Why:* ToS prohibit derivation, offline caching and AI training; a visual-only carve-out needs strict no-derivation controls.
- *Options:* (a) hard deny everywhere; (b) allow visual-only under reviewed integration; (c) ignore until legal review.
- *Recommendation:* (a) for the slice; revisit (b) only with written legal clearance.
- *Blocks architecture:* **YES** — determines the lawful-source gate (ADR-A06).

**Q7. What cloud and collaboration scope do you want, if any?** (D-015, D-016)
- *Why:* spec §5 requires local-first; remote execution must remain an adapter, not a different pipeline.
- *Options:* (a) no cloud in slice, design job-local/remote parameter now; (b) self-hosted workers later; (c) Genesis-hosted compute + collaboration.
- *Recommendation:* (a); keep remote worker isolation/mTLS design as a later research item.
- *Blocks architecture:* **No** — can defer, provided S5 keeps local/remote as a parameter.

**Q8. What hardware floor / GPU backend promise and quality-profile names do you want for the first release?** (D-012, spec §§26, 35; **[PENDING]** quality/CI/GPU consolidation)
- *Why:* CUDA 13.5 has no macOS path, Vulkan-on-Metal (MoltenVK) is deprecated by Khronos, and all performance ranges are estimates; survey-grade claims require independent checkpoints.
- *Options:* (a) CPU fallback everywhere + optional backend detection with honest "unavailable" messaging; (b) promise CUDA-first workstations; (c) defer all promises until benchmarks.
- *Recommendation:* (a), with capability manifest, no survey-grade label and named profiles treated as provisional.
- *Blocks architecture:* **Partly** — capability detection is needed; numeric budgets can be deferred until measured.

---

## 8. Handoff notes (not decisions)

- This file is discovery output only. Do not create crates, issues or code from it; do not mark any ADR accepted.
- The consolidation step is **done**: `research/ECOSYSTEM_RESEARCH.md` §§6–7, `FORMAT_AND_STANDARDS.md` §4a and `LICENCE_AND_DATA_RIGHTS.md` §3a now include geospatial and quality/CI/GPU evidence. Before any proposal is **approved**, resolve the eight conflicts in `RESEARCH_REGISTER.md` §4 (OSM2World licence, Microsoft footprint count, WASI TCP default, ORT repo availability, OSM2World CLI stability, Vulkan-on-Metal, `geo` precision fixes, quantized-mesh/3D Tiles repo licences).
- Re-verification needed before any approval: OSM2World MIT vs LGPL; Wasmtime supported-version table and WASI TCP default; ORT repository availability; exact project-file licence treatment of ODbL-derived exports; signed-download/custom-quantisation licences for AI weights.
- Owner answers to Q1–Q8 feed `DECISIONS.md`; each approved answer becomes an ADR following `adr/0001-template.md`, with independent review per spec §39 before implementation tickets are marked `verified`.
