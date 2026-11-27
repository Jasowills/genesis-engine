# Genesis Engine

An open-source, cross-platform platform for reconstructing real-world environments into editable, validated, game-ready 3D worlds.

> **Status: M1 foundation in progress.** The repository contains the specification, research, accepted architecture, product requirements, roadmap, ticket backlog and a Rust Core crate implementing the portable project model, versioned World IR, lawful-source gate, geospatial import, deterministic procedural geometry, structural validation gates and a validated GLB export. A Core-backed Studio foundation (Rust scene snapshot + React/TypeScript shell) exists; no Tauri window has been visually verified. No CLI, worker pipeline or game-engine integration yet. Do not interpret planning documents as shipped features.

## What Genesis is

- **Genesis Core** — engine-agnostic reconstruction, world-data, validation and export foundation (Rust-first).
- **Genesis Studio** — desktop visual environment for creating, inspecting, editing, validating and exporting worlds.
- Progressively: CLI, headless execution, SDK, plugin interfaces and integration APIs.

Local-first with optional cloud; desktop-first; Windows, macOS and Linux; game developers and technical artists equally.

## Documentation

| Document | Purpose |
| --- | --- |
| [GENESIS_ENGINE_SPEC.md](GENESIS_ENGINE_SPEC.md) | Foundational specification and operating charter (authority) |
| [PRD.md](PRD.md) | Product requirements, users, journeys, first-slice scope and non-goals |
| [ARCHITECTURE.md](ARCHITECTURE.md) | Accepted system design, components and seams |
| [ROADMAP.md](ROADMAP.md) | Milestones M0–M5 and release stages |
| [DECISIONS.md](DECISIONS.md) | Approved decisions and open questions |
| [progress.md](progress.md) | Live progress and exact next action |
| [adr/](adr/) | Accepted ADRs 0002–0008 and the proposal set |
| [research/](research/) | Ecosystem, competitive, licence, format and standards evidence |
| [engineering/](engineering/) | Environment, workflow, architecture discovery, test/security/performance/release plans |
| [TICKETS/](TICKETS/) | 32 tracer-bullet tickets with blocking edges |
| [docs/developer/](docs/developer/) | Implemented Core primitives: project layout, World IR, lawful sources, boundary/OSM import, procedural meshes, validation, GLB export |

## Accepted architecture foundation (2026-11-27)

1. Thin versioned World IR with adapters, not USD-based IR ([ADR-0002](adr/0002-thin-versioned-world-ir-with-adapters-vs-usd-based-ir.md)).
2. Tauri + React/TypeScript + three.js, with shell and viewport as separate PoC-gated decisions ([ADR-0003](adr/0003-tauri-react-ts-three-js-shell-and-viewport-as-two-decisions.md)).
3. External-process workers with normalised progress events ([ADR-0004](adr/0004-external-process-workers-with-normalised-progress-events.md)).
4. ONNX Runtime primary behind a swappable AI adapter contract ([ADR-0005](adr/0005-onnx-runtime-primary-behind-a-swappable-ai-adapter-contract.md)).
5. Wasmtime/WASI 0.2 sandbox for untrusted plugins ([ADR-0006](adr/0006-wasmtime-wasi-0-2-plugin-sandbox.md)).
6. Lawful-source gate and provenance-first metadata ([ADR-0007](adr/0007-lawful-source-adapter-boundary-and-provenance-first-metadata.md)).
7. Floating-origin/local-frame precision strategy ([ADR-0008](adr/0008-floating-origin-local-frame-precision-strategy.md)).

**Hard source constraint:** Google Maps/Street View/Photorealistic 3D Tiles content and Cesium ion-hosted content cannot be reconstruction inputs; their terms prohibit derivation, offline caching and AI training. Open standards and permissive software remain usable with Genesis-owned content.

## Licence

Core source code is **Apache-2.0** ([LICENSE](LICENSE), owner-approved 2026-11-27). SDK, documentation, test fixtures, sample generated assets and model weights are separate licence decisions recorded as they are made. Third-party code, datasets and model weights retain their own licences; see [research/LICENCE_AND_DATA_RIGHTS.md](research/LICENCE_AND_DATA_RIGHTS.md).

## Contributing

Read [CONTRIBUTING.md](CONTRIBUTING.md), [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md), [SECURITY.md](SECURITY.md) and [engineering/WORKFLOW.md](engineering/WORKFLOW.md). Every code-changing ticket is implemented in one focused unit, tested, independently reviewed by a different agent, and only then marked `verified`. Use Conventional Commits. Never commit secrets, personal data, restricted imagery or large model weights.
