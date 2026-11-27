---
id: GE-0017
title: Select, inspect and provenance display
type: feature
priority: P1
status: verified
milestone: M1
labels: [type:feature, area:studio]
depends_on: [GE-0016]
blocks: [GE-0020]
created: 2026-11-27
updated: 2026-11-27
owner_approval_required: false
---

# GE-0017 — Select, inspect and provenance display

## Problem
Users must see where a road/building came from and whether attributes are measured, estimated or inferred.

## User / engineering value
Delivers spec §50 steps 8-9 and the honest-uncertainty principle.

## Scope
### Included
- Picking/selection mapped to stable IR IDs
- Inspector shows source, method, licence, confidence and source/inferred flags
- Keyboard-driven selection path

### Excluded
- Full editing, undo history, multi-select gizmos

## Context and research
- Relevant sources:
  - `Spec sections 9, 24-26`
  - `PRD.md section 6.4`
- Relevant ADRs:
  - `A01`
  - `A02`
  - `A06`
- Related tickets: `TICKETS/README.md` (dependency index)

## Proposed approach
DOM panels use ARIA tree/grid semantics; viewport emits selection events only.

## Acceptance criteria
- [x] Pick maps to correct stable ID
- [x] Provenance fields visible and accurate
- [x] Keyboard path selects and inspects an object
- [x] Tests and documentation updated

## Test plan
- Unit: Component tests
- Integration: Selection integration
- End-to-end: n/a
- Manual: Manual keyboard walkthrough
- Platform coverage: macOS/Linux/Windows

## Security / licensing / data considerations
No network calls from UI.

## Dependencies and risks
Canvas accessibility has no standard pattern; numeric path is required.

## Implementation notes
axe and keyboard tests planned.

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

GitHub issue: **#15** — https://github.com/Jasowills/genesis-engine/issues/15
- Files: `apps/studio/src/selection.test.tsx` (4 component tests), `apps/studio/vite.config.ts` (vitest/jsdom), `apps/studio/package.json` (`test` script + vitest/jsdom/testing-library dev deps), CI step for Studio frontend.
- `cd apps/studio && npx vitest run` → **6 passed, 0 failed**: exact stable-ID selection with two distinct nodes (not always first), ArrowDown navigation, `selectable: false` respected, inspector shows source/attribution/licence/method/evidence/derivation chain, negative numeric input commits `-7.5`, shell selection reveals provenance.
- `cd apps/studio && npx tsc --noEmit` clean; `cargo test --workspace --locked` still 64 passed (Core + Studio Rust).
- No domain logic in React; selection/inspector present only presentation of Core snapshot fields. No network calls from UI.
- Actual webview/visual verification remains UNVERIFIED (inherited from GE-0016).
- Reviewer result: first pass `CHANGES_REQUESTED`; CI step, bookkeeping, discriminating tests, evidence assertions and selectable handling fixed; re-review **`APPROVED`** (session `ses_de23ba900ffeH7BbTS5p2KVWUj`).

## Changelog
- 2026-11-27: Ticket created from the proposed research/architecture programme; current status: `in-review`. Pending owner approval of scope and independent review where required.
- 2026-11-27: Independent re-review APPROVED; ticket marked verified.
