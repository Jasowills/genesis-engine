# Changelog

All notable changes to this project will be documented in this file. Format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and the project uses Conventional Commits.

## [Unreleased]

### Added — M1 foundation

- `feat(core)`: portable project model and layout (GE-0010).
- `feat(core)`: versioned World IR v2 with v1 migrations (GE-0011).
- `feat(core)`: lawful-source gate and provenance export sidecar (GE-0012).
- `feat(geodata)`: GeoJSON boundary import with CRS/epoch validation (GE-0013).
- `feat(geodata)`: OSM footprint and road centreline ingestion (GE-0014).
- `feat(geometry)`: deterministic procedural road/building meshes (GE-0015).
- `feat(quality)`: structural validation gates with honest accuracy reporting (GE-0018).
- `feat(export)`: validated GLB interchange with provenance sidecar (GE-0019).
- `feat(studio)`: Core-backed scene snapshot and React shell (GE-0016).
- `test(studio)`: keyboard selection and provenance component tests (GE-0017).
- `feat(cli)`: headless Core command surface with versioned JSON output (GE-0022).
- `feat(core)`: quality profiles and honest preflight estimates (GE-0024).
- `feat(core)`: terrain and secondary environment layers (GE-0028).
- `feat(core)`: external worker, plugin SDK and selective regeneration contracts (GE-0023, GE-0025, GE-0027).
- `chore(ci)`: cross-platform test matrix, later fixed for workspace excludes.

### Added — 2026-11-27 (planning baseline, not product functionality)

Owner-approved at the Phase 8 checkpoint: all seven architecture ADRs (`adr/0002`–`0008`), Apache-2.0 for **core source only**, a public `Jasowills/genesis-engine` repository, and a hard source deny-list. PRD first-slice scope, CLI timing, first engine integration, cloud/collaboration scope and asset licences remain proposed or open.

- `GENESIS_ENGINE_SPEC.md`: foundational specification (owner-provided).
- `research/`: eight primary-source research briefs and five consolidated deliverables (ecosystem, competitive, licence/data rights, formats/standards, register).
- `engineering/`: environment report, workflow/skill provenance, architecture discovery, engineering report, test strategy, security model, performance budgets, release process.
- `adr/`: seven accepted architecture ADRs (0002–0008), the proposal set and a template.
- `PRD.md`, `ARCHITECTURE.md`, `ROADMAP.md`, `DECISIONS.md`, `progress.md`: approved foundational plan with open decisions recorded.
- `TICKETS/`: ticket workflow and 32 tracer-bullet tickets with blocking edges.
- `LICENSE`: Apache-2.0 for core source (owner-approved).

### Notes

- No product code, crates, UI, datasets, CI or GitHub issues are included in this baseline beyond the approved repository setup.
- Open decisions (D-007…D-009, D-013…D-018, D-021…D-024 and non-core licence questions) and eight evidence conflicts remain recorded; no capability is claimed as implemented.
