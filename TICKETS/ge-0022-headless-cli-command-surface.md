---
id: GE-0022
title: Headless CLI command surface
type: feature
priority: P1
status: in-review
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
- [x] Every E2E step reachable headlessly
- [x] Output schema versioned and documented
- [x] Same results as Studio (CLI scene descriptor compared with Core/GLB re-import; Studio uses the same Core snapshot source)
- [x] Tests and documentation updated

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

GitHub issue: **#18** — https://github.com/Jasowills/genesis-engine/issues/18
- Files: `crates/genesis-cli/` (Cargo.toml, src/main.rs, tests/cli.rs), workspace member, `docs/developer/cli.md`.
- `cargo test --workspace --locked` → **67 passed, 0 failed** (65 previous + 2 CLI integration tests). `cargo clippy --workspace --locked -- -D warnings` clean; `cargo fmt --all -- --check` clean.
- CLI integration test drives project create → boundary → features → generate → validate → export → scene → job status through the binary, asserts versioned JSON `status: ok`, and verifies the exported GLB with Core `import_glb` (2 meshes, 14 triangles, attribution preserved).
- Structured error test asserts `status: error` with `schema_version: 1` on an invalid profile.
- Job status/cancel return structured `unsupported` (no fake success); worker orchestration is GE-0023.
- Reviewer result: pending independent review.

## Changelog
- 2026-11-27: Ticket created from the proposed research/architecture programme; current status: `proposed`. Pending owner approval of scope and independent review where required.
