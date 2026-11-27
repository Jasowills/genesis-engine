# World IR v2 (GE-0011)

The World IR is the engine-agnostic, versioned description of a reconstructed world. It is the seam shared by Studio, CLI, SDK, adapters and exports (ADR-0002).

## Design rules

- **Metadata only.** Structured documents hold IDs, CRS/frame, provenance, evidence flags, seeds, dependencies, validation state and content hashes. Geometry, textures and indexes are referenced by project-relative `AssetRef` paths — never inlined into one giant JSON document.
- **Stable IDs.** `derive_id(namespace, key)` is SHA-256 based and deterministic; names alone are not stable across edits.
- **Honest uncertainty.** Every object carries `EvidenceKind`: `measured`, `derived_with_confidence`, `inferred_by_rule` or `unknown`. A footprint does not establish height; a centreline is not a road surface.
- **Precision.** Authoritative geodetic positions are f64; a `LocalFrame` records an f64 ECEF origin for local ENU maths. The GE-0003 golden vectors are preserved through serialization (ADR-0008).
- **Validation.** `WorldIr::validate()` rejects duplicate IDs, unresolved dependencies, empty frame IDs and payload paths that escape the project root.
- **Licensing.** `Provenance` carries the elected licence, attribution, vintage, filters, checksum, source feature IDs and derivation chain. Licence text is data, never executable input.

## Schema versions and migration

Current schema: **2**. Legacy schema: **1**.

- `migrate_forward(bytes)` upgrades v1 → v2: moves `objects[].height_m` into `objects[].dimensions.height_m` and adds a default `local_frame`.
- `migrate_backward(bytes)` downgrades v2 → v1: reverses that transformation.
- Both are idempotent; `WorldIr::from_bytes` rejects any other version with `UnsupportedSchemaVersion`.

## Verification

```
cargo test --workspace --locked
cargo clippy --workspace --locked -- -D warnings
```

Five World IR tests cover lossless round-trip, f64 golden-vector preservation, deterministic IDs, forward/backward migration and structured errors for unsupported versions, path traversal and unresolved dependencies.
