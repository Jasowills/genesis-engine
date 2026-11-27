---
id: GE-0028
title: Terrain and secondary layers
type: feature
priority: P2
status: in-review
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
- [x] Each new layer has its own fixture and validation
- [x] No high-resolution claim from low-resolution DEM
- [x] Attribution preserved
- [x] Tests and documentation updated

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
- Files: `crates/genesis-core/src/layers.rs`, `tests/layers.rs`, `docs/developer/layers.md`, `fixtures/ikeja-dem.json`, `fixtures/ikeja-markings.geojson`, `fixtures/ikeja-vegetation.geojson`, `fixtures/ikeja-sidewalks.geojson`, semantic `GeneratedKind` variants propagated to Studio.
- `cargo test --workspace --locked` → **86 passed, 0 failed** (5 layer tests: 3×3 DEM → 9 vertices/24 indices with recorded 30 m resolution and 12 m accuracy, invalid DEM rejection, markings/sidewalks/vegetation with separate fixtures and evidence, source-gate enforcement, +Z winding and true frame origins across all layers, observed-marking evidence branch and OSM attribution retention).
- `cargo clippy --workspace --locked -- -D warnings` clean; `cargo fmt --all -- --check` clean.
- No high-resolution or survey claim; resolution/claim recorded as parameters; all layer evidence is observed/derived/inferred explicitly.
- Reviewer result: first pass `CHANGES_REQUESTED` (inverted winding, false frame origins, misattributed commit, hardcoded vegetation origin, overstated fixture/attribution claims); fixes applied, re-review pending.

## Changelog
- 2026-11-27: Ticket created from the proposed research/architecture programme; current status: `proposed`. Pending owner approval of scope and independent review where required.
