# Lawful-source gate and provenance (GE-0012)

Genesis enforces the owner-approved hard source constraint (D-028/ADR-0007) in code, not only in documentation. `prepare_import(&SourceDescriptor)` gates import; `enforce_sources` and `attribution_sidecar(&WorldIr, &[SourceDescriptor])` gate export. `check_source` is the underlying deny-list check.

## Deny-list

The deny-list checks publisher, URL, id, version, attribution and filters (case-insensitive) and rejects matches with `CoreError::SourceDenied`. Patterns include Google Maps Platform hosts (`maps.googleapis.com`, `tile.googleapis.com`), Street View (`street view`, `street-view`, `streetview`), Photorealistic 3D Tiles, and Cesium ion (`cesium ion`, `cesium-ion`, `ion.cesium.com`, `assets.cesium.com`, `api.cesium.com`, `cesium.com/ion/`).

- Google Maps Platform endpoints (`maps.googleapis.com`, `maps.google.com`, `google maps`)
- Street View and Photorealistic 3D Tiles content
- Cesium ion content (`cesium ion`, `ion.cesium.com`, `cesium.com/ion/`, `api.cesium.com`)
- Licences tagged non-commercial (`non-commercial`, `by-nc`, `research only`)

Open standards software (CesiumJS, 3D Tiles) is **allowed** with Genesis-owned content. A visual-only basemap carve-out is deferred and requires written legal clearance.

## Provenance

`SourceDescriptor` records licence election, attribution, vintage, filters, checksum and purpose. `From<&SourceDescriptor> for Provenance` transfers those fields into World IR objects. The object's `EvidenceKind` records whether a value was measured, derived with confidence, inferred by rule or unknown — a footprint is not a height, and a centreline is not a road surface.

`attribution_sidecar(&WorldIr, &[SourceDescriptor])` emits a machine-readable export sidecar with distinct attribution/licence/source/vintage/checksum records. Attribution and the elected licence survive IR → export.

## Verification

`cargo test --workspace --locked` runs six dedicated tests covering allowed OSM sources, all deny-list patterns with case/hyphen variants, licence spellings, purpose behaviour, malformed descriptors, full provenance preservation, and export gating with distinct-source sidecars. Twenty-two Core tests pass in total.
