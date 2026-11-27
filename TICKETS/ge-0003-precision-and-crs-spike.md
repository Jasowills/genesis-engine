---
id: GE-0003
title: Precision and CRS golden-vector spike
type: spike
priority: P0
status: in-review
milestone: M0
labels: [spike, area:geometry]
depends_on: []
blocks: [GE-0004, GE-0007, GE-0011]
created: 2026-11-27
updated: 2026-11-27
owner_approval_required: false
---

# GE-0003 — Precision and CRS golden-vector spike

## Problem
glTF float32 positions imply roughly 0.5 m ULP at Earth-radius coordinates; origin rebasing must be correct before projects exist.

## User / engineering value
Directly answers spec questions on internal CRS and precision loss far from origin; foundational and hard to reverse.

## Scope
### Included
- Prototype authoritative geodetic + per-tile ENU/local frame
- Golden vectors at ~1e7 m, +/-180 longitude, large projected coordinates
- Document rebasing invariants

### Excluded
- Building the full World IR or viewport

## Context and research
- Relevant sources:
  - `research/FORMAT_AND_STANDARDS.md section 2 and 4a`
  - `research/notes/geospatial.md section 3.2`
  - `research/notes/desktop-viewport-formats.md section 6`
- Relevant ADRs:
  - `A07`
  - `A01`
- Related tickets: `TICKETS/README.md` (dependency index)

## Proposed approach
Throwaway prototype producing decision-rich snippets (state/transform shape), kept as primary source; fold findings into A07 and IR requirements.

## Acceptance criteria
- [x] Prototype measures and records coordinate error in local frames
- [x] Golden vectors reproducible and committed
- [x] ADR A07 evidence updated with measured result
- [x] Tests and documentation updated

## Test plan
- Unit: Coordinate transform unit checks
- Integration: Prototype round-trip across frames
- End-to-end: n/a
- Manual: Manual inspection of jitter
- Platform coverage: macOS host; CI Linux/Windows afterwards

## Security / licensing / data considerations
No external data or credentials.

## Dependencies and risks
geo 0.33.1 fixes are unreleased; include far-from-origin regression fixtures.

## Implementation notes
Prototype is evidence, not product code; do not mark verified without independent review.

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

GitHub issue: **#1** — https://github.com/Jasowills/genesis-engine/issues/1
- `cd prototype/precision-crs && cargo test` → **7 passed, 0 failed** (rustc 1.98.1), including absolute WGS84→ECEF/ENU-basis golden vectors and negative-ULP coverage.
- `cargo run --quiet` → f32 ULP 0.5 m @ 6.37e6 m, 1.0 m @ 1e7 m, 2.0 m @ 2.00375e7 m; naive 1 cm offset at 1e7 m lost (stored delta 0.0 m); f64 origin + f32 local ENU error 1.92e-6 m; ECEF↔ENU round trip 0.0 m; antimeridian cuts at ±180°.
- Evidence files: `prototype/precision-crs/src/main.rs`, `prototype/precision-crs/RESULTS.md`, `prototype/precision-crs/README.md`.
- `adr/0008-floating-origin-local-frame-precision-strategy.md` Verification plan updated with measured results.
- Reviewer result: first pass `CHANGES_REQUESTED` on test strength (inverse-only round trip) and a one-character transcript error; both fixed, re-review pending.

## Changelog
- 2026-11-27: Ticket created from the proposed research/architecture programme; current status: `ready`. Pending owner approval of scope and independent review where required.
- 2026-11-27: Prototype implemented and tested; status moved to in-review pending independent review.
- 2026-11-27: Added absolute golden vectors, verbatim transcript and known limits; re-review requested.
