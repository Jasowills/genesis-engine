---
id: GE-0020
title: End-to-end vertical slice and benchmark harness
type: test
priority: P1
status: in-review
milestone: M1
labels: [type:test, area:core]
depends_on: [GE-0013, GE-0014, GE-0015, GE-0016, GE-0017, GE-0018, GE-0019]
blocks: [GE-0022, GE-0027, GE-0028]
created: 2026-11-27
updated: 2026-11-27
owner_approval_required: false
---

# GE-0020 — End-to-end vertical slice and benchmark harness

## Problem
No single automated test proves the architecture works end to end.

## User / engineering value
The spec requires a small complete workflow and benchmark-first evidence.

## Scope
### Included
- E2E from project create to export and reopen on one fixture
- Benchmark harness with frozen inputs and recorded boundary
- Runtime/memory/disk measurements reported as ranges

### Excluded
- Full-city benchmark, ground-truth accuracy claim

## Context and research
- Relevant sources:
  - `Spec sections 49-50`
  - `research/COMPETITIVE_ANALYSIS.md section 4`
- Relevant ADRs:
  - `A01`
  - `A02`
  - `A06`
  - `A07`
- Related tickets: `TICKETS/README.md` (dependency index)

## Proposed approach
Drive the same Core seams headlessly; UI is optional in the harness.

## Acceptance criteria
- [x] E2E passes on all three OSes
- [x] Boundary and input revisions recorded by owner (D-031 delegated boundary selection to the agent; recorded in research/BENCHMARK_AREA.md with input SHA-256)
- [x] No accuracy claim without independent checkpoints
- [x] Tests and documentation updated

## Test plan
- Unit: n/a
- Integration: E2E fixture
- End-to-end: Full journey
- Manual: n/a
- Platform coverage: Windows/macOS/Linux

## Security / licensing / data considerations
Only redistributable small fixtures.

## Dependencies and risks
Exact Ikeja boundary must be owner-selected before comparative benchmark.

## Implementation notes
Do not compare outputs from different source data as an objective benchmark.

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

GitHub issue: **#17** — https://github.com/Jasowills/genesis-engine/issues/17
- Files: `crates/genesis-core/tests/vertical_slice.rs` (1 E2E test), `fixtures/ikeja-boundary.geojson`, `scripts/benchmark_slice.sh`, `docs/developer/vertical-slice.md`.
- `cargo test -p genesis-core --test vertical_slice -- --nocapture` → **1 passed, 0 failed**; JSON evidence: boundary bbox `[3.3480, 6.5985, 3.3552, 6.6051]`, boundary SHA-256 `41f8a94f…35a52`, OSM SHA-256 `042135a1…64d1`, 2 meshes, 14 triangles, `geographic_accuracy_claim: "none"`, `ground_truth_available: false`, empty `benchmark_comparisons`.
- `cargo test --workspace --locked` → **65 passed, 0 failed** (64 previous + E2E); runs on Ubuntu/Windows/macOS CI matrix.
- No accuracy claim without checkpoints; ground truth unavailable; memory/disk peak UNVERIFIED; competitor comparisons deferred (no same-input outputs).
- Reviewer result: substantive checks **`APPROVED`** (session `ses_de22f2231ffect4Jr00prVHKVA`); index/changelog drift fixed, ticket marked verified after final confirmation.

## Changelog
- 2026-11-27: Ticket created from the proposed research/architecture programme; current status: `in-review` after implementation; independent review requested.
