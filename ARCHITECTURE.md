# Genesis Engine — Architecture

**Date:** 2026-11-27. **Status:** Accepted by owner 2026-11-27 (D-025). Seven accepted ADRs: `adr/0002`–`adr/0008`. No product code exists yet. **Authority:** `GENESIS_ENGINE_SPEC.md` Parts VI–IX. **Evidence:** `engineering/ARCHITECTURE_DISCOVERY.md`, `adr/ADR_PROPOSALS.md`, `research/*.md`.

## 1. Architectural shape

One engine, several front doors. **Genesis Core** owns all domain state and exposes versioned commands/events/data contracts. Studio, CLI, SDK, headless and plugins are consumers — never owners — of CRS maths, provenance, job state or validation rules. A capability that cannot be driven headlessly is not yet a Core capability (spec §8).

```
  Studio (Tauri+React+three.js, proposed)   CLI/headless   SDK   Optional cloud
                    |                          |            |          |
                    +--------- versioned commands / events / snapshots ------+
                                             |
                                        Genesis Core
                                             |
   +----+----+----+----+----+----+----+----+----+----+----+----+
   | C1 | C2 | C3 | C4 | C5 | C6 | C7 | C8 | C9 | C10| C11| C12|
   | IR |Geo |Wrkr| AI |Jobs|Stor|Qual|Exp |Plug|UI  |CLI |Law |
   +----+----+----+----+----+----+----+----+----+----+----+----+
```

## 2. Components and seams

| ID | Component | Owns | Seam |
| --- | --- | --- | --- |
| C1 | World IR + schema versioning | Stable IDs, CRS/epoch/datum/units/up-axis, local frame, provenance, observed-vs-inferred flags, seeds, content hashes, validation state, migrations | S1 versioned documents + referenced binary payloads; no giant inline-geometry JSON |
| C2 | Geospatial adapters | Source descriptors, CRS/epoch, antimeridian policy, tile sources | S2 `SourceDescriptor` → normalised features + provenance |
| C3 | Reconstruction worker adapters | Worker versions, checkpoints, licence tier, staged artefacts | S3 argv-only, scrubbed env, manifests, normalised progress, cancellation |
| C4 | AI adapter / runtime | Model identity/checksum, code vs weight licence, device/EP priority | S4 typed tensors + prompts → typed results + per-run evidence |
| C5 | Job orchestration | Job graph, cancellation tokens, budgets, cache keys | S5 versioned job records, bounded queues, checkpoints, retry |
| C6 | Storage & cache | Project layout, manifests, content-addressed blobs, eviction | S6 atomic promotion; portable state never only in hidden cache |
| C7 | Quality / validation | Metric records, pass/fail gates, profile policy | S7 `{value, unit, reference, crs/datum, method, version, params, seed, limitations}` |
| C8 | Export adapters | Extension allowlist, target versions, fixtures | S8 IR → target bundle + validation + attribution sidecar |
| C9 | Plugin SDK & runtime | Manifest, semver, capability grants, trust | S9 versioned WIT world, deny-by-default, epoch + timeout, explicit caps |
| C10 | Desktop shell + viewport | UI interaction state, GPU resources only | S10 explicit commands/events/versioned snapshots; keyboard-numeric alternative |
| C11 | CLI (progressive) | Same command contract as Studio | S11 machine-readable output, no UI dependency |
| C12 | Lawful-source boundary & provenance (cross-cutting) | Allow/deny list, required provenance fields | S12 consulted before import, inference, export, plugin execution |

Full responsibilities, evidence and version/licence facts are in `engineering/ARCHITECTURE_DISCOVERY.md` §§1–2.

## 3. Data flows

Three diagrams (import→reconstruct→IR→validate→export; job execution with progress/cancellation; plugin invocation through WIT/WASI) are in `engineering/ARCHITECTURE_DISCOVERY.md` §3. Key invariants:

- All_paths pass S12 lawful-source gate before data enters C1.
- Every job is inspectable: inputs+hashes, params+seed, stages, progress, checkpoint, canonical errors, output manifest.
- Cancellation is one token mapped per adapter; reconstruction tools have no documented cancel API, so process-tree kill + checkpoint resume is the portable path.
- Plugin capabilities are explicitly granted; no ambient filesystem/network/process/credential access.

## 4. Cross-cutting constraints (non-negotiable)

1. **Precision:** authoritative geodetic coordinates; per-tile/region ENU local frame and floating origin for mesh maths/rendering; golden tests at ~1e7 m and ±180° (A07).
2. **Provenance:** licence election, attribution, vintage, filters, checksum, derivation chain, observed-vs-inferred flags; attribution survives export (A06).
3. **Lawful sources:** hard deny Google Maps/Street View/Photorealistic 3D Tiles and Cesium ion content as reconstruction inputs; open standards (3D Tiles/cesium-native Apache-2.0) permitted with Genesis-owned content.
4. **Licence boundaries:** AGPL/MPL/GPL tools behind process boundaries; GEOS dynamic-linked; GDAL binary licence audit; no non-commercial model weights in a commercial path.
5. **Security:** archive containment/decompression quotas; no unpickling untrusted models; argv-only workers; Wasmtime ≥51 with explicit caps; secrets never in source/prompts/logs/tickets/commits.
6. **Honest uncertainty:** no survey-grade claim without independent checkpoints; metrics carry reference data and limitations; image similarity is not geographic accuracy.
7. **Bounded resources:** no unbounded channels/caches; graceful backend detection; no silent fallback hiding data loss.

## 5. Decision state

All seven architecture ADRs were **accepted by the owner on 2026-11-27** (D-025): A01 → `adr/0002`, A02 → `adr/0003`, A03 → `adr/0004`, A04 → `adr/0005`, A05 → `adr/0006`, A06 → `adr/0007`, A07 → `adr/0008`. The shell and viewport remain two separate approvals within A02, and the A02/A03/A04/A05 PoC gates are retained — acceptance of direction is not a waiver of verification.

**Open (do not silently assume):** the authoritative list is `DECISIONS.md` §2 and §3 — platform boundaries D-007; CLI timing **resolved by D-030**; first engine/DCC integration **resolved by D-030**; **first-slice layer scope resolved by D-029** (PRD §5 approved); benchmark boundary **resolved by D-031** (`research/BENCHMARK_AREA.md`); cloud D-015; collaboration D-016; **skill identity D-017 and setup-skill tracker D-018** (not explicitly answered; `to-tickets` was used and GitHub Issues adopted by D-027); benchmark boundary D-021; signing budget D-022; Rust MSRV D-023; ODbL publication review D-024; and the **SDK/docs/fixture/sample-asset/model-weight licence questions** under D-014 (core Apache-2.0 only was approved as D-026). Eight evidence conflicts remain logged in `research/RESEARCH_REGISTER.md` §4; where one touches an accepted ADR (OSM2World licence, WASI TCP defaults, ORT repo availability, Vulkan-on-Metal, `geo` precision fixes, repository licences), the relevant ticket must resolve it before implementation proceeds.

## 6. What is deliberately not in the architecture yet

No crate layout is prescribed (spec §42: do not scaffold empty crates). No final dependency versions are pinned in code. No GPU backend promise beyond CPU fallback + capability detection. No plugin registry. No cloud topology. No SDK/docs/fixture/model-weight licence (core Apache-2.0 only). A public GitHub repository `Jasowills/genesis-engine` was approved (D-027) and is being created with issues from the approved tickets.
