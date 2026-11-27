---
id: GE-0013
title: GeoJSON boundary import and CRS validation
type: feature
priority: P0
status: in-review
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
- [x] Valid boundary imports; invalid produces structured error
- [x] CRS/epoch mismatch reported clearly (projected CRS over lon/lat and missing epoch now rejected)
- [x] Provenance recorded
- [x] Tests and documentation updated

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

GitHub issue: **#11** — https://github.com/Jasowills/genesis-engine/issues/11
- Files: `crates/genesis-core/src/geojson_boundary.rs`, `crates/genesis-core/tests/geojson_boundary.rs`, `docs/developer/boundary-import.md`, re-exports.
- `cargo test --workspace --locked` → **31 passed, 0 failed** (7 project + 9 World IR + 6 lawful-source + 9 GeoJSON boundary), including known-answer SHA-256/ID, bbox `[3.3480, 6.5985, 3.3552, 6.6051]` matching `research/BENCHMARK_AREA.md`, CRS mismatch/epoch rejection, malformed/empty/multiple-feature/hole/degenerate geometry, gate-before-parse and antimeridian heuristic.
- `cargo clippy --workspace --locked -- -D warnings` → clean; `cargo fmt --all -- --check` → clean.
- Import is gated by `check_source`; SHA-256 source checksum and derived stable boundary ID recorded. No global cache.
- Reviewer result: first pass `CHANGES_REQUESTED` (unimplemented CRS mismatch, test gaps, stale bookkeeping); fixes applied, re-review pending.

## Changelog
- 2026-11-27: Ticket created from the proposed research/architecture programme; current status: `proposed`. Pending owner approval of scope and independent review where required.
