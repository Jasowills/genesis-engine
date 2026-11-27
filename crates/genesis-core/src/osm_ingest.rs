//! Small OSM footprint / centreline ingestion (GE-0014).
//!
//! The first slice consumes a documented, redistributable GeoJSON fixture of
//! OSM features — not a planet extract, PBF parser, PostGIS or OSM2World.
//! Source identity, version, timestamp, licence and observed-vs-inferred
//! evidence travel into the World IR.

use crate::{
    CoreError, CrsMetadata, Result, SourceDescriptor, check_source, derive_id,
    geojson_boundary::validate_crs,
    world_ir::{EvidenceKind, Provenance, WorldObject},
};
use serde_json::Value;
use std::collections::BTreeMap;

const MAX_FEATURES: usize = 10_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OsmFeatureKind {
    Building,
    Road,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OsmFeature {
    pub source_feature_id: String,
    pub kind: OsmFeatureKind,
    pub coordinates: Vec<[f64; 2]>,
    pub version: Option<u64>,
    pub timestamp: Option<String>,
    pub tags: BTreeMap<String, String>,
    pub evidence: EvidenceKind,
}

fn position(value: &Value) -> Result<[f64; 2]> {
    let array = value.as_array().ok_or_else(|| {
        CoreError::InvalidManifest("OSM geometry position must be an array".to_string())
    })?;
    let lon = array
        .first()
        .and_then(Value::as_f64)
        .ok_or_else(|| CoreError::InvalidManifest("OSM longitude must be a number".to_string()))?;
    let lat = array
        .get(1)
        .and_then(Value::as_f64)
        .ok_or_else(|| CoreError::InvalidManifest("OSM latitude must be a number".to_string()))?;
    if !lon.is_finite()
        || !lat.is_finite()
        || !(-180.0..=180.0).contains(&lon)
        || !(-90.0..=90.0).contains(&lat)
    {
        return Err(CoreError::InvalidManifest(
            "OSM position out of WGS84 range".to_string(),
        ));
    }
    Ok([lon, lat])
}

fn id_from(value: Option<&Value>) -> Option<String> {
    let id = match value? {
        Value::String(s) => s.clone(),
        Value::Number(n) => n.to_string(),
        _ => return None,
    };
    if id.trim().is_empty() { None } else { Some(id) }
}

/// Standard GeoJSON puts `id` / `@id` on the Feature; OSM extracts often put
/// `osm_id` in properties. Both are accepted, feature level taking precedence.
fn feature_id(feature: &Value) -> Result<String> {
    for key in ["id", "@id"] {
        if let Some(id) = id_from(feature.get(key)) {
            return Ok(id);
        }
    }
    if let Some(properties) = feature.get("properties").and_then(Value::as_object) {
        for key in ["id", "osm_id", "@id"] {
            if let Some(id) = id_from(properties.get(key)) {
                return Ok(id);
            }
        }
    }
    Err(CoreError::InvalidManifest(
        "OSM feature needs id, osm_id or @id".to_string(),
    ))
}

fn evidence_from_tags(tags: &BTreeMap<String, String>) -> EvidenceKind {
    // A numeric height is an explicit value; `building:levels` is a de facto
    // tag with invalid values in the database and a 3 m/floor renderer guess,
    // so it is derived-with-confidence, never measured.
    if tags
        .get("height")
        .and_then(|s| s.parse::<f64>().ok())
        .is_some_and(|h| h.is_finite())
    {
        EvidenceKind::Measured
    } else if tags.contains_key("building:levels") || tags.contains_key("confidence") {
        EvidenceKind::DerivedWithConfidence
    } else {
        EvidenceKind::InferredByRule
    }
}

/// Shoelace centroid of a closed ring (duplicate closing vertex ignored).
fn ring_centroid(ring: &[[f64; 2]]) -> Result<[f64; 3]> {
    let points = if ring.first() == ring.last() {
        &ring[..ring.len() - 1]
    } else {
        ring
    };
    if points.len() < 3 {
        return Err(CoreError::InvalidManifest(
            "ring has no area for a centroid".to_string(),
        ));
    }
    let mut twice_area = 0.0;
    let mut x = 0.0;
    let mut y = 0.0;
    for i in 0..points.len() {
        let (x0, y0) = (points[i][0], points[i][1]);
        let (x1, y1) = (
            points[(i + 1) % points.len()][0],
            points[(i + 1) % points.len()][1],
        );
        let cross = x0 * y1 - x1 * y0;
        twice_area += cross;
        x += (x0 + x1) * cross;
        y += (y0 + y1) * cross;
    }
    if twice_area.abs() < f64::EPSILON {
        return Err(CoreError::InvalidManifest(
            "ring is degenerate (zero area)".to_string(),
        ));
    }
    let centroid = [x / (3.0 * twice_area), y / (3.0 * twice_area), 0.0];
    if centroid.iter().any(|v| !v.is_finite()) {
        return Err(CoreError::InvalidManifest(
            "centroid is not finite".to_string(),
        ));
    }
    Ok(centroid)
}

/// Length-weighted midpoint along a centreline.
fn line_midpoint(line: &[[f64; 2]]) -> Result<[f64; 3]> {
    if line.len() < 2 {
        return Err(CoreError::InvalidManifest(
            "line needs two positions".to_string(),
        ));
    }
    let mut segments = Vec::with_capacity(line.len() - 1);
    let mut total = 0.0;
    for pair in line.windows(2) {
        let dx = pair[1][0] - pair[0][0];
        let dy = pair[1][1] - pair[0][1];
        let length = (dx * dx + dy * dy).sqrt();
        if !length.is_finite() {
            return Err(CoreError::InvalidManifest(
                "line length is not finite".to_string(),
            ));
        }
        total += length;
        segments.push(length);
    }
    if total <= 0.0 || !total.is_finite() {
        return Err(CoreError::InvalidManifest(
            "line has zero length".to_string(),
        ));
    }
    let mut remaining = total / 2.0;
    for (i, length) in segments.iter().enumerate() {
        if remaining <= *length {
            let t = remaining / length;
            let p = [
                line[i][0] + (line[i + 1][0] - line[i][0]) * t,
                line[i][1] + (line[i + 1][1] - line[i][1]) * t,
                0.0,
            ];
            return if p.iter().all(|v| v.is_finite()) {
                Ok(p)
            } else {
                Err(CoreError::InvalidManifest(
                    "midpoint is not finite".to_string(),
                ))
            };
        }
        remaining -= length;
    }
    Err(CoreError::InvalidManifest(
        "midpoint could not be computed".to_string(),
    ))
}

/// Import a small OSM GeoJSON fixture after gating the source and CRS.
pub fn import_osm_geojson(
    bytes: &[u8],
    source: &SourceDescriptor,
    crs: &CrsMetadata,
) -> Result<Vec<OsmFeature>> {
    check_source(source)?;
    validate_crs(crs)?;
    let root: Value = serde_json::from_slice(bytes)?;
    let features = root
        .get("features")
        .and_then(Value::as_array)
        .ok_or_else(|| {
            CoreError::InvalidManifest("OSM GeoJSON FeatureCollection expected".to_string())
        })?;
    if features.len() > MAX_FEATURES {
        return Err(CoreError::InvalidManifest(format!(
            "OSM fixture exceeds {MAX_FEATURES} features; split into tiles"
        )));
    }

    let mut output = Vec::with_capacity(features.len());
    for feature in features {
        if feature.get("type").and_then(Value::as_str) != Some("Feature") {
            return Err(CoreError::InvalidManifest(
                "expected GeoJSON Feature".to_string(),
            ));
        }
        let properties = feature
            .get("properties")
            .and_then(Value::as_object)
            .ok_or_else(|| {
                CoreError::InvalidManifest("OSM feature properties missing".to_string())
            })?;
        let tags: BTreeMap<String, String> = properties
            .iter()
            .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
            .collect();

        let kind = if tags.contains_key("building") {
            OsmFeatureKind::Building
        } else if tags.contains_key("highway") {
            OsmFeatureKind::Road
        } else {
            continue; // non-target features are ignored, not mislabelled
        };

        let geometry = feature
            .get("geometry")
            .and_then(Value::as_object)
            .ok_or_else(|| {
                CoreError::InvalidManifest("OSM feature geometry missing".to_string())
            })?;
        let geometry_type = geometry.get("type").and_then(Value::as_str).unwrap_or("");
        let expected = match kind {
            OsmFeatureKind::Building => "Polygon",
            OsmFeatureKind::Road => "LineString",
        };
        if geometry_type != expected {
            return Err(CoreError::InvalidManifest(format!(
                "{kind:?} needs GeoJSON {expected}, got {geometry_type}"
            )));
        }
        let raw = geometry
            .get("coordinates")
            .and_then(Value::as_array)
            .ok_or_else(|| {
                CoreError::InvalidManifest("OSM geometry coordinates missing".to_string())
            })?;
        let coordinates: Vec<[f64; 2]> = match kind {
            OsmFeatureKind::Building => {
                // Polygon coordinates are an array of rings; use the outer ring.
                let first = raw.first().and_then(Value::as_array).ok_or_else(|| {
                    CoreError::InvalidManifest("Polygon outer ring missing".to_string())
                })?;
                if first.len() < 4 {
                    return Err(CoreError::InvalidManifest(
                        "building footprint ring needs at least four positions".to_string(),
                    ));
                }
                let ring: Vec<[f64; 2]> = first.iter().map(position).collect::<Result<_>>()?;
                if ring.first() != ring.last() {
                    return Err(CoreError::InvalidManifest(
                        "building footprint ring is not closed".to_string(),
                    ));
                }
                ring
            }
            OsmFeatureKind::Road => {
                let line: Vec<[f64; 2]> = raw.iter().map(position).collect::<Result<_>>()?;
                if line.len() < 2 {
                    return Err(CoreError::InvalidManifest(
                        "road centreline needs at least two positions".to_string(),
                    ));
                }
                line
            }
        };

        let version = properties.get("version").and_then(|v| {
            v.as_u64()
                .or_else(|| v.as_str().and_then(|s| s.parse().ok()))
        });
        let timestamp = properties
            .get("timestamp")
            .and_then(Value::as_str)
            .map(str::to_string);

        output.push(OsmFeature {
            source_feature_id: feature_id(feature)?,
            kind,
            coordinates,
            version,
            timestamp,
            tags: tags.clone(),
            evidence: evidence_from_tags(&tags),
        });
    }
    Ok(output)
}

/// Convert imported features into World IR objects with provenance intact.
pub fn to_world_objects(
    features: &[OsmFeature],
    source: &SourceDescriptor,
) -> Result<Vec<WorldObject>> {
    // Never let a hand-built feature list bypass the lawful-source gate.
    check_source(source)?;
    let base = Provenance::from(source);
    let mut objects = Vec::with_capacity(features.len());
    for feature in features {
        let mut provenance = base.clone();
        provenance.source_feature_ids = vec![feature.source_feature_id.clone()];
        provenance.version = feature
            .version
            .map(|v| v.to_string())
            .or(base.version.clone());
        provenance.vintage = feature.timestamp.clone().or(base.vintage.clone());
        provenance.derivation_chain.push(format!(
            "osm:{}:{}",
            match feature.kind {
                OsmFeatureKind::Building => "building",
                OsmFeatureKind::Road => "road",
            },
            feature.source_feature_id
        ));
        for (key, value) in &feature.tags {
            provenance
                .derivation_chain
                .push(format!("tag:{key}={value}"));
        }

        let geographic_position = match feature.kind {
            OsmFeatureKind::Building => ring_centroid(&feature.coordinates)?,
            OsmFeatureKind::Road => line_midpoint(&feature.coordinates)?,
        };

        objects.push(WorldObject {
            id: derive_id(
                "object",
                &format!("{}:{}", source.id, feature.source_feature_id),
            ),
            object_type: match feature.kind {
                OsmFeatureKind::Building => "building".to_string(),
                OsmFeatureKind::Road => "road".to_string(),
            },
            geographic_position,
            local_position: [0.0, 0.0, 0.0],
            evidence: feature.evidence,
            provenance,
            dimensions: Default::default(),
            generation_seed: None,
            dependencies: Vec::new(),
            validation_state: Default::default(),
            payloads: Vec::new(),
        });
    }
    Ok(objects)
}
