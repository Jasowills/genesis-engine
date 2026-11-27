# GE-0003 — Precision/CRS prototype results

**Status:** throwaway evidence prototype (not product code). **Date:** 2026-11-27.
**Toolchain:** `rustc 1.98.1 (48a229cea 2026-09-01)`, cargo 1.98.1, macOS 26.5.2 x86_64.
**Source:** `prototype/precision-crs/src/main.rs`. No external dependencies; no network access, credentials or external data.

## Commands and actual output

```
$ cd prototype/precision-crs && cargo test
running 5 tests
test tests::antimeridian_golden_vectors ... ok
test tests::enu_round_trip_is_exact_to_nanometre ... ok
test tests::f32_ulp_matches_derived_metre_scale_values ... ok
test tests::naive_f32_global_storage_loses_centimetre_offset ... ok
test tests::local_frame_f32_storage_is_sub_millimetre ... ok
test result: ok. 5 passed; 0 failed

$ cargo run --quiet
f32_ulp(6370000.0) = 0.500000 m
f32_ulp(10000000.0) = 1.000000 m
f32_ulp(123456.0) = 0.007812 m
naive_f32_add_error(10000000 + 0.01) = -0.010000 m (wanted 0.01)
local_frame_f32_storage_error = 0.000001920 m
enu_round_trip_error_f64 = 0.000000000000 m
antimeridian_cut = [179.999,180.000] [-180.000,-179.999]
antimeridian_cut = [-179.900,-180.000] [180.0,179.900]
```

## Findings (measured, not estimated)

1. **The precision risk is real.** Binary32 ULP is **0.5 m** at Earth-radius scale (6.37e6 m) and **1.0 m** at 1e7 m. At 1e7 m, adding a 1 cm local offset in f32 loses the offset entirely (stored delta 0.0 m). This corroborates the derived values in `research/FORMAT_AND_STANDARDS.md` §2 and `adr/0008`.
2. **The accepted local-frame strategy works.** With an authoritative f64 geodetic/ECEF origin and a local ENU delta stored in f32, reconstruction error for a ~130 m offset was **1.92e-6 m** (sub-millimetre), versus total loss of a centimetre offset in the naive global-f32 path. This is direct evidence for ADR-0008 (floating-origin / local-frame precision).
3. **ECEF→ENU→ECEF round-trip in f64 is exact to the limit of measurement** (0.0 m to 12 decimal places) for an Ikeja-area pair (6.6°N, 3.3°E, 40 m → 6.601°N, 3.301°E, 47 m).
4. **Antimeridian golden vectors pass:** a 179.999° → -179.999° segment is cut into `(179.999, 180.0)` and `(-180.0, -179.999)`; the reverse case cuts at -180/+180; a non-crossing segment is unchanged. This matches RFC 7946 §3.1.9.

## Rebasing invariants (to be carried into the World IR, GE-0011)

1. Authoritative coordinates remain f64 geodetic/ECEF with CRS, epoch, vertical datum and height unit; **never** stored as GPU float32 vertex data.
2. Each tile/region declares one local frame origin; all mesh maths and rendering use deltas relative to that origin.
3. Rebasing is a pure function `geodetic → ECEF → ENU(local) → ECEF → geodetic`; golden vectors above must pass after any change.
4. Changing the origin must update picking, undo, physics and metadata atomically — a stale frame in any consumer produces silent offsets.
5. Antimeridian-crossing geometry is cut at import and reconstructed from the cut pieces; the internal longitude policy is explicit, never implicit.
6. Do not design around draft glTF `KHR_accessor_float64`; GPU f64 buffers/shaders are not broadly available.

## Limits of this evidence

Single host (macOS/x86_64), CPU-only arithmetic; no GPU jitter measurement, no WebGL/WebGPU probe (GE-0004), and no `geo`/PROJ library integration (their far-from-origin fixes remain unreleased as of 2026-11-27). These are follow-ups, not claims of platform coverage.
