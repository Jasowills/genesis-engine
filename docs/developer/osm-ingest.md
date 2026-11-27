# OSM footprint and road ingestion (GE-0014)

`import_osm_geojson(bytes, source, crs)` reads a small, documented GeoJSON fixture of OSM features. It is not a planet/PBF parser, PostGIS loader or OSM2World wrapper (all deferred).

## What is retained

- `source_feature_id` from `id`, `osm_id` or `@id`, plus OSM `version` and `timestamp` when present.
- All string tags, carried into provenance `derivation_chain` as `tag:key=value` for inspection.
- The licence election, attribution, checksum and source identity from the lawful-source gate.
- Observed-vs-inferred evidence: explicit `height` or `building:levels` → `measured`; `confidence` → `derived_with_confidence`; otherwise `inferred_by_rule`. A footprint alone never establishes height, floor count, roof form or facade.

Building features require a closed Polygon outer ring; road features require a LineString centreline of at least two positions. Non-target features are skipped rather than mislabelled. A 10,000-feature cap bounds resource use.

`to_world_objects` converts features into World IR objects with deterministic IDs derived from source and OSM feature ID, so re-import of the same revision is stable. Attribution survives IR persistence and the export sidecar.

## Verification

`cargo test --workspace --locked` includes three OSM ingestion tests: source IDs/version/timestamp/evidence, World IR provenance and sidecar round-trip, and denied-source/malformed-geometry rejection. Thirty-four Core tests pass in total.
