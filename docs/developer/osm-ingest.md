# OSM footprint and road ingestion (GE-0014)

`import_osm_geojson(bytes, source, crs)` reads a small, documented GeoJSON fixture of OSM features. It is not a planet/PBF parser, PostGIS loader or OSM2World wrapper (all deferred).

## What is retained

- `source_feature_id` from `id`, `osm_id` or `@id`, plus OSM `version` and `timestamp` when present.
- All string tags, carried into provenance `derivation_chain` as `tag:key=value` for inspection.
- The licence election, attribution, checksum and source identity from the lawful-source gate.
- Observed-vs-inferred evidence: a **numeric** `height` → `measured`; `building:levels` or `confidence` → `derived_with_confidence` (levels are a de facto tag with invalid values and a renderer 3 m/floor guess); otherwise `inferred_by_rule`. A footprint alone never establishes height, floor count, roof form or facade.

Building features require a closed Polygon outer ring; road features require a LineString centreline of at least two positions. Non-target features are skipped rather than mislabelled. A 10,000-feature cap bounds resource use.

`to_world_objects` applies the same lawful-source gate before conversion, then creates World IR objects with deterministic IDs derived from source and OSM feature ID. Building positions use a shoelace centroid; road positions use a length-weighted centreline midpoint; empty/degenerate inputs are rejected. Attribution survives IR persistence and the export sidecar.

## Verification

`cargo test --workspace --locked` includes eight OSM ingestion tests: golden IDs and positions, top-level feature IDs, evidence tiers, discriminating geometry errors, 10k cap, gate ordering, denied conversion and sidecar attribution. Thirty-nine Core tests pass in total.
