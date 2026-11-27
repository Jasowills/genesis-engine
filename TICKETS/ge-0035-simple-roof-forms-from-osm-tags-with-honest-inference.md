---
id: GE-0035
title: Simple roof forms from OSM tags with honest inference
type: feature
priority: P2
status: ready
milestone: M4
labels: [type:feature, priority:P2, status:ready, area:geometry]
depends_on: [GE-0015]
blocks: []
created: 2026-11-28
updated: 2026-11-28
owner_approval_required: false
---

# GE-0035 — Simple roof forms from OSM tags with honest inference

## Problem
Buildings are flat-topped prisms even when roof tags or a seeded inference could produce a recognisable roof.

## User / engineering value
A visibly more building-like asset set while recording exactly which roof attributes were sourced or inferred.

## Scope
### Included
- Support flat, gabled, hipped and pyramidal roof shapes
- Read roof:shape/height/angle tags when present; infer otherwise
- Record roof_height_source and unsupported_roof_shape parameters
- Bump building generator version

### Excluded
- Facade openings
- Real facade reconstruction
- Non-commercial imagery

## Context and research
- Relevant sources: `research/notes/procedural.md`, `research/notes/desktop-viewport-formats.md`, `docs/product/E2E_QUALITY_VERDICT.md`
- Relevant ADRs: `adr/0002`, `adr/0003`, `adr/0007`, `adr/0008`
- Related tickets: `TICKETS/README.md`

## Proposed approach
Small tracer-bullet slice; keep existing versioned contracts and lawful-source gate. No proprietary or uncached visual assets.

## Acceptance criteria
- [ ] Golden mesh vectors per supported shape
- [ ] Top-cap +Z winding and verify_mesh pass
- [ ] Sourced vs inferred recorded in parameters and provenance
- [ ] Lawful-source gate still enforced

## Test plan
- Unit: procedural geometry tests; Integration: CLI generate with the Ikeja fixture
- Platform coverage: three-OS CI matrix

## Security / licensing / data considerations
Hard-denied sources remain rejected (D-028). No map imagery, no textured assets, no unsupported quality claim.

## Dependencies and risks
Generator version and seed changes affect stable IDs; record them in evidence. Synthetic DEM and inferred layers must never be labelled surveyed.

## Implementation notes
Proceed only after blockers are verified; evidence and independent review required before `verified`.

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

GitHub issue: **#35** — https://github.com/Jasowills/genesis-engine/issues/35
Commands, output summaries, fixture references and reviewer result to be recorded here when the ticket is implemented. No claim of completion without an independent reviewer.

## Changelog
- 2026-11-28: Ticket created from the product visual-fidelity review and owner approval to proceed.
