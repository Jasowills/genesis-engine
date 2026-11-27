# ADR-0005 — ONNX Runtime primary behind a swappable AI adapter contract

**Status:** Accepted
**Date:** 2026-11-27
**Decision ID:** D-004 detail (approved as part of D-025, 2026-11-27)
**Owner approval:** Approved 2026-11-27 (Phase 8 checkpoint)
**Supersedes:** none
**Superseded by:** none
**Original proposal:** `adr/ADR_PROPOSALS.md` (A04)

**Context.** Spec §18 mandates an adapter contract with model identity, version, checksum, I/O schema, device requirements, licence and attribution. ONNX Runtime 1.32.0 (2026-11-13, MIT, Production/Stable) has the broadest EP matrix and `RunOptions::SetTerminate()`; CoreML EP is **preview** with restrictive op tables; DirectML sustained. Candle is dual MIT/Apache-2.0 but hand-written per architecture; Burn requires Rust 1.95 and is changing. No independent benchmark was run; ORT's GitHub repo was unreachable during research (version/licence verified via PyPI/docs).

**Options.** A — ORT via C ABI behind the adapter; B — Candle primary; C — multi-runtime from day one.

**Decision.** A as default local runtime, with full adapter contract (identity, SHA-256, code/weight licence, typed schemas, device/EP priority, settings, cancellation, canonical errors, per-run evidence). Candle optional where an exact implementation exists. Model shortlist: SAM ViT-B, YOLOX, Depth Anything V2 **Small**, LaMa (verify weights). Avoid Ultralytics YOLO (AGPL/paid), DA-V2 Base/Large/Giant (CC-BY-NC), DUSt3R, Hunyuan3D 2, VGGT non-commercial checkpoint.

**Rationale.** The spec forbids choosing a runtime merely because it is Rust; ORT's operator/EP coverage plus cancellation best fit a three-OS local-first Studio; the adapter keeps external providers swappable.

**Consequences.** Positive: commercially clean default path, explicit fallback logging. Negative: no first-class Rust API; dynamic-shape models may lose EP acceleration; CoreML→CPU fallback changes performance and licence posture and must be recorded.

**Evidence.** Spec §§18, 29; `research/notes/ai-ml.md` §§1–5; `research/LICENCE_AND_DATA_RIGHTS.md` §1.

---

## Reuse vs build

Wrap ONNX Runtime via C ABI; Candle optional adapter; build only the adapter contract.

## Cross-platform, performance and resource implications

Cancellation via RunOptions::SetTerminate; EP fallback must be recorded; no OS promise beyond capability detection. Rust toolchain verified locally: 1.98.1. Windows, Linux, ARM64 and CUDA remain unverified and require CI evidence.

## Security, privacy and licence implications

Code/weight licence divergence; no AGPL/non-commercial weights; no unpickling.

## Evidence

| Source | Location | Access date | Supports |
| --- | --- | --- | --- |
| Primary-source research and spec | `Spec §§18,29; notes/ai-ml.md §§1-5` | 2026-11-27 | Decision context, options, rationale |

Full citations and licence texts are in the referenced research files and `research/RESEARCH_REGISTER.md`.

## Verification plan

GE-0006 smoke test with checksums and cancellation. No implementation claim is made here.

## Notes

Accepted 2026-11-27 by owner (Phase 8, D-025). Review status: see `DECISIONS.md` §3a Phase 7 checklist; independent review is required before publication. Evidence conflicts relevant to this ADR are tracked in `research/RESEARCH_REGISTER.md` §4 and must be closed by the linked ticket before implementation.
