---
id: GE-0018
title: Geometry and structural validation gates
type: feature
priority: P1
status: in-review
milestone: M1
labels: [type:feature, area:quality]
depends_on: [GE-0011, GE-0009]
blocks: [GE-0019, GE-0024]
created: 2026-11-27
updated: 2026-11-27
owner_approval_required: false
---

# GE-0018 — Geometry and structural validation gates

## Problem
Exports need explicit validation with honest limitations.

## User / engineering value
Delivers spec §50 step 10 and the baseline quality gates.

## Scope
### Included
- Format gate via glTF-Validator for later export
- Structural checks: zero-area faces, non-manifold edges, self-intersections, normals
- Metric records with reference/method/version/limitations
- No geographic accuracy claim without checkpoints

### Excluded
- Survey profile, ASPRS checkpoint ingestion, texture coverage metrics

## Context and research
- Relevant sources:
  - `engineering/TEST_STRATEGY.md`
  - `research/notes/quality-ci-gpu.md section 1`
- Relevant ADRs:
  - `A06`
- Related tickets: `TICKETS/README.md` (dependency index)

## Proposed approach
Offline validation sidecar behind a Core command; no Python in core runtime.

## Acceptance criteria
- [x] Gates produce machine-readable reports with provenance
- [x] No unqualified accuracy claim emitted
- [x] CI runs the gates
- [x] Tests and documentation updated

## Test plan
- Unit: Metric calculation tests
- Integration: Sidecar integration
- End-to-end: n/a
- Manual: n/a
- Platform coverage: macOS/Linux/Windows

## Security / licensing / data considerations
No third-party binary with unclear licence bundled.

## Dependencies and risks
Rust trimesh crate is unrelated (ODE physics); avoid name confusion.

## Implementation notes
Validation thresholds are project policy, not standards claims.

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

GitHub issue: **#8** — https://github.com/Jasowills/genesis-engine/issues/8
- Files: `crates/genesis-core/src/validation.rs`, `crates/genesis-core/tests/validation.rs`, `docs/developer/validation.md`, re-exports.
- `cargo test --workspace --locked` → **53 passed, 0 failed** (9 GeoJSON + 6 lawful-source + 8 OSM + 9 procedural + 5 validation + 7 project + 9 World IR). Five validation tests cover defect detection, no-accuracy-claim, survey-profile rejection, JSON round-trip and provenance flagging.
- `cargo clippy --workspace --locked -- -D warnings` clean; `cargo fmt --all -- --check` clean.
- Validation tests run in the cross-platform CI matrix via the existing Core test step. Self-intersection detection is explicitly recorded as `UNVERIFIED` (value -1 with limitation) and not claimed.
- Format gate via glTF-Validator is deferred: no GLB export exists yet (GE-0019). No texture coverage or survey/checkpoint ASPRS reporting in this slice.
- Reviewer result: pending independent review.

## Changelog
- 2026-11-27: Ticket created from the proposed research/architecture programme; current status: `proposed`. Pending owner approval of scope and independent review where required.
