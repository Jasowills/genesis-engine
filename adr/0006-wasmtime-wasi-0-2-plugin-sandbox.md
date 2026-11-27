# ADR-0006 — Wasmtime / WASI 0.2 plugin sandbox

**Status:** Accepted
**Date:** 2026-11-27
**Decision ID:** D-014 adjacent (SDK/trust policy still open) (approved as part of D-025, 2026-11-27)
**Owner approval:** Approved 2026-11-27 (Phase 8 checkpoint)
**Supersedes:** none
**Superseded by:** none
**Original proposal:** `adr/ADR_PROPOSALS.md` (A05)

**Context.** Spec §33 suggests Wasmtime/WASM but requires validation. Wasmtime 51.0.0 (2026-11-20, Apache-2.0 WITH LLVM-exception) has monthly majors; WASI 0.2 is a stable preview; WASI 0.3 released 2026-06-11 but Wasmtime `p3` is experimental. Windows sandbox escape GHSA-f2pq-268w-886c fixed in 36.0.22/48.0.10/49.0.7/50.0.3; host stream DoS fixed in 51.0.0; TCP default docs conflict; `StoreLimitsBuilder::memory_size` has no default.

**Options.** A — Wasmtime ≥51 + WASI 0.2 components + typed WIT world; B — native dynamic libraries (first-party trusted only); C — sandboxed OS process per plugin; D — defer plugins past the slice.

**Decision.** A for untrusted third-party plugins; combine with C for native/high-risk workers; defer D only for the *registry*, keep a WIT PoC spike. Pin ≥51.0.0 with patch monitoring; zero preopens/env/network; epoch interruption + `tokio::time::timeout` (epochs do not cover blocking hostcalls); explicit Store/WASI caps; bounded log/progress hostcalls; signed SHA-256-pinned manifest and atomic activation with retained prior version. Track WASI 0.3, do not depend on it in v1.

**Rationale.** WASI has no ambient authority and Wasmtime is the maturest Rust embedding with a typed component ABI; CVE density is a patch-discipline requirement, not a reason to use unsafe native loading.

**Consequences.** Positive: no ambient host access; language-neutral ABI. Negative: recurring upgrade work; precompiled artefacts tied to exact version; host functions are the weak point; manifest/signing/rollback are our design (no sourced standard); component overhead percentage unverified.

**Evidence.** Spec §§33–34; `research/notes/plugin-security.md` §§1–4; `research/RESEARCH_REGISTER.md` §4 conflicts.

---

## Reuse vs build

Wrap Wasmtime 51.0.0+; build the WIT world, manifest and rollback policy ourselves.

## Cross-platform, performance and resource implications

Explicit Store/WASI caps; epoch ~10% overhead; monthly major upgrade cost. Rust toolchain verified locally: 1.98.1. Windows, Linux, ARM64 and CUDA remain unverified and require CI evidence.

## Security, privacy and licence implications

Apache-2.0 WITH LLVM-exception; deny-by-default capabilities; no inherit_env/stdio.

## Evidence

| Source | Location | Access date | Supports |
| --- | --- | --- | --- |
| Primary-source research and spec | `Spec §§33,34; notes/plugin-security.md §§1-4` | 2026-11-27 | Decision context, options, rationale |

Full citations and licence texts are in the referenced research files and `research/RESEARCH_REGISTER.md`.

## Verification plan

GE-0005 evil-plugin fixtures on all three OSes. No implementation claim is made here.

## Notes

Accepted 2026-11-27 by owner (Phase 8, D-025). Review status: see `DECISIONS.md` §3a Phase 7 checklist; independent review is required before publication. Evidence conflicts relevant to this ADR are tracked in `research/RESEARCH_REGISTER.md` §4 and must be closed by the linked ticket before implementation.
