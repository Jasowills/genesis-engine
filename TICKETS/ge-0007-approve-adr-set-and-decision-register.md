---
id: GE-0007
title: Approve ADR set and decision register
type: documentation
priority: P0
status: verified
milestone: M0
labels: [documentation]
depends_on: [GE-0002, GE-0003, GE-0004]
blocks: [GE-0008, GE-0010, GE-0011, GE-0023]
created: 2026-11-27
updated: 2026-11-27
owner_approval_required: true
---

# GE-0007 — Approve ADR set and decision register

## Problem
Seven ADRs are proposals only; product boundaries, licence and scope decisions remain open.

## User / engineering value
Spec Phase 8 requires explicit owner approval before implementation and GitHub setup.

## Scope
### Included
- Owner decisions on D-007..D-019 and Q1..Q8
- Accepted ADR files using adr/0001-template.md
- Updated DECISIONS.md and progress.md

### Excluded
- Any product implementation

## Context and research
- Relevant sources:
  - `adr/ADR_PROPOSALS.md`
  - `engineering/ARCHITECTURE_DISCOVERY.md section 7`
  - `DECISIONS.md`
- Relevant ADRs:
  - `A01`
  - `A02`
  - `A03`
  - `A04`
  - `A05`
  - `A06`
  - `A07`
- Related tickets: `TICKETS/README.md` (dependency index)

## Proposed approach
Present the decision package with recommendation, alternatives, consequences and known/unknown for each question; record answers.

## Acceptance criteria
- [x] Owner answer recorded for every blocking question — D-025…D-031 cover the foundational plan, first-slice scope, CLI/integration timing and benchmark boundary; D-007/D-015/D-016/D-017/D-018/D-022/D-023/D-024 are explicitly deferred/non-blocking and tracked in `DECISIONS.md`
- [x] Accepted ADRs `adr/0002`–`0008` have status/date/approval/evidence; governance reviewer final verdict `APPROVED`
- [x] No product implementation started before approval (verified by repo state: research/A DR/ticket docs only)
- [x] `DECISIONS.md` §3a, `PRD.md`, `ROADMAP.md`, `ARCHITECTURE.md` and `research/BENCHMARK_AREA.md` updated to match answers

## Test plan
- Unit: n/a
- Integration: n/a
- End-to-end: n/a
- Manual: n/a
- Platform coverage: All

## Security / licensing / data considerations
Licence decision must precede publication; no secrets in tickets or commits.

## Dependencies and risks
This is the Phase 8 checkpoint; stop and wait for explicit approval.

## Implementation notes
Owner may answer partially; record deferred items as open, not assumed.

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

GitHub issue: **#4** — https://github.com/Jasowills/genesis-engine/issues/4
- Owner answer batch 2026-11-27: all seven ADRs approved (D-025); Apache-2.0 core (D-026); public repository (D-027); hard source deny (D-028); PRD §5 scope (D-029/D-013); CLI in early preview + GLB then Blender (D-030/D-008/D-009); agent-selected benchmark boundary (D-031/D-021).
- `DECISIONS.md` §3a records D-025…D-031 with dates and scope; §2/§3 mark resolved vs open decisions.
- Accepted ADR files `adr/0002`–`0008`; independent governance reviewer final verdict `APPROVED` (session `ses_de59f7bffffeZu5LcI2JsshkFy`).
- Benchmark area verified with read-only Nominatim reverse geocode (OSM way 134404795, Allen Avenue, Ikeja, Lagos) and frozen in `research/BENCHMARK_AREA.md`.
- No product code or commits existed before approvals; status verified after all gates.

## Changelog
- 2026-11-27: Ticket created from the proposed research/architecture programme; current status: `blocked`. Pending owner approval of scope and independent review where required.
