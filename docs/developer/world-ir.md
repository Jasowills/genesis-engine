# World IR v2 (GE-0011)

The World IR is the engine-agnostic, versioned description of a reconstructed world. It is the seam shared by Studio, CLI, SDK, adapters and exports (ADR-0002).

## Design rules

- **Metadata only.** Structured documents hold IDs, CRS/frame, provenance, evidence flags, seeds, dependencies, validation state and content hashes. Geometry, textures and indexes live in separate payload documents referenced by project-relative paths; `AssetRef` is the planned typed form of that reference. The IR document itself stays metadata-only and is never one giant inline geometry JSON.
- **Stable IDs.** `derive_id(namespace, key)` is SHA-256 based and deterministic; names alone are not stable across edits.
- **Honest uncertainty.** Every object carries `EvidenceKind`: `measured`, `derived_with_confidence`, `inferred_by_rule` or `unknown`. A footprint does not establish height; a centreline is not a road surface.
- **Precision.** Authoritative geodetic positions are f64; `LocalFrame::from_geodetic` computes a WGS84 ECEF origin and records the geodetic origin. `geodetic_to_enu` converts points into local east/north/up, and procedural geometry uses it rather than an equirectangular approximation. The GE-0003 golden vectors are preserved through serialization and the ENU round trip (ADR-0008).
- **Validation.** `WorldIr::validate()` rejects duplicate IDs, unresolved dependencies, empty frame IDs and payload paths that escape the project root.
- **Licensing.** `Provenance` carries the elected licence, attribution, vintage, filters, checksum, source feature IDs and derivation chain. Licence text is data, never executable input.

## Schema versions and migration

Current schema: **2**. Legacy schema: **1**.

- `migrate_forward(bytes)` upgrades v1 → v2: moves `objects[].height_m` into `objects[].dimensions.height_m` and adds a default `local_frame`.
- `migrate_backward(bytes)` downgrades v2 → v1: adds the top-level `height_m` for v1 readers but **retains** the `dimensions` object so fields such as `footprint_area_m2` are never destroyed.
- Both are idempotent; `WorldIr::from_bytes` checks `schema_version` on the raw document **before** deserialization, so legacy shape-incompatible documents produce `UnsupportedSchemaVersion`, not a JSON error.
- `StableId` validation runs on deserialization as well as in constructors; `validate()` checks world/layer/object/asset/dependency IDs, duplicate IDs, unresolved dependencies, empty frame IDs and payload path containment.

## Verification

```
cargo test --workspace --locked
cargo clippy --workspace --locked -- -D warnings
```

Counts in this page are point-in-time verification snapshots; see `progress.md` for the current total. cover lossless round-trip, GE-0003 golden vectors (independent Ikeja origin constants, the 1e7 m binary32 contrast and an antimeridian vector), deterministic IDs and deserialization validation, dimension-preserving migration with nested-height precedence, legacy version gating, and structured errors for path traversal, duplicate IDs, empty frames and unresolved dependencies.
