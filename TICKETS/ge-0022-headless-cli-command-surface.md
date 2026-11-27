---
id: GE-0022
title: Headless CLI command surface
type: feature
priority: P1
status: verified
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
- `cargo test --workspace --locked` → **72 passed, 0 failed** (Core + Studio + CLI + incremental tests). CLI integration tests drive the full E2E workflow and a structured invalid-profile error on a valid project. `cargo clippy --workspace --locked -- -D warnings` clean; `cargo fmt --all -- --check` clean.
- CLI integration test drives project create → boundary → features → generate → validate → export → scene → job status through the binary, asserts versioned JSON `status: ok`, and verifies the exported GLB with Core `import_glb` (2 meshes, 14 triangles, attribution preserved).
- Structured error test asserts `status: error` with `schema_version: 1` on an invalid profile.
- Job status/cancel return top-level `status: "unsupported"` (no fake success); worker orchestration is GE-0023. CRS is loaded from the project manifest and provenance from `metadata/source.json`; arbitrary input is never implicitly attributed to OSM. `--help`/clap parse errors remain plain text (documented exception); orchestration job records and a dedicated reproducibility subcommand are deferred.
- Reviewer result: two `CHANGES_REQUESTED` passes (CRS/provenance, misleading error test, stale bookkeeping, CRS/persistence, scene reproducibility); all findings fixed; final confirmation **`APPROVED`** (session `ses_de225eaa3ffeHEXKLzAh32Yh5k`).

## Changelog
- 2026-11-27: Ticket created from the proposed research/architecture programme; current status: `proposed`. Pending owner approval of scope and independent review where required.
- 2026-11-27: Independent review APPROVED; ticket marked verified.
- 2026-11-27: Second adversarial audit — reject symlink destinations, atomic source writes, validate imports before persisting provenance, and non-UTF-8 argv without panic. Regression tests added; ticket remains verified.
