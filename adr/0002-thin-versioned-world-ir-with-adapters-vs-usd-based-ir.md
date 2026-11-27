# ADR-0002 — Thin versioned World IR with adapters vs USD-based IR

**Status:** Accepted
**Date:** 2026-11-27
**Decision ID:** D-010 (approved as part of D-025, 2026-11-27)
**Owner approval:** Approved 2026-11-27 (Phase 8 checkpoint)
**Supersedes:** none
**Superseded by:** none
**Original proposal:** `adr/ADR_PROPOSALS.md` (A01)

**Context.** Spec §30 requires an engine-agnostic versioned IR with stable IDs, CRS/frame, provenance, confidence, seeds, partial updates, migrations and export mapping, and forbids one giant inline-geometry JSON. glTF 2.0 is explicitly a transmission format, not authoring/streaming; 3D Tiles 1.1 is streaming-oriented; neither carries provenance/confidence. OpenUSD v26.11.1 has layers/references/variants/payloads but a heavy C++ dependency and the Tomorrow Open Source Technology License 1.0 (Apache-2.0 with modified trademark §6) requiring legal review.

**Options.** A — thin purpose-built versioned IR + adapters; B — USD-based IR; C — glTF/GLB as internal format.

**Decision.** A. Study USD concepts, do not take the C++/licence dependency in the first slice; GLB is the first interchange export.

**Rationale.** A satisfies §30 and the provenance-first differentiator, keeps the core Rust-first and licence-clean; B adds an unapproved licence and dependency before the slice validates the architecture; C is contradicted by the Khronos spec itself and by glTF `extras` having no shared schema.

**Consequences.** Positive: stable IDs/migrations/selective regeneration are first-class. Negative: no ecosystem tooling; schema and round-trip tests are net-new; risk of reinventing USD concepts.

**Evidence.** Spec §§16, 30; `research/FORMAT_AND_STANDARDS.md` §§1–2; `research/ECOSYSTEM_RESEARCH.md` §2. **Re-verification:** none blocking; USD licence review still open.

---

## Reuse vs build

Integrate `geo`/`geozero`, wrap GLB/3D Tiles adapters; build only the thin IR.

## Cross-platform, performance and resource implications

No GPU change; schema migrations must be tested on all three OSes. Rust toolchain verified locally: 1.98.1. Windows, Linux, ARM64 and CUDA remain unverified and require CI evidence.

## Security, privacy and licence implications

No new licence; provenance fields are data, not executable input.

## Evidence

| Source | Location | Access date | Supports |
| --- | --- | --- | --- |
| Primary-source research and spec | `Spec §§16,30; research/FORMAT_AND_STANDARDS.md §1-2; notes/desktop-viewport-formats.md §7` | 2026-11-27 | Decision context, options, rationale |

Full citations and licence texts are in the referenced research files and `research/RESEARCH_REGISTER.md`.

## Verification plan

GE-0011 golden round-trip and migration fixtures. No implementation claim is made here.

## Notes

Accepted 2026-11-27 by owner (Phase 8, D-025). Review status: see `DECISIONS.md` §3a Phase 7 checklist; independent review is required before publication. Evidence conflicts relevant to this ADR are tracked in `research/RESEARCH_REGISTER.md` §4 and must be closed by the linked ticket before implementation.
