# Genesis Engine — Live Progress

## Current phase
Phase 7 — GitHub project setup. Phase 8 owner checkpoint completed 2026-11-27; foundational plan approved.

## Current milestone
M0 — Research, architecture and approval. Research and planning complete; repository setup is the remaining action.

## Current objective
Create the approved public GitHub repository with a small verified documentation-only commit, labels, milestones and issues from the 32 approved tickets with native blocking links; report exact URL/branch/hash/issue count. Then run the unblocked M0 spikes with independent review. No product implementation.

## Overall status
Pre-implementation. All research, architecture, ADRs, PRD, roadmap and tickets are complete. Owner approved seven ADRs, Apache-2.0 core, a public repository and a hard source deny-list. No product code, crates, UI, datasets or CI exist. The public repository now has 2 documentation-only commits, 40 labels, 6 milestones and 32 issues with native blocking links. Independent review of the governance records ended `APPROVED`; post-execution verification of GitHub setup returned `CHANGES_REQUESTED` only for stale local documentation (being corrected here).

## Last updated
2026-11-27 (Phase 8 approvals recorded; GitHub setup executed; post-execution documentation reconciliation).

## Approved product decisions
D-001 engine + platform progressively; D-002 desktop-first Studio; D-003 local-first, cloud optional; D-004 bundled open models + optional external providers; D-005 game developers and technical artists equally; D-006 Windows/macOS/Linux (spec Part III). **Owner-approved 2026-11-27:** D-025 seven ADRs; D-026 Apache-2.0 **core only**; D-027 public `Jasowills/genesis-engine`; D-028 hard source deny. See `DECISIONS.md`.

## Approved architecture decisions
All seven: thin versioned World IR + adapters (`adr/0002`); Tauri + React/TS + three.js with shell/viewport as two PoC-gated decisions (`0003`); external-process workers with normalised progress (`0004`); ONNX Runtime primary behind swappable adapter (`0005`); Wasmtime/WASI 0.2 sandbox (`0006`); lawful-source gate + provenance-first metadata (`0007`); floating-origin/ENU precision (`0008`). PoC and spike gates are retained; acceptance of direction is not a waiver of verification.

## Open owner questions
D-007 platform boundaries; D-008 CLI timing; D-009 first engine/DCC integration; **D-013 first-slice layer scope** (PRD §5 remains proposed); D-015 cloud; D-016 collaboration; D-017 skill identity (not explicitly answered; `to-tickets` was used); D-018 setup-skill tracker (not explicitly answered; GitHub Issues adopted via D-027); D-021 Ikeja benchmark boundary; D-022 signing budget; D-023 Rust MSRV; D-024 ODbL publication legal review; SDK/docs/fixture/model-weight licences under D-014. Questions Q1–Q8 are in `engineering/ARCHITECTURE_DISCOVERY.md` §7.

## Current repository state
- Repository URL: **https://github.com/Jasowills/genesis-engine** (PUBLIC, verified via `gh repo view`)
- Branch: `main`, tracking `origin/main`
- Latest commit: **read live with `git rev-parse HEAD`** (this file is itself committed after each update, so a hard-coded hash would always lag by one commit). Reconciliation baseline: `15e5b4548c180cc4cf4160cf81a837c50efa04ef`; earlier commits `5fb68abba2f1ec801d2c887f63b4c30f4360f545` (issue links) and `aa728fd9a9aa5e0bf2ca05b94006434816be973d` (initial 81-file documentation-only commit).
- Working tree: clean (`## main...origin/main`); 40 labels, 6 milestones, **32 issues** created in dependency order with native `--blocked-by` links (verified on issue #2: `blocked_by_added` → issue #1). Local tickets and the index carry issue links. No product code.
- Toolchain: macOS 26.5.2 x86_64; Rust 1.98.1; Node 24.16.0; Python 3.15.2; Go 1.27.1; Docker 29.8.2; git 2.50.1; gh 2.97.0 (auth: `Jasowills`)
- Platforms tested: macOS/x86_64 only. Windows, Linux, ARM64, Apple Silicon, CUDA untested.

## Completed and verified tickets
GE-0008 — GitHub repository and issue setup (independent post-execution review `APPROVED`; evidence recorded in its ticket).
GE-0003 — precision/CRS golden-vector spike (7 tests pass; independent re-review `APPROVED`).
GE-0002 — resolve eight evidence conflicts (7 resolved, 1 partial; independent re-review `APPROVED`).
GE-0001 — environment/workflow records (re-verified commands, skill provenance; independent re-review `APPROVED`).

## In-progress tickets
None.

## In-review tickets
None.

## Blocked tickets
GE-0007 (approve ADR set and decision register) — blocked on remaining owner answers and reviewer result.
GE-0005 — plugin sandbox spike evidence independently APPROVED, but `blocked` on cross-platform CI: "all three OSes" AC only met on macOS; network-policy denial UNVERIFIED on WASIp1 (issue #22).
GE-0006 — AI runtime smoke test evidence independently APPROVED, but `blocked`: ORT 1.32.0 has no macOS x86_64 wheel; only 1.23.2 executed locally, no named models loaded (issue #9).

## Ready next tickets
GE-0001 (environment/workflow records), GE-0005 (plugin feasibility spike), GE-0006 (AI runtime smoke test). GE-0004 is proposed pending GE-0003.

## Current architecture snapshot
Accepted Core-centric shape: 12 components (C1–C12) and seams S1–S12 in `ARCHITECTURE.md`. Thin versioned World IR with binary payload references; authoritative geodetic coordinates + local ENU frames; lawful-source gate on every import/inference/export; external-process workers with normalised progress; swappable AI adapter; Wasmtime/WASI plugin sandbox.

## Important module boundaries
UI owns presentation only; domain truth behind Core versioned commands/events/contracts. A capability must be headlessly drivable to be a Core capability. No crate layout prescribed (spec §42).

## Key data contracts and schema versions
World IR schema v0 not yet implemented (GE-0011). Job records, AI adapter contract, provenance/metric records defined as requirements in `ARCHITECTURE.md` §2 and `engineering/ARCHITECTURE_DISCOVERY.md` §2.1. No schema is shipped.

## Research completed
All 8 primary-source briefs, 5 consolidated deliverables (`ECOSYSTEM_RESEARCH.md`, `COMPETITIVE_ANALYSIS.md`, `LICENCE_AND_DATA_RIGHTS.md`, `FORMAT_AND_STANDARDS.md`, `RESEARCH_REGISTER.md`), 8 logged conflicts.

## Research still required
Close the 8 conflicts via their linked tickets; obtain owner answers to open questions; legal review of ODbL §4.6 and AGPL worker boundaries; D-021 benchmark boundary; measured performance (all current figures are estimates).

## Tests and verification
- Last full test run: n/a (no code).
- Platform coverage: environment discovery commands only; host macOS/x86_64.
- Known failures: none outstanding. Independent reviewer returned `CHANGES_REQUESTED` twice on governance/reconciliation defects; all findings corrected and final verdict `APPROVED` (2026-11-27).
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
GE-0008 is `verified` and Phase 7 is complete. Begin the unblocked M0 spikes (GE-0003 precision/CRS golden vectors, GE-0004 shell/viewport PoC, GE-0005 plugin feasibility, GE-0006 AI smoke test) and the GE-0001/GE-0002 documentation/research tickets, with independent review of every code-changing ticket. Final setup report: https://github.com/Jasowills/genesis-engine, branch `main`, 4 commits (latest via `git rev-parse HEAD`), 32 issues.

## Session handoff notes
Server restarted on 2026-11-27 during research; two subagent sessions were resumed and all durable files survived. Do not start product implementation. Do not create further public artefacts beyond D-027. Never print tokens or commit secrets, restricted data or large model weights. Maintain this file and ticket states after every verified unit of work.
