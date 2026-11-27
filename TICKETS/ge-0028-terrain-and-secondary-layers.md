---
id: GE-0028
title: Terrain and secondary layers
type: feature
priority: P2
status: proposed
milestone: M3
labels: [type:feature, area:geometry]
depends_on: [GE-0020]
blocks: []
created: 2026-11-27
updated: 2026-11-27
owner_approval_required: false
---

# GE-0028 — Terrain and secondary layers

## Problem
The first slice covers only roads and simple buildings; more layers are needed for useful worlds.

## User / engineering value
Extends generation coverage while preserving provenance and quality policy.

## Scope
### Included
- Terrain from small DEM/COG fixture with source resolution and accuracy recorded
- Sidewalks, curbs, road markings and basic vegetation as separately sourced layers
- Each layer records sourced vs inferred attributes

### Excluded
- Bridges/tunnels full modelling, facade reconstruction, hydrology

## Context and research
- Relevant sources:
  - `Spec sections 11, 23`
  - `research/notes/geospatial.md`
- Relevant ADRs:
  - `A06`
- Related tickets: `TICKETS/README.md` (dependency index)

## Proposed approach
Vertical slices per layer; do not create one giant layer ticket.

## Acceptance criteria
- [ ] Each new layer has its own fixture and validation
- [ ] No high-resolution claim from low-resolution DEM
- [ ] Attribution preserved
- [ ] Tests and documentation updated

## Test plan
- Unit: Per-layer unit tests
- Integration: Fixture integration
- End-to-end: n/a
- Manual: Visual inspection
- Platform coverage: macOS/Linux/Windows

## Security / licensing / data considerations
No restricted imagery.

## Dependencies and risks
DEM vertical datum ambiguity must be recorded.

## Implementation notes
Split by layer if review diff is too large.

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

GitHub issue: **#24** — https://github.com/Jasowills/genesis-engine/issues/24
Commands, output summaries, fixture references and reviewer result to be recorded here when the ticket is implemented. No claim of completion without an independent reviewer.

## Changelog
- 2026-11-27: Ticket created from the proposed research/architecture programme; current status: `proposed`. Pending owner approval of scope and independent review where required.
