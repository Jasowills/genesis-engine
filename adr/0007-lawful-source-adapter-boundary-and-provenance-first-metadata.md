# ADR-0007 — Lawful-source adapter boundary and provenance-first metadata

**Status:** Accepted
**Date:** 2026-11-27
**Decision ID:** new hard source constraint (D-028) (approved as part of D-025, 2026-11-27)
**Owner approval:** Approved 2026-11-27 (Phase 8 checkpoint)
**Supersedes:** none
**Superseded by:** none
**Original proposal:** `adr/ADR_PROPOSALS.md` (A06)

**Context.** Spec §§5, 12, 16, 26–27 require evidence, provenance and respect for data rights. OSM ODbL §§4.2–4.6 (§4.6 machine-readable derivative offer); Microsoft GlobalML CDLA Permissive 2.0; Google Open Buildings CC BY 4.0 or ODbL (election must be recorded); Overture theme-dependent licences; Google Maps Platform ToS §3.2.3(a)–(c) and Cesium ion ToS prohibit derivation, offline caching and AI training. No reviewed open map-to-3D tool advertises per-asset provenance.

**Options.** A — enforceable allow/deny gate + provenance in IR and export; B — documentation-only policy; C — allow all with user attestation.

**Decision.** A. Deny Google Maps/Street View/Photorealistic 3D Tiles and Cesium ion-hosted content as reconstruction inputs; permit open-standards software (CesiumJS/3D Tiles Apache-2.0) with Genesis-owned content. Record licence election per snapshot; keep structured derived data separable from rendered assets; require legal review before publishing any OSM-derived database or GLB.

**Rationale.** This is the evidence-backed differentiator and the only way to satisfy honest uncertainty and attribution-survives-export simultaneously; no standard provenance vocabulary exists.

**Consequences.** Positive: lawful multi-source fusion, reproducible revisions, defensible exports. Negative: blocks some high-fidelity commercial inputs; more bookkeeping; §4.6 offer design unresolved; generated-GLB legal boundary unverified.

**Evidence.** Spec §§5, 12, 16, 18, 26–27; `research/LICENCE_AND_DATA_RIGHTS.md` §§1–4; `research/COMPETITIVE_ANALYSIS.md` §3.

---

## Reuse vs build

Build the gate and provenance profile; no ecosystem tool provides this.

## Cross-platform, performance and resource implications

No GPU impact; provenance travels with IR and export. Rust toolchain verified locally: 1.98.1. Windows, Linux, ARM64 and CUDA remain unverified and require CI evidence.

## Security, privacy and licence implications

ODbL §4.4/§4.6; Google/Cesium ion terms; legal review of generated GLB is open (D-024).

## Evidence

| Source | Location | Access date | Supports |
| --- | --- | --- | --- |
| Primary-source research and spec | `Spec §§5,12,16,26,27; research/LICENCE_AND_DATA_RIGHTS.md §§1-4` | 2026-11-27 | Decision context, options, rationale |

Full citations and licence texts are in the referenced research files and `research/RESEARCH_REGISTER.md`.

## Verification plan

GE-0012 gate tests and export attribution fixture. No implementation claim is made here.

## Notes

Accepted 2026-11-27 by owner (Phase 8, D-025). Review status: see `DECISIONS.md` §3a Phase 7 checklist; independent review is required before publication. Evidence conflicts relevant to this ADR are tracked in `research/RESEARCH_REGISTER.md` §4 and must be closed by the linked ticket before implementation.
