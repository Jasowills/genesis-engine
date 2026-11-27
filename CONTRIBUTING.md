# Contributing to Genesis Engine

## Before you start

Read `GENESIS_ENGINE_SPEC.md` (the authority), `PRD.md`, `ARCHITECTURE.md`, the ADRs in `adr/`, and `TICKETS/README.md`. Start from the documented next action in `progress.md`. Do not implement work that has no ticket or whose ADR/decision is open.

## Workflow

1. Pick a ticket whose state is `ready` and whose blocking edges are all verified. Work the frontier.
2. Read the ticket, its dependencies, relevant ADRs and the sections of PRD/architecture it touches.
3. Implement **only** the ticket scope. If it is too large, split it into child tickets before proceeding.
4. Add or update tests; run formatter, linter, type checker, tests and build. Record actual commands and results in the ticket's "Verification evidence" section.
5. Update documentation so it matches real behaviour. Do not document planned features as shipped.
6. Open review. A **fresh independent reviewer** must inspect the full diff, run relevant checks and return `APPROVED`, `CHANGES_REQUESTED` or `BLOCKED`. The author cannot approve their own work.
7. Address findings and repeat review. Mark `verified` only after all gates pass, then update `progress.md`.

## Commits

Use Conventional Commits: `feat(core):`, `fix(geometry):`, `test(terrain):`, `docs:`, `chore(ci):`, `refactor(jobs):`, `security:`. One ticket per commit; no giant "initial implementation" commits. Never force-push, rewrite shared history, or push secrets.

## Data, licences and secrets

- Never commit credentials, tokens, personal data, private imagery, restricted datasets, large model weights or generated city-scale output.
- Respect source-data licences. OSM data is ODbL; attribution and any derivative-database obligations must survive export (see `research/LICENCE_AND_DATA_RIGHTS.md`).
- Google Maps/Street View/Photorealistic 3D Tiles and Cesium ion content are hard-denied as reconstruction inputs.
- Add new dependencies only with a recorded licence, maintenance and cross-platform check; update lockfiles.

## Getting help

Open a GitHub issue using the template, or discuss in the relevant ticket. For security issues follow `SECURITY.md` — do not open a public issue.
