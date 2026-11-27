---
id: GE-0024
title: Quality profiles and preflight estimates
type: feature
priority: P1
status: proposed
milestone: M2
labels: [type:feature, area:quality]
depends_on: [GE-0018, GE-0006]
blocks: []
created: 2026-11-27
updated: 2026-11-27
owner_approval_required: false
---

# GE-0024 — Quality profiles and preflight estimates

## Problem
Users need named profiles and a preflight estimate before expensive operations.

## User / engineering value
Spec §26/§51 requires explicit profiles and cost/resource warnings.

## Scope
### Included
- Preview/Game-ready/High-fidelity/Survey-reference profiles with explicit dimensions
- Preflight outputs ranges + confidence, paid-cost consent gate
- Capability report for CPU/GPU suitability

### Excluded
- Survey-grade label without checkpoints
- Precise timing promises

## Context and research
- Relevant sources:
  - `engineering/TEST_STRATEGY.md`
  - `research/notes/quality-ci-gpu.md section 4`
- Relevant ADRs:
  - `A04`
- Related tickets: `TICKETS/README.md` (dependency index)

## Proposed approach
Profile policy is project-defined; all numbers labelled estimates until measured.

## Acceptance criteria
- [ ] Profiles map to explicit gates
- [ ] Preflight never emits a single guaranteed number
- [ ] Paid operations require confirmation
- [ ] Tests and documentation updated

## Test plan
- Unit: Policy tests
- Integration: n/a
- End-to-end: n/a
- Manual: Manual estimate review
- Platform coverage: All

## Security / licensing / data considerations
No external API charge without explicit confirmation.

## Dependencies and risks
All performance ranges are estimates anchored on runner specs and COLMAP formula.

## Implementation notes
Calibrate after fixtures exist.

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

GitHub issue: **#30** — https://github.com/Jasowills/genesis-engine/issues/30
Commands, output summaries, fixture references and reviewer result to be recorded here when the ticket is implemented. No claim of completion without an independent reviewer.

## Changelog
- 2026-11-27: Ticket created from the proposed research/architecture programme; current status: `proposed`. Pending owner approval of scope and independent review where required.
