# First end-to-end vertical slice (GE-0020)

`cargo test -p genesis-core --test vertical_slice` drives the spec §50 workflow headlessly through Core only:

project create → GeoJSON boundary import → CRS validation → OSM fixture import → World IR → procedural road/building meshes → validation with `geographic_accuracy_claim: "none"` → GLB export → GLB re-import.

Inputs are frozen: `fixtures/ikeja-boundary.geojson` (boundary from `research/BENCHMARK_AREA.md`, D-031) and `fixtures/ikeja-small.geojson`. Their SHA-256 digests, the measured wall-clock time, platform, mesh counts and triangle count are printed as JSON. Run `scripts/benchmark_slice.sh [output.json]` to archive that JSON report.

## Honest limits

No independent ground truth exists for the study area, so **no real-world accuracy claim** is made. No competitor output from the same frozen inputs is available, so `benchmark_comparisons` is empty and cross-tool comparison is deferred. Peak memory and disk usage are not instrumented in this slice (`UNVERIFIED`). Wall-clock numbers are local measurements, not performance budgets.

The E2E test runs in the existing cross-platform CI matrix via `cargo test --workspace --locked`.
