---
id: GE-0016
title: Studio shell and viewport scene rendering
type: feature
priority: P1
status: in-progress
milestone: M1
labels: [type:feature, area:studio, area:viewport]
depends_on: [GE-0004, GE-0011]
blocks: [GE-0017]
created: 2026-11-27
updated: 2026-11-27
owner_approval_required: false
---

# GE-0016 — Studio shell and viewport scene rendering

## Problem
The world must be inspectable in a desktop viewport without UI owning domain logic.

## User / engineering value
Delivers spec §50 step 7 and validates the UI↔Core seam.

## Scope
### Included
- Shell + React/TS panels + viewport rendering IR-derived scene
- Commands/events/versioned snapshots only
- Capability report with graceful GPU degradation

### Excluded
- Gizmo polish, advanced editing, LOD streaming

## Context and research
- Relevant sources:
  - `engineering/ARCHITECTURE_DISCOVERY.md sections 1-3`
  - `research/notes/desktop-viewport-formats.md`
- Relevant ADRs:
  - `A02`
- Related tickets: `TICKETS/README.md` (dependency index)

## Proposed approach
Implement per accepted ADR; keep Core authoritative.

## Acceptance criteria
- [ ] Scene renders from IR with no domain logic in React
- [ ] WebGL2/WebGPU capability handled on all supported OSes
- [ ] Keyboard-numeric alternative exists
- [ ] Tests and documentation updated

## Test plan
- Unit: Component tests
- Integration: UI integration
- End-to-end: n/a
- Manual: Manual inspection
- Platform coverage: macOS/Linux/Windows

## Security / licensing / data considerations
No secrets in UI config; signed update deferred.

## Dependencies and risks
PoC results gate this ticket; local host only macOS.

## Implementation notes
Separate shell and viewport approvals.

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

GitHub issue: **#14** — https://github.com/Jasowills/genesis-engine/issues/14
Commands, output summaries, fixture references and reviewer result to be recorded here when the ticket is implemented. No claim of completion without an independent reviewer.

## Changelog
- 2026-11-27: Ticket created from the proposed research/architecture programme; current status: `proposed`. Pending owner approval of scope and independent review where required.
