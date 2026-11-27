---
id: GE-0010
title: Project model and portable local layout
type: feature
priority: P0
status: proposed
milestone: M1
labels: [type:feature, area:core]
depends_on: [GE-0007]
blocks: [GE-0011]
created: 2026-11-27
updated: 2026-11-27
owner_approval_required: false
---

# GE-0010 — Project model and portable local layout

## Problem
Users need a project that can be created, reopened and moved without a hidden global cache.

## User / engineering value
Foundation for all generation, storage and reproducibility behaviour.

## Scope
### Included
- Project manifest with schema version, CRS/epoch/datum/units
- Separate directories for metadata, world manifests, binary payloads, source refs, jobs, validation, exports
- Atomic write and portable state

### Excluded
- Autosave/crash recovery, templates, multi-user, migration tooling beyond basic version field

## Context and research
- Relevant sources:
  - `Spec sections 21, 32`
  - `ARCHITECTURE.md sections 2 and 4`
- Relevant ADRs:
  - `A01`
  - `A07`
- Related tickets: `TICKETS/README.md` (dependency index)

## Proposed approach
Implement the minimal manifest and layout behind Core commands usable headlessly.

## Acceptance criteria
- [ ] Project create/open/save round-trip passes
- [ ] All paths relative and portable
- [ ] Unknown/malformed manifest produces structured error
- [ ] Tests and documentation updated

## Test plan
- Unit: Manifest serialization and path validation
- Integration: Project round-trip across OSes
- End-to-end: n/a
- Manual: Manual move/reopen
- Platform coverage: macOS/Linux/Windows

## Security / licensing / data considerations
Archive extraction hardening and path containment required.

## Dependencies and risks
A01/A07 must be accepted first.

## Implementation notes
Headless-drivable is the test that this is a Core capability.

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

GitHub issue: **#27** — https://github.com/Jasowills/genesis-engine/issues/27
Commands, output summaries, fixture references and reviewer result to be recorded here when the ticket is implemented. No claim of completion without an independent reviewer.

## Changelog
- 2026-11-27: Ticket created from the proposed research/architecture programme; current status: `proposed`. Pending owner approval of scope and independent review where required.
