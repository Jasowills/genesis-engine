# Genesis Engine — Live Progress

## Current phase
Phase 7 — GitHub project setup. Phase 8 owner checkpoint completed 2026-11-27; foundational plan approved.

## Current milestone
M1 — Foundation and first vertical slice. Research, architecture, repository setup and GitHub issues are complete; Core primitives are implemented and independently verified.

## Current objective
Advance M1 tickets in dependency order: Studio shell/viewport, E2E vertical slice, and remaining foundation tickets. Each code-changing ticket must pass tests, lint, formatting and independent review before `verified`. No release or deployment without owner approval.

## Overall status
M1 implementation in progress. Research, architecture, ADRs, PRD, roadmap, tickets and GitHub setup are complete. Rust Core implements project model, World IR, lawful-source gate, geospatial import, procedural meshes, validation gates and validated GLB export. A Core-backed Studio foundation (Rust snapshot + React shell) exists but no Tauri window has been visually verified; a headless CLI, but no worker pipeline or game-engine integration yet.

## Last updated
2026-11-27 (Phase 8 approvals recorded; GitHub setup executed; post-execution documentation reconciliation).

## Approved product decisions
D-001 engine + platform progressively; D-002 desktop-first Studio; D-003 local-first, cloud optional; D-004 bundled open models + optional external providers; D-005 game developers and technical artists equally; D-006 Windows/macOS/Linux (spec Part III). **Owner-approved 2026-11-27:** D-025 seven ADRs; D-026 Apache-2.0 **core only**; D-027 public `Jasowills/genesis-engine`; D-028 hard source deny. See `DECISIONS.md`.

## Approved architecture decisions
All seven: thin versioned World IR + adapters (`adr/0002`); Tauri + React/TS + three.js with shell/viewport as two PoC-gated decisions (`0003`); external-process workers with normalised progress (`0004`); ONNX Runtime primary behind swappable adapter (`0005`); Wasmtime/WASI 0.2 sandbox (`0006`); lawful-source gate + provenance-first metadata (`0007`); floating-origin/ENU precision (`0008`). PoC and spike gates are retained; acceptance of direction is not a waiver of verification.

## Open owner questions
D-007 platform boundaries; D-015 cloud execution; D-016 collaboration/marketplace scope; D-017 skill identity (assumed `to-tickets`); D-018 setup-skill tracker (GitHub Issues assumed via D-027); D-022 signing identities and budget; D-023 Rust MSRV; D-024 ODbL §4.6 publication legal review; non-core/plugin/model-weight licences under D-014; named AI model weights cost/licence. Resolved: D-008, D-009, D-013, D-021 (D-029…D-031), D-032 fixture licence. Questions Q1–Q8 are in `engineering/ARCHITECTURE_DISCOVERY.md` §7. None blocks the approved first slice; no purchase or publication happens without approval.

## Current repository state
- Repository URL: **https://github.com/Jasowills/genesis-engine** (PUBLIC, verified via `gh repo view`)
- Branch: `main`, tracking `origin/main`
- Latest commit: **read live with `git rev-parse HEAD`** (this file is itself committed after each update, so a hard-coded hash would always lag by one commit). Reconciliation baseline: `15e5b4548c180cc4cf4160cf81a837c50efa04ef`; earlier commits `5fb68abba2f1ec801d2c887f63b4c30f4360f545` (issue links) and `aa728fd9a9aa5e0bf2ca05b94006434816be973d` (initial 81-file documentation-only commit).
- Working tree: clean (`## main...origin/main`); 40 labels, 6 milestones, **32 issues** created in dependency order with native `--blocked-by` links (verified on issue #2: `blocked_by_added` → issue #1). Local tickets and the index carry issue links. Product code exists in `crates/genesis-core` (project model, World IR, import, procedural, validation, GLB export, headless CLI commands) and `crates/genesis-studio` with a React shell in `apps/studio`; no Tauri window has been visually verified; the headless CLI is implemented and reviewed.
- Toolchain: macOS 26.5.2 x86_64; Rust 1.98.1; Node 24.16.0; Python 3.15.2; Go 1.27.1; Docker 29.8.2; git 2.50.1; gh 2.97.0 (auth: `Jasowills`)
- Platforms tested: macOS/x86_64 only. Windows, Linux, ARM64, Apple Silicon, CUDA untested.

## Completed and verified tickets
GE-0008 — GitHub repository and issue setup (independent post-execution review `APPROVED`; evidence recorded in its ticket).
GE-0009 — cross-platform CI skeleton (review `APPROVED`; all 4 jobs green on Windows/macOS/Ubuntu, final `--locked` run `45968210802`).
GE-0007 — ADR set and decision register (D-025…D-031 recorded; governance review `APPROVED`).
GE-0010 — project model and portable layout (7 tests pass; independent re-review `APPROVED`).
GE-0011 — World IR v2 with v1 migrations (16 Core tests pass; independent re-review `APPROVED`).
GE-0012 — lawful-source gate and provenance sidecar (22 Core tests pass; independent re-review `APPROVED`).
GE-0013 — GeoJSON boundary import and CRS validation (31 Core tests pass; independent re-review `APPROVED`).
GE-0014 — OSM footprint and road feature ingestion (39 Core tests pass; independent re-review `APPROVED`).
GE-0004 — desktop/viewport capability PoC (three-OS CI green; independent review `APPROVED`).
GE-0015 — deterministic procedural road/building meshes (48 Core tests pass; independent re-review `APPROVED`).
GE-0018 — structural validation gates (53 Core tests pass; independent review `APPROVED`).
GE-0019 — GLB export with 0-error glTF-Validator result (57 Core tests pass; independent review `APPROVED`).
GE-0021 — contributor/governance documentation sweep (independent review `APPROVED`).
GE-0022 — headless CLI over Core (72 tests; final independent confirmation `APPROVED`).
GE-0027 — selective regeneration planner (planning seam only; independent confirmation `APPROVED`).
GE-0023 — external worker supervision contract (contract only; independent re-review `APPROVED`).
GE-0026 — AI adapter contract (adapter only; independent review `APPROVED`).
GE-0028 — terrain/secondary layer generators (5 layer tests; independent re-review `APPROVED`).
GE-0025 — plugin SDK contract (contract only; independent review `APPROVED`).
GE-0016 — Core-backed Studio snapshot + React shell (64 tests; independent review `APPROVED`).
GE-0017 — select/inspect/provenance display (6 component tests; independent re-review `APPROVED`).
GE-0020 — E2E vertical slice + frozen benchmark harness (65 tests; independent review `APPROVED`).
GE-0003 — precision/CRS golden-vector spike (7 tests pass; independent re-review `APPROVED`).
GE-0002 — resolve eight evidence conflicts (7 resolved, 1 partial; independent re-review `APPROVED`).
GE-0001 — environment/workflow records (re-verified commands, skill provenance; independent re-review `APPROVED`).

## In-progress tickets
None.

## In-review tickets
GE-0030 — LOD/collision/compression allowlist (3 tests; independent review pending, issue #26).
GE-0029 — Blender 4.5.17 LTS round-trip (2 tests; independent review pending, issue #25).

## Blocked tickets
GE-0005 — plugin sandbox spike: deliverable evidence independently `APPROVED`, cross-platform CI green (6/6 per OS); WASIp1 network-policy denial remains UNVERIFIED (no sockets in wasm32-wasip1 std). Not marked `verified` (issue #22).
GE-0006 — ORT 1.32.0 smoke test passes in CI; named model weights (SAM/YOLOX/Depth Anything) not loaded — requires owner cost/licence decision (issue #9).
GE-0030 / GE-0031 — LOD/collision/texture compression and engine export fixtures: blocked on GE-0029.
GE-0032 — signed release/auto-update: signing identities and budget require owner approval (D-022, issue #21).

## Ready next tickets
GE-0025 (WIT/WASI plugin SDK PoC) is dependency-ready. GE-0024 is in review.

## Current architecture snapshot
Accepted Core-centric shape: 12 components (C1–C12) and seams S1–S12 in `ARCHITECTURE.md`. Implemented so far: Rust workspace with three crates — `crates/genesis-core`, `crates/genesis-cli` (portable project model, versioned World IR with stable IDs/provenance/f64 local frames/v1↔v2 migrations, lawful-source gate, geospatial import, procedural geometry, structural validation, GLB export) and `crates/genesis-studio` (Core-backed scene snapshots, optional Tauri command wrappers) plus the React shell in `apps/studio`. No Tauri window has been visually verified; a headless CLI (`crates/genesis-cli`), a worker supervision contract (no process execution), no AI adapters, plugin runtime or game-engine integration yet.

## Important module boundaries
UI owns presentation only; domain truth behind Core versioned commands/events/contracts. A capability must be headlessly drivable to be a Core capability. No crate layout prescribed (spec §42).

## Key data contracts and schema versions
World IR schema v2 with v1 migrations is implemented and verified (GE-0011); provenance/metric records and versioned job records (GE-0023) are implemented in `crates/genesis-core`. AI adapter contract and real worker payload execution remain requirements in `ARCHITECTURE.md` §2 and `engineering/ARCHITECTURE_DISCOVERY.md` §2.1, not yet shipped.

## Research completed
All 8 primary-source briefs, 5 consolidated deliverables (`ECOSYSTEM_RESEARCH.md`, `COMPETITIVE_ANALYSIS.md`, `LICENCE_AND_DATA_RIGHTS.md`, `FORMAT_AND_STANDARDS.md`, `RESEARCH_REGISTER.md`), 8 logged conflicts.

## Research still required
Close the 8 conflicts via their linked tickets; obtain owner answers to open questions; legal review of ODbL §4.6 and AGPL worker boundaries; D-021 benchmark boundary; measured performance (all current figures are estimates).

## Tests and verification
- Last full test run: `cargo test --workspace --locked` → 91 passed, 0 failed (Core incl. 6 incremental + 8 job + 5 layer + 5 profile tests, Studio, CLI; macOS/x86_64, rustc 1.98.1).
- Platform coverage: environment discovery commands only; host macOS/x86_64.
- Known failures: CI runs `45979438978` (commit `5626277`) and `45979776607` (commit `be8d359`) failed in the formatting step because the newly added root workspace made throwaway prototype crates believe they were members. Fixed in commit `4a78f0e` by adding explicit `exclude` entries; verification run `45980065945` **completed / success** (all jobs green). Regression recorded in `TICKETS/ge-0009-ci-skeleton.md`. No product-code test failures were reported.
- Commands: version checks, `git status`/`rev-parse`, `gh auth status`/`api user`/`repo list`, `gh issue create --help`, skill hash comparison, `shasum`, file listings, secret-pattern grep.

## Known bugs and technical debt
None in code. Design risks: precision must be right before projects exist; incremental regeneration has no mature ecosystem contract; plugin manifest/rollback is design inference; no standard provenance vocabulary.

## Risks and assumptions
Local host is Intel macOS only; cross-platform claims need CI evidence. Wasmtime has a fast CVE cadence; pin ≥51.0.0. ODbL §4.4/§4.6 treatment of generated GLBs needs legal review. GitHub token carries `delete_repo` scope — no destructive operations. Never print tokens.

## Files/docs that must be read next
1. `DECISIONS.md` §3a (owner approvals) and §2/§3 (open decisions)
2. `ARCHITECTURE.md` and `adr/0002`–`0008`
3. `TICKETS/README.md` and the ready M0 ticket files
4. `engineering/WORKFLOW.md`, `ENVIRONMENT_REPORT.md`, `SECURITY_MODEL.md`

## Exact next action
Finish GE-0024/GE-0025 reviews; blocked tickets require owner/tooling decisions. Every code-changing ticket needs tests, lint, formatting and independent review before `verified`. GitHub: https://github.com/Jasowills/genesis-engine, branch `main`, 32 issues.

## Session handoff notes
Server restarted on 2026-11-27 during research; two subagent sessions were resumed and all durable files survived. Do not start product implementation. Do not create further public artefacts beyond D-027. Never print tokens or commit secrets, restricted data or large model weights. Maintain this file and ticket states after every verified unit of work.
