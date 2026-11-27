---
id: GE-0013
title: GeoJSON boundary import and CRS validation
type: feature
priority: P0
status: proposed
milestone: M1
labels: [type:feature, area:geospatial]
depends_on: [GE-0011, GE-0012]
blocks: [GE-0014, GE-0020]
created: 2026-11-27
updated: 2026-11-27
owner_approval_required: false
---

# GE-0013 — GeoJSON boundary import and CRS validation

## Problem
Users must define a small study area and have CRS/datum/epoch validated before generation.

## User / engineering value
Implements spec §50 steps 2-4 and the geographic-selection journey.

## Scope
### Included
- Import small documented GeoJSON boundary
- Validate CRS, epoch, vertical datum and units; explicit errors
- Store source checksum and provenance

### Excluded
- Geocoding provider, area cost estimates, boundary drawing UI

## Context and research
- Relevant sources:
  - `research/FORMAT_AND_STANDARDS.md section 4a`
  - `Spec section 22`
- Relevant ADRs:
  - `A06`
  - `A07`
- Related tickets: `TICKETS/README.md` (dependency index)

## Proposed approach
Small fixture-driven importer behind a Core command.

## Acceptance criteria
- [ ] Valid boundary imports; invalid produces structured error
- [ ] CRS/epoch mismatch reported clearly
- [ ] Provenance recorded
- [ ] Tests and documentation updated

## Test plan
- Unit: Parser and CRS unit tests
- Integration: Fixture integration
- End-to-end: n/a
- Manual: Manual import
- Platform coverage: macOS/Linux/Windows

## Security / licensing / data considerations
Treat GeoJSON content as untrusted; size limits.

## Dependencies and risks
Antimeridian edge cases deferred with explicit tests.

## Implementation notes
Fixture must be small and redistributable.

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
Commands, output summaries, fixture references and reviewer result to be recorded here when the ticket is implemented. No claim of completion without an independent reviewer.

## Changelog
- 2026-11-27: Ticket created from the proposed research/architecture programme; current status: `proposed`. Pending owner approval of scope and independent review where required.
