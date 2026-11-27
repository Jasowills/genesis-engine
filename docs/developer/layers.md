# Secondary environment layers (GE-0028)

Each layer has its own fixture and generator so provenance and validation stay separable:

- `generate_terrain(DemGrid, source)` — local-metre heightfield from a documented elevation grid. `source_resolution_m` and `vertical_accuracy_m` are recorded; the mesh is explicitly labelled `resolution_claim: source-resolution only; no upsampling`. No high-resolution or survey accuracy is inferred.
- `generate_markings(geojson, source)` — centreline-derived road markings, `inferred_by_rule` unless the fixture records an observed marking.
- `generate_sidewalks(geojson, source)` — sidewalk/curb strips offset from road centrelines, `inferred_by_rule`.
- `generate_vegetation(geojson, source)` — one one-sided billboard quad per tree point, `inferred_by_rule` (proxy geometry, not a botanical model).

All generators apply the lawful-source gate. Bridges/tunnels, hydrology, facade reconstruction and full vegetation modelling are out of scope.

## Verification

`cargo test --workspace --locked` includes five layer tests: DEM resolution/accuracy recording and +Z winding, invalid DEM rejection, separate layer fixtures with evidence tiers and true local-frame origins, gate enforcement across all four generators, and the observed-marking/OSM-attribution branch. All surfaces use +Z-facing winding; each mesh records the geodetic origin of its local metre frame; feature IDs from fixtures are retained in provenance.
