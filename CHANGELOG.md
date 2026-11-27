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
- D-008, D-009, D-013 and D-021 are resolved; D-022 signing is declined; seven of eight evidence conflicts are resolved and one is partial. The entries below record implemented capabilities with their verified evidence.
- `chore(tickets)`: verify GE-0022 CLI, GE-0027 regeneration planner, GE-0023 worker contract, GE-0028 layers, GE-0025 plugin SDK (with explicit contract-only limitations).
- `feat(core)`: swappable AI adapter contract with canonical error codes (GE-0026).
- `feat(integration)`: pinned Blender 4.5.17 LTS GLB round-trip fixture (GE-0029).
- `feat(core)`: LOD/collision/compression and adversarial-hardening fixes (GE-0030, audits).
- `test(ai)`: owner-approved named ONNX models (SAM ViT-B quantised encoder, YOLOX-S, Depth Anything V2 Small) executed with checksum and licence records (GE-0006).
- `fix`: second adversarial audit — index/panic guards, DEM validation, evidence negation, CLI symlink/atomic writes, script defensiveness.
- `test`: end-to-end vertical slice and frozen benchmark harness (GE-0020).
- `docs`: contributor and governance documentation sweep (GE-0021).
- `test`: ADR conformance review — CLI IR validation, lawful-source gate on inference/plugin paths, normalised progress checkpoints, Tauri/Studio render tests, sandbox flake fix, full-adapter descriptor fields.
- `feat(core)`: WGS84 ECEF→ENU product precision path and local-frame origin from boundary bbox (ADR-0008).
