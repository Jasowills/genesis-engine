---
id: GE-0012
title: Lawful-source gate and provenance record
type: feature
priority: P0
status: proposed
milestone: M1
labels: [type:feature, area:core, type:security]
depends_on: [GE-0002, GE-0011]
blocks: [GE-0013, GE-0015]
created: 2026-11-27
updated: 2026-11-27
owner_approval_required: false
---

# GE-0012 — Lawful-source gate and provenance record

## Problem
Terms prohibit derivation from Google Maps/Cesium ion content; ODbL requires attribution and possible derivative offer.

## User / engineering value
Makes the provenance differentiator real and prevents rights violations in code paths.

## Scope
### Included
- SourceDescriptor with licence election, attribution, vintage, filters, checksum
- Deny-list enforced before import/inference/export
- Provenance record carried into IR and export sidecar

### Excluded
- Visual-only basemap carve-out, legal determination of generated GLB status

## Context and research
- Relevant sources:
  - `research/LICENCE_AND_DATA_RIGHTS.md`
  - `adr/ADR_PROPOSALS.md A06`
- Relevant ADRs:
  - `A06`
- Related tickets: `TICKETS/README.md` (dependency index)

## Proposed approach
Cross-cutting service consulted by all import and export paths.

## Acceptance criteria
- [ ] Denied sources are rejected in tests
- [ ] Attribution and licence election survive IR to export
- [ ] Provenance record covers all fields from research synthesis
- [ ] Tests and documentation updated

## Test plan
- Unit: Gate decision logic
- Integration: Import/export path integration
- End-to-end: n/a
- Manual: n/a
- Platform coverage: All

## Security / licensing / data considerations
This is a security/licence boundary; no secrets; no restricted scraping.

## Dependencies and risks
Legal review of ODbL §4.6 remains open.

## Implementation notes
Do not defer: hard blocker for any data import.

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
