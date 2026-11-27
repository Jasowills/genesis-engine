---
id: GE-0031
title: Target engine export fixtures
type: integration
priority: P3
status: proposed
milestone: M4
labels: [type:integration, area:export]
depends_on: [GE-0030]
blocks: []
created: 2026-11-27
updated: 2026-11-27
owner_approval_required: false
---

# GE-0031 — Target engine export fixtures

## Problem
Support claims for Unreal/Unity/Godot need versioned compatibility fixtures.

## User / engineering value
Spec §27 requires version, mapping, metadata retention and round-trip evidence.

## Scope
### Included
- One fixture per claimed target engine and version
- Document coordinate/axis/material/collision mapping
- Record round-trip limitations

### Excluded
- Claiming support without a fixture

## Context and research
- Relevant sources:
  - `Spec section 27`
  - `research/notes/desktop-viewport-formats.md`
- Relevant ADRs:
- Related tickets: `TICKETS/README.md` (dependency index)

## Proposed approach
Each target is its own vertical-slice ticket if the diff is large.

## Acceptance criteria
- [ ] Each fixture passes on all supported OSes
- [ ] No undocumented metadata loss
- [ ] Support claim limited to tested versions
- [ ] Tests and documentation updated

## Test plan
- Unit: n/a
- Integration: Per-engine integration
- End-to-end: n/a
- Manual: Manual engine inspection
- Platform coverage: macOS/Linux/Windows

## Security / licensing / data considerations
No proprietary SDK redistribution without licence review.

## Dependencies and risks
D-009 open; owner chooses first target.

## Implementation notes
Split per engine before implementation.

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
