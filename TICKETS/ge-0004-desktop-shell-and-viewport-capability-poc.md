---
id: GE-0004
title: Desktop shell and viewport capability PoC
type: spike
priority: P0
status: in-progress
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
- [ ] Capability matrix verified on all three OSes (CI where local host cannot)
- [ ] PoC report cites exact versions and commands
- [ ] Shell and viewport recommendations kept separable
- [ ] Tests and documentation updated

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
Commands, output summaries, fixture references and reviewer result to be recorded here when the ticket is implemented. No claim of completion without an independent reviewer.

## Changelog
- 2026-11-27: Ticket created from the proposed research/architecture programme; current status: `proposed`. Pending owner approval of scope and independent review where required.
