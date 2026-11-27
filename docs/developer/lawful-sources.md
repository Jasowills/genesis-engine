# Lawful-source gate and provenance (GE-0012)

Genesis enforces the owner-approved hard source constraint (D-028/ADR-0007) in code, not only in documentation. `check_source(&SourceDescriptor)` must pass before any source enters import, inference or export.

## Deny-list

Sources whose publisher or URL matches these patterns are rejected with `CoreError::SourceDenied`:

- Google Maps Platform endpoints (`maps.googleapis.com`, `maps.google.com`, `google maps`)
- Street View and Photorealistic 3D Tiles content
- Cesium ion content (`cesium ion`, `ion.cesium.com`, `cesium.com/ion/`, `api.cesium.com`)
- Licences tagged non-commercial (`non-commercial`, `by-nc`, `research only`)

Open standards software (CesiumJS, 3D Tiles) is **allowed** with Genesis-owned content. A visual-only basemap carve-out is deferred and requires written legal clearance.

## Provenance

`SourceDescriptor` records licence election, attribution, vintage, filters, checksum and purpose. `From<&SourceDescriptor> for Provenance` transfers those fields into World IR objects. The object's `EvidenceKind` records whether a value was measured, derived with confidence, inferred by rule or unknown — a footprint is not a height, and a centreline is not a road surface.

`attribution_sidecar(&WorldIr)` emits a machine-readable export sidecar with distinct attribution/licence/source/vintage/checksum records. Attribution and the elected licence survive IR → export.

## Verification

`cargo test --workspace --locked` runs four dedicated tests covering allowed OSM sources, all deny-list patterns, non-commercial licences, malformed descriptors, and attribution survival through IR serialization and the export sidecar. Twenty Core tests pass in total.
