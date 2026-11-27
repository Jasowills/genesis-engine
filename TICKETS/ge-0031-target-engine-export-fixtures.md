---
id: GE-0031
title: Target engine export fixtures
type: integration
priority: P3
status: blocked
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
D-009 resolved by D-030; owner selected Unreal as first target (D-034). Unreal Editor is not installed and Epic account login is a manual step only the owner can perform.

## Implementation notes
Unreal first. Contract and external probe scaffolded; fixture cannot run until Unreal is installed and licensed.

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

GitHub issue: **#32** — https://github.com/Jasowills/genesis-engine/issues/32
- Files (scaffold only): `tools/unreal_import_probe.py`, `docs/developer/unreal-fixture.md`. No fixture report exists.
- D-034 owner selection: Unreal is the first target engine. `ls /Applications | grep -iE 'unreal|epic'` → no Unreal/Epic app; `which UnrealEditor` → not found; no Homebrew cask. Installation requires Epic account login (manual owner step).
- NOT MET: all four acceptance criteria. No engine version pinned, no imported-asset report, no cross-OS evidence, no metadata-loss measurement. **No Unreal support is claimed.**
- Status: `blocked` pending manual Unreal Editor installation/licensing; independent review has not run and must not be requested until a fixture exists.

## Changelog
- 2026-11-27: Ticket created from the proposed research/architecture programme; current status: `proposed`. Pending owner approval of scope and independent review where required.
