# Headless CLI (GE-0022)

`genesis` is a thin adapter over Genesis Core. Every command prints one versioned JSON document on stdout (`schema_version: 1`, `command`, `status`, `data` or `error`) and exits non-zero on structured errors. No domain logic (CRS, provenance, validation, geometry) lives in the CLI.

## Commands

| Command | Purpose |
| --- | --- |
| `genesis <project-path> <name> [--wkt2 ...] [--datum ...] [--epoch ...]` | Create a project |
| `genesis boundary <project-path> <geojson> <name>` | Import and validate a boundary |
| `genesis features <project-path> <geojson>` | Import OSM features and write World IR |
| `genesis generate <project-path> <seed> [--road-width N]` | Deterministic procedural meshes |
| `genesis validate <project-path> [--profile preview\|game_ready\|high_fidelity\|survey_reference]` | Structural validation report |
| `genesis export <project-path> <output.glb>` | Validated GLB plus attribution sidecar |
| `genesis scene <project-path>` | Versioned scene descriptor (IDs only) |
| `genesis job <project-path> <id>` / `genesis job-cancel ...` | Structured `unsupported` until Core job orchestration (GE-0023) exists |

Command names are flat (the first token selects the operation). Run `genesis --help` for exact arguments.

## Verification

`cargo test -p genesis-cli` runs two integration tests: the full E2E workflow through the binary and a structured-error case. `cargo test --workspace --locked` includes them in the cross-platform CI matrix. `crates/genesis-cli/tests/cli.rs` compares CLI output with Core `import_glb` results (mesh count, triangles, provenance).

## Limits

No cloud CLI, interactive shell, worker orchestration or job cancellation yet; those commands return structured `unsupported`, not fake success. Signing and distribution are deferred (D-022).
