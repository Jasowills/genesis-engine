---
id: GE-0006
title: AI runtime and model smoke test
type: spike
priority: P1
status: ready
milestone: M0
labels: [spike, area:ai]
depends_on: []
blocks: [GE-0007, GE-0024, GE-0026]
created: 2026-11-27
updated: 2026-11-27
owner_approval_required: false
---

# GE-0006 — AI runtime and model smoke test

## Problem
Model code and weight licences diverge; runtime operator coverage and cancellation need verification in our environment.

## User / engineering value
Gates A04 and prevents a licence or capability surprise in later milestones.

## Scope
### Included
- Smoke-test ONNX Runtime 1.32.0 with SAM ViT-B, YOLOX, Depth Anything V2 Small
- Verify RunOptions::SetTerminate cancellation and EP fallback logging
- Record checksums, licences and hardware requirements

### Excluded
- Production AI features or model bundling

## Context and research
- Relevant sources:
  - `research/notes/ai-ml.md`
  - `research/LICENCE_AND_DATA_RIGHTS.md section 1`
- Relevant ADRs:
  - `A04`
- Related tickets: `TICKETS/README.md` (dependency index)

## Proposed approach
Small headless harness; no desktop dependency; record per-run evidence fields.

## Acceptance criteria
- [ ] Each model loads and produces typed output
- [ ] Cancellation demonstrated
- [ ] Checksum/licence record complete for each artefact
- [ ] Tests and documentation updated

## Test plan
- Unit: n/a
- Integration: n/a
- End-to-end: n/a
- Manual: Manual output inspection
- Platform coverage: macOS host; Linux/Windows CI

## Security / licensing / data considerations
No non-commercial weights; never unpickle untrusted files; no external API costs without approval.

## Dependencies and risks
ORT GitHub repo was unreachable during research; re-verify from another network.

## Implementation notes
No paid cloud inference.

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

GitHub issue: **#9** — https://github.com/Jasowills/genesis-engine/issues/9
Commands, output summaries, fixture references and reviewer result to be recorded here when the ticket is implemented. No claim of completion without an independent reviewer.

## Changelog
- 2026-11-27: Ticket created from the proposed research/architecture programme; current status: `ready`. Pending owner approval of scope and independent review where required.
