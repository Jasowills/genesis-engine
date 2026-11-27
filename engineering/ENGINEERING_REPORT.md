# Genesis Engine — Engineering Report

**Date:** 2026-11-27. **Status:** Pre-implementation assessment. **Scope:** research and architecture discovery only; no product code, no builds, no test suites. All facts below are verified in the referenced files; estimates are labelled.

## 1. Current architecture and implemented boundaries

No product code is implemented. The architecture **was accepted by the owner on 2026-11-27** (D-025): `ARCHITECTURE.md` is the accepted synthesis and `adr/0002`–`adr/0008` are accepted ADRs. `engineering/ARCHITECTURE_DISCOVERY.md` and `adr/ADR_PROPOSALS.md` remain as the evidence trail. Mandatory boundary rule: UI owns presentation only; all domain truth lives behind Core versioned commands/events/data contracts. Shell/viewport PoC gates and the lawful-source and precision spikes are retained.

## 2. What is working and verified

- Environment and skill provenance: `engineering/ENVIRONMENT_REPORT.md`, `engineering/WORKFLOW.md`.
- Research: eight primary-source briefs in `research/notes/*.md`, five consolidated deliverables in `research/*.md`, eight logged conflicts in `research/RESEARCH_REGISTER.md` §4.
- Planning: `PRD.md`, `ARCHITECTURE.md`, `ROADMAP.md`, `DECISIONS.md`, `adr/ADR_PROPOSALS.md`, `adr/0001-template.md`, `TICKETS/README.md` and 32 individual ticket files.
- Engineering baselines: `TEST_STRATEGY.md`, `SECURITY_MODEL.md`, `PERFORMANCE_BUDGETS.md`, `RELEASE_PROCESS.md`.
- Verification commands actually run: git/gh status and auth, tool `--version`, `sw_vers`/`sysctl`/`system_profiler`, `rustup target list`, filesystem listing, skill hash comparison, `gh issue create --help`, GitHub API read-only queries.

## 3. What is incomplete

No **product** code, crates or UI. Four M0 tickets are independently verified (GE-0001, GE-0002, GE-0003, GE-0008). Two spikes are deliberately `blocked`, not `verified`: GE-0005 (plugin sandbox: WASIp1 network-policy proof unavailable) and GE-0006 (ORT 1.32.0 proven on Ubuntu CI; named model weights not loaded). GE-0009 CI skeleton is in review pending its `--locked` Actions run; GE-0007 is blocked on remaining owner answers; GE-0004 is proposed pending GE-0003. Seven ADRs accepted and four owner answers recorded (D-025…D-028). Still open: D-007…D-009, D-013…D-018, D-021…D-024 and Q1…Q8. Eight evidence conflicts unresolved. No benchmark or performance measurement performed. A public repository was approved (D-027) and setup is complete and verified (GE-0008).

## 4. Quality of implementation and known limitations

Not applicable yet — nothing implemented. The *plan* is evidence-backed: licence obligations, versions, capabilities and maintenance signals were re-verified on 2026-11-27 rather than inherited. Weak areas are marked explicitly: performance numbers are estimates; several vendor EULAs and repository licences are unverified; macOS sandbox guidance is thin; no independent cross-runtime or cross-viewport benchmark was run.

## 5. Performance measurements and methodology

None measured. `engineering/PERFORMANCE_BUDGETS.md` records verified anchors (GitHub runner specs, COLMAP's SIFT memory formula) and clearly labelled **[E]** estimates. All numeric claims must be replaced with fixture measurements before publication.

## 6. Cross-platform status

Verified local: macOS 26.5.2 x86_64, 16 GiB, Intel UHD 630 + Radeon Pro 5300M. Windows, Linux, ARM64, Apple Silicon and CUDA are **untested** and require CI evidence. Platform matrix is recorded in `research/notes/quality-ci-gpu.md` §2.

## 7. Security and licence risks

Highest: (1) ODbL §4.4/§4.6 treatment of distributable project files and generated GLBs — needs legal review; (2) Google Maps/Cesium ion terms prohibit the core derivation workflow — must be enforced in code; (3) code-vs-weight licence divergence in AI models; (4) AGPL/MPL/GPL worker boundaries; (5) Wasmtime CVE density and permissive resource defaults; (6) untrusted archives/model files. Full register: `engineering/SECURITY_MODEL.md`, `research/LICENCE_AND_DATA_RIGHTS.md`.

## 8. Dependency health and upgrade risks

Verified active: Tauri 2.12.7, React 19.3.1, TypeScript 7.0.2, three.js r187, Babylon 9.38.0, Wasmtime 51.0.0, ONNX Runtime 1.32.0, COLMAP 4.2.1, Open3D 0.20.0, GDAL 3.13.3, PROJ 9.9.0, GEOS 3.15.1, `geo` 0.33.1, `geozero` 0.15.1. Slow/stale or risky: Nerfstudio (last release 2024-11-11), INRIA 3DGS/SuGaR (stale + non-commercial), Microsoft legacy footprint repos (archived), gen-map (source unverifiable). MSRV creep: `rstar` 0.14 needs 1.90; `geoparquet`/`geoarrow` 0.10 need 1.95; Tauri updater needs ≥1.90. Local toolchain is Rust 1.98.1, so current MSRVs are satisfiable.

## 9. Technical debt

None in code. Planned-design debt: precision/IR schema must be right before projects exist; incremental regeneration has no mature ecosystem contract; plugin manifest/rollback design is inference; no standard provenance vocabulary exists.

## 10. Reliability, failure handling and test coverage

Planned in `engineering/TEST_STRATEGY.md`: six layers, validation gates, bounded resources, structured errors, cancellation mapping, independent reviewer for every code ticket. Coverage is currently zero because there is no code.

## 11. Deviations from approved architecture

The accepted ADRs were independently reviewed on 2026-11-27; first pass returned `CHANGES_REQUESTED` on governance records (stale progress/register/ticket states, duplicate ADR headers, licence scope). Those findings are being resolved before repository publication. No product-code deviation is possible yet.

## 12. Recommended engineering improvements / next priorities

1. Close the reviewer's governance findings; re-review; then complete GitHub setup (Phase 7).
2. Resolve the eight evidence conflicts, especially OSM2World licence, WASI TCP defaults and ORT repo availability.
3. Run the P0/P1 spikes: precision/CRS golden vectors (GE-0003), shell/viewport capability PoC (GE-0004), AI runtime smoke test (GE-0006), plugin feasibility (GE-0005).
4. Obtain owner decision on first-slice layer scope (D-013) — the PRD §5 boundary is still proposed.
5. Legal review of ODbL §4.6 and AGPL worker boundaries before any publication; confirm SDK/docs/fixture/model-weight licences.
6. Establish CI skeleton and frozen Ikeja benchmark boundary (D-021) once approved.
