---
id: GE-0019
title: GLB export and re-import fixture
type: feature
priority: P1
status: proposed
milestone: M1
labels: [type:feature, area:export]
depends_on: [GE-0011, GE-0018]
blocks: [GE-0020, GE-0029]
created: 2026-11-27
updated: 2026-11-27
owner_approval_required: false
---

# GE-0019 — GLB export and re-import fixture

## Problem
The first interchange target must be validated, not assumed.

## User / engineering value
Delivers spec §50 steps 11-12 and proves metadata survival.

## Scope
### Included
- IR to GLB with extension allowlist and attribution sidecar
- Round-trip fixture: export then re-import/open in a verified tool
- Validation report archived

### Excluded
- USD, 3D Tiles, engine plugins, LOD/collision export

## Context and research
- Relevant sources:
  - `research/FORMAT_AND_STANDARDS.md`
  - `research/notes/quality-ci-gpu.md section 1`
- Relevant ADRs:
  - `A01`
  - `A06`
- Related tickets: `TICKETS/README.md` (dependency index)

## Proposed approach
Map IR to glTF 2.0; record unsupported fields explicitly rather than silently dropping.

## Acceptance criteria
- [ ] glTF-Validator exit code 0
- [ ] Provenance/attribution survive round-trip
- [ ] Compatibility fixture documented
- [ ] Tests and documentation updated

## Test plan
- Unit: Mapping unit tests
- Integration: Round-trip integration
- End-to-end: n/a
- Manual: Open export in verified tool
- Platform coverage: macOS/Linux/Windows

## Security / licensing / data considerations
No unrestricted third-party asset download.

## Dependencies and risks
Unreal/Unity/Godot support not claimed until fixtures exist.

## Implementation notes
GLB is interchange, not internal format.

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

GitHub issue: **#16** — https://github.com/Jasowills/genesis-engine/issues/16
Commands, output summaries, fixture references and reviewer result to be recorded here when the ticket is implemented. No claim of completion without an independent reviewer.

## Changelog
- 2026-11-27: Ticket created from the proposed research/architecture programme; current status: `proposed`. Pending owner approval of scope and independent review where required.
