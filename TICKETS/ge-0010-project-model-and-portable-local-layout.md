---
id: GE-0010
title: Project model and portable local layout
type: feature
priority: P0
status: in-review
milestone: M1
labels: [type:feature, area:core]
depends_on: [GE-0007]
blocks: [GE-0011]
created: 2026-11-27
updated: 2026-11-27
owner_approval_required: false
---

# GE-0010 — Project model and portable local layout

## Problem
Users need a project that can be created, reopened and moved without a hidden global cache.

## User / engineering value
Foundation for all generation, storage and reproducibility behaviour.

## Scope
### Included
- Project manifest with schema version, CRS/epoch/datum/units
- Separate directories for metadata, world manifests, binary payloads, source refs, jobs, validation, exports
- Atomic write and portable state

### Excluded
- Autosave/crash recovery, templates, multi-user, migration tooling beyond basic version field

## Context and research
- Relevant sources:
  - `Spec sections 21, 32`
  - `ARCHITECTURE.md sections 2 and 4`
- Relevant ADRs:
  - `A01`
  - `A07`
- Related tickets: `TICKETS/README.md` (dependency index)

## Proposed approach
Implement the minimal manifest and layout behind Core commands usable headlessly.

## Acceptance criteria
- [x] Project create/open/save round-trip passes
- [x] All paths relative and portable
- [x] Unknown/malformed manifest produces structured error
- [x] Tests and documentation updated

## Test plan
- Unit: Manifest serialization and path validation
- Integration: Project round-trip across OSes
- End-to-end: n/a
- Manual: Manual move/reopen
- Platform coverage: macOS/Linux/Windows

## Security / licensing / data considerations
Archive extraction hardening and path containment required.

## Dependencies and risks
A01/A07 must be accepted first.

## Implementation notes
Headless-drivable is the test that this is a Core capability.

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

GitHub issue: **#27** — https://github.com/Jasowills/genesis-engine/issues/27
- Files: root `Cargo.toml` (minimal workspace), `crates/genesis-core/` (Cargo.toml, src/lib.rs, tests/project.rs), `docs/developer/project-layout.md`, `Cargo.lock`, CI Core test/lint steps.
- `cargo test --workspace --locked` → **6 passed, 0 failed** (round-trip, portability after move, unknown schema version, malformed/missing manifest, path containment, duplicate project).
- `cargo clippy --workspace --locked -- -D warnings` → clean after fixing a collapsible-if warning.
- Path validator hardened after a failing test: `C:secret` and backslash paths are rejected on Unix too, not only where Rust reports a Prefix component.
- Atomic manifest write via temp file + rename; Windows fallback documented as non-atomic in code.
- Documentation: `docs/developer/project-layout.md` describes layout, API and schema version 1.
- Reviewer result: pending independent review.

## Changelog
- 2026-11-27: Ticket created from the proposed research/architecture programme; current status: `proposed`. Pending owner approval of scope and independent review where required.
- 2026-11-27: Project model implemented and tested; status moved to in-review.
