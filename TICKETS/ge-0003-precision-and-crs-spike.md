---
id: GE-0003
title: Precision and CRS golden-vector spike
type: spike
priority: P0
status: ready
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
- [ ] Prototype measures and records coordinate error in local frames
- [ ] Golden vectors reproducible and committed
- [ ] ADR A07 evidence updated with measured result
- [ ] Tests and documentation updated

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
Commands, output summaries, fixture references and reviewer result to be recorded here when the ticket is implemented. No claim of completion without an independent reviewer.

## Changelog
- 2026-11-27: Ticket created from the proposed research/architecture programme; current status: `ready`. Pending owner approval of scope and independent review where required.
