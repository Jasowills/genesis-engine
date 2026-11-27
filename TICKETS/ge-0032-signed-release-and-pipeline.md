---
id: GE-0032
title: Signed release and auto-update pipeline
type: release
priority: P3
status: needs-owner-input
milestone: M5
labels: [type:release, area:infra]
depends_on: [GE-0008, GE-0009]
blocks: []
created: 2026-11-27
updated: 2026-11-27
owner_approval_required: true
---

# GE-0032 — Signed release and auto-update pipeline

## Problem
Public releases need signing, notarisation and safe update key custody.

## User / engineering value
Spec §52 and research on Apple/Windows signing costs and Tauri updater.

## Scope
### Included
- Two-track build: reproducible unsigned core + signed installers
- Tauri updater with mandatory signature verification
- Key custody, rollback and entitlement plan

### Excluded
- Publishing before approval, exposing private keys

## Context and research
- Relevant sources:
  - `research/notes/quality-ci-gpu.md section 3`
  - `research/notes/desktop-viewport-formats.md section 3`
- Relevant ADRs:
- Related tickets: `TICKETS/README.md` (dependency index)

## Proposed approach
Owner must approve purchases and identities before any signing work.

## Acceptance criteria
- [ ] Second-build hash comparison passes for core
- [ ] Signed artefacts verified on all three OSes
- [ ] No secret in CI logs or repo
- [ ] Tests and documentation updated

## Test plan
- Unit: n/a
- Integration: Release pipeline dry run
- End-to-end: n/a
- Manual: Manual install/update
- Platform coverage: macOS/Linux/Windows

## Security / licensing / data considerations
Private key custody is critical; no secrets in repo.

## Dependencies and risks
Apple $99/yr; Windows signing $9.99/mo or $150-400+/yr; owner approval required.

## Implementation notes
Do not announce stable support for untested features.

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

GitHub issue: **#21** — https://github.com/Jasowills/genesis-engine/issues/21
Commands, output summaries, fixture references and reviewer result to be recorded here when the ticket is implemented. No claim of completion without an independent reviewer.

## Changelog
- 2026-11-27: Ticket created from the proposed research/architecture programme; current status: `needs-owner-input`. Pending owner approval of scope and independent review where required.
