---
id: GE-0034
title: Per-kind viewport materials
type: feature
priority: P2
status: ready
milestone: M4
labels: [type:feature, priority:P2, status:ready, area:viewport]
depends_on: [GE-0016]
blocks: []
created: 2026-11-28
updated: 2026-11-28
owner_approval_required: false
---

# GE-0034 — Per-kind viewport materials

## Problem
All scene nodes render the same grey material, so terrain, road, building and vegetation are indistinguishable.

## User / engineering value
Distinct, honest display colours make the generated world legible without textures or lighting claims.

## Scope
### Included
- Pure display_colour(kind, selected) mapping in genesis-studio scene seam
- Consume the mapping in Scene.tsx
- Rust unit test covering all four kinds plus selection

### Excluded
- Texture images
- Lighting/PBR claims
- New GeneratedKind variants

## Context and research
- Relevant sources: `research/notes/procedural.md`, `research/notes/desktop-viewport-formats.md`, `docs/product/E2E_QUALITY_VERDICT.md`
- Relevant ADRs: `adr/0002`, `adr/0003`, `adr/0007`, `adr/0008`
- Related tickets: `TICKETS/README.md`

## Proposed approach
Small tracer-bullet slice; keep existing versioned contracts and lawful-source gate. No proprietary or uncached visual assets.

## Acceptance criteria
- [ ] Four distinct colours for four kinds
- [ ] Selection colour still distinguishable
- [ ] No GLB extras or provenance change
- [ ] Existing snapshot and Studio tests remain green

## Test plan
- Unit: Rust mapping test; Integration: Studio component tests
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

GitHub issue: **#?** — https://github.com/Jasowills/genesis-engine/issues/?
Commands, output summaries, fixture references and reviewer result to be recorded here when the ticket is implemented. No claim of completion without an independent reviewer.

## Changelog
- 2026-11-28: Ticket created from the product visual-fidelity review and owner approval to proceed.
