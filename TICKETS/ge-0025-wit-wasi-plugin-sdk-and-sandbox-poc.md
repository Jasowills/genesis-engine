---
id: GE-0025
title: WIT/WASI plugin SDK and sandbox PoC
type: security
priority: P2
status: proposed
milestone: M2
labels: [type:security, area:plugins]
depends_on: [GE-0005, GE-0011]
blocks: []
created: 2026-11-27
updated: 2026-11-27
owner_approval_required: false
---

# GE-0025 — WIT/WASI plugin SDK and sandbox PoC

## Problem
Third-party extensions must not gain ambient filesystem, network, process or credential access.

## User / engineering value
Spec §33 requires a safe extension model; A05 must be proven before any registry.

## Scope
### Included
- Versioned WIT world and generated bindings
- Signed manifest with semver, digest, capability grants, dependency pins
- Atomic activation and rollback; bounded log/progress hostcalls

### Excluded
- Plugin marketplace, native dynamic library plugins

## Context and research
- Relevant sources:
  - `research/notes/plugin-security.md`
  - `adr/ADR_PROPOSALS.md A05`
- Relevant ADRs:
  - `A05`
- Related tickets: `TICKETS/README.md` (dependency index)

## Proposed approach
PoC graduates into SDK only after adversarial fixtures pass.

## Acceptance criteria
- [ ] Evil plugin fixtures pass
- [ ] Rollback demonstrated
- [ ] Capabilities deny-by-default
- [ ] Tests and documentation updated

## Test plan
- Unit: WIT schema tests
- Integration: Adversarial integration
- End-to-end: n/a
- Manual: n/a
- Platform coverage: Windows/macOS/Linux

## Security / licensing / data considerations
Deny-by-default; no inherit_env/inherit_stdio; signed artefacts.

## Dependencies and risks
Manifest/rollback design is inference; component overhead unverified.

## Implementation notes
No registry before legal/trust policy.

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
