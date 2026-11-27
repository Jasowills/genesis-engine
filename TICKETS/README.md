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

| ID | GitHub | Title | Type | Priority | Milestone | Status | Blocked by |
| --- | --- | --- | --- | --- | --- | --- | --- |
| GE-0001 | [#31](https://github.com/Jasowills/genesis-engine/issues/31) | Record environment and workflow capabilities | documentation | P0 | M0 | verified | [] |
| GE-0002 | [#3](https://github.com/Jasowills/genesis-engine/issues/3) | Resolve licence and source-rights conflicts | research | P0 | M0 | verified | [] |
| GE-0003 | [#1](https://github.com/Jasowills/genesis-engine/issues/1) | Precision and CRS golden-vector spike | spike | P0 | M0 | verified | [] |
| GE-0004 | [#2](https://github.com/Jasowills/genesis-engine/issues/2) | Desktop shell and viewport capability PoC | spike | P0 | M0 | verified | [GE-0003] |
| GE-0005 | [#22](https://github.com/Jasowills/genesis-engine/issues/22) | Plugin runtime feasibility spike | spike | P1 | M0 | in-review | [] |
| GE-0006 | [#9](https://github.com/Jasowills/genesis-engine/issues/9) | AI runtime and model smoke test | spike | P1 | M0 | in-review | [] |
| GE-0007 | [#4](https://github.com/Jasowills/genesis-engine/issues/4) | Approve ADR set and decision register | documentation | P0 | M0 | verified | [GE-0002, GE-0003, GE-0004] |
| GE-0008 | [#6](https://github.com/Jasowills/genesis-engine/issues/6) | GitHub repository and issue setup | engineering | P0 | M0 | verified | [GE-0007] |
| GE-0009 | [#7](https://github.com/Jasowills/genesis-engine/issues/7) | Cross-platform CI skeleton | engineering | P0 | M0 | verified | [GE-0008] |
| GE-0010 | [#27](https://github.com/Jasowills/genesis-engine/issues/27) | Project model and portable local layout | feature | P0 | M1 | verified | [GE-0007] |
| GE-0011 | [#5](https://github.com/Jasowills/genesis-engine/issues/5) | World IR schema v0 with migrations | feature | P0 | M1 | verified | [GE-0003, GE-0007] |
| GE-0012 | [#10](https://github.com/Jasowills/genesis-engine/issues/10) | Lawful-source gate and provenance record | feature | P0 | M1 | verified | [GE-0002, GE-0011] |
| GE-0013 | [#11](https://github.com/Jasowills/genesis-engine/issues/11) | GeoJSON boundary import and CRS validation | feature | P0 | M1 | verified | [GE-0011, GE-0012] |
| GE-0014 | [#12](https://github.com/Jasowills/genesis-engine/issues/12) | OSM footprint and road feature ingestion | feature | P1 | M1 | verified | [GE-0013] |
| GE-0015 | [#13](https://github.com/Jasowills/genesis-engine/issues/13) | Procedural road surface and building volume | feature | P1 | M1 | verified | [GE-0014] |
| GE-0016 | [#14](https://github.com/Jasowills/genesis-engine/issues/14) | Studio shell and viewport scene rendering | feature | P1 | M1 | verified | [GE-0004, GE-0011] |
| GE-0017 | [#15](https://github.com/Jasowills/genesis-engine/issues/15) | Select, inspect and provenance display | feature | P1 | M1 | verified | [GE-0016] |
| GE-0018 | [#8](https://github.com/Jasowills/genesis-engine/issues/8) | Geometry and structural validation gates | feature | P1 | M1 | verified | [GE-0011, GE-0009] |
| GE-0019 | [#16](https://github.com/Jasowills/genesis-engine/issues/16) | GLB export and re-import fixture | feature | P1 | M1 | verified | [GE-0011, GE-0018] |
| GE-0020 | [#17](https://github.com/Jasowills/genesis-engine/issues/17) | End-to-end vertical slice and benchmark harness | test | P1 | M1 | verified | [GE-0013, GE-0014, GE-0015, GE-0016, GE-0017, GE-0018, GE-0019] |
| GE-0021 | [#29](https://github.com/Jasowills/genesis-engine/issues/29) | Contributor and governance documentation | documentation | P1 | M1 | verified | [GE-0007] |
| GE-0022 | [#18](https://github.com/Jasowills/genesis-engine/issues/18) | Headless CLI command surface | feature | P1 | M2 | verified | [GE-0020] |
| GE-0023 | [#19](https://github.com/Jasowills/genesis-engine/issues/19) | External worker adapter framework | engineering | P1 | M2 | verified | [GE-0007, GE-0022] |
| GE-0024 | [#30](https://github.com/Jasowills/genesis-engine/issues/30) | Quality profiles and preflight estimates | feature | P1 | M2 | verified | [GE-0018, GE-0006] |
| GE-0025 | [#23](https://github.com/Jasowills/genesis-engine/issues/23) | WIT/WASI plugin SDK and sandbox PoC | security | P2 | M2 | verified | [GE-0005, GE-0011] |
| GE-0026 | [#20](https://github.com/Jasowills/genesis-engine/issues/20) | AI segmentation and depth adapters | integration | P2 | M3 | verified | [GE-0006, GE-0023] |
| GE-0027 | [#28](https://github.com/Jasowills/genesis-engine/issues/28) | Selective and incremental regeneration | feature | P2 | M3 | verified | [GE-0011, GE-0020] |
| GE-0028 | [#24](https://github.com/Jasowills/genesis-engine/issues/24) | Terrain and secondary layers | feature | P2 | M3 | verified | [GE-0020] |
| GE-0029 | [#25](https://github.com/Jasowills/genesis-engine/issues/25) | Blender export and round-trip fixture | integration | P2 | M3 | in-review | [GE-0019] |
| GE-0030 | [#26](https://github.com/Jasowills/genesis-engine/issues/26) | LOD, collision and texture compression | feature | P3 | M4 | proposed | [GE-0029] |
| GE-0031 | [#32](https://github.com/Jasowills/genesis-engine/issues/32) | Target engine export fixtures | integration | P3 | M4 | proposed | [GE-0030] |
| GE-0032 | [#21](https://github.com/Jasowills/genesis-engine/issues/21) | Signed release and auto-update pipeline | release | P3 | M5 | needs-owner-input | [GE-0008, GE-0009] |

Statuses in this index are generated from each ticket's frontmatter.
