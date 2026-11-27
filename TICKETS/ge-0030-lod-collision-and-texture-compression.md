---
id: GE-0030
title: LOD, collision and texture compression
type: feature
priority: P3
status: in-review
milestone: M4
labels: [type:feature, area:geometry, area:export]
depends_on: [GE-0029]
blocks: [GE-0031]
created: 2026-11-27
updated: 2026-11-27
owner_approval_required: false
---

# GE-0030 — LOD, collision and texture compression

## Problem
Game-ready claims require collision, LOD chains and budgeted textures.

## User / engineering value
Spec §26 game-readiness dimension and §27 export requirements.

## Scope
### Included
- LOD chain generation with validation
- Collision proxy generation with fixture-based coverage checks
- KTX2/Basis and Draco/meshopt export allowlist

### Excluded
- Full target-engine optimisation

## Context and research
- Relevant sources:
  - `research/notes/desktop-viewport-formats.md sections 7-8`
  - `research/notes/quality-ci-gpu.md`
- Relevant ADRs:
- Related tickets: `TICKETS/README.md` (dependency index)

## Proposed approach
Delivery-time compression only; source IR stays uncompressed.

## Acceptance criteria
- [x] Budgets verified against named target profile
- [x] Collision fixture coverage measured
- [x] Precision regression tests pass
- [x] Tests and documentation updated

## Test plan
- Unit: Geometry tests
- Integration: Export integration
- End-to-end: n/a
- Manual: Manual tool inspection
- Platform coverage: macOS/Linux/Windows

## Security / licensing / data considerations
No unlicensed codec distribution.

## Dependencies and risks
Collision coverage definition is project-specific.

## Implementation notes
Calibrate budgets with measured fixtures.

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

GitHub issue: **#26** — https://github.com/Jasowills/genesis-engine/issues/26
- Files: `crates/genesis-core/src/optimization.rs`, `tests/optimization.rs`, `docs/developer/optimization.md`, lib.rs re-exports.
- `cargo test --workspace --locked` → **117 passed, 0 failed** (3 new optimization tests: named-budget LOD chain with frame/seed/provenance preservation and determinism, collision AABB coverage including a deliberately incomplete proxy, licence-aware codec allowlist). `cargo clippy --workspace --locked --all-targets -- -D warnings` clean; `cargo fmt --all -- --check` clean.
- NOT IMPLEMENTED: visually faithful simplification, real KTX2/Basis/Draco/meshopt encoders, codec binaries, texture transcoding, engine-specific optimisation (GE-0031), measured frame-time/memory budgets. LODs are deterministic screening levels; budgets are policy thresholds.
- Reviewer result: pending independent review.

## Changelog
- 2026-11-27: Ticket created from the proposed research/architecture programme; current status: `proposed`. Pending owner approval of scope and independent review where required.
