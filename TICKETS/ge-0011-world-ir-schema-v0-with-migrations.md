---
id: GE-0011
title: World IR schema v0 with migrations
type: feature
priority: P0
status: ready
milestone: M1
labels: [type:feature, area:core]
depends_on: [GE-0003, GE-0007]
blocks: [GE-0012, GE-0013, GE-0016, GE-0019]
created: 2026-11-27
updated: 2026-11-27
owner_approval_required: false
---

# GE-0011 — World IR schema v0 with migrations

## Problem
There is no engine-agnostic representation for stable IDs, provenance, confidence, seeds and validation state.

## User / engineering value
Every other component depends on this seam; changing it after projects exist is expensive.

## Scope
### Included
- Versioned IR document with stable IDs and binary payload references
- CRS/epoch/datum/up-axis/local frame origin fields
- Observed-vs-inferred flags, provenance, seeds, content hashes, validation state
- Forward/backward migration tests

### Excluded
- Full USD composition, LOD/collision/instancing systems

## Context and research
- Relevant sources:
  - `Spec section 30`
  - `adr/ADR_PROPOSALS.md A01 and A07`
- Relevant ADRs:
  - `A01`
  - `A07`
- Related tickets: `TICKETS/README.md` (dependency index)

## Proposed approach
Small schema core with adapters; no inline giant geometry JSON.

## Acceptance criteria
- [ ] Schema round-trips losslessly
- [ ] Migration fixtures pass forward and backward
- [ ] Golden vector tests from GE-0003 pass
- [ ] Tests and documentation updated

## Test plan
- Unit: Serialization, ID stability, hash behaviour
- Integration: IR persistence via storage seam
- End-to-end: n/a
- Manual: n/a
- Platform coverage: macOS/Linux/Windows

## Security / licensing / data considerations
Do not embed licence text as executable input; provenance fields are data.

## Dependencies and risks
No mature ecosystem contract for incremental regeneration; keep dependency graph explicit.

## Implementation notes
Independent review mandatory; schema is foundational.

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

GitHub issue: **#5** — https://github.com/Jasowills/genesis-engine/issues/5
Commands, output summaries, fixture references and reviewer result to be recorded here when the ticket is implemented. No claim of completion without an independent reviewer.

## Changelog
- 2026-11-27: Ticket created from the proposed research/architecture programme; current status: `proposed`. Pending owner approval of scope and independent review where required.
