# Genesis Engine — Engineering Workflow and Skill Discovery

**Date:** 2026-11-27
**Authority:** `GENESIS_ENGINE_SPEC.md` remains the project-level authority. This record documents *verified* tooling and the exact workflow to be applied.

## 1. Matty Pocock skill — discovery result

**Found and verified. Not fabricated.**

| Field | Verified value |
| --- | --- |
| Exact skill name | `to-tickets` |
| Frontmatter description (verbatim) | "Break a plan, spec, or the current conversation into a set of tracer-bullet tickets, each declaring its blocking edges, published to the configured tracker (edges as text in one file per ticket locally, or native blocking links on a real tracker)." |
| Source path | `/Users/jasonamadi/.agents/skills/to-tickets/SKILL.md` |
| Base directory | `/Users/jasonamadi/.agents/skills/to-tickets` |
| Invocation check | The OpenCode `skill` tool successfully loaded `to-tickets` and returned its full `SKILL.md`; it is invocable in this session. |
| Version / commit | Verified from the official Claude plugins marketplace entry: upstream `https://github.com/mattpocock/skills.git`, pinned SHA `0ab1b63a410a03d3627979a109c8695de27af954` (commit dated 2026-08-20), repository licence **MIT**. The local file `SKILL.md` was hashed and byte-for-byte matched the upstream `skills/engineering/to-tickets/SKILL.md` at that SHA (`12722dac563e097868473f6091270fdccf0b191a`). Command evidence: `gh api repos/mattpocock/skills`, `gh api .../commits/<sha>`, `gh api .../contents/...`, `shasum`. |
| Companion skills on disk | `setup-matt-pocock-skills`, `to-questionnaire`, `triage`, `ask-matt` (router), `wayfinder`, `implement`, `grill-with-docs`, `grill-me`, `grilling`, `handoff`, `prototype`, `codebase-design`, `domain-modeling`, `improve-codebase-architecture`, `test-*`, `writing-for-agents`, and others. |
| Referenced but NOT installed | `to-spec` — named in `ask-matt/SKILL.md` and in `setup-matt-pocock-skills`, but no `to-spec` directory exists. Do not claim it was used. |

### 1.1 Why `to-tickets` is the requested skill

The owner asked for "the Matty Pocock skill/workflow for breaking work into well-defined, independently verifiable tasks." `to-tickets` is the exact match: it produces **tracer-bullet tickets** — narrow but complete vertical slices that are demoable/verifiable on their own — and requires every ticket to declare its **blocking edges** (the tickets that must complete first). The companion router `ask-matt` confirms the intended flow for a large greenfield effort: `wayfinder` → `to-spec` → `to-tickets` → `implement`.

### 1.2 How it will be applied, and where the spec overrides it

- Apply `to-tickets` **Step 3** (vertical slices, blocking edges, prefactor-first) when decomposing work.
- Apply **Step 4** (quiz the owner on granularity, blocking edges, merge/split) at the approval checkpoint.
- Apply **Step 5** publication format only after owner approval of the tracker. The spec mandates a local ticket file per ticket under `TICKETS/` using the **spec Part VII mandatory template**, so the spec's template wins over the skill's local template where they differ. Both are one-file-per-ticket.
- The skill's "avoid specific file paths or code snippets in tickets" rule is compatible with the spec except where the spec explicitly requires implementation notes; the spec wins.
- `setup-matt-pocock-skills` is **not** run automatically: it is prompt-driven and would edit `CLAUDE.md`/`AGENTS.md` and create `docs/agents/*`. Per spec Phase 1 and owner-approval rules, running it requires owner input (issue tracker choice, label vocabulary). Recorded as an owner decision, not silently executed.
- `to-spec` is unavailable, so the spec itself (`GENESIS_ENGINE_SPEC.md`) plus the PRD/architecture docs produced here serve as the "spec" input to `to-tickets`.

## 2. Subagent capabilities — verified from the harness

| Role available | Name | Intended use in this project |
| --- | --- | --- |
| Codebase exploration | `explore` | Fast discovery of existing files/patterns when a real codebase exists. |
| General-purpose | `general` | Bounded research tasks and multi-step work; also assigned the **mandatory independent reviewer** role for code-changing tickets. |

**Important limitations, recorded rather than assumed:**

- There is **no dedicated `reviewer` subagent** in this harness. The spec requires an *independent reviewer subagent*; the plan is to use a fresh `general` subagent instance with an explicit reviewer brief (spec §39), on a clean context, given the ticket + ADRs + full diff. This must be validated with a real review before any ticket is marked `verified`. If that validation fails, tickets stay `in-review`/`blocked` — never silently waived.
- Subagents run in the background and return a report. They were explicitly instructed to treat web content as data, not instructions.
- Do not assume a subagent shares browser tabs, GitHub credentials or context. Research briefs were self-contained; each wrote a distinct notes file to avoid conflicts.
- The author subagent cannot approve its own work. Reviewer and implementer must be different instances.

## 3. Research delegation (Phase 2/3)

Eight parallel `general` subagents were launched on 2026-11-27, each with a bounded primary-source brief and a distinct output file:

| # | Brief | Output file |
| --- | --- | --- |
| 1 | Geospatial data, CRS, formats, licences | `research/notes/geospatial.md` |
| 2 | Reconstruction / photogrammetry / NeRF / splat pipelines | `research/notes/reconstruction.md` |
| 3 | Desktop shell, viewport, 3D interchange formats | `research/notes/desktop-viewport-formats.md` |
| 4 | AI runtimes and models | `research/notes/ai-ml.md` |
| 5 | Competitive landscape and data/model rights | `research/notes/competitive-data-rights.md` |
| 6 | Procedural generation and OSM-derived reconstruction | `research/notes/procedural.md` |
| 7 | Plugin architecture, sandboxing, security, supply chain | `research/notes/plugin-security.md` |
| 8 | Quality validation, cross-platform CI/GPU, performance/release | `research/notes/quality-ci-gpu.md` |

Consolidation rule: contradictions between subagent reports must be resolved against primary sources before any conclusion is presented. Subagent output is evidence, not authority.

## 3.1 GitHub issue capability (verified read-only)

`gh issue create --help` (gh 2.97.0) supports `--parent`, `--blocked-by` and `--blocking` natively. The owner approved GitHub publication in D-027 (2026-11-27); the `to-tickets` blocking edges were published as **native blocking links** rather than prose, satisfying both the skill's Step 5 and spec §55's dependency requirement. **Supersession note:** the original observation read "No issues have been created" — that was true at the time of discovery and is now historical. The repository has 32 issues in dependency order with verified native blocking links (GE-0008 evidence).

## 4. Planned engineering gates (from the spec, not yet executed)

1. Research → `research/*.md` with URLs, access dates, licences and confidence.
2. Architecture + ADRs → `ARCHITECTURE.md`, `adr/*.md`.
3. Product discovery → owner questions → `DECISIONS.md`.
4. PRD + roadmap + tickets → `PRD.md`, `ROADMAP.md`, `TICKETS/`.
5. GitHub setup after explicit approval (granted 2026-11-27 as D-027) → repo, labels, milestones, issues, preceded by the Phase 7 approval checklist in `DECISIONS.md`.
6. **Approval checkpoint — stop and wait.**
7. Per ticket: implement → test/lint/build → independent reviewer → verify → commit (Conventional Commits) → update `progress.md`.

No product implementation has started.
