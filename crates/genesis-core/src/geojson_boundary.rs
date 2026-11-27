//! Small-area GeoJSON boundary import and CRS validation (GE-0013).
//!
//! A deliberately constrained GeoJSON Polygon reader for the first vertical
//! slice. It emits a structured error rather than guessing CRS or datum.

use crate::{
    CoreError, CrsMetadata, Result, SourceDescriptor, StableId, check_source, derive_id,
    validate_relative_path,
};
use serde_json::Value;
use sha2::{Digest, Sha256};

/// An imported area of interest. Coordinates are GeoJSON CRS84 (lon, lat).
#[derive(Debug, Clone, PartialEq)]
pub struct Boundary {
    pub id: StableId,
    pub name: String,
    /// [west, south, east, north] in degrees.
    pub bbox: [f64; 4],
    /// Closed rings in lon/lat order.
    pub rings: Vec<Vec<[f64; 2]>>,
    pub source_checksum: String,
    pub source_id: String,
}

fn position(value: &Value) -> Result<[f64; 2]> {
    let array = value.as_array().ok_or_else(|| {
        CoreError::InvalidManifest("GeoJSON position must be an array".to_string())
    })?;
    if array.len() < 2 {
        return Err(CoreError::InvalidManifest(
            "GeoJSON position needs at least lon and lat".to_string(),
        ));
    }
    let lon = array[0].as_f64().ok_or_else(|| {
        CoreError::InvalidManifest("GeoJSON longitude must be a number".to_string())
    })?;
    let lat = array[1].as_f64().ok_or_else(|| {
        CoreError::InvalidManifest("GeoJSON latitude must be a number".to_string())
    })?;
    if !lon.is_finite()
        || !lat.is_finite()
        || !(-180.0..=180.0).contains(&lon)
        || !(-90.0..=90.0).contains(&lat)
    {
        return Err(CoreError::InvalidManifest(
            "GeoJSON position out of WGS84 range".to_string(),
        ));
    }
    Ok([lon, lat])
}

fn geometry(value: &Value) -> Result<&Value> {
    if let Some(features) = value.get("features").and_then(Value::as_array) {
        if value.get("type").and_then(Value::as_str) != Some("FeatureCollection") {
            return Err(CoreError::InvalidManifest(
                "GeoJSON FeatureCollection type mismatch".to_string(),
            ));
        }
        if features.len() != 1 {
            return Err(CoreError::InvalidManifest(
                "import expects exactly one GeoJSON feature".to_string(),
            ));
        }
        return features[0].get("geometry").ok_or_else(|| {
            CoreError::InvalidManifest("GeoJSON feature has no geometry".to_string())
        });
    }
    if let Some(geometry) = value.get("geometry") {
        if value.get("type").and_then(Value::as_str) != Some("Feature") {
            return Err(CoreError::InvalidManifest(
                "GeoJSON Feature type mismatch".to_string(),
            ));
        }
        return Ok(geometry);
    }
    if value.get("coordinates").is_some() {
        return Ok(value);
    }
    Err(CoreError::InvalidManifest(
        "GeoJSON Feature or Geometry expected".to_string(),
    ))
}

/// Validate that the project CRS metadata is sufficient for a reconstruction.
pub fn validate_crs(crs: &CrsMetadata) -> Result<()> {
    if crs.wkt2.is_none() && crs.projjson.is_none() {
        return Err(CoreError::InvalidManifest(
            "project CRS must record WKT2 or PROJJSON".to_string(),
        ));
    }
    if crs
        .vertical_datum
        .as_deref()
        .unwrap_or("")
        .trim()
        .is_empty()
    {
        return Err(CoreError::InvalidManifest(
            "vertical datum must be recorded (GeoJSON Z is ellipsoidal height)".to_string(),
        ));
    }
    if crs.unit.trim().is_empty() || crs.up_axis.trim().is_empty() {
        return Err(CoreError::InvalidManifest(
            "CRS unit and up-axis must be recorded".to_string(),
        ));
    }
    let Some(epoch) = crs.epoch else {
        return Err(CoreError::InvalidManifest(
            "coordinate epoch must be recorded for GeoJSON WGS84/CRS84 import".to_string(),
        ));
    };
    if !epoch.is_finite() {
        return Err(CoreError::InvalidManifest(
            "CRS epoch must be finite".to_string(),
        ));
    }
    // GeoJSON is implicitly WGS84 lon/lat in degrees. A projected CRS over
    // those coordinates is a mismatch, not a silent transformation.
    let reference = format!(
        "{} {}",
        crs.wkt2.as_deref().unwrap_or(""),
        crs.projjson.as_deref().unwrap_or("")
    );
    let lower = reference.to_lowercase();
    if lower.contains("projcrs")
        || lower.contains("projectedcrs")
        || lower.contains("\"projected\"")
    {
        return Err(CoreError::InvalidManifest(
            "declared CRS is projected; GeoJSON coordinates are WGS84 lon/lat degrees".to_string(),
        ));
    }
    Ok(())
}

/// Import a GeoJSON Polygon boundary after gating the source and CRS.
pub fn import_geojson(
    bytes: &[u8],
    name: &str,
    source: &SourceDescriptor,
    crs: &CrsMetadata,
) -> Result<Boundary> {
    check_source(source)?;
    validate_crs(crs)?;
    if name.trim().is_empty() {
        return Err(CoreError::InvalidManifest(
            "boundary name must not be empty".to_string(),
        ));
    }
    let root: Value = serde_json::from_slice(bytes)?;
    let geometry = geometry(&root)?;
    let kind = geometry.get("type").and_then(Value::as_str).unwrap_or("");
    if kind != "Polygon" {
        return Err(CoreError::InvalidManifest(format!(
            "unsupported GeoJSON geometry type '{kind}' (expected Polygon)"
        )));
    }
    let rings_value = geometry
        .get("coordinates")
        .and_then(Value::as_array)
        .ok_or_else(|| CoreError::InvalidManifest("Polygon coordinates missing".to_string()))?;
    if rings_value.is_empty() {
        return Err(CoreError::InvalidManifest(
            "Polygon has no rings".to_string(),
        ));
    }

    let mut rings = Vec::with_capacity(rings_value.len());
    let mut west = f64::INFINITY;
    let mut south = f64::INFINITY;
    let mut east = f64::NEG_INFINITY;
    let mut north = f64::NEG_INFINITY;
    for ring_value in rings_value {
        let raw = ring_value.as_array().ok_or_else(|| {
            CoreError::InvalidManifest("Polygon ring must be an array".to_string())
        })?;
        if raw.len() < 4 {
            return Err(CoreError::InvalidManifest(
                "Polygon ring needs at least four positions".to_string(),
            ));
        }
        let ring: Vec<[f64; 2]> = raw.iter().map(position).collect::<Result<_>>()?;
        if ring.first() != ring.last() {
            return Err(CoreError::InvalidManifest(
                "Polygon ring is not closed".to_string(),
            ));
        }
        if ring.iter().all(|p| p == &ring[0]) {
            return Err(CoreError::InvalidManifest(
                "Polygon ring is degenerate (zero area)".to_string(),
            ));
        }
        for p in &ring {
            west = west.min(p[0]);
            south = south.min(p[1]);
            east = east.max(p[0]);
            north = north.max(p[1]);
        }
        rings.push(ring);
    }
    if east - west > 180.0 {
        return Err(CoreError::InvalidManifest(
            "longitude span exceeds 180 degrees (antimeridian heuristic); explicit cut policy required (GE-0011 follow-up)"
                .to_string(),
        ));
    }

    let checksum = format!("{:x}", Sha256::digest(bytes));
    Ok(Boundary {
        id: derive_id("boundary", &format!("{name}:{checksum}")),
        name: name.to_string(),
        bbox: [west, south, east, north],
        rings,
        source_checksum: checksum,
        source_id: source.id.clone(),
    })
}

/// Resolve a project-relative boundary payload path safely.
pub fn boundary_payload_path(relative: &str) -> Result<String> {
    validate_relative_path(std::path::Path::new(relative))?;
    Ok(relative.to_string())
}
