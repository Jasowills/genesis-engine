---
id: GE-0029
title: Blender export and round-trip fixture
type: integration
priority: P2
status: in-review
milestone: M3
labels: [type:integration, area:export]
depends_on: [GE-0019]
blocks: [GE-0030]
created: 2026-11-27
updated: 2026-11-27
owner_approval_required: false
---

# GE-0029 — Blender export and round-trip fixture

## Problem
Blender is the likely first DCC hand-off, but support must be proven by a fixture.

## User / engineering value
Spec §27 forbids claiming integration without automated/documented compatibility test.

## Scope
### Included
- Documented Blender version and import/export path
- Hierarchy/material/metadata round-trip fixture
- Known limitations documented

### Excluded
- Unreal/Unity/Godot integrations, GPL Blender code in core

## Context and research
- Relevant sources:
  - `research/notes/competitive-data-rights.md`
  - `research/notes/reconstruction.md`
- Relevant ADRs:
  - `A03`
- Related tickets: `TICKETS/README.md` (dependency index)

## Proposed approach
Invoke Blender as external process; never link GPL code into core.

## Acceptance criteria
- [x] Fixture round-trip passes with documented version
- [x] Metadata loss explicitly recorded
- [x] No GPL code in core
- [x] Tests and documentation updated

## Test plan
- Unit: n/a
- Integration: Round-trip integration
- End-to-end: n/a
- Manual: Manual Blender inspection
- Platform coverage: macOS/Linux/Windows

## Security / licensing / data considerations
GPL process boundary; output not covered by GPL.

## Dependencies and risks
Blender bake data not guaranteed portable across versions.

## Implementation notes
D-009 first integration decision still open.

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

GitHub issue: **#25** — https://github.com/Jasowills/genesis-engine/issues/25
- Files: `tools/blender_roundtrip.py` (external probe, not linked into Core), `fixtures/ikeja-blender-roundtrip.glb`, `fixtures/ikeja-blender-roundtrip-report.json`, `crates/genesis-core/tests/blender_roundtrip.rs`, `docs/developer/blender-roundtrip.md`.
- Pinned Blender **4.5.17 LTS** (build `2f5ec98689b8`) installed locally via free Homebrew cask. Probe run on a Core-exported Ikeja GLB: 2 meshes before/after, object names preserved, `genesis` extras identical (`metadata_identical: true`, `metadata_loss: []`).
- `cargo test --workspace --locked` → **114 passed, 0 failed** (2 new round-trip tests: Core re-import of the Blender GLB with provenance/attribution assertions; pinned report validation). `cargo clippy --workspace --locked --all-targets -- -D warnings` clean; `cargo fmt --all -- --check` clean.
- Metadata loss recorded: none observed for extras; Blender may rename objects/merge materials across versions; material round-trip unverified (fixture has no materials); non-macOS platforms and component overhead unmeasured.
- No GPL code in Core; Blender is an external process only.
- Reviewer result: pending independent review.

## Changelog
- 2026-11-27: Ticket created from the proposed research/architecture programme; current status: `proposed`. Pending owner approval of scope and independent review where required.
