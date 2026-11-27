# GeoJSON boundary import (GE-0013)

`import_geojson(bytes, name, source, crs)` is the first geographic-selection primitive. It is deliberately constrained for the initial slice: a single GeoJSON `Feature`/`FeatureCollection` containing one `Polygon`.

## Validation performed

- Source must pass `check_source` before geometry is parsed (lawful-source gate).
- CRS metadata must record WKT2 or PROJJSON, a vertical datum, unit and up-axis. GeoJSON's third coordinate is **ellipsoidal** height, which is not orthometric height; the datum must be explicit.
- Every position must be finite WGS84 lon/lat; rings need at least four closed positions.
- Antimeridian-crossing boundaries are rejected pending an explicit cut/unwrap policy (GE-0011 follow-up) rather than silently mis-tiled.
- The import is deterministic: a SHA-256 source checksum is recorded and the boundary ID is derived by `derive_id`.

`boundary_payload_path` applies the same project-relative path containment as the project model.

## Verification

`cargo test --workspace --locked` includes four boundary tests covering a valid Ikeja fixture with bbox/checksum, malformed geometry variants, CRS validation and denied-source/antimeridian rejection. Twenty-six Core tests pass in total.
