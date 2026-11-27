---
id: GE-0009
title: Cross-platform CI skeleton
type: engineering
priority: P0
status: in-review
milestone: M0
labels: [area:infra, type:engineering]
depends_on: [GE-0008]
blocks: [GE-0018, GE-0032]
created: 2026-11-27
updated: 2026-11-27
owner_approval_required: false
---

# GE-0009 — Cross-platform CI skeleton

## Problem
No CI exists; cross-platform claims require native runner evidence.

## User / engineering value
Provides the verification surface for every later ticket and controls cost.

## Scope
### Included
- GitHub Actions jobs for ubuntu-latest, windows-latest, macos-latest and ARM64 where available
- Formatter, linter, test and build placeholders; cache/artifact limits
- Budget alerts and pinned Actions SHAs

### Excluded
- GPU lanes in public CI; paid signing

## Context and research
- Relevant sources:
  - `research/notes/quality-ci-gpu.md sections 2-3`
  - `ENVIRONMENT_REPORT.md section 5`
- Relevant ADRs:
  - `A02`
- Related tickets: `TICKETS/README.md` (dependency index)

## Proposed approach
Start with checks that pass on an empty workspace; expand per crate as code lands.

## Acceptance criteria
- [ ] All jobs green on an empty workspace
- [ ] Actions pinned to SHAs; lockfiles and --locked planned
- [ ] Cost notes recorded
- [ ] Tests and documentation updated

## Test plan
- Unit: n/a
- Integration: CI job run
- End-to-end: n/a
- Manual: Manual workflow_dispatch verify
- Platform coverage: Windows/macOS/Linux x64 + ARM64 where available

## Security / licensing / data considerations
No secrets; use OIDC/keyless where possible.

## Dependencies and risks
macOS runners are expensive ($0.062/min) and ARM64 runners have no static UDID.

## Implementation notes
No code yet: skeleton only.

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

GitHub issue: **#7** — https://github.com/Jasowills/genesis-engine/issues/7
- Files: `.github/workflows/ci.yml`; prototypes formatted with `cargo fmt --manifest-path …` (behaviour unchanged; tests re-run).
- Actions pinned to full commit SHAs: checkout `f548e57e…`, dtolnay/rust-toolchain `df2a88e9…`, cache `3edfce90…`.
- Jobs: `docs` (required-document baseline + 32-ticket count) on ubuntu; `prototype-tests` matrix ubuntu/windows/macos with Rust stable, `wasm32-wasip1` target, cargo cache, fmt check, WASM fixture build, precision (7 tests) and plugin sandbox (6 tests).
- No GPU runners, no signing, no secrets in workflow; `permissions: contents: read`.
- Local validation: `ruby -e YAML.load_file('.github/workflows/ci.yml')` → YAML OK; `cargo fmt --check` on all three prototypes → OK; precision tests 7 passed; plugin sandbox 6 integration tests passed.
- GitHub Actions run evidence: to be appended after workflow execution.
- Reviewer result: pending independent review.

## Changelog
- 2026-11-27: Ticket created from the proposed research/architecture programme; current status: `proposed`. Pending owner approval of scope and independent review where required.
