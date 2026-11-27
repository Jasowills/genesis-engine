---
id: GE-0015
title: Procedural road surface and building volume
type: feature
priority: P1
status: in-review
milestone: M1
labels: [type:feature, area:geometry]
depends_on: [GE-0014]
blocks: [GE-0020]
created: 2026-11-27
updated: 2026-11-27
owner_approval_required: false
---

# GE-0015 — Procedural road surface and building volume

## Problem
Imported centrelines and footprints are not usable game geometry without explicit generation rules.

## User / engineering value
Delivers the first visible reconstruction capability (spec §50 step 5).

## Scope
### Included
- Centreline to road surface with simple junction handling
- Building footprint to volume using supported height/levels tags
- Deterministic named RNG with recorded seed

### Excluded
- Sidewalks, bridges, tunnels, facades, vegetation, LOD, collision

## Context and research
- Relevant sources:
  - `research/notes/procedural.md`
  - `research/ECOSYSTEM_RESEARCH.md section 2`
- Relevant ADRs:
  - `A01`
  - `A06`
- Related tickets: `TICKETS/README.md` (dependency index)

## Proposed approach
Internal procedural generator; wrap OSM2World only as an optional benchmark, not core.

## Acceptance criteria
- [x] Same inputs+seed produce identical output
- [x] Unsupported attributes reported as inferred or unknown
- [x] Generated assets carry provenance
- [x] Tests and documentation updated

## Test plan
- Unit: Determinism golden vectors
- Integration: Fixture integration
- End-to-end: n/a
- Manual: Visual inspection
- Platform coverage: macOS/Linux/Windows

## Security / licensing / data considerations
No non-commercial assets; no external data fetch.

## Dependencies and risks
No mature incremental contract yet; keep dependency graph explicit.

## Implementation notes
Selective regeneration design belongs to GE-0027.

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

GitHub issue: **#13** — https://github.com/Jasowills/genesis-engine/issues/13
- Files: `crates/genesis-core/src/procedural.rs`, `crates/genesis-core/tests/procedural.rs`, `docs/developer/procedural.md`, `Cargo.toml` (rand 0.9, rand_chacha 0.9), re-exports.
- `cargo test --workspace --locked` → **43 passed, 0 failed** (9 GeoJSON + 6 lawful-source + 8 OSM + 4 procedural + 7 project + 9 World IR). Four procedural tests cover determinism, evidence tiers/named ChaCha8 seed, provenance + `verify_mesh` + golden vertex counts, and gate/invalid requests.
- `cargo clippy --workspace --locked -- -D warnings` clean; `cargo fmt --all -- --check` clean.
- Both generators call the lawful-source gate. No LOD/collision/facade/bridge/sidewalk code.
- Reviewer result: pending independent review.

## Changelog
- 2026-11-27: Ticket created from the proposed research/architecture programme; current status: `proposed`. Pending owner approval of scope and independent review where required.
