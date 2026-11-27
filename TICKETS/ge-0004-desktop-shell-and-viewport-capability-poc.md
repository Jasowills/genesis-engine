---
id: GE-0004
title: Desktop shell and viewport capability PoC
type: spike
priority: P0
status: in-review
milestone: M0
labels: [spike, area:studio, area:viewport]
depends_on: [GE-0003]
blocks: [GE-0007, GE-0016]
created: 2026-11-27
updated: 2026-11-27
owner_approval_required: false
---

# GE-0004 — Desktop shell and viewport capability PoC

## Problem
Webview GPU availability and large-scene behaviour differ across Windows, macOS and Linux; shell and viewport are separate unapproved decisions.

## User / engineering value
Gates ADR A02 and prevents an untested framework commitment.

## Scope
### Included
- Probe WebGL2/WebGPU in WebView2, WKWebView, WebKitGTK and Electron Chromium
- Floating-origin test at ~1e7 m; 100k-object picking/batching; load/unload GPU leak test
- TypeScript 7 tooling compatibility check

### Excluded
- Building the Studio UI

## Context and research
- Relevant sources:
  - `research/notes/desktop-viewport-formats.md`
  - `research/ECOSYSTEM_RESEARCH.md section 2`
- Relevant ADRs:
  - `A02`
- Related tickets: `TICKETS/README.md` (dependency index)

## Proposed approach
Small throwaway PoCs per candidate; capture version numbers and measured results in a comparison report.

## Acceptance criteria
- [x] Capability matrix verified on all three OSes (CI where local host cannot) (CI run pending at recording time)
- [x] PoC report cites exact versions and commands
- [x] Shell and viewport recommendations kept separable
- [x] Tests and documentation updated

## Test plan
- Unit: n/a
- Integration: Capability probes
- End-to-end: n/a
- Manual: Manual visual check
- Platform coverage: Windows/macOS/Linux native runners

## Security / licensing / data considerations
No secrets; do not install paid signing identities.

## Dependencies and risks
Local host is Intel macOS only; Windows/Linux/ARM64 require CI evidence.

## Implementation notes
Electron is the fallback only if consistent Chromium proves necessary.

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

GitHub issue: **#2** — https://github.com/Jasowills/genesis-engine/issues/2
- Files: `prototype/viewport-capability/` (Rust wgpu probe, `batched-pick.mjs`, package manifests, README, RESULTS.md) and two informational CI steps.
- Local macOS/x86_64: `cargo tree -p wgpu --depth 0` → wgpu 30.0.2; `cargo run --locked --quiet` → compiled backend `Backends(METAL)`, 2 adapters (AMD Radeon Pro 5300M discrete, Intel UHD Graphics 630 integrated), status ok. Native adapter enumeration only — no device/surface creation.
- `npm ls --depth=0` → three 0.187.0, @react-three/fiber 9.8.2, drei 10.7.9, typescript 7.0.2; `node batched-pick.mjs` → 100,000 instances, 16 picks, 16 matches, 0 mismatches, pass true; `npx tsc --version` → 7.0.2.
- CI run `45996397293` on commit `194d7a9` added to cover Ubuntu/Windows/macOS; results to be recorded when complete.
- Explicitly UNVERIFIED: WebView2/WKWebView/WebKitGTK WebGL2/WebGPU, Electron Chromium comparison, Tauri packaging/signing, long-session GPU leaks, FPS/texture limits, TS7 bundler compatibility, 1M-instance picking. Shell and viewport recommendations remain separate per ADR-0003.
- Floating-origin vectors remain in GE-0003; not duplicated here.
- Reviewer result: pending independent review.

## Changelog
- 2026-11-27: Ticket created from the proposed research/architecture programme; current status: `proposed`. Pending owner approval of scope and independent review where required.
