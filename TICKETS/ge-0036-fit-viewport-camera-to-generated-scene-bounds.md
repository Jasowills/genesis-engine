---
id: GE-0036
title: Fit viewport camera to generated scene bounds
type: feature
priority: P2
status: ready
milestone: M4
labels: [type:feature, priority:P2, status:ready, area:studio]
depends_on: [GE-0016, GE-0033]
blocks: []
created: 2026-11-28
updated: 2026-11-28
owner_approval_required: false
---

# GE-0036 — Fit viewport camera to generated scene bounds

## Problem
The viewport camera is hard-coded, so a single building fills the frame and larger worlds may be clipped.

## User / engineering value
Users see every generated object framed with margin, from data rather than a magic constant.

## Scope
### Included
- Pure scene_bounds(nodes) in genesis-studio covering translation plus all mesh vertices
- Compute camera distance/fov from bounds in Scene.tsx
- Rust unit test covering every vertex and empty scenes

### Excluded
- User camera controls
- WebGL capability changes
- New dependencies

## Context and research
- Relevant sources: `research/notes/procedural.md`, `research/notes/desktop-viewport-formats.md`, `docs/product/E2E_QUALITY_VERDICT.md`
- Relevant ADRs: `adr/0002`, `adr/0003`, `adr/0007`, `adr/0008`
- Related tickets: `TICKETS/README.md`

## Proposed approach
Small tracer-bullet slice; keep existing versioned contracts and lawful-source gate. No proprietary or uncached visual assets.

## Acceptance criteria
- [ ] All generated nodes visible with margin in a headless snapshot test
- [ ] Bounds resolved from data only
- [ ] Existing selection/render tests remain green

## Test plan
- Unit: Rust bounds test; Integration: Studio component tests
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
