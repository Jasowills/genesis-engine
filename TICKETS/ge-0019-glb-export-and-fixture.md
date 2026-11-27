---
id: GE-0019
title: GLB export and re-import fixture
type: feature
priority: P1
status: in-review
milestone: M1
labels: [type:feature, area:export]
depends_on: [GE-0011, GE-0018]
blocks: [GE-0020, GE-0029]
created: 2026-11-27
updated: 2026-11-27
owner_approval_required: false
---

# GE-0019 — GLB export and re-import fixture

## Problem
The first interchange target must be validated, not assumed.

## User / engineering value
Delivers spec §50 steps 11-12 and proves metadata survival.

## Scope
### Included
- IR to GLB with extension allowlist and attribution sidecar
- Round-trip fixture: export then re-import/open in a verified tool
- Validation report archived

### Excluded
- USD, 3D Tiles, engine plugins, LOD/collision export

## Context and research
- Relevant sources:
  - `research/FORMAT_AND_STANDARDS.md`
  - `research/notes/quality-ci-gpu.md section 1`
- Relevant ADRs:
  - `A01`
  - `A06`
- Related tickets: `TICKETS/README.md` (dependency index)

## Proposed approach
Map IR to glTF 2.0; record unsupported fields explicitly rather than silently dropping.

## Acceptance criteria
- [x] glTF-Validator exit code 0
- [x] Provenance/attribution survive round-trip
- [x] Compatibility fixture documented
- [x] Tests and documentation updated

## Test plan
- Unit: Mapping unit tests
- Integration: Round-trip integration
- End-to-end: n/a
- Manual: Open export in verified tool
- Platform coverage: macOS/Linux/Windows

## Security / licensing / data considerations
No unrestricted third-party asset download.

## Dependencies and risks
Unreal/Unity/Godot support not claimed until fixtures exist.

## Implementation notes
GLB is interchange, not internal format.

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

GitHub issue: **#16** — https://github.com/Jasowills/genesis-engine/issues/16
- Files: `crates/genesis-core/src/export_glb.rs`, `tests/export_glb.rs`, `prototype/gltf-fixture/` (validator script, package manifest/lock, RESULTS.md, generated `fixture.glb`), `docs/developer/glb-export.md`, CI format-gate step, re-exports.
- `cargo test --workspace --locked` → **56 passed, 0 failed** (including four GLB tests after the gate-binding fix) (9 GeoJSON + 6 lawful-source + 8 OSM + 9 procedural + 5 validation + 3 GLB + 7 project + 9 World IR). Three GLB tests cover round-trip/provenance, gate enforcement and malformed input.
- Official Khronos `gltf-validator` 2.0.0-dev.3.10 on generated `fixture.glb`: **0 errors, 0 warnings, 0 infos**, exit 0. Two earlier validator errors (`EMPTY_ENTITY`, `BUFFER_VIEW_TARGET_MISSING`) were fixed and are recorded in `prototype/gltf-fixture/RESULTS.md`.
- Source/validation gates run before export; unsupported fields are emitted as warnings, not silently dropped.
- CI format-gate step added for Ubuntu; validation report now carries `subject_hash` and the export gate rejects report/world/mesh mismatch, unregistered mesh sources and incomplete provenance.
- Reviewer result: first pass `CHANGES_REQUESTED` (clippy unused_mut, unbound validation gate, stale bookkeeping, validator command precision); fixes applied, re-review pending.

## Changelog
- 2026-11-27: Ticket created from the proposed research/architecture programme; current status: `proposed`. Pending owner approval of scope and independent review where required.
