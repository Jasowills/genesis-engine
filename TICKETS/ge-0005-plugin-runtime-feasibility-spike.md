---
id: GE-0005
title: Plugin runtime feasibility spike
type: spike
priority: P1
status: in-review
milestone: M0
labels: [spike, area:plugins, type:security]
depends_on: []
blocks: [GE-0007, GE-0025]
created: 2026-11-27
updated: 2026-11-27
owner_approval_required: false
---

# GE-0005 — Plugin runtime feasibility spike

## Problem
Wasmtime has a recent CVE history, permissive resource defaults and no sourced plugin manifest/rollback standard.

## User / engineering value
Validates A05 before any plugin SDK work; establishes adversarial fixtures.

## Scope
### Included
- Install wasm32-wasip1 target; build a minimal WIT world and host
- Evil-plugin fixtures: infinite loop, huge memory.grow, empty writes, invalid imports
- Test epoch + timeout and explicit resource caps

### Excluded
- Publishing a plugin registry

## Context and research
- Relevant sources:
  - `research/notes/plugin-security.md`
  - `research/RESEARCH_REGISTER.md section 4`
- Relevant ADRs:
  - `A05`
- Related tickets: `TICKETS/README.md` (dependency index)

## Proposed approach
Throwaway PoC; record measured overhead, cap behaviour and lifecycle design inference separately from sourced facts.

## Acceptance criteria
- [x] Evil plugins are terminated within timeout on all three OSes — CI run `46000916440`: 6/6 plugin tests pass on ubuntu-latest, macos-latest and windows-latest (epoch trap observed locally ~48–68 ms inside the 5 s backstop)
- [x] Zero preopens/env demonstrated; **network-policy runtime denial UNVERIFIED** (wasm32-wasip1 std has no sockets/DNS; permissive control gave identical results)
- [x] Component overhead explicitly UNVERIFIED (core Wasm/WASIp1 path used, not typed WASI 0.2 components)
- [x] Tests and documentation updated

## Test plan
- Unit: n/a
- Integration: n/a
- End-to-end: n/a
- Manual: Manual adversarial review
- Platform coverage: Windows/macOS/Linux

## Security / licensing / data considerations
Deny-by-default; no inherit_env/inherit_stdio; no secrets in plugin context.

## Dependencies and risks
Wasmtime monthly majors; pin >=51.0.0 and monitor advisories.

## Implementation notes
Manifest/signing/rollback are design inference, not sourced standards.

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

GitHub issue: **#22** — https://github.com/Jasowills/genesis-engine/issues/22
- Files: `prototype/plugin-sandbox/` (Cargo.toml, src/lib.rs, src/main.rs, tests/sandbox.rs, guest/ fixtures, README.md, RESULTS.md). Build artifacts ignored by `.gitignore:3`.
- `cargo build --target wasm32-wasip1 --release --manifest-path guest/Cargo.toml` → five real Wasm MVP modules (35K–61K).
- `cargo test` → **6 passed, 0 failed**: normal plugin returns [42]; infinite loop trapped by epochs (~48–68 ms, 5 s timeout backstop configured but not exercised); 64 MiB memory.grow denied by 8 MiB StoreLimits (`[-1, 17]`); report hostcall capped at 64 (flood of 100 truncates); zero env/stdio/preopens; live-listener TCP probe.
- `cargo run` → setup listener + normal/probe/infinite_loop/memory_grow results; no stdout/stderr sentinel leak.
- Reviewer: first pass `CHANGES_REQUESTED` (tautological network probe, unbounded hostcall); fixes applied; narrow re-review **`APPROVED`** (session `ses_de3e654f8ffeuRYs4nb8VLZtDA`) with 6 tests re-run. The reviewer explicitly required retaining UNVERIFIED labels verbatim.
- **Residual UNVERIFIED (do not promote):** host network-policy denial on wasm32-wasip1; typed WASI 0.2 component/WIT path and component overhead; empty-write/invalid-import fixtures; blocking-hostcall cancellation and bulk-op preemption; Windows/Linux platforms; manifest/signing/rollback.
- Follow-up 2026-11-27: an explicit `wasi_snapshot_preview1::sock_connect` import fixture was added to the throwaway prototype. Wasmtime's Preview 1 adapter does not define that import (`unknown import ... sock_connect`), so no `wasm32-wasip1` guest — via std or raw WASI — can reach a host TCP policy decision. Test `wasi_preview1_lacks_sock_connect_import` records this; prototype `cargo test` now 7 passed, 0 failed. This **confirms** the UNVERIFIED label; it is not proof of denial. Closing it requires the typed WASI 0.2 component path with `wasi:sockets`, separate follow-up work.
- **Status rationale:** deliverable evidence independently approved, cross-platform CI green; the network-policy denial cannot be verified on this target and the ticket's "all three OSes" acceptance criterion is not met locally. State remains `in-review` / partial, not `verified`.

## Changelog
- 2026-11-27: Ticket created from the proposed research/architecture programme; current status: `ready`. Pending owner approval of scope and independent review where required.
- 2026-11-27: Evidence recorded; reviewer approved; partially complete pending cross-platform verification.
- 2026-11-27: Cross-platform CI evidence recorded; ticket moved from blocked to in-review.
