---
id: GE-0033
title: Wire secondary layers into the product generate journey
type: feature
priority: P1
status: ready
milestone: M4
labels: [type:feature, priority:P1, status:ready, area:geometry]
depends_on: [GE-0028, GE-0030, GE-0022]
blocks: []
created: 2026-11-28
updated: 2026-11-28
owner_approval_required: false
---

# GE-0033 — Wire secondary layers into the product generate journey

## Problem
The shipped CLI generates only buildings and roads; terrain, markings, sidewalks and vegetation exist in Core but never reach payloads/meshes.json, the GLB or Studio.

## User / engineering value
A user asking for a world gets a visibly richer, source-attributed asset set without new generators.

## Scope
### Included
- Optional --layers <dir> on genesis generate using the four bundled lawful fixtures
- All generated layer meshes appended to payloads/meshes.json with the project frame
- Run seed and generator version recorded on every layer mesh (replacing hard-coded zeros)
- Denied layer source produces a structured error and no partial write

### Excluded
- New layer algorithms
- Texture/image assets
- Facade or bridge modelling

## Context and research
- Relevant sources: `research/notes/procedural.md`, `research/notes/desktop-viewport-formats.md`, `docs/product/E2E_QUALITY_VERDICT.md`
- Relevant ADRs: `adr/0002`, `adr/0003`, `adr/0007`, `adr/0008`
- Related tickets: `TICKETS/README.md`

## Proposed approach
Small tracer-bullet slice; keep existing versioned contracts and lawful-source gate. No proprietary or uncached visual assets.

## Acceptance criteria
- [ ] Same inputs and seed produce an identical mesh_subject_hash
- [ ] With Ikeja fixtures the scene reports 7 meshes and includes layers-v0.1
- [ ] Every mesh passes verify_mesh and retains lawful provenance
- [ ] A denied source fixture produces exit 1 with error.kind cli_error

## Test plan
- Unit: CLI integration assertions; Integration: product_e2e.py harness; E2E: headless journey only
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

GitHub issue: **#33** — https://github.com/Jasowills/genesis-engine/issues/33
Commands, output summaries, fixture references and reviewer result to be recorded here when the ticket is implemented. No claim of completion without an independent reviewer.

## Changelog
- 2026-11-28: Ticket created from the product visual-fidelity review and owner approval to proceed.
