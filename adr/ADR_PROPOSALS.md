# Genesis Engine — ADR Proposals (SUPERSEDED — historical record)

**Date:** 2026-11-27. **Status of this document:** SUPERSEDED. On 2026-11-27 the owner approved all seven proposals (D-025). The authoritative records are the accepted ADRs: `adr/0002` (A01), `0003` (A02), `0004` (A03), `0005` (A04), `0006` (A05), `0007` (A06), `0008` (A07). This file is retained as the evidence trail showing options and rationale before approval; its approval checklist below is historical and no longer the active gate.

These are the seven candidate architecture decisions distilled from `engineering/ARCHITECTURE_DISCOVERY.md` and the eight verified research briefs. Each uses the required fields from `adr/0001-template.md`. Upon owner approval, each becomes a numbered ADR file (`adr/0002-*.md` …) with status **Accepted**; until then this document is the authoritative proposal set. Do not implement any of these decisions before approval (spec rules 4 and 5).

---

## A01 — Thin versioned World IR with adapters vs USD-based IR

**Status:** PROPOSED · **Decision ID:** D-010 (open) · **Owner approval:** YES · **Reversibility:** Moderate before projects exist, expensive after.

**Context.** Spec §30 requires an engine-agnostic versioned IR with stable IDs, CRS/frame, provenance, confidence, seeds, partial updates, migrations and export mapping, and forbids one giant inline-geometry JSON. glTF 2.0 is explicitly a transmission format, not authoring/streaming; 3D Tiles 1.1 is streaming-oriented; neither carries provenance/confidence. OpenUSD v26.11.1 has layers/references/variants/payloads but a heavy C++ dependency and the Tomorrow Open Source Technology License 1.0 (Apache-2.0 with modified trademark §6) requiring legal review.

**Options.** A — thin purpose-built versioned IR + adapters; B — USD-based IR; C — glTF/GLB as internal format.

**Decision (proposed).** A. Study USD concepts, do not take the C++/licence dependency in the first slice; GLB is the first interchange export.

**Rationale.** A satisfies §30 and the provenance-first differentiator, keeps the core Rust-first and licence-clean; B adds an unapproved licence and dependency before the slice validates the architecture; C is contradicted by the Khronos spec itself and by glTF `extras` having no shared schema.

**Consequences.** Positive: stable IDs/migrations/selective regeneration are first-class. Negative: no ecosystem tooling; schema and round-trip tests are net-new; risk of reinventing USD concepts.

**Evidence.** Spec §§16, 30; `research/FORMAT_AND_STANDARDS.md` §§1–2; `research/ECOSYSTEM_RESEARCH.md` §2. **Re-verification:** none blocking; USD licence review still open.

---

## A02 — Tauri + React/TS + three.js (shell and viewport as two decisions)

**Status:** PROPOSED · **Decision IDs:** D-011, partly D-012 · **Owner approval:** YES · **Reversibility:** Moderate if the UI↔Core seam (S10) holds.

**Context.** Spec §§17, 28 leave framework/renderer open. Verified: Tauri core 2.12.7 / CLI 2.12.6 / bundler 2.10.6 (Apache-2.0/MIT, 18 Nov 2026), updater on Win/macOS/Linux with mandatory signatures (Rust ≥1.90); Electron 45.5.0 (MIT) has no Linux built-in updater; Wails 3.1.0 (MIT, Go 1.26+). React 19.3.1 (MIT), TypeScript 7.0.2 (Apache-2.0); three.js r187 (MIT, TransformControls, BatchedMesh `batchId`, LOD hysteresis); Babylon.js 9.38.0 (Apache-2.0, GizmoManager + snapDistance); Bevy 0.20.1 / wgpu 30.0.2 / egui 0.37.0 are higher-cost alternates.

**Options.** A — Tauri + React/TS + three.js via R3F 9.8.2/drei 10.7.9; B — Electron + React/TS + three.js; C — native-only Bevy/wgpu/egui; D — Tauri + Babylon.js.

**Decision (proposed).** A, gated on a PoC: WebGL2/WebGPU capability probe in all three webviews, floating-origin test at ~1e7 m, 100k-object picking/batching test, load/unload GPU-leak test, TS 7 tooling check. Keep D as fallback if gizmo/snap work dominates. Approve **shell and viewport separately**.

**Rationale.** Tauri is the only shortlisted shell with a verified signed updater on all three OSes and a native Rust bridge; three.js has the richest editor primitives/loaders. Electron's consistent Chromium must be weighed against bundle size and the Linux updater gap.

**Consequences.** Positive: aligns with Rust-first core and React/TS UI. Negative: three webviews differ; canvas accessibility has no standard pattern (keyboard-numeric transforms must be built); TS 7 tooling unverified; viewport does not solve streaming by itself. Signing identities/key custody needed before release (Apple $99/yr; Windows ~$9.99/mo or $150–400+/yr).

**Evidence.** Spec §§17, 28; `research/notes/desktop-viewport-formats.md` §§1–6; `research/notes/quality-ci-gpu.md` §§2–3.

---

## A03 — External-process workers with normalised progress events

**Status:** PROPOSED · **Owner approval:** YES (packaging, sandboxing, copyleft boundary) · **Reversibility:** Moderate-high at the seam.

**Context.** Spec §29 permits mature non-Rust tools behind stable adapters with a mandatory requirement list. Mature reconstruction tools are CLI/process-oriented; no cancellation API was found in any primary source; progress formats are heterogeneous; licences favour process separation (OpenMVS/ODM AGPL, Meshroom MPL, Blender GPL).

**Options.** A — separate OS process per job + normalised event protocol; B — in-process FFI for permissive tools only; C — containers/microVMs for every worker.

**Decision (proposed).** A for specialist/non-Rust workers; B only for reviewed permissive libraries; defer C to optional cloud/headless. Normalise progress to `{stage, progress 0..1, units, checkpoint}`; cancellation is one token mapped per adapter (`SetTerminate` for ORT, kill-tree for processes, epoch+timeout for plugins).

**Rationale.** Process isolation is the only evidenced pattern handling licensing, crashes, version detection and missing cancellation APIs simultaneously.

**Consequences.** Positive: blast-radius reduction, licence compliance, tool upgrade independence. Negative: IPC overhead; per-OS sandbox profiles diverge; seccomp alone is **not** a sandbox.

**Evidence.** Spec §§29, 31; `research/notes/reconstruction.md` §§3.4, 5–6; `research/notes/plugin-security.md` §§2–3.

---

## A04 — ONNX Runtime primary behind a swappable AI adapter contract

**Status:** PROPOSED · **Decision ID:** D-004 approves strategy, not runtime · **Owner approval:** YES · **Reversibility:** High if capabilities never import ORT types directly.

**Context.** Spec §18 mandates an adapter contract with model identity, version, checksum, I/O schema, device requirements, licence and attribution. ONNX Runtime 1.32.0 (2026-11-13, MIT, Production/Stable) has the broadest EP matrix and `RunOptions::SetTerminate()`; CoreML EP is **preview** with restrictive op tables; DirectML sustained. Candle is dual MIT/Apache-2.0 but hand-written per architecture; Burn requires Rust 1.95 and is changing. No independent benchmark was run; ORT's GitHub repo was unreachable during research (version/licence verified via PyPI/docs).

**Options.** A — ORT via C ABI behind the adapter; B — Candle primary; C — multi-runtime from day one.

**Decision (proposed).** A as default local runtime, with full adapter contract (identity, SHA-256, code/weight licence, typed schemas, device/EP priority, settings, cancellation, canonical errors, per-run evidence). Candle optional where an exact implementation exists. Model shortlist: SAM ViT-B, YOLOX, Depth Anything V2 **Small**, LaMa (verify weights). Avoid Ultralytics YOLO (AGPL/paid), DA-V2 Base/Large/Giant (CC-BY-NC), DUSt3R, Hunyuan3D 2, VGGT non-commercial checkpoint.

**Rationale.** The spec forbids choosing a runtime merely because it is Rust; ORT's operator/EP coverage plus cancellation best fit a three-OS local-first Studio; the adapter keeps external providers swappable.

**Consequences.** Positive: commercially clean default path, explicit fallback logging. Negative: no first-class Rust API; dynamic-shape models may lose EP acceleration; CoreML→CPU fallback changes performance and licence posture and must be recorded.

**Evidence.** Spec §§18, 29; `research/notes/ai-ml.md` §§1–5; `research/LICENCE_AND_DATA_RIGHTS.md` §1.

---

## A05 — Wasmtime / WASI 0.2 plugin sandbox

**Status:** PROPOSED · **Owner approval:** YES (trust/licence, D-014) · **Reversibility:** Moderate via WIT versioning.

**Context.** Spec §33 suggests Wasmtime/WASM but requires validation. Wasmtime 51.0.0 (2026-11-20, Apache-2.0 WITH LLVM-exception) has monthly majors; WASI 0.2 is a stable preview; WASI 0.3 released 2026-06-11 but Wasmtime `p3` is experimental. Windows sandbox escape GHSA-f2pq-268w-886c fixed in 36.0.22/48.0.10/49.0.7/50.0.3; host stream DoS fixed in 51.0.0; TCP default docs conflict; `StoreLimitsBuilder::memory_size` has no default.

**Options.** A — Wasmtime ≥51 + WASI 0.2 components + typed WIT world; B — native dynamic libraries (first-party trusted only); C — sandboxed OS process per plugin; D — defer plugins past the slice.

**Decision (proposed).** A for untrusted third-party plugins; combine with C for native/high-risk workers; defer D only for the *registry*, keep a WIT PoC spike. Pin ≥51.0.0 with patch monitoring; zero preopens/env/network; epoch interruption + `tokio::time::timeout` (epochs do not cover blocking hostcalls); explicit Store/WASI caps; bounded log/progress hostcalls; signed SHA-256-pinned manifest and atomic activation with retained prior version. Track WASI 0.3, do not depend on it in v1.

**Rationale.** WASI has no ambient authority and Wasmtime is the maturest Rust embedding with a typed component ABI; CVE density is a patch-discipline requirement, not a reason to use unsafe native loading.

**Consequences.** Positive: no ambient host access; language-neutral ABI. Negative: recurring upgrade work; precompiled artefacts tied to exact version; host functions are the weak point; manifest/signing/rollback are our design (no sourced standard); component overhead percentage unverified.

**Evidence.** Spec §§33–34; `research/notes/plugin-security.md` §§1–4; `research/RESEARCH_REGISTER.md` §4 conflicts.

---

## A06 — Lawful-source adapter boundary and provenance-first metadata

**Status:** PROPOSED · **Owner approval:** YES (acknowledge blocked sources; ODbL publication policy) · **Reversibility:** Low once assets ship with provenance.

**Context.** Spec §§5, 12, 16, 26–27 require evidence, provenance and respect for data rights. OSM ODbL §§4.2–4.6 (§4.6 machine-readable derivative offer); Microsoft GlobalML CDLA Permissive 2.0; Google Open Buildings CC BY 4.0 or ODbL (election must be recorded); Overture theme-dependent licences; Google Maps Platform ToS §3.2.3(a)–(c) and Cesium ion ToS prohibit derivation, offline caching and AI training. No reviewed open map-to-3D tool advertises per-asset provenance.

**Options.** A — enforceable allow/deny gate + provenance in IR and export; B — documentation-only policy; C — allow all with user attestation.

**Decision (proposed).** A. Deny Google Maps/Street View/Photorealistic 3D Tiles and Cesium ion-hosted content as reconstruction inputs; permit open-standards software (CesiumJS/3D Tiles Apache-2.0) with Genesis-owned content. Record licence election per snapshot; keep structured derived data separable from rendered assets; require legal review before publishing any OSM-derived database or GLB.

**Rationale.** This is the evidence-backed differentiator and the only way to satisfy honest uncertainty and attribution-survives-export simultaneously; no standard provenance vocabulary exists.

**Consequences.** Positive: lawful multi-source fusion, reproducible revisions, defensible exports. Negative: blocks some high-fidelity commercial inputs; more bookkeeping; §4.6 offer design unresolved; generated-GLB legal boundary unverified.

**Evidence.** Spec §§5, 12, 16, 18, 26–27; `research/LICENCE_AND_DATA_RIGHTS.md` §§1–4; `research/COMPETITIVE_ANALYSIS.md` §3.

---

## A07 — Floating-origin / local-frame precision strategy

**Status:** PROPOSED · **Owner approval:** YES (foundational + GPU promise) · **Reversibility:** Very low after projects exist.

**Context.** Spec §§10, 30, 35. glTF core POSITION is float32 (~0.5 m ULP at Earth radius, ~1 m at 1e7 m); f64 vertex attributes need draft `KHR_accessor_float64`; WebGL/WebGPU broadly lack f64 buffers/shaders. 3D Tiles uses `RTC_CENTER` per tile, glTF has `CESIUM_RTC` (Stable); PROJ `+proj=topocentric` (since 8.0, EPSG:9836/9837) converts ECEF→ENU; `geo` 0.33.1 far-from-origin fixes are unreleased.

**Options.** A — authoritative geodetic coordinates + per-tile/region ENU frame + floating origin; B — f64 global ECEF and transform at draw; C — wait for glTF 2.1 f64.

**Decision (proposed).** A. Store CRS + epoch + vertical datum + height unit and frame origin in the IR; do mesh maths and rendering in local ENU/tile frames; record rebasing invariants across picking, undo, physics and metadata; add golden fixtures at ~1e7 m, ±180 longitude and large projected eastings/northings. Do not design around glTF 2.1 f64.

**Rationale.** Industry-proven and the only option consistent with both the precision numbers and current GPU capability; §30 requires local-origin precision from the start.

**Consequences.** Positive: correct rendering at city/global scale; explicit datum/epoch model. Negative: origin switching is a cross-cutting invariant; tests must cover every coordinate consumer.

**Evidence.** Spec §§10, 16, 30, 35; `research/FORMAT_AND_STANDARDS.md` §§2, 4a; `research/notes/geospatial.md` §3.2.

---

## Approval checklist (before any proposal becomes an accepted ADR)

- [ ] Owner decision recorded in `DECISIONS.md` with date and scope.
- [ ] Relevant conflicts in `RESEARCH_REGISTER.md` §4 resolved against a primary source.
- [ ] Independent reviewer subagent checks the ADR against the spec and evidence.
- [ ] Accepted ADR written as its own numbered file using `adr/0001-template.md`.
- [ ] `progress.md` and `ARCHITECTURE.md` updated. No implementation until all of the above pass.
