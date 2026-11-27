# Genesis Engine — Product Requirements Document

**Date:** 2026-11-27. **Status:** Requirements baseline accepted with the foundational plan (D-025). **Not approved:** the §5 first-slice boundary (D-013) and the open decisions in §8. **Authority:** `GENESIS_ENGINE_SPEC.md`. **Evidence:** the eight research briefs and five consolidated research documents.

This PRD specifies what Genesis must do and for whom. Architecture is accepted in `ARCHITECTURE.md` / `adr/0002`–`0008`; core licence is Apache-2.0 (D-026). The first-release boundary in §5 remains a **recommendation pending explicit owner approval** — approving the ADRs did not approve MVP layers.

## 1. Product definition

Genesis Engine is an open-source, cross-platform platform for turning permitted real-world geographic and visual data into editable, validated, game-ready 3D worlds. It has two progressively developed faces: **Genesis Core** (engine-agnostic reconstruction, world data, processing, validation, export) and **Genesis Studio** (desktop visual environment). CLI, SDK, headless execution and plugins follow progressively.

## 2. Users and jobs to be done

| User | Job | What they need |
| --- | --- | --- |
| Game developer | Build a geographically grounded world that fits runtime budgets | Collision, LOD, streaming, target-engine export, deterministic rebuilds, reproducibility |
| 3D / technical artist | Inspect and correct reconstructed geometry with provenance | Editable hierarchy, source/method/confidence per asset, selective regeneration, DCC hand-off |
| Technical lead / producer | Trust and reproduce the output | Source revisions, licences, validation reports, cost preflight, no unlabelled guesses |

## 3. Product principles (from spec §5, binding)

Geographic grounding; editable output; evidence and provenance; honest uncertainty; local-first; optional cloud; engine-agnostic internal representation; cross-platform; composable architecture; reproducibility; incremental regeneration; quality gates; open ecosystem; respect data rights; progressive complexity.

## 4. User journey (target, spec §7)

Create project → select place/boundary → inspect sources, dates, licences, costs → choose layers → select quality profile and target → preflight → review dependencies → generate preview → inspect result and provenance → correct/regenerate selected assets → validate five quality dimensions → optimise chunks/LOD/collision/materials → export → reopen and reproduce/incrementally update.

The first milestone delivers a **small complete version** of this journey, not all layers.

## 5. First vertical slice (proposed scope, spec §50)

1. Create a local project.
2. Import a small documented GeoJSON boundary.
3. Import a small OSM extract / footprint fixture.
4. Validate CRS and coordinate transformation (incl. floating-origin golden vectors).
5. Generate a basic road network and simple building footprints/volumes from supported attributes.
6. Create a structured World IR.
7. Display the world in the desktop viewport.
8. Select and inspect a road/building; show source, method and confidence.
9. Run basic geometry and structural validation.
10. Export GLB; re-import/open in a verified target tool.
11. Record reproducible test and benchmark results.

**Non-goals for the slice:** photogrammetry/SfM workers, AI facade reconstruction, external AI providers, splats/NeRF, full-city scale, 3D Tiles/USD generation, cloud workers, collaboration/marketplace, full plugin registry, LOD chains and collision generation, survey-grade claims, autosave/crash recovery, engine integrations beyond GLB re-import, final licence publication and GitHub issue creation before approval.

## 6. Functional requirements

### 6.1 Project and world management (spec §21)
- FR-P1 Create a project from a minimal template with a local portable layout.
- FR-P2 Set project CRS (WKT2/PROJJSON), epoch, vertical datum, units and up-axis; migrate schema versions.
- FR-P3 Manage a source catalogue with recorded licence election, attribution, vintage, filters, checksum.
- FR-P4 Manage world layers, visibility, hierarchy, generation settings and target profiles.
- FR-P5 Separate portable project state from cache; show disk usage; atomic writes and recovery (later milestone).

### 6.2 Geographic selection and data discovery (spec §22)
- FR-G1 Coordinate entry and small GeoJSON boundary import (geocoding provider optional/later).
- FR-G2 Show source availability, dates, licences, resolution/accuracy and preflight warnings.
- FR-G3 Never require a hosted provider when local data is supplied; no restricted scraping.
- FR-G4 Enforce a lawful-source gate: deny Google Maps/Street View/Photorealistic 3D Tiles content and Cesium ion-hosted content as reconstruction inputs (proposed A06).

### 6.3 Generation controls (spec §23)
- FR-C1 Selective generation by layer and area.
- FR-C2 Dependency identification with explanations and safe expert overrides.
- FR-C3 Deterministic output from named pinned RNG + recorded seed/settings.
- FR-C4 Selective/incremental regeneration of affected objects/layers (design risk: no mature ecosystem contract found).

### 6.4 Editing model (spec §24)
- FR-E1 Studio: lightweight corrections, parameter changes, regeneration of selected objects.
- FR-E2 Advanced modelling delegated to Blender/DCC; round-trip must preserve metadata.
- FR-E3 Keyboard-numeric transforms as a first-class alternative to pointer gizmos.

### 6.5 Asset management (spec §25)
- FR-A1 Stable IDs, hierarchy, geometry/material refs, dependencies, source feature IDs.
- FR-A2 Provenance/licensing, generation params, source revision, content hash, schema version.
- FR-A3 LOD/collision/instancing/overrides: later milestones.

### 6.6 Quality profiles and validation (spec §26)
- Profiles (provisional): Preview, Game-ready, High fidelity, Survey/reference.
- Six independent dimensions: geographic accuracy, geometric accuracy, visual fidelity, structural completeness, game/runtime readiness, evidence quality/confidence.
- A "Survey/reference" label is **only** available with independent checkpoints (ASPRS Ed2 V2: ≥30 checkpoints, ≥2× product accuracy, NVA/VVA split). Otherwise the system states "no geographic accuracy claim".
- Every metric records reference data, CRS/datum, method, tool version, parameters, seed, timestamp and limitations.

### 6.7 Export and integrations (spec §27)
- First target: glTF 2.0/GLB with a defined extension allowlist and provenance sidecar.
- Later: Blender, Unreal, Unity, Godot, USD, 3D Tiles, OpenDRIVE. No support claim without an automated/documented compatibility fixture.

### 6.8 Developer interfaces (spec §3, §8)
- CLI/headless and SDK must drive the same Core seams as Studio; headless-drivable is the test of a Core capability.

## 7. Non-functional requirements

| ID | Requirement | Verification |
| --- | --- | --- |
| NFR-1 | Windows, macOS, Linux; x86_64 and ARM64 progressively; no unverified backend promise | Cross-platform CI on native runners; capability manifest with graceful degradation |
| NFR-2 | CPU fallback for preview/workflow | CPU-only test on all three OSes |
| NFR-3 | Large-coordinate correctness | Golden tests at ~1e7 m, ±180°, large projected coordinates; floating-origin invariants |
| NFR-4 | Reproducibility | Recorded seeds/hashes/tool versions; two identical fixture runs produce identical IR |
| NFR-5 | Security | Archive/path/decompression hardening; model-file sandbox; deny-by-default plugins; no secrets in artefacts |
| NFR-6 | Licence provenance | Attribution survives IR→export; no rights-blocked source accepted; licence audit of bundled dependencies |
| NFR-7 | Testability | Unit/property/integration/E2E/cross-platform/performance layers per `engineering/TEST_STRATEGY.md` |
| NFR-8 | Accessibility | ARIA tree/grid panels and keyboard paths; axe + keyboard tests |

## 8. Open questions for the owner

See `engineering/ARCHITECTURE_DISCOVERY.md` §7 (Q1–Q8) and `DECISIONS.md` §3 (D-017–D-019). They cover platform boundaries/CLI timing, IR approval, shell/viewport approval, first export fixture, licence direction and ODbL publication policy, acknowledgement of blocked sources, cloud/collaboration scope, and hardware/quality-profile promises. Product answers to these change the scope of §5–§6 and must be recorded before ticket implementation starts.
