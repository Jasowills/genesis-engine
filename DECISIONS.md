# Genesis Engine — Decision Register

**Date opened:** 2026-11-27
**Source of approved items:** `GENESIS_ENGINE_SPEC.md` Part III and Appendix A, supplied by the project owner. Items below marked "Approved (owner-supplied spec)" are transcribed from that document, **not** independently re-approved in this session. No new decision has been made by the agent.

## 1. Approved decisions (owner-supplied spec)

| ID | Area | Decision | Status |
| --- | --- | --- | --- |
| D-001 | Product identity | Both reconstruction engine and full environment platform, developed progressively | Approved (owner-supplied spec) |
| D-002 | Application | Desktop-first visual Studio | Approved (owner-supplied spec) |
| D-003 | Compute | Local-first, cloud optional | Approved (owner-supplied spec) |
| D-004 | AI strategy | Both curated/bundled open models and optional external providers/custom models | Approved (owner-supplied spec) |
| D-005 | Target users | Game developers and 3D/technical artists equally | Approved (owner-supplied spec) |
| D-006 | Initial operating systems | Windows, macOS and Linux | Approved (owner-supplied spec) |

## 2. Decision disposition after Phase 8 (not to be assumed beyond what was approved)

| ID | Area | Current state | Notes |
| --- | --- | --- | --- |
| D-007 | Full platform boundaries: Studio, Core, SDK, CLI, headless | **Open** | Not resolved by D-025; scope still to be decided (Q1). |
| D-008 | CLI release timing | **Resolved by D-030** | CLI/headless in M2 (early preview), not first public release. |
| D-009 | First engine/DCC integration | **Resolved by D-030** | GLB in the first slice; Blender first DCC fixture afterwards. |
| D-010 | World IR and project storage | **Settled for the IR direction by D-025** (`adr/0002`, `adr/0008`) | Exact project storage layout and schema fields remain engineering work; full platform boundary still open. |
| D-011 | Viewport technology | **Settled by D-025** (`adr/0003`) | three.js + R3F/drei direction accepted; PoC gate retained (GE-0004). |
| D-012 | GPU backend strategy | **Partly settled by D-025** (`adr/0003`, `adr/0008`) | Capability detection + CPU fallback accepted; exact GPU backend promise remains open (Q8). |
| D-013 | First-release layer scope | **Resolved by D-029** | PRD §5 approved as-is; no extra layers in the first slice. |
| D-014 | Core and plugin licence | **Core settled by D-026 (Apache-2.0); partially open** | Plugin SDK, docs, fixtures, sample assets and model-weight licences remain separate decisions; D-024 legal review open. |
| D-015 | Cloud execution model | **Open** | Optional cloud; no topology promised. |
| D-016 | Collaboration and asset marketplace scope | **Deferred** | No scope expansion. |

## 3. Additional questions surfaced during discovery

| ID | Question | Origin | Status |
| --- | --- | --- | --- |
| D-017 | Is `to-tickets` the specific Matty Pocock skill intended, or a different one from the family? | Phase 1 discovery found `to-tickets`, `triage`, `to-questionnaire`, `wayfinder`; `to-spec` is referenced but not installed. | **Not explicitly answered.** `to-tickets` was found, loaded, verified against upstream commit and used for decomposition; record as an assumption to confirm. |
| D-018 | Issue tracker for the setup skill: GitHub Issues, GitLab, or local markdown? | `setup-matt-pocock-skills` requires this choice. | **Not explicitly answered.** GitHub Issues is now the project tracker by D-027; the setup skill itself was not run. |
| D-019 | Repository name, visibility, description and licence direction for GitHub creation. | Spec Phase 7. | **Resolved** by D-026 + D-027 (public `genesis-engine`, Apache-2.0 core). |
| D-020 | Acknowledge a hard deny-list: Google Maps/Street View/Photorealistic 3D Tiles content and Cesium ion-hosted content cannot be reconstruction inputs. | Research verified ToS prohibitions. | **Resolved** by D-028 (hard deny acknowledged). |
| D-021 | Exact Ikeja, Lagos benchmark boundary (~500 m–1 km), frozen input revisions and available ground truth. | Spec §49 requires owner selection before any comparative benchmark. | **Resolved by D-031** — agent-selected boundary in `research/BENCHMARK_AREA.md` |
| D-022 | Approval of signing identities and budgets: Apple $99/yr plus one Windows path (~$9.99/mo or $150–400+/yr), and updater key custody. | `engineering/RELEASE_PROCESS.md`; no purchase may occur without approval. | **Owner declined 2026-11-27** (open source; users obtain OS-appropriate source/builds). No signing identity, notarisation, auto-update keys or purchase. GE-0032 closed as declined; source-distribution path stands. |
| D-023 | Rust MSRV baseline: 1.90 (covers `rstar`, Tauri updater, likely `geo`) vs 1.95 (needed by `geoparquet`/`geoarrow` 0.10). | Local toolchain is 1.98.1; contributor machines vary. | Needed before M2 |
| D-024 | Legal review of whether exported GLBs / project files are ODbL Derivative Databases and whether Genesis will publish a §4.6 machine-readable offer. | `research/LICENCE_AND_DATA_RIGHTS.md` §3a; cannot be settled by engineering alone. | Needed before publication |

## 3a. Owner decisions recorded 2026-11-27 (Phase 8 checkpoint)

| ID | Decision | Owner answer | Scope / consequences |
| --- | --- | --- | --- |
| D-025 | Architecture foundation A01–A07 | **Approved** | All seven proposed ADRs accepted as the architecture foundation. Shell and viewport remain two separate decisions with PoC gates retained (spec §17). Accepted ADR files: `adr/0002`–`adr/0008`. |
| D-026 | Core licence | **Apache-2.0** | Core source code is Apache-2.0 (express patent grant). SDK, documentation, test fixtures, sample generated assets and model weights remain **separate** licence decisions to be recorded after the dependency audit — the owner's approval was for the core direction only. |
| D-027 | GitHub repository | **Public `genesis-engine` under `Jasowills`** | Owner approved creation, labels/milestones/issues from approved ticket files, and native blocking links. Description (live repository value, verified by `gh repo view`): "Open-source, cross-platform environment reconstruction platform for editable, validated, game-ready 3D worlds" (no trailing period — cosmetic difference from the proposed wording; treated as approved in substance). |
| D-028 | Lawful-source boundary | **Hard deny acknowledged** | Google Maps/Street View/Photorealistic 3D Tiles content and Cesium ion-hosted content are denied as reconstruction inputs, enforced in code (GE-0012). Open standards/software (CesiumJS/3D Tiles Apache-2.0) remain usable with Genesis-owned content. (Agent note, **not** an owner decision: a visual-only carve-out could be revisited only with written legal clearance.) |

### Documented lifecycle exception — GE-0008 (recorded before any `gh` mutation)

GE-0008 (`GitHub repository and issue setup`) has `depends_on: [GE-0007]`, and GE-0007 is `blocked` pending remaining owner answers. The normal lifecycle (`TICKETS/README.md` §5; spec §40 step 2) requires a ticket to be `ready` with all blockers verified before work begins. **Exception:** the owner directly authorised the repository action in D-027 (public `genesis-engine`, with labels/milestones/issues from approved tickets), so GE-0008 is set to `in-progress` on that explicit authority. This is **not** a waiver of review or verification: GE-0008 still requires evidence, independent review and the normal gates before it can be marked `verified`, and the deferred items in the Phase 7 checklist below remain deferred.

### Phase 7 approval checklist (recorded before any `gh` mutation)

- [x] Repository name `genesis-engine`, owner `Jasowills` — confirmed by D-027
- [x] Visibility **public** — explicitly approved in D-027
- [x] Description — "Open-source, cross-platform environment reconstruction platform for editable, validated, game-ready 3D worlds."
- [x] Core licence Apache-2.0 — D-026; `LICENSE` + `NOTICE` record the scope carve-out for SDK/docs/fixtures/model weights
- [x] Authenticated account confirmed read-only (`gh api user` → `Jasowills`); no existing genesis-named repo found in the first 100 repos
- [x] No token value exposed; no destructive git operation planned; no secrets/restricted data/large weights staged (pattern grep passed)
- [x] Lockfiles: none exist yet (no dependencies); policy recorded in `engineering/TEST_STRATEGY.md` and `RELEASE_PROCESS.md` — commit lockfiles and build with `--locked` once introduced
- [x] Git LFS: not needed at this stage; no large binary fixtures committed. Revisit before any large dataset/model artefact
- [ ] Branch protection / repository settings: **not explicitly approved — do not enable.** Record as a remaining action for owner decision
- [x] Reviewer gate: independent reviewer subagent returned **`APPROVED`** on 2026-11-27 after four passes (first two `CHANGES_REQUESTED` on governance/reconciliation defects, all findings corrected and reverified). The reviewer explicitly cleared GitHub setup under D-027. Evidence: reviewer session `ses_de59f7bffffeZu5LcI2JsshkFy`, final verdict `APPROVED`.

| D-029 | First vertical slice scope | **Approved as-is (PRD §5)** | Project create → small GeoJSON boundary → documented OSM extract → roads + simple buildings → World IR → viewport inspect → basic validation → GLB export/re-import. Photogrammetry, AI, splats, LOD, cloud and marketplace stay deferred. Unblocks M1 tickets. |
| D-030 | CLI timing and first integration | **CLI in early preview; GLB then Blender** | D-008: CLI/headless in M2, not the first public release; Core seams remain headless-drivable from M1. D-009: GLB re-import fixture in the slice (GE-0019), Blender as the first DCC fixture afterwards (GE-0029). Target game-engine integrations remain open. |
| D-031 | Ikeja benchmark boundary | **Agent may select and record** | D-021 delegated to the agent; boundary frozen in `research/BENCHMARK_AREA.md` with source, rationale and licence. No independent ground truth is assumed. |

| D-033 | Named model weights for GE-0006 | **Owner-approved 2026-11-27** | Download and use SAM ViT-B, YOLOX-S and Depth Anything V2 Small weights in ONNX form; owner accepted the named artefacts' actual permissive licences. Correction (2026-11-27): the Depth Anything V2 Small ONNX conversion is **BSD-3-Clause** at `AXERA-TECH/Depth-Anything-V2` while the upstream base model is Apache-2.0; both are permissive, so no re-approval is required. Weight binaries are evidence artefacts, ignored by git; checksums and source URLs are recorded in the ticket. |
| D-034 | First target engine | **Owner-selected 2026-11-27: Unreal** | GE-0031 targets Unreal first. No support claim until a fixture passes on the tested engine version; engine SDK/licence provisioning is a separate manual step. |
| D-032 | Test fixture licence | **Recorded (agent, 2026-11-27)** | `fixtures/ikeja-small.geojson` is derived from OSM and carries ODbL-1.0 with `© OpenStreetMap contributors`; it is **not** relicensed under the core Apache-2.0 grant. `NOTICE` records the carve-out. ODbL §4.6 publication questions (D-024) remain open for generated world data, not for this small attribution-bearing fixture. |

These answers satisfy the Phase 8 checkpoint for the foundational plan. **Resolved by later owner answers:** D-008, D-009, D-013 and D-021 (D-029…D-031). **Still open:** D-007 (platform boundaries), D-015 (cloud), D-016 (collaboration), D-017 (skill identity), D-018 (setup-skill tracker), D-022 (signing — owner declined 2026-11-27), D-023 (MSRV), D-024 (ODbL legal review) and the non-core licence questions under D-014. **Newly resolved:** D-033 (named model weights), D-034 (Unreal as first target engine). None of these blocks the approved first slice; they are tracked here rather than assumed.

## 4. Decision-record rules

- Every material change to an approved decision requires an ADR and owner approval (spec Part III).
- Distinguish facts, estimates, assumptions, hypotheses and recommendations in all records.
- No decision is marked approved here because the agent recommended it — only because the owner supplied it in the spec or explicitly confirms it.
