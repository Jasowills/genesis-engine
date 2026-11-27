# ADR-0003 — Tauri + React/TS + three.js (shell and viewport as two decisions)

**Status:** Accepted
**Date:** 2026-11-27
**Decision ID:** D-011 and D-012 (two separate decisions) (approved as part of D-025, 2026-11-27)
**Owner approval:** Approved 2026-11-27 (Phase 8 checkpoint)
**Supersedes:** none
**Superseded by:** none
**Original proposal:** `adr/ADR_PROPOSALS.md` (A02)

**Context.** Spec §§17, 28 leave framework/renderer open. Verified: Tauri core 2.12.7 / CLI 2.12.6 / bundler 2.10.6 (Apache-2.0/MIT, 18 Nov 2026), updater on Win/macOS/Linux with mandatory signatures (Rust ≥1.90); Electron 45.5.0 (MIT) has no Linux built-in updater; Wails 3.1.0 (MIT, Go 1.26+). React 19.3.1 (MIT), TypeScript 7.0.2 (Apache-2.0); three.js r187 (MIT, TransformControls, BatchedMesh `batchId`, LOD hysteresis); Babylon.js 9.38.0 (Apache-2.0, GizmoManager + snapDistance); Bevy 0.20.1 / wgpu 30.0.2 / egui 0.37.0 are higher-cost alternates.

**Options.** A — Tauri + React/TS + three.js via R3F 9.8.2/drei 10.7.9; B — Electron + React/TS + three.js; C — native-only Bevy/wgpu/egui; D — Tauri + Babylon.js.

**Decision.** A, gated on a PoC: WebGL2/WebGPU capability probe in all three webviews, floating-origin test at ~1e7 m, 100k-object picking/batching test, load/unload GPU-leak test, TS 7 tooling check. Keep D as fallback if gizmo/snap work dominates. Approve **shell and viewport separately**.

**Rationale.** Tauri is the only shortlisted shell with a verified signed updater on all three OSes and a native Rust bridge; three.js has the richest editor primitives/loaders. Electron's consistent Chromium must be weighed against bundle size and the Linux updater gap.

**Consequences.** Positive: aligns with Rust-first core and React/TS UI. Negative: three webviews differ; canvas accessibility has no standard pattern (keyboard-numeric transforms must be built); TS 7 tooling unverified; viewport does not solve streaming by itself. Signing identities/key custody needed before release (Apple $99/yr; Windows ~$9.99/mo or $150–400+/yr).

**Evidence.** Spec §§17, 28; `research/notes/desktop-viewport-formats.md` §§1–6; `research/notes/quality-ci-gpu.md` §§2–3.

---

## Reuse vs build

Wrap Tauri, React, three.js/R3F; build only Studio glue and the UI↔Core seam.

## Cross-platform, performance and resource implications

PoC: WebGL2/WebGPU in WebView2/WKWebView/WebKitGTK; 100k-object picking; leak test. Rust toolchain verified locally: 1.98.1. Windows, Linux, ARM64 and CUDA remain unverified and require CI evidence.

## Security, privacy and licence implications

No secrets in UI config; signing identities require owner approval (D-022).

## Evidence

| Source | Location | Access date | Supports |
| --- | --- | --- | --- |
| Primary-source research and spec | `Spec §§17,28; notes/desktop-viewport-formats.md §§1-6; notes/quality-ci-gpu.md §2-3` | 2026-11-27 | Decision context, options, rationale |

Full citations and licence texts are in the referenced research files and `research/RESEARCH_REGISTER.md`.

## Verification plan

GE-0004 capability PoC before GE-0016. No implementation claim is made here.

## Notes

Accepted 2026-11-27 by owner (Phase 8, D-025). Review status: see `DECISIONS.md` §3a Phase 7 checklist; independent review is required before publication. Evidence conflicts relevant to this ADR are tracked in `research/RESEARCH_REGISTER.md` §4 and must be closed by the linked ticket before implementation.
