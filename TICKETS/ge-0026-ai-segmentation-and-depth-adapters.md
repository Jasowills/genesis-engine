---
id: GE-0026
title: AI segmentation and depth adapters
type: integration
priority: P2
status: verified
milestone: M3
labels: [type:integration, area:ai]
depends_on: [GE-0006, GE-0023]
blocks: []
created: 2026-11-27
updated: 2026-11-27
owner_approval_required: false
---

# GE-0026 — AI segmentation and depth adapters

## Problem
AI-assisted extraction is a spec capability but requires clean licensing and swappable providers.

## User / engineering value
Delivers optional AI capability without making Python or a runtime mandatory.

## Scope
### Included
- Adapter contract for SAM ViT-B segmentation and Depth Anything V2 Small depth behind S4 (named weights NOT implemented in this slice)
- Typed I/O, cancellation, per-run evidence and licence record
- External provider adapter stub (no paid calls by default)

### Excluded
- Facade reconstruction, image-to-3D, non-commercial weights

## Context and research
- Relevant sources:
  - `research/notes/ai-ml.md`
  - `research/LICENCE_AND_DATA_RIGHTS.md`
- Relevant ADRs:
  - `A04`
- Related tickets: `TICKETS/README.md` (dependency index)

## Proposed approach
Adapter only; no capability code imports runtime types.

## Acceptance criteria
- [x] Outputs typed and provenance-recorded
- [x] Cancellation demonstrated
- [x] No non-commercial weight used
- [x] Tests and documentation updated

## Test plan
- Unit: Schema tests
- Integration: adapter contract and deterministic fake; real model integration is GE-0006 follow-up
- End-to-end: n/a
- Manual: Manual output review
- Platform coverage: macOS/Linux/Windows

## Security / licensing / data considerations
No unpickling; checksum verification; no external cost without approval.

## Dependencies and risks
Code/weight licence divergence; gated model cards.

## Implementation notes
External providers are adapters, never hard dependencies.

## Reviewer checklist
- [ ] Correctness
- [ ] Architecture and boundaries
- [ ] Error handling
- [ ] Tests
- [ ] Performance/resource use
- [ ] Security
- [ ] Licensing/provenance
- [ ] Documentation
- [ ] No unrelated scope creep

## Verification evidence

GitHub issue: **#20** — https://github.com/Jasowills/genesis-engine/issues/20
- Files: `crates/genesis-core/src/ai.rs`, `tests/ai.rs`, `docs/developer/ai-adapters.md`, lib.rs re-exports.
- `cargo test --workspace --locked` → **102 passed, 0 failed** (5 new AI adapter tests: typed outputs with per-run evidence, cancellation on both ports, non-commercial weight rejection, external provider disabled by default, invalid request/descriptor rejection). `cargo clippy --workspace --locked --all-targets -- -D warnings` clean; `cargo fmt --all -- --check` clean.
- NOT IMPLEMENTED: named model weights (SAM ViT-B, YOLOX, Depth Anything V2 Small), real ONNX inference, weight checksum verification against a loaded artefact, GPU execution providers, external provider transport. This ticket delivers the swappable typed adapter contract and deterministic fake only.
- Reviewer result: independent review **`APPROVED`** (session `ses_de1e9703dffezohnp3X22PPwI5`); canonical error codes, reduced-evidence note and scope wording applied.

## Changelog
- 2026-11-27: Ticket created from the proposed research/architecture programme; current status `in-review`; adapter-contract-only slice. Pending owner approval of scope and independent review where required.
- 2026-11-27: Independent review APPROVED with explicit adapter-contract-only limitations; ticket marked verified.
