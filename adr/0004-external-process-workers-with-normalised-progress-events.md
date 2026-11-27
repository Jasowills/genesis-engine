# ADR-0004 — External-process workers with normalised progress events

**Status:** Accepted
**Date:** 2026-11-27
**Decision ID:** D-025 (implementation architecture; no separate spec decision ID)
**Owner approval:** Approved 2026-11-27 (Phase 8 checkpoint)
**Supersedes:** none
**Superseded by:** none
**Original proposal:** `adr/ADR_PROPOSALS.md` (A03)

**Context.** Spec §29 permits mature non-Rust tools behind stable adapters with a mandatory requirement list. Mature reconstruction tools are CLI/process-oriented; no cancellation API was found in any primary source; progress formats are heterogeneous; licences favour process separation (OpenMVS/ODM AGPL, Meshroom MPL, Blender GPL).

**Options.** A — separate OS process per job + normalised event protocol; B — in-process FFI for permissive tools only; C — containers/microVMs for every worker.

**Decision.** A for specialist/non-Rust workers; B only for reviewed permissive libraries; defer C to optional cloud/headless. Normalise progress to `{stage, progress 0..1, units, checkpoint}`; cancellation is one token mapped per adapter (`SetTerminate` for ORT, kill-tree for processes, epoch+timeout for plugins).

**Rationale.** Process isolation is the only evidenced pattern handling licensing, crashes, version detection and missing cancellation APIs simultaneously.

**Consequences.** Positive: blast-radius reduction, licence compliance, tool upgrade independence. Negative: IPC overhead; per-OS sandbox profiles diverge; seccomp alone is **not** a sandbox.

**Evidence.** Spec §§29, 31; `research/notes/reconstruction.md` §§3.4, 5–6; `research/notes/plugin-security.md` §§2–3.

---

## Reuse vs build

Wrap permissive tools in-process only when reviewed; all others as external processes.

## Cross-platform, performance and resource implications

CPU/GPU caps per job; kill-tree on timeout; seccomp not a sandbox; Apple specifics thin. Rust toolchain verified locally: 1.98.1. Windows, Linux, ARM64 and CUDA remain unverified and require CI evidence.

## Security, privacy and licence implications

AGPL/MPL/GPL process boundary; scrubbed env; no shell invocation.

## Evidence

| Source | Location | Access date | Supports |
| --- | --- | --- | --- |
| Primary-source research and spec | `Spec §§29,31; notes/reconstruction.md §§3.4,5-6` | 2026-11-27 | Decision context, options, rationale |

Full citations and licence texts are in the referenced research files and `research/RESEARCH_REGISTER.md`.

## Verification plan

GE-0023 fake-worker progress/cancel/error fixtures. No implementation claim is made here.

## Notes

Accepted 2026-11-27 by owner (Phase 8, D-025). Review status: see `DECISIONS.md` §3a Phase 7 checklist; independent review is required before publication. Evidence conflicts relevant to this ADR are tracked in `research/RESEARCH_REGISTER.md` §4 and must be closed by the linked ticket before implementation.
