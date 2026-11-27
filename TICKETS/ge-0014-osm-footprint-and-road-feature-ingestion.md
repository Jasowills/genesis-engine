---
id: GE-0014
title: OSM footprint and road feature ingestion
type: feature
priority: P1
status: proposed
milestone: M1
labels: [type:feature, area:geospatial]
depends_on: [GE-0013]
blocks: [GE-0015]
created: 2026-11-27
updated: 2026-11-27
owner_approval_required: false
---

# GE-0014 — OSM footprint and road feature ingestion

## Problem
Genesis needs sourced road and building features from documented small OSM extracts.

## User / engineering value
Provides the evidence layer for the first vertical slice.

## Scope
### Included
- Parse small OSM extract/footprint fixture
- Retain source feature IDs, version, timestamp and licence
- Merge with World IR with observed/inferred flags

### Excluded
- Planet-scale ingestion, PostGIS, osm2pgsql, Google/Cesium content

## Context and research
- Relevant sources:
  - `research/notes/geospatial.md`
  - `research/notes/procedural.md`
- Relevant ADRs:
  - `A06`
- Related tickets: `TICKETS/README.md` (dependency index)

## Proposed approach
Pure-Rust PBF/GeoJSON path; no external GPL tool in this ticket.

## Acceptance criteria
- [ ] Fixture features import with stable source IDs
- [ ] Sourced vs inferred attributes distinguished
- [ ] OSM attribution recorded
- [ ] Tests and documentation updated

## Test plan
- Unit: Parser unit tests
- Integration: Fixture integration
- End-to-end: n/a
- Manual: n/a
- Platform coverage: macOS/Linux/Windows

## Security / licensing / data considerations
No raw OSM bundle committed; attribution text preserved.

## Dependencies and risks
OSM2World licence conflict unresolved; do not vendor it here.

## Implementation notes
Use a small, documented extract.

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
