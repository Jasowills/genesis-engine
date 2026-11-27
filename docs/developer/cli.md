# Headless CLI (GE-0022)

`genesis` is a thin adapter over Genesis Core. Executed commands print one versioned JSON document on stdout (`schema_version: 1`, `command`, `status`, `data` or `error`) and exit non-zero on structured errors. `--help` and clap parse failures are plain text with clap exit code 2 — a deliberate, documented exception to the machine-readable contract. No domain logic (CRS, provenance, validation, geometry) lives in the CLI.

## Commands

| Command | Purpose |
| --- | --- |
| `genesis <project-path> <name> [--wkt2 ...] [--datum ...] [--epoch ...]` | Create a project |
| `genesis boundary <project-path> <geojson> <name> [--source-id ID] [--licence L] [--attribution S]` | Import and validate a boundary; persist caller-supplied provenance |
| `genesis features <project-path> <geojson> [--source-id ID] [--licence L] [--attribution S]` | Import OSM features and write World IR; persist caller-supplied provenance |
| `genesis generate <project-path> <seed> [--road-width N]` | Deterministic procedural meshes |
| `genesis validate <project-path> [--profile preview\|game_ready\|high_fidelity\|survey_reference]` | Structural validation report |
| `genesis export <project-path> <output.glb>` | Validated GLB plus attribution sidecar |
| `genesis scene <project-path>` | Versioned scene descriptor (IDs only) |
| `genesis job <project-path> <id>` / `genesis job-cancel ...` | Structured `unsupported` until Core job orchestration (GE-0023) exists |

Command names are flat (the first token selects the operation). Project CRS is read from the project manifest and provenance is read from `metadata/source.json`, so later commands cannot silently substitute defaults. Unknown source data is never attributed to OpenStreetMap unless the caller passes `--attribution`. Run `genesis --help` for exact arguments.

## Verification

`cargo test -p genesis-cli` runs two integration tests: the full E2E workflow through the binary and a structured-error case. `cargo test --workspace --locked` includes them in the cross-platform CI matrix. `crates/genesis-cli/tests/cli.rs` compares CLI output with Core `import_glb` results (mesh count, triangles, provenance).

## Limits

No cloud CLI, interactive shell, worker orchestration or job cancellation yet; those commands return structured `unsupported`, not fake success. Signing and distribution are deferred (D-022).
