# Genesis Engine — Ticket System

## 1. Two compatible workflows

Genesis uses the **spec Part VII mandatory ticket template** (one Markdown file per ticket under `TICKETS/`) and the **Matty Pocock `to-tickets` skill** for decomposition. Where they differ, the spec wins.

- `to-tickets` contributes: **tracer-bullet** vertical slices (narrow but complete paths through schema/API/UI/tests), each demoable/verifiable on its own, sized for one fresh context window; explicit **blocking edges**; prefactor-first ("make the change easy, then make the easy change"); work the frontier (tickets whose blockers are all done); expand–contract for wide refactors.
- The spec contributes: the mandatory frontmatter + section schema, priorities, states, test plan, reviewer checklist, verification evidence and definition of done.
- Provenance of the skill is recorded in `engineering/WORKFLOW.md` (`to-tickets` at `mattpocock/skills` commit `0ab1b63a410a03d3627979a109c8695de27af954`, MIT).

## 2. States

`proposed` → `needs-owner-input` → `ready` → `in-progress` → `in-review` → `changes-requested` → `verified`; plus `blocked`, `deferred`, `cancelled`. **`verified` never means "code written"** — it means acceptance criteria passed and an independent reviewer approved.

## 3. Priorities

P0 blocks safe progress or foundational architecture · P1 required for the current milestone · P2 important but not blocking · P3 later improvement.

## 4. Categories

`research`, `feature`, `engineering`, `bug`, `refactor`, `security`, `performance`, `documentation`, `test`, `integration`, `release`, `spike`.

## 5. Lifecycle (spec §40)

1. Read ticket, project docs, ADRs, dependency tickets.
2. Confirm state is `ready`; if ambiguous, stop and ask or open a spike.
3. Assign one implementation agent; implement only the ticket scope.
4. Add/update tests; run formatter, linter, type checker, tests, build.
5. Update docs/changelog; record files changed and evidence in the ticket.
6. Assign a **fresh, independent reviewer subagent** with ticket + ADRs + full diff; it returns `APPROVED`, `CHANGES_REQUESTED` or `BLOCKED` with findings ranked by severity. The author cannot approve its own work.
7. Fix findings and repeat review; run final verification.
8. Update ticket, `progress.md`, commit with Conventional Commits, push only when repository workflow/approval allow it.
9. Mark `verified` only after every gate passes.

If independent review cannot run, the ticket stays `in-review` or `blocked` — never silently waived.

## 6. Local vs GitHub tracking

Local ticket files are authoritative until Phase 7 approval. After approval, publish one GitHub issue per ticket in dependency order with labels (`type:*`, `priority:*`, `status:*`, `area:*`), milestone, and native `--blocked-by` links (`gh issue create` supports them). Keep both in sync. Do not close or modify parent issues.

## 7. Ticket index

| ID | Title | Type | Priority | Milestone | Status | Blocked by |
| --- | --- | --- | --- | --- | --- | --- |
| GE-0001 | Record environment and workflow capabilities | documentation | P0 | M0 | ready | [] |
| GE-0002 | Resolve licence and source-rights conflicts | research | P0 | M0 | ready | [] |
| GE-0003 | Precision and CRS golden-vector spike | spike | P0 | M0 | ready | [] |
| GE-0004 | Desktop shell and viewport capability PoC | spike | P0 | M0 | proposed | [GE-0003] |
| GE-0005 | Plugin runtime feasibility spike | spike | P1 | M0 | ready | [] |
| GE-0006 | AI runtime and model smoke test | spike | P1 | M0 | ready | [] |
| GE-0007 | Approve ADR set and decision register | documentation | P0 | M0 | blocked | [GE-0002, GE-0003, GE-0004] |
| GE-0008 | GitHub repository and issue setup | engineering | P0 | M0 | in-progress | [GE-0007] |
| GE-0009 | Cross-platform CI skeleton | engineering | P0 | M0 | proposed | [GE-0008] |
| GE-0010 | Project model and portable local layout | feature | P0 | M1 | proposed | [GE-0007] |
| GE-0011 | World IR schema v0 with migrations | feature | P0 | M1 | proposed | [GE-0003, GE-0007] |
| GE-0012 | Lawful-source gate and provenance record | feature | P0 | M1 | proposed | [GE-0002, GE-0011] |
| GE-0013 | GeoJSON boundary import and CRS validation | feature | P0 | M1 | proposed | [GE-0011, GE-0012] |
| GE-0014 | OSM footprint and road feature ingestion | feature | P1 | M1 | proposed | [GE-0013] |
| GE-0015 | Procedural road surface and building volume | feature | P1 | M1 | proposed | [GE-0014] |
| GE-0016 | Studio shell and viewport scene rendering | feature | P1 | M1 | proposed | [GE-0004, GE-0011] |
| GE-0017 | Select, inspect and provenance display | feature | P1 | M1 | proposed | [GE-0016] |
| GE-0018 | Geometry and structural validation gates | feature | P1 | M1 | proposed | [GE-0011, GE-0009] |
| GE-0019 | GLB export and re-import fixture | feature | P1 | M1 | proposed | [GE-0011, GE-0018] |
| GE-0020 | End-to-end vertical slice and benchmark harness | test | P1 | M1 | proposed | [GE-0013, GE-0014, GE-0015, GE-0016, GE-0017, GE-0018, GE-0019] |
| GE-0021 | Contributor and governance documentation | documentation | P1 | M1 | proposed | [GE-0007] |
| GE-0022 | Headless CLI command surface | feature | P1 | M2 | proposed | [GE-0020] |
| GE-0023 | External worker adapter framework | engineering | P1 | M2 | proposed | [GE-0007, GE-0022] |
| GE-0024 | Quality profiles and preflight estimates | feature | P1 | M2 | proposed | [GE-0018, GE-0006] |
| GE-0025 | WIT/WASI plugin SDK and sandbox PoC | security | P2 | M2 | proposed | [GE-0005, GE-0011] |
| GE-0026 | AI segmentation and depth adapters | integration | P2 | M3 | proposed | [GE-0006, GE-0023] |
| GE-0027 | Selective and incremental regeneration | feature | P2 | M3 | proposed | [GE-0011, GE-0020] |
| GE-0028 | Terrain and secondary layers | feature | P2 | M3 | proposed | [GE-0020] |
| GE-0029 | Blender export and round-trip fixture | integration | P2 | M3 | proposed | [GE-0019] |
| GE-0030 | LOD, collision and texture compression | feature | P3 | M4 | proposed | [GE-0029] |
| GE-0031 | Target engine export fixtures | integration | P3 | M4 | proposed | [GE-0030] |
| GE-0032 | Signed release and auto-update pipeline | release | P3 | M5 | needs-owner-input | [GE-0008, GE-0009] |

All tickets remain non-`verified`. M0 tickets are `ready` only where their blockers are closed; GE-0007 is `blocked` pending remaining owner answers. GE-0008 is `in-progress` under the explicit owner authorisation D-027 — a documented lifecycle exception recorded in `DECISIONS.md` §3a, not a silent waiver. No code-changing ticket may start before its ADR is accepted (all seven are accepted) **and** its blockers are verified.
