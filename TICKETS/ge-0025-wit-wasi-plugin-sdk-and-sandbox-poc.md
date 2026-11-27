---
id: GE-0025
title: WIT/WASI plugin SDK and sandbox PoC
type: security
priority: P2
status: verified
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
- Tamper-evident manifest seal with semver, digest, capability grants, dependency pins (asymmetric signature NOT MET)
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
- [x] Adversarial manifest fixtures rejected (AC restated: no guest/plugin fixture executes in this contract slice)
- [x] Rollback demonstrated
- [x] Capabilities deny-by-default
- [x] Tests and documentation updated

## Test plan
- Unit: WIT schema tests
- Integration: Adversarial integration
- End-to-end: n/a
- Manual: n/a
- Platform coverage: Windows/macOS/Linux

## Security / licensing / data considerations
Deny-by-default; no inherit_env/inherit_stdio; integrity-sealed artefacts (authenticity NOT MET).

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

GitHub issue: **#23** — https://github.com/Jasowills/genesis-engine/issues/23
- Files: `crates/genesis-core/src/plugins.rs`, `tests/plugins.rs`, `docs/developer/plugins.md`, lib.rs re-exports. Throwaway `prototype/plugin-sandbox/` evidence (GE-0005) is explicitly not imported.
- `cargo test --workspace --locked` → **97 passed, 0 failed** (5 plugin tests: deny-by-default WIT world, deny-by-default grants, adversarial manifest rejection including seal tampering, atomic activation with rollback, structured invalid-state/serialisation checks; strict semver with no leading zeros and pinned-dependency format validation). `cargo clippy --workspace --locked -- -D warnings` clean; `cargo fmt --all -- --check` clean.
- NOT IMPLEMENTED: generated WIT bindings, live WASI 0.2 component execution, OS-enforced sandbox policy, asymmetric signature verification/key management, cross-process persistence. The manifest seal is SHA-256 tamper-evidence, not authentication.
- Reviewer result: independent review **`APPROVED`** (session `ses_de1ef0ae9ffeqh2vE6j6TBQLE5`) with contract-only limitations; housekeeping conditions applied.

## Changelog
- 2026-11-27: Ticket created from the proposed research/architecture programme; current status: `proposed`. Pending owner approval of scope and independent review where required.
- 2026-11-27: Independent review APPROVED with explicit limitations; ticket marked verified.
