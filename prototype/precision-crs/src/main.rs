//! Throwaway evidence prototype for GE-0003 (not product code).
//! Measures float32 precision loss at georeferenced-scale coordinates and
//! demonstrates the authoritative-geodetic + local-ENU-frame strategy from ADR-0008.

const WGS84_A: f64 = 6_378_137.0;
const WGS84_F: f64 = 1.0 / 298.257_223_563;

/// ULP of a value when stored as IEEE-754 binary32.
/// Uses `next_up`/`next_down` so negative binades and `f32::MAX` are handled correctly.
fn f32_ulp(x: f64) -> f64 {
    let v = x as f32;
    if !v.is_finite() {
        return f64::NAN;
    }
    let neighbour = if v.is_sign_positive() {
        v.next_up()
    } else {
        v.next_down()
    };
    (neighbour as f64 - v as f64).abs()
}

fn geodetic_to_ecef(lat_deg: f64, lon_deg: f64, height_m: f64) -> (f64, f64, f64) {
    let lat = lat_deg.to_radians();
    let lon = lon_deg.to_radians();
    let e2 = WGS84_F * (2.0 - WGS84_F);
    let (sin_lat, cos_lat) = (lat.sin(), lat.cos());
    let n = WGS84_A / (1.0 - e2 * sin_lat * sin_lat).sqrt();
    let x = (n + height_m) * cos_lat * lon.cos();
    let y = (n + height_m) * cos_lat * lon.sin();
    let z = (n * (1.0 - e2) + height_m) * sin_lat;
    (x, y, z)
}

/// Rotate an ECEF delta into a local East/North/Up frame at the given origin.
fn ecef_delta_to_enu(lat_deg: f64, lon_deg: f64, d: (f64, f64, f64)) -> (f64, f64, f64) {
    let lat = lat_deg.to_radians();
    let lon = lon_deg.to_radians();
    let (sin_lat, cos_lat, sin_lon, cos_lon) = (lat.sin(), lat.cos(), lon.sin(), lon.cos());
    let east = -sin_lon * d.0 + cos_lon * d.1;
    let north = -sin_lat * cos_lon * d.0 - sin_lat * sin_lon * d.1 + cos_lat * d.2;
    let up = cos_lat * cos_lon * d.0 + cos_lat * sin_lon * d.1 + sin_lat * d.2;
    (east, north, up)
}

/// Inverse of `ecef_delta_to_enu`.
fn enu_to_ecef_delta(lat_deg: f64, lon_deg: f64, e: (f64, f64, f64)) -> (f64, f64, f64) {
    let lat = lat_deg.to_radians();
    let lon = lon_deg.to_radians();
    let (sin_lat, cos_lat, sin_lon, cos_lon) = (lat.sin(), lat.cos(), lon.sin(), lon.cos());
    let dx = -sin_lon * e.0 - sin_lat * cos_lon * e.1 + cos_lat * cos_lon * e.2;
    let dy = cos_lon * e.0 - sin_lat * sin_lon * e.1 + cos_lat * sin_lon * e.2;
    let dz = cos_lat * e.1 + sin_lat * e.2;
    (dx, dy, dz)
}

/// Cut a longitude segment crossing the antimeridian into RFC 7946 pieces.
fn cut_antimeridian(lon0: f64, lon1: f64) -> Vec<(f64, f64)> {
    if (lon0 - lon1).abs() <= 180.0 {
        return vec![(lon0, lon1)];
    }
    if lon0 > lon1 {
        vec![(lon0, 180.0), (-180.0, lon1)]
    } else {
        vec![(lon0, -180.0), (180.0, lon1)]
    }
}

fn main() {
    println!("GE-0003 precision/CRS prototype — throwaway evidence, not product code");

    for coord in [6_370_000.0_f64, 10_000_000.0, 123_456.0, 20_037_508.0] {
        println!("f32_ulp({coord:.1}) = {:.6} m", f32_ulp(coord));
    }

    // Naive global-f32 storage: large absolute value + small local offset.
    let global = 10_000_000.0_f64;
    let offset = 0.01_f64;
    let naive = ((global as f32 + offset as f32) as f64) - global;
    println!("naive_f32_add_error({global:.0} + {offset}) = {:.6} m (wanted {offset})", naive - offset);

    // Local-frame strategy: authoritative f64 origin, f32 local ENU delta.
    let (lat0, lon0, h0) = (6.6_f64, 3.3_f64, 40.0_f64);
    let origin = geodetic_to_ecef(lat0, lon0, h0);
    let enu_exact = (123.456_f64, -45.678, 12.345);
    let enu_f32 = (enu_exact.0 as f32, enu_exact.1 as f32, enu_exact.2 as f32);
    let delta_exact = enu_to_ecef_delta(lat0, lon0, enu_exact);
    let delta_stored = enu_to_ecef_delta(
        lat0,
        lon0,
        (enu_f32.0 as f64, enu_f32.1 as f64, enu_f32.2 as f64),
    );
    let reconstructed = (
        origin.0 + delta_stored.0,
        origin.1 + delta_stored.1,
        origin.2 + delta_stored.2,
    );
    let exact = (
        origin.0 + delta_exact.0,
        origin.1 + delta_exact.1,
        origin.2 + delta_exact.2,
    );
    let err = ((reconstructed.0 - exact.0).powi(2)
        + (reconstructed.1 - exact.1).powi(2)
        + (reconstructed.2 - exact.2).powi(2))
        .sqrt();
    println!("local_frame_f32_storage_error = {:.9} m", err);

    // ECEF -> ENU -> ECEF round trip, exact f64, with a second nearby point.
    let target = geodetic_to_ecef(6.601_f64, 3.301_f64, 47.0);
    let delta = (target.0 - origin.0, target.1 - origin.1, target.2 - origin.2);
    let enu = ecef_delta_to_enu(lat0, lon0, delta);
    let back = enu_to_ecef_delta(lat0, lon0, enu);
    let rt = ((back.0 - delta.0).powi(2) + (back.1 - delta.1).powi(2) + (back.2 - delta.2).powi(2)).sqrt();
    println!("enu_round_trip_error_f64 = {:.12} m", rt);

    for seg in [cut_antimeridian(179.999, -179.999), cut_antimeridian(-179.9, 179.9)] {
        println!("antimeridian_cut = [{:.3},{:.3}] [{:.3},{:.3}]", seg[0].0, seg[0].1, seg[1].0, seg[1].1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn f32_ulp_matches_derived_metre_scale_values() {
        assert!((f32_ulp(6_370_000.0) - 0.5).abs() < 1e-9);
        assert!((f32_ulp(10_000_000.0) - 1.0).abs() < 1e-9);
    }

    #[test]
    fn absolute_geodetic_and_enu_golden_vectors() {
        let (x, y, z) = geodetic_to_ecef(0.0, 0.0, 0.0);
        assert!((x - WGS84_A).abs() < 1e-9 && y.abs() < 1e-9 && z.abs() < 1e-9);
        let (x, y, z) = geodetic_to_ecef(0.0, 90.0, 0.0);
        assert!(x.abs() < 1e-6 && (y - WGS84_A).abs() < 1e-6 && z.abs() < 1e-6);
        let (x, y, z) = geodetic_to_ecef(90.0, 0.0, 0.0);
        let b = WGS84_A * (1.0 - WGS84_F);
        assert!(x.abs() < 1e-6 && y.abs() < 1e-6 && (z - b).abs() < 1e-6);

        // ENU basis at (lat=0, lon=0): dX -> Up, dY -> East, dZ -> North.
        let up = ecef_delta_to_enu(0.0, 0.0, (1.0, 0.0, 0.0));
        let east = ecef_delta_to_enu(0.0, 0.0, (0.0, 1.0, 0.0));
        let north = ecef_delta_to_enu(0.0, 0.0, (0.0, 0.0, 1.0));
        assert!((up.2 - 1.0).abs() < 1e-12 && up.0.abs() < 1e-12 && up.1.abs() < 1e-12);
        assert!((east.0 - 1.0).abs() < 1e-12 && east.1.abs() < 1e-12 && east.2.abs() < 1e-12);
        assert!((north.1 - 1.0).abs() < 1e-12 && north.0.abs() < 1e-12 && north.2.abs() < 1e-12);
    }

    #[test]
    fn f32_ulp_handles_negative_values() {
        assert!((f32_ulp(-10_000_000.0) - 1.0).abs() < 1e-9);
    }

    #[test]
    fn naive_f32_global_storage_loses_centimetre_offset() {
        let global = 10_000_000.0_f64;
        let offset = 0.01_f64;
        let stored = ((global as f32 + offset as f32) as f64) - global;
        // At 1e7 the binary32 ULP is 1 m, so a 1 cm offset is lost entirely.
        assert!(stored.abs() < 1e-6, "stored delta {stored}");
    }

    #[test]
    fn local_frame_f32_storage_is_sub_millimetre() {
        let (lat0, lon0) = (6.6_f64, 3.3_f64);
        let enu = (123.456_f64, -45.678, 12.345);
        let stored = enu_to_ecef_delta(lat0, lon0, (enu.0 as f32 as f64, enu.1 as f32 as f64, enu.2 as f32 as f64));
        let exact = enu_to_ecef_delta(lat0, lon0, enu);
        let err = ((stored.0 - exact.0).powi(2) + (stored.1 - exact.1).powi(2) + (stored.2 - exact.2).powi(2)).sqrt();
        assert!(err < 1e-3, "local-frame error {} m", err);
    }

    #[test]
    fn enu_round_trip_is_exact_to_nanometre() {
        let (lat0, lon0, h0) = (6.6_f64, 3.3_f64, 40.0);
        let origin = geodetic_to_ecef(lat0, lon0, h0);
        let target = geodetic_to_ecef(6.601, 3.301, 47.0);
        let d = (target.0 - origin.0, target.1 - origin.1, target.2 - origin.2);
        let back = enu_to_ecef_delta(lat0, lon0, ecef_delta_to_enu(lat0, lon0, d));
        let err = ((back.0 - d.0).powi(2) + (back.1 - d.1).powi(2) + (back.2 - d.2).powi(2)).sqrt();
        assert!(err < 1e-9, "round-trip error {} m", err);
    }

    #[test]
    fn antimeridian_golden_vectors() {
        assert_eq!(cut_antimeridian(179.999, -179.999), vec![(179.999, 180.0), (-180.0, -179.999)]);
        assert_eq!(cut_antimeridian(-179.9, 179.9), vec![(-179.9, -180.0), (180.0, 179.9)]);
        assert_eq!(cut_antimeridian(3.3, 3.301), vec![(3.3, 3.301)]);
    }
}
