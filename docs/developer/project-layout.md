# Genesis project layout and Core API (GE-0010)

A Genesis project is a portable directory. No state is stored in a hidden global cache. The only entry point is the Rust `genesis-core` crate, so Studio, CLI and SDK share one implementation.

## Layout

```
my-project/
├── metadata/project.json     # schema version, id, name, timestamp, CRS metadata
├── worlds/                   # world and layer manifests
├── payloads/                 # binary geometry, textures, indexes
├── sources/                  # source descriptors / permitted local copies
├── jobs/                     # job records and logs
├── validation/               # metric records and reports
└── exports/                  # generated export artefacts
```

`metadata/project.json` contains **no inline geometry**. Authoritative coordinates stay high precision with CRS, epoch, vertical datum and unit recorded; local-frame details belong in the World IR (ADR-0008).

## Core API

- `create_project(root, name, crs) -> Result<Project>` — creates the layout and manifest.
- `open_project(root) -> Result<Project>` — validates schema version and manifest; never reads a global cache.
- `Project::save()` — atomic manifest write (temp file + rename). On Windows, where rename cannot replace an existing file, the original is first renamed to a `.bak`, the replacement is renamed into place, and the backup is restored if that second rename fails — the only valid manifest is never deleted.
- `Project::resolve(relative)` — resolves a project-relative path after rejecting traversal, absolute paths, Windows drive prefixes and backslashes.
- `CoreError` — structured errors including `UnsupportedSchemaVersion`, `InvalidManifest`, `PathEscapesProject` and `AlreadyExists`.

Supported manifest schema version: **1**. Unknown versions produce a structured error rather than silent fallback.

## Verification

```
cargo test --workspace --locked
cargo clippy --workspace --locked -- -D warnings
```

Six integration tests cover round-trip, portability after directory move, unknown version, malformed/missing manifest, path containment and duplicate project detection.
