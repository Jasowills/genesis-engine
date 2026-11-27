---
id: GE-0022
title: Headless CLI command surface
type: feature
priority: P1
status: proposed
milestone: M2
labels: [type:feature, area:core]
depends_on: [GE-0020]
blocks: [GE-0023, GE-0024]
created: 2026-11-27
updated: 2026-11-27
owner_approval_required: false
---

# GE-0022 — Headless CLI command surface

## Problem
Core capabilities must be usable without the desktop UI.

## User / engineering value
Spec §8 requires the first slice to use the architecture later CLI/headless paths use.

## Scope
### Included
- Commands: project create/import/generate/validate/export/job status+cancel
- Machine-readable output and reproducibility report
- No UI dependency

### Excluded
- Cloud CLI, interactive shell

## Context and research
- Relevant sources:
  - `Spec sections 3, 8, 28`
  - `ARCHITECTURE.md section 1`
- Relevant ADRs:
  - `A01`
  - `A03`
- Related tickets: `TICKETS/README.md` (dependency index)

## Proposed approach
Thin adapter over Core commands; no duplicated domain logic.

## Acceptance criteria
- [ ] Every E2E step reachable headlessly
- [ ] Output schema versioned and documented
- [ ] Same results as Studio
- [ ] Tests and documentation updated

## Test plan
- Unit: Command parsing tests
- Integration: CLI integration
- End-to-end: n/a
- Manual: n/a
- Platform coverage: macOS/Linux/Windows

## Security / licensing / data considerations
No credentials in CLI args or logs.

## Dependencies and risks
D-008 release timing open; implementation may precede public release.

## Implementation notes
Do not fork Core logic into the CLI.

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
