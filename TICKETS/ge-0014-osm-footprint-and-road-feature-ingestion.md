---
id: GE-0014
title: OSM footprint and road feature ingestion
type: feature
priority: P1
status: in-review
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
- [x] Fixture features import with stable source IDs
- [x] Sourced vs inferred attributes distinguished
- [x] OSM attribution recorded
- [x] Tests and documentation updated

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

GitHub issue: **#12** — https://github.com/Jasowills/genesis-engine/issues/12
- Files: `crates/genesis-core/src/osm_ingest.rs`, `crates/genesis-core/tests/osm_ingest.rs`, `docs/developer/osm-ingest.md`, re-exports.
- `cargo test --workspace --locked` → **39 passed, 0 failed** (9 GeoJSON + 6 lawful-source + 8 OSM + 7 project + 9 World IR); 8 OSM tests cover golden IDs, shoelace/length-weighted positions, source IDs/version/timestamp, top-level feature IDs, evidence tiers, discriminating geometry errors, 10k cap, gate ordering and denied `to_world_objects`.
- `cargo clippy --workspace --locked -- -D warnings` clean; `cargo fmt --all -- --check` clean.
- Source and CRS gates precede parsing in both `import_osm_geojson` and `to_world_objects`; 10,000-feature cap; no external GPL tool or PBF dependency. `building:levels` is derived-with-confidence, never measured.
- Reviewer result: first pass `CHANGES_REQUESTED` (conversion-API gate bypass, non-discriminating geometry tests, centroid/evidence overclaims, top-level IDs, bookkeeping); fixes applied, re-review pending.

## Changelog
- 2026-11-27: Ticket created from the proposed research/architecture programme; current status: `in-review`; independent review requested.
