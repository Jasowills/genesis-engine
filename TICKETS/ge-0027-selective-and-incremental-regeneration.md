---
id: GE-0027
title: Selective and incremental regeneration
type: feature
priority: P2
status: proposed
milestone: M3
labels: [type:feature, area:core]
depends_on: [GE-0011, GE-0020]
blocks: []
created: 2026-11-27
updated: 2026-11-27
owner_approval_required: false
---

# GE-0027 — Selective and incremental regeneration

## Problem
Users should not rebuild an entire city after one object changes.

## User / engineering value
Spec principle 11; depends on IR IDs and dependency graph being right.

## Scope
### Included
- Dependency graph per asset/layer
- Change detection from inputs/seeds/hashes
- Regenerate affected subgraph only; override handling

### Excluded
- Full undo history, multi-user editing

## Context and research
- Relevant sources:
  - `research/COMPETITIVE_ANALYSIS.md section 2.3`
  - `research/notes/procedural.md section 4`
- Relevant ADRs:
  - `A01`
- Related tickets: `TICKETS/README.md` (dependency index)

## Proposed approach
No mature ecosystem contract found; design and validate with fixtures.

## Acceptance criteria
- [ ] Changing one building regenerates only its subgraph
- [ ] Unchanged assets keep stable IDs and hashes
- [ ] Rebuild report lists affected assets
- [ ] Tests and documentation updated

## Test plan
- Unit: Graph unit tests
- Integration: Fixture integration
- End-to-end: n/a
- Manual: n/a
- Platform coverage: macOS/Linux/Windows

## Security / licensing / data considerations
No external calls.

## Dependencies and risks
This is a known design risk; keep dependency graph explicit from IR v0.

## Implementation notes
Split further if the diff becomes too large.

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
