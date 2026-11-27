---
id: GE-0021
title: Contributor and governance documentation
type: documentation
priority: P1
status: proposed
milestone: M1
labels: [type:documentation]
depends_on: [GE-0007]
blocks: []
created: 2026-11-27
updated: 2026-11-27
owner_approval_required: false
---

# GE-0021 — Contributor and governance documentation

## Problem
An open-source project needs contribution, conduct, security and changelog documentation aligned to actual behaviour.

## User / engineering value
Enables external contribution and safe disclosure.

## Scope
### Included
- README, LICENSE (per owner decision), CONTRIBUTING, CODE_OF_CONDUCT, SECURITY, CHANGELOG
- Docs do not describe planned features as existing
- Issue/PR templates

### Excluded
- Publishing before licence approval

## Context and research
- Relevant sources:
  - `Spec sections 42-45`
  - `DECISIONS.md`
- Relevant ADRs:
  - `A06`
- Related tickets: `TICKETS/README.md` (dependency index)

## Proposed approach
Documentation agent checks commands and examples against implemented behaviour.

## Acceptance criteria
- [ ] All required root docs exist
- [ ] Examples verified
- [ ] No planned feature documented as shipped
- [ ] Tests and documentation updated

## Test plan
- Unit: n/a
- Integration: n/a
- End-to-end: n/a
- Manual: Manual doc review
- Platform coverage: All

## Security / licensing / data considerations
No secrets in docs or templates.

## Dependencies and risks
Licence direction must be approved first.

## Implementation notes
Continuously maintained, not a one-off dump.

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
