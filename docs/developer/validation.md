# Structural validation gates (GE-0018)

`validate_world` and `validate_mesh` produce machine-readable `ValidationReport` / `MetricRecord` values. They are Core functions, so Studio, CLI and headless paths share one gate.

## Metrics emitted

- `zero_area_faces`, `non_manifold_edges`, `boundary_edges`, `normal_orientation_conflicts`, `self_intersections`, `triangle_count` for meshes.
- `object_count`, `missing_provenance` for the World IR.
- Every record carries `method`, `tool_version`, `parameters` and `limitations`.

`self_intersections` is deliberately recorded as `-1` with an explicit `UNVERIFIED` limitation — triangle-triangle intersection testing is not implemented in this slice. Normal orientation uses a coarse up-direction heuristic, not a full adjacency-consistency test. No texture-coverage or target-engine budget claim is made.

## Honest accuracy

Without independent checkpoints, `geographic_accuracy_claim` is exactly `"none"`. With checkpoints the report says ASPRS-style reporting is not implemented; it never invents an accuracy number. `check_export_ready` rejects a `survey_reference` profile without checkpoints, and rejects zero-area faces or non-manifold edges. `ValidationReport.passed` covers provenance and profile only — it is **not** the structural export gate; callers must use `check_export_ready`.

## Profiles

`preview`, `game_ready`, `high_fidelity`, `survey_reference` are accepted enum values. Only the structural gates above are implemented; performance budgets are not yet calibrated.

## Verification

`cargo test --workspace --locked` includes five validation tests: defect detection, no-accuracy-claim, survey-profile rejection, JSON round-trip and missing-provenance flagging. Counts in this page are point-in-time verification snapshots; see `progress.md` for the current total.; these tests run in the cross-platform CI matrix.
