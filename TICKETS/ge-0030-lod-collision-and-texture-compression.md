---
id: GE-0030
title: LOD, collision and texture compression
type: feature
priority: P3
status: proposed
milestone: M4
labels: [type:feature, area:geometry, area:export]
depends_on: [GE-0029]
blocks: [GE-0031]
created: 2026-11-27
updated: 2026-11-27
owner_approval_required: false
---

# GE-0030 — LOD, collision and texture compression

## Problem
Game-ready claims require collision, LOD chains and budgeted textures.

## User / engineering value
Spec §26 game-readiness dimension and §27 export requirements.

## Scope
### Included
- LOD chain generation with validation
- Collision proxy generation with fixture-based coverage checks
- KTX2/Basis and Draco/meshopt export allowlist

### Excluded
- Full target-engine optimisation

## Context and research
- Relevant sources:
  - `research/notes/desktop-viewport-formats.md sections 7-8`
  - `research/notes/quality-ci-gpu.md`
- Relevant ADRs:
- Related tickets: `TICKETS/README.md` (dependency index)

## Proposed approach
Delivery-time compression only; source IR stays uncompressed.

## Acceptance criteria
- [ ] Budgets verified against named target profile
- [ ] Collision fixture coverage measured
- [ ] Precision regression tests pass
- [ ] Tests and documentation updated

## Test plan
- Unit: Geometry tests
- Integration: Export integration
- End-to-end: n/a
- Manual: Manual tool inspection
- Platform coverage: macOS/Linux/Windows

## Security / licensing / data considerations
No unlicensed codec distribution.

## Dependencies and risks
Collision coverage definition is project-specific.

## Implementation notes
Calibrate budgets with measured fixtures.

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
