---
id: GE-0005
title: Plugin runtime feasibility spike
type: spike
priority: P1
status: ready
milestone: M0
labels: [spike, area:plugins, type:security]
depends_on: []
blocks: [GE-0007, GE-0025]
created: 2026-11-27
updated: 2026-11-27
owner_approval_required: false
---

# GE-0005 — Plugin runtime feasibility spike

## Problem
Wasmtime has a recent CVE history, permissive resource defaults and no sourced plugin manifest/rollback standard.

## User / engineering value
Validates A05 before any plugin SDK work; establishes adversarial fixtures.

## Scope
### Included
- Install wasm32-wasip1 target; build a minimal WIT world and host
- Evil-plugin fixtures: infinite loop, huge memory.grow, empty writes, invalid imports
- Test epoch + timeout and explicit resource caps

### Excluded
- Publishing a plugin registry

## Context and research
- Relevant sources:
  - `research/notes/plugin-security.md`
  - `research/RESEARCH_REGISTER.md section 4`
- Relevant ADRs:
  - `A05`
- Related tickets: `TICKETS/README.md` (dependency index)

## Proposed approach
Throwaway PoC; record measured overhead, cap behaviour and lifecycle design inference separately from sourced facts.

## Acceptance criteria
- [ ] Evil plugins are terminated within timeout on all three OSes
- [ ] Zero preopens/env/network demonstrated
- [ ] Component overhead measured or explicitly UNVERIFIED
- [ ] Tests and documentation updated

## Test plan
- Unit: n/a
- Integration: n/a
- End-to-end: n/a
- Manual: Manual adversarial review
- Platform coverage: Windows/macOS/Linux

## Security / licensing / data considerations
Deny-by-default; no inherit_env/inherit_stdio; no secrets in plugin context.

## Dependencies and risks
Wasmtime monthly majors; pin >=51.0.0 and monitor advisories.

## Implementation notes
Manifest/signing/rollback are design inference, not sourced standards.

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

GitHub issue: **#22** — https://github.com/Jasowills/genesis-engine/issues/22
Commands, output summaries, fixture references and reviewer result to be recorded here when the ticket is implemented. No claim of completion without an independent reviewer.

## Changelog
- 2026-11-27: Ticket created from the proposed research/architecture programme; current status: `ready`. Pending owner approval of scope and independent review where required.
