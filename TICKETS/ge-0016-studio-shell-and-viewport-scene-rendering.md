---
id: GE-0016
title: Studio shell and viewport scene rendering
type: feature
priority: P1
status: in-review
milestone: M1
labels: [type:feature, area:studio, area:viewport]
depends_on: [GE-0004, GE-0011]
blocks: [GE-0017]
created: 2026-11-27
updated: 2026-11-27
owner_approval_required: false
---

# GE-0016 — Studio shell and viewport scene rendering

## Problem
The world must be inspectable in a desktop viewport without UI owning domain logic.

## User / engineering value
Delivers spec §50 step 7 and validates the UI↔Core seam.

## Scope
### Included
- Shell + React/TS panels + viewport rendering IR-derived scene
- Commands/events/versioned snapshots only
- Capability report with graceful GPU degradation

### Excluded
- Gizmo polish, advanced editing, LOD streaming

## Context and research
- Relevant sources:
  - `engineering/ARCHITECTURE_DISCOVERY.md sections 1-3`
  - `research/notes/desktop-viewport-formats.md`
- Relevant ADRs:
  - `A02`
- Related tickets: `TICKETS/README.md` (dependency index)

## Proposed approach
Implement per accepted ADR; keep Core authoritative.

## Acceptance criteria
- [x] Scene renders from IR with no domain logic in React
- [x] WebGL2/WebGPU capability handled on all supported OSes (probe/generic capability handling; real webview verification still UNVERIFIED)
- [x] Keyboard-numeric alternative exists
- [x] Tests and documentation updated

## Test plan
- Unit: Component tests
- Integration: UI integration
- End-to-end: n/a
- Manual: Manual inspection
- Platform coverage: macOS/Linux/Windows

## Security / licensing / data considerations
No secrets in UI config; signed update deferred.

## Dependencies and risks
PoC results gate this ticket; local host only macOS.

## Implementation notes
Separate shell and viewport approvals.

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

GitHub issue: **#14** — https://github.com/Jasowills/genesis-engine/issues/14
- Files: `crates/genesis-studio/` (scene snapshot, optional Tauri commands, 5 unit + 2 integration tests), `apps/studio/` (React 19.3.1 / TS 7.0.2 / three 0.187.0 / R3F 9.8.2 / drei 10.7.9 frontend), `fixtures/ikeja-small.geojson`, root workspace member.
- `cargo test --workspace --locked` → **64 passed, 0 failed** (57 Core + 5 Studio unit + 2 Studio integration); `cargo clippy --workspace --locked -- -D warnings` clean; `cargo fmt --all -- --check` clean.
- `cargo check -p genesis-studio --features tauri-bindings` and `cargo clippy -p genesis-studio --features tauri-bindings -- -D warnings` clean (Tauri optional so headless CI stays portable).
- `cd apps/studio && npx tsc --noEmit` clean; `npm run build` succeeded (only a chunk-size advisory).
- Snapshot derives from Core IR + generated meshes; hierarchy is ARIA tree with roving tabindex, inspector has keyboard-numeric inputs; React contains presentation only (no CRS/provenance rules).
- UNVERIFIED: real Tauri window/webview, WebGL2/WebGPU in WebView2/WKWebView/WebKitGTK, visual rendering, GPU leaks, 100k-object picking, auto-update/signing.
- Reviewer result: pending independent review.

## Changelog
- 2026-11-27: Ticket created from the proposed research/architecture programme; current status: `proposed`. Pending owner approval of scope and independent review where required.
