# Genesis Engine — Roadmap

**Date:** 2026-11-27. **Status:** Milestone sequence accepted with the foundational plan (D-025); M1 scope approved (D-029), CLI timing and first export order approved (D-030), benchmark boundary delegated (D-031); D-007, D-015, D-016 and non-core licence questions remain open. Dates are intentionally absent — commitments require approved scope and measured throughput.

## Milestones

| ID | Name | Outcome (verifiable) | Depends on | Exit gate |
| --- | --- | --- | --- | --- |
| **M0** | Research, architecture and approval | Eight research briefs consolidated; conflicts resolved; seven ADRs accepted (D-025); core licence Apache-2.0 (D-026); public repo approved (D-027); hard source deny acknowledged (D-028); GitHub setup complete | Owner decisions D-025…D-028 obtained; residual conflicts to be closed by tickets | GitHub setup verified and independently reviewed; no product code |
| **M1** | Foundation and first vertical slice | Project creation → boundary import → CRS/IR → roads+simple buildings → viewport inspect → validation → GLB export → re-import, on one small fixture, headlessly drivable | M0; A01, A02, A06, A07 accepted | E2E fixture passes on all three OSes; independent reviewer approves every code ticket |
| **M2** | Early developer preview | CLI/headless, documented Core SDK seams, worker adapters (COLMAP/Open3D via processes), quality profiles Preview/Game-ready, plugin WIT PoC, licence/legal closure | M1; A03, A04, A05 accepted | Public preview with known limitations; reproducible builds; licence audit |
| **M3** | Public alpha | More layers (terrain, sidewalks, markings, vegetation), selective/incremental regeneration, AI segmentation/depth adapters, Blender export fixture, preflight estimates | M2 | Alpha release evidence; no survey-grade claims |
| **M4** | Public beta | LOD/collision/material compression, target engine exports (Unity/Unreal/Godot as researched), 3D Tiles streaming export, plugin registry (signed manifests) | M3 | Compatibility fixtures for each claimed integration |
| **M5** | Stable | Cross-platform release, signed auto-update, full validation suite, survey/reference profile only where checkpoints exist | M4 | Release-readiness checklist and post-deploy verification |

## Dependency rules

- M0 decisions block all product tickets. No ticket may be `ready` if its ADR is only PROPOSED.
- Every code-changing ticket requires an independent reviewer subagent before `verified` (spec §39).
- Wide refactors use expand–contract (`to-tickets` skill) rather than a single breaking change.
- Tickets are tracer-bullet vertical slices; horizontal "build all of layer X" tickets are split.
- Benchmark-first rule (spec §49): the Ikeja, Lagos study boundary must be owner-selected and frozen before any comparative benchmark.

## Release stages (spec §52)

Research/architecture prototype → core data/geometry foundation → first E2E vertical slice → early developer preview → public alpha → public beta → stable. Each stage publishes scope, non-goals, platform matrix, known limitations, install/migration notes, test evidence and security/licence review. No stable support claim for untested features.

## Cost factors (verified, not estimates)

Public standard CI runners are free; private rates $0.006 Linux / $0.010 Windows / $0.062 macOS per minute; GPU larger runners $0.052 Linux / $0.102 Windows per minute (Team/Enterprise only, billed even for public repos). Signing: Apple $99/yr; Windows Artifact Signing ~$9.99/mo, OV $150–300/yr, EV $400+/yr. All purchase decisions require owner approval.
