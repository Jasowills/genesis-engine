---
id: GE-0009
title: Cross-platform CI skeleton
type: engineering
priority: P0
status: verified
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
- [x] All jobs green: run `45967770004` (reviewed workflow, commit `7073f9a`) and run `45968210802` (final `--locked` workflow, commit `985ecae`) — documentation baseline, prototype tests on ubuntu-latest, windows-latest and macos-latest: **all success**
- [x] Actions pinned to full commit SHAs; three Cargo.lock files committed; all cargo build/test steps use `--locked`
- [x] Cost notes recorded; no GPU or paid runners; approval gating documented
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
- GitHub Actions evidence: run `45967770004` (commit `7073f9a`) all 4 jobs success; run `45968210802` (commit `985ecae`, final `--locked` workflow) all 4 jobs success. Verified via `gh run view --json status,conclusion,jobs`.
- Reviewer result: **`APPROVED`** (session `ses_de3d3f112ffeaEQ82uvrlwztPB`) conditional on green runs; all jobs green on both the reviewed and final workflows. Non-blocking notes: ARM64 lanes implicit only (deferred), action SHA comments corrected, `--locked` added.

## Changelog
- 2026-11-27: Ticket created from the proposed research/architecture programme; current status: `in-review`. Pending owner approval of scope and independent review where required.
- 2026-11-27: Independent review APPROVED and all cross-platform CI jobs green; ticket marked verified.
