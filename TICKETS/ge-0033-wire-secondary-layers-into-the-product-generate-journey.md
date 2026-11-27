---
id: GE-0033
title: Wire secondary layers into the product generate journey
type: feature
priority: P1
status: in-review
milestone: M4
labels: [type:feature, priority:P1, status:ready, area:geometry]
depends_on: [GE-0028, GE-0030, GE-0022]
blocks: []
created: 2026-11-28
updated: 2026-11-28
owner_approval_required: false
---

# GE-0033 — Wire secondary layers into the product generate journey

## Problem
The shipped CLI generates only buildings and roads; terrain, markings, sidewalks and vegetation exist in Core but never reach payloads/meshes.json, the GLB or Studio.

## User / engineering value
A user asking for a world gets a visibly richer, source-attributed asset set without new generators.

## Scope
### Included
- Optional --layers <dir> on genesis generate using the four bundled lawful fixtures
- All generated layer meshes appended to payloads/meshes.json with the project frame
- Run seed and generator version recorded on every layer mesh (replacing hard-coded zeros)
- Denied layer source produces a structured error and no partial write

### Excluded
- New layer algorithms
- Texture/image assets
- Facade or bridge modelling

## Context and research
- Relevant sources: `research/notes/procedural.md`, `research/notes/desktop-viewport-formats.md`, `docs/product/E2E_QUALITY_VERDICT.md`
- Relevant ADRs: `adr/0002`, `adr/0003`, `adr/0007`, `adr/0008`
- Related tickets: `TICKETS/README.md`

## Proposed approach
Small tracer-bullet slice; keep existing versioned contracts and lawful-source gate. No proprietary or uncached visual assets.

## Acceptance criteria
- [x] Same inputs and seed produce an identical mesh_subject_hash
- [x] With Ikeja fixtures the scene reports 7 meshes and includes layers-v0.1
- [x] Every mesh passes verify_mesh and retains lawful provenance
- [x] A denied source fixture produces exit 1 with error.kind cli_error

## Test plan
- Unit: CLI integration assertions; Integration: product_e2e.py harness; E2E: headless journey only
- Platform coverage: three-OS CI matrix

## Security / licensing / data considerations
Hard-denied sources remain rejected (D-028). No map imagery, no textured assets, no unsupported quality claim.

## Dependencies and risks
Generator version and seed changes affect stable IDs; record them in evidence. Synthetic DEM and inferred layers must never be labelled surveyed.

## Implementation notes
Proceed only after blockers are verified; evidence and independent review required before `verified`.

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

GitHub issue: **#33** — https://github.com/Jasowills/genesis-engine/issues/33
- Files: `crates/genesis-cli/src/main.rs` (`generate --layers`, layer source registration, World IR object registration, export source list), `crates/genesis-core/src/layers.rs` (`apply_run_metadata`), `crates/genesis-core/src/lib.rs`, `crates/genesis-cli/tests/cli.rs`, `tools/product_e2e.py`, `docs/product/evidence/studio-layers.png`.
- `cargo test --workspace --locked` → **135 passed, 0 failed** (10 CLI tests incl. 2 new layer tests: 7 meshes with layers and denied-source failure without payload overwrite). `cargo clippy --workspace --locked --all-targets -- -D warnings` clean; `cargo fmt --all -- --check` clean.
- Product harness on the shipped CLI: 7 meshes / 30 triangles, 0 degenerate, 0 downward, all finite; GLB parses with 7 meshes / 30 triangles and `genesis` extras; `deterministic_regeneration: true`; scene reports 7 selectable nodes including terrain and vegetation.
- Layer sources are persisted in `metadata/layer_sources.json` and registered in World IR, so export accepts them and Studio makes layer nodes selectable.
- Reviewer result: pending independent review.

## Changelog
- 2026-11-28: Ticket created from the product visual-fidelity review and owner approval to proceed.
