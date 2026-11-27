---
id: GE-0008
title: GitHub repository and issue setup
type: engineering
priority: P0
status: in-progress
milestone: M0
labels: [area:infra]
depends_on: [GE-0007]
blocks: [GE-0009]
created: 2026-11-27
updated: 2026-11-27
owner_approval_required: true
---

# GE-0008 — GitHub repository and issue setup

## Problem
No Git repository or GitHub repository exists; spec Phase 7 requires explicit approval before creation.

## User / engineering value
Establishes the public engineering source of truth with labels, milestones and native blocking links.

## Scope
### Included
- gh/git init only after owner confirms name, visibility, description and licence
- Labels, milestones and one issue per approved ticket in dependency order
- Native --blocked-by links; .gitignore, secret scanning guidance, LFS policy

### Excluded
- Force-push, history rewrite, pushing secrets or generated data

## Context and research
- Relevant sources:
  - `Spec Phase 7`
  - `engineering/WORKFLOW.md section 3.1`
  - `TICKETS/README.md`
- Relevant ADRs:
  - `None (process)`
- Related tickets: `TICKETS/README.md` (dependency index)

## Proposed approach
Small initial commit; verify; push; report exact URL, branch, commit hash, issue count and remaining actions.

## Acceptance criteria
- [ ] Repository URL and visibility confirmed by owner before creation
- [ ] Issue count matches approved ticket count
- [ ] No tokens or large data committed
- [ ] Tests and documentation updated

## Test plan
- Unit: n/a
- Integration: n/a
- End-to-end: n/a
- Manual: Verify remote and issue list
- Platform coverage: n/a

## Security / licensing / data considerations
Token with delete_repo scope exists; never use destructive operations.

## Dependencies and risks
Existing genesis-named repo check was read-only and found none under Jasowills.

## Implementation notes
Do not create anything before explicit approval.

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

GitHub issue: **#6** — https://github.com/Jasowills/genesis-engine/issues/6
- `git init -b main`, `git add -A` (81 files), Conventional Commit `aa728fd9a9aa5e0bf2ca05b94006434816be973d`.
- `gh repo create Jasowills/genesis-engine --public --description ... --source=. --remote=origin --push` → `https://github.com/Jasowills/genesis-engine`, branch `main` pushed and tracking `origin/main`.
- `gh repo view` → `Jasowills/genesis-engine PUBLIC https://github.com/Jasowills/genesis-engine main`.
- `git status --short --branch` → `## main...origin/main` (clean).
- 30 project labels created (40 total incl. defaults); 6 milestones M0–M5; 32 issues created in dependency order; `gh issue list` → 32.
- Native blocking links verified: `gh api repos/Jasowills/genesis-engine/issues/2/timeline` → event `blocked_by_added` with `blocked_by.number = 1` (GE-0004 blocked by GE-0003).
- Secret-pattern grep across docs/templates: no matches. Branch protection intentionally not enabled (not owner-approved; recorded in `DECISIONS.md` §3a).
- Independent reviewer: final verdict `APPROVED` on the Phase 7 plan (session `ses_de59f7bffffeZu5LcI2JsshkFy`). Post-execution verification pending.

## Changelog
- 2026-11-27: Owner approved public repository (D-027); setup staged pending reviewer re-check. Status set to in-progress for the approved Phase 7 action.
