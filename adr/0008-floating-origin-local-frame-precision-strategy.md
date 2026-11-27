# ADR-0008 — Floating-origin / local-frame precision strategy

**Status:** Accepted
**Date:** 2026-11-27
**Decision ID:** D-010/D-012 detail (approved as part of D-025, 2026-11-27)
**Owner approval:** Approved 2026-11-27 (Phase 8 checkpoint)
**Supersedes:** none
**Superseded by:** none
**Original proposal:** `adr/ADR_PROPOSALS.md` (A07)

**Context.** Spec §§10, 30, 35. glTF core POSITION is float32 (~0.5 m ULP at Earth radius, ~1 m at 1e7 m); f64 vertex attributes need draft `KHR_accessor_float64`; WebGL/WebGPU broadly lack f64 buffers/shaders. 3D Tiles uses `RTC_CENTER` per tile, glTF has `CESIUM_RTC` (Stable); PROJ `+proj=topocentric` (since 8.0, EPSG:9836/9837) converts ECEF→ENU; `geo` 0.33.1 far-from-origin fixes are unreleased.

**Options.** A — authoritative geodetic coordinates + per-tile/region ENU frame + floating origin; B — f64 global ECEF and transform at draw; C — wait for glTF 2.1 f64.

**Decision.** A. Store CRS + epoch + vertical datum + height unit and frame origin in the IR; do mesh maths and rendering in local ENU/tile frames; record rebasing invariants across picking, undo, physics and metadata; add golden fixtures at ~1e7 m, ±180 longitude and large projected eastings/northings. Do not design around glTF 2.1 f64.

**Rationale.** Industry-proven and the only option consistent with both the precision numbers and current GPU capability; §30 requires local-origin precision from the start.

**Consequences.** Positive: correct rendering at city/global scale; explicit datum/epoch model. Negative: origin switching is a cross-cutting invariant; tests must cover every coordinate consumer.

**Evidence.** Spec §§10, 16, 30, 35; `research/FORMAT_AND_STANDARDS.md` §§2, 4a; `research/notes/geospatial.md` §3.2.

---

## Reuse vs build

Adopt proven industry pattern (3D Tiles RTC_CENTER, CESIUM_RTC); build IR frame fields and golden tests.

## Cross-platform, performance and resource implications

CPU maths + f32 GPU boundary; no f64 vertex promise; all OSes affected. Rust toolchain verified locally: 1.98.1. Windows, Linux, ARM64 and CUDA remain unverified and require CI evidence.

## Security, privacy and licence implications

No new licence; `geo` far-from-origin fixes are unreleased.

## Evidence

| Source | Location | Access date | Supports |
| --- | --- | --- | --- |
| Primary-source research and spec | `Spec §§10,16,30,35; research/FORMAT_AND_STANDARDS.md §§2,4a; notes/geospatial.md §3.2` | 2026-11-27 | Decision context, options, rationale |

Full citations and licence texts are in the referenced research files and `research/RESEARCH_REGISTER.md`.

## Verification plan

GE-0003 golden vectors at ~1e7 m and ±180°. **Measured 2026-11-27 in the throwaway prototype `prototype/precision-crs/`:** f32 ULP 0.5 m at 6.37e6 m and 1.0 m at 1e7 m; a 1 cm offset at 1e7 m is lost entirely; the f64-origin + f32-local-ENU strategy had 1.92e-6 m error for a ~130 m offset; f64 ECEF↔ENU round-trip exact to 12 decimals; antimeridian cut vectors pass. Five unit tests pass (`cargo test`, rustc 1.98.1). Full evidence: `prototype/precision-crs/RESULTS.md`. No product implementation claim is made here.

## Notes

Accepted 2026-11-27 by owner (Phase 8, D-025). Review status: see `DECISIONS.md` §3a Phase 7 checklist; independent review is required before publication. Evidence conflicts relevant to this ADR are tracked in `research/RESEARCH_REGISTER.md` §4 and must be closed by the linked ticket before implementation.
