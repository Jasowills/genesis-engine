---
id: GE-0023
title: External worker adapter framework
type: engineering
priority: P1
status: verified
milestone: M2
labels: [type:engineering, area:core]
depends_on: [GE-0007, GE-0022]
blocks: [GE-0024, GE-0026]
created: 2026-11-27
updated: 2026-11-27
owner_approval_required: false
---

# GE-0023 — External worker adapter framework

## Problem
Specialist tools need a stable, inspectable boundary with version detection, progress and cancellation.

## User / engineering value
Spec §29 requires this for every non-Rust worker; licence boundaries depend on it.

## Scope
### Included
- Worker descriptor, argv-only execution, scrubbed env, restricted cwd
- Per-job CPU/memory/time caps, kill tree, version probe
- Normalised progress events and canonical errors; checkpoint resume

### Excluded
- Bundling AGPL tools into the core
- GPU containers

## Context and research
- Relevant sources:
  - `Spec sections 29, 31`
  - `research/notes/reconstruction.md section 6`
- Relevant ADRs:
  - `A03`
- Related tickets: `TICKETS/README.md` (dependency index)

## Proposed approach
Implement the seam first, then one permissive worker adapter.

## Acceptance criteria
- [ ] Fake worker proves progress/cancel/timeout/error mapping — **PARTIAL:** progress, pre-start cancellation, provider failure and *post-stage* timeout mapping are proven; intra-stage preemption is NOT MET (follow-up)
- [ ] Process tree killed on timeout on all three OSes — NOT MET: process spawning/kill-tree is follow-up; contract tests use a fake worker
- [x] Structured logs contain no secrets (bounded redaction helper + test; provider errors redacted before persistence)
- [x] Tests and documentation updated

## Test plan
- Unit: Adapter unit tests
- Integration: Fake-worker integration
- End-to-end: n/a
- Manual: n/a
- Platform coverage: Windows/macOS/Linux

## Security / licensing / data considerations
No shell invocation; least privilege; no secrets in env.

## Dependencies and risks
macOS sandbox docs thin; seccomp alone is not a sandbox.

## Implementation notes
No cancellation API exists in surveyed tools; process supervision is the portable path.

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

GitHub issue: **#19** — https://github.com/Jasowills/genesis-engine/issues/19
- Files: `crates/genesis-core/src/jobs.rs`, `tests/jobs.rs`, `docs/developer/jobs.md`, lib.rs re-exports.
- `cargo test --workspace --locked` → **85 passed, 0 failed** (8 job tests: completion with checkpoints/JSON round-trip, cancellation, provider-failure mapping, descriptor validation, post-stage timeout mapping, terminal-state guard, redaction). `cargo clippy --workspace --locked -- -D warnings` clean; `cargo fmt --all -- --check` clean.
- WorkerDescriptor rejects non-empty environments, zero memory cap and zero timeout; argv-only contract. Supervisor emits normalised events and canonical errors; no shell invocation.
- NOT IMPLEMENTED: OS process spawning, argv execution, kill-tree termination, per-OS sandbox profiles, CPU caps, real worker version probes, checkpoint resume, intra-stage timeout preemption. This ticket delivers the supervision contract and fake-worker state machine only. This ticket delivers the contract and fake-worker validation only.
- Reviewer result: first pass `CHANGES_REQUESTED`; all findings fixed; re-review **`APPROVED`** (session `ses_de207f532ffeQj86bgV5KRC7vW`). Limitation: supervision contract and fake-worker state machine only — no OS process spawning, argv execution, kill-tree, per-OS sandbox, CPU caps, checkpoint resume, real version probe or intra-stage timeout preemption.

## Changelog
- 2026-11-27: Ticket created from the proposed research/architecture programme; current status: `proposed`. Pending owner approval of scope and independent review where required.
- 2026-11-27: Independent re-review APPROVED; ticket marked verified with explicit limitations.
