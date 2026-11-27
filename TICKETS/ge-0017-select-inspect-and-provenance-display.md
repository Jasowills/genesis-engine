---
id: GE-0017
title: Select, inspect and provenance display
type: feature
priority: P1
status: proposed
milestone: M1
labels: [type:feature, area:studio]
depends_on: [GE-0016]
blocks: [GE-0020]
created: 2026-11-27
updated: 2026-11-27
owner_approval_required: false
---

# GE-0017 — Select, inspect and provenance display

## Problem
Users must see where a road/building came from and whether attributes are measured, estimated or inferred.

## User / engineering value
Delivers spec §50 steps 8-9 and the honest-uncertainty principle.

## Scope
### Included
- Picking/selection mapped to stable IR IDs
- Inspector shows source, method, licence, confidence and source/inferred flags
- Keyboard-driven selection path

### Excluded
- Full editing, undo history, multi-select gizmos

## Context and research
- Relevant sources:
  - `Spec sections 9, 24-26`
  - `PRD.md section 6.4`
- Relevant ADRs:
  - `A01`
  - `A02`
  - `A06`
- Related tickets: `TICKETS/README.md` (dependency index)

## Proposed approach
DOM panels use ARIA tree/grid semantics; viewport emits selection events only.

## Acceptance criteria
- [ ] Pick maps to correct stable ID
- [ ] Provenance fields visible and accurate
- [ ] Keyboard path selects and inspects an object
- [ ] Tests and documentation updated

## Test plan
- Unit: Component tests
- Integration: Selection integration
- End-to-end: n/a
- Manual: Manual keyboard walkthrough
- Platform coverage: macOS/Linux/Windows

## Security / licensing / data considerations
No network calls from UI.

## Dependencies and risks
Canvas accessibility has no standard pattern; numeric path is required.

## Implementation notes
axe and keyboard tests planned.

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
