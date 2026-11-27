---
id: GE-0012
title: Lawful-source gate and provenance record
type: feature
priority: P0
status: verified
milestone: M1
labels: [type:feature, area:core, type:security]
depends_on: [GE-0002, GE-0011]
blocks: [GE-0013, GE-0015]
created: 2026-11-27
updated: 2026-11-27
owner_approval_required: false
---

# GE-0012 — Lawful-source gate and provenance record

## Problem
Terms prohibit derivation from Google Maps/Cesium ion content; ODbL requires attribution and possible derivative offer.

## User / engineering value
Makes the provenance differentiator real and prevents rights violations in code paths.

## Scope
### Included
- SourceDescriptor with licence election, attribution, vintage, filters, checksum
- Deny-list enforced before import/inference/export
- Provenance record carried into IR and export sidecar

### Excluded
- Visual-only basemap carve-out, legal determination of generated GLB status

## Context and research
- Relevant sources:
  - `research/LICENCE_AND_DATA_RIGHTS.md`
  - `adr/ADR_PROPOSALS.md A06`
- Relevant ADRs:
  - `A06`
- Related tickets: `TICKETS/README.md` (dependency index)

## Proposed approach
Cross-cutting service consulted by all import and export paths.

## Acceptance criteria
- [x] Denied sources are rejected in tests
- [x] Attribution and licence election survive IR to export
- [x] Provenance record covers all fields from research synthesis
- [x] Tests and documentation updated

## Test plan
- Unit: Gate decision logic
- Integration: Import/export path integration
- End-to-end: n/a
- Manual: n/a
- Platform coverage: All

## Security / licensing / data considerations
This is a security/licence boundary; no secrets; no restricted scraping.

## Dependencies and risks
Legal review of ODbL §4.6 remains open.

## Implementation notes
Do not defer: hard blocker for any data import.

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

GitHub issue: **#10** — https://github.com/Jasowills/genesis-engine/issues/10
- Files: `crates/genesis-core/src/lawful_source.rs`, `crates/genesis-core/tests/lawful_source.rs`, `docs/developer/lawful-sources.md`, `CoreError::SourceDenied` variant.
- `cargo test --workspace --locked` → **22 passed, 0 failed** (7 project + 9 World IR + 6 lawful-source), covering all 15 deny patterns, case/hyphen variants, licence spellings, purpose behaviour, full provenance and export gating.
- `cargo clippy --workspace --locked -- -D warnings` → clean; `cargo fmt --all -- --check` → clean.
- Deny-list checks publisher, url, id, version, attribution and filters; covers Google Maps/Street View/Photorealistic 3D Tiles and Cesium ion content (hosts, hyphens, casing) plus non-commercial licences; open CesiumJS/3D Tiles software remains permitted.
- `prepare_import` gates import; `enforce_sources`/`attribution_sidecar` gate export. Provenance now carries publisher, url, version, licence URL, source feature IDs, derivation chain and evidence; sidecar dedups on full source identity and cannot export unregistered or denied sources.
- Reviewer result: first pass `CHANGES_REQUESTED` (deny-list bypasses, isolated helper, incomplete provenance, overclaimed evidence, stale bookkeeping); all six findings fixed; narrow re-review **`APPROVED`** (session `ses_de391253dffevFwKtYS1ppuboE`) with 22 tests, clippy and fmt re-run.

## Changelog
- 2026-11-27: Ticket created from the proposed research/architecture programme; current status: `proposed`. Pending owner approval of scope and independent review where required.
- 2026-11-27: Gate wired into import/export, deny-list and provenance hardened; status in-review.
- 2026-11-27: Independent re-review APPROVED; ticket marked verified.
