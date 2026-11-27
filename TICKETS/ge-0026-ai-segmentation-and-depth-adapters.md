---
id: GE-0026
title: AI segmentation and depth adapters
type: integration
priority: P2
status: proposed
milestone: M3
labels: [type:integration, area:ai]
depends_on: [GE-0006, GE-0023]
blocks: []
created: 2026-11-27
updated: 2026-11-27
owner_approval_required: false
---

# GE-0026 — AI segmentation and depth adapters

## Problem
AI-assisted extraction is a spec capability but requires clean licensing and swappable providers.

## User / engineering value
Delivers optional AI capability without making Python or a runtime mandatory.

## Scope
### Included
- SAM ViT-B segmentation and Depth Anything V2 Small depth behind S4
- Typed I/O, cancellation, per-run evidence and licence record
- External provider adapter stub (no paid calls by default)

### Excluded
- Facade reconstruction, image-to-3D, non-commercial weights

## Context and research
- Relevant sources:
  - `research/notes/ai-ml.md`
  - `research/LICENCE_AND_DATA_RIGHTS.md`
- Relevant ADRs:
  - `A04`
- Related tickets: `TICKETS/README.md` (dependency index)

## Proposed approach
Adapter only; no capability code imports runtime types.

## Acceptance criteria
- [ ] Outputs typed and provenance-recorded
- [ ] Cancellation demonstrated
- [ ] No non-commercial weight used
- [ ] Tests and documentation updated

## Test plan
- Unit: Schema tests
- Integration: Model integration
- End-to-end: n/a
- Manual: Manual output review
- Platform coverage: macOS/Linux/Windows

## Security / licensing / data considerations
No unpickling; checksum verification; no external cost without approval.

## Dependencies and risks
Code/weight licence divergence; gated model cards.

## Implementation notes
External providers are adapters, never hard dependencies.

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
