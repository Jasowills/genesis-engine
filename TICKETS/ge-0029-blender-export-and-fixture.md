---
id: GE-0029
title: Blender export and round-trip fixture
type: integration
priority: P2
status: proposed
milestone: M3
labels: [type:integration, area:export]
depends_on: [GE-0019]
blocks: [GE-0030]
created: 2026-11-27
updated: 2026-11-27
owner_approval_required: false
---

# GE-0029 — Blender export and round-trip fixture

## Problem
Blender is the likely first DCC hand-off, but support must be proven by a fixture.

## User / engineering value
Spec §27 forbids claiming integration without automated/documented compatibility test.

## Scope
### Included
- Documented Blender version and import/export path
- Hierarchy/material/metadata round-trip fixture
- Known limitations documented

### Excluded
- Unreal/Unity/Godot integrations, GPL Blender code in core

## Context and research
- Relevant sources:
  - `research/notes/competitive-data-rights.md`
  - `research/notes/reconstruction.md`
- Relevant ADRs:
  - `A03`
- Related tickets: `TICKETS/README.md` (dependency index)

## Proposed approach
Invoke Blender as external process; never link GPL code into core.

## Acceptance criteria
- [ ] Fixture round-trip passes with documented version
- [ ] Metadata loss explicitly recorded
- [ ] No GPL code in core
- [ ] Tests and documentation updated

## Test plan
- Unit: n/a
- Integration: Round-trip integration
- End-to-end: n/a
- Manual: Manual Blender inspection
- Platform coverage: macOS/Linux/Windows

## Security / licensing / data considerations
GPL process boundary; output not covered by GPL.

## Dependencies and risks
Blender bake data not guaranteed portable across versions.

## Implementation notes
D-009 first integration decision still open.

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

GitHub issue: **#25** — https://github.com/Jasowills/genesis-engine/issues/25
Commands, output summaries, fixture references and reviewer result to be recorded here when the ticket is implemented. No claim of completion without an independent reviewer.

## Changelog
- 2026-11-27: Ticket created from the proposed research/architecture programme; current status: `proposed`. Pending owner approval of scope and independent review where required.
