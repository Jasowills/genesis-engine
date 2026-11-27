//! Secondary environment layers for the first slice (GE-0028).
//!
//! Terrain, road markings, sidewalks/curbs and basic vegetation are separate
//! generators with separate fixtures and provenance. No bridge/tunnel/facade
//! or hydrology modelling here.

use crate::{
    CoreError, GeneratedGeometry, GeneratedKind, Result, SourceDescriptor, check_source, derive_id,
    verify_mesh, world_ir::EvidenceKind,
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DemGrid {
    pub name: String,
    pub crs: String,
    pub origin_lon_lat: [f64; 2],
    pub cell_size_degrees: f64,
    pub source_resolution_m: f64,
    pub vertical_accuracy_m: f64,
    pub vertical_datum: String,
    pub attribution: String,
    pub elevations_m: Vec<Vec<f64>>,
}

impl DemGrid {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let grid: Self = serde_json::from_slice(bytes)?;
        if grid.origin_lon_lat[0] < -180.0
            || grid.origin_lon_lat[0] > 180.0
            || grid.origin_lon_lat[1] < -90.0
            || grid.origin_lon_lat[1] > 90.0
            || !grid.cell_size_degrees.is_finite()
            || grid.cell_size_degrees <= 0.0
            || grid.source_resolution_m <= 0.0
            || grid.vertical_accuracy_m <= 0.0
            || grid.elevations_m.len() < 2
            || grid
                .elevations_m
                .iter()
                .any(|row| row.len() != grid.elevations_m[0].len() || row.len() < 2)
            || grid.elevations_m.iter().flatten().any(|v| !v.is_finite())
        {
            return Err(CoreError::InvalidManifest("invalid DEM grid".to_string()));
        }
        Ok(grid)
    }
}

fn provenance(source: &SourceDescriptor) -> crate::Provenance {
    let mut p = crate::Provenance::from(source);
    p.derivation_chain.push("layer:procedural".to_string());
    p
}

fn mesh(
    kind: GeneratedKind,
    key: &str,
    vertices: Vec<[f64; 3]>,
    indices: Vec<u32>,
    evidence: EvidenceKind,
    source: &SourceDescriptor,
    parameters: std::collections::BTreeMap<String, String>,
) -> Result<GeneratedGeometry> {
    let geometry = GeneratedGeometry {
        asset_id: derive_id("mesh", key),
        kind,
        vertices,
        indices,
        evidence,
        generation_seed: 0,
        generator_version: "layers-v0.1".to_string(),
        origin_lon_lat: [0.0, 0.0],
        source_object_ids: Vec::new(),
        provenance: provenance(source),
        parameters,
    };
    verify_mesh(&geometry)?;
    Ok(geometry)
}

/// Convert a lon/lat grid to a local-metre heightfield. Resolution and
/// vertical accuracy are recorded as parameters; no upsampling claim is made.
pub fn generate_terrain(grid: &DemGrid, source: &SourceDescriptor) -> Result<GeneratedGeometry> {
    check_source(source)?;
    let rows = grid.elevations_m.len();
    let cols = grid.elevations_m[0].len();
    let east_scale = 111_320.0 * grid.origin_lon_lat[1].to_radians().cos();
    let north_scale = 110_540.0;
    let mut vertices = Vec::with_capacity(rows * cols);
    for (row, values) in grid.elevations_m.iter().enumerate() {
        for (col, height) in values.iter().enumerate() {
            vertices.push([
                col as f64 * grid.cell_size_degrees * east_scale,
                row as f64 * grid.cell_size_degrees * north_scale,
                *height,
            ]);
        }
    }
    let mut indices = Vec::with_capacity((rows - 1) * (cols - 1) * 6);
    for row in 0..rows - 1 {
        for col in 0..cols - 1 {
            let a = (row * cols + col) as u32;
            let b = a + 1;
            let c = a + cols as u32;
            let d = c + 1;
            indices.extend_from_slice(&[a, c, b, b, c, d]);
        }
    }
    let mut parameters = std::collections::BTreeMap::new();
    parameters.insert(
        "source_resolution_m".to_string(),
        grid.source_resolution_m.to_string(),
    );
    parameters.insert(
        "vertical_accuracy_m".to_string(),
        grid.vertical_accuracy_m.to_string(),
    );
    parameters.insert("vertical_datum".to_string(), grid.vertical_datum.clone());
    parameters.insert(
        "resolution_claim".to_string(),
        "source-resolution only; no upsampling".to_string(),
    );
    mesh(
        GeneratedKind::Terrain,
        &format!("terrain:{}", grid.name),
        vertices,
        indices,
        EvidenceKind::Measured,
        source,
        parameters,
    )
}

fn line_points(geometry: &Value) -> Result<Vec<[f64; 2]>> {
    let array = geometry
        .get("coordinates")
        .and_then(Value::as_array)
        .ok_or_else(|| CoreError::InvalidManifest("LineString coordinates missing".to_string()))?;
    array
        .iter()
        .map(|p| {
            let a = p
                .as_array()
                .ok_or_else(|| CoreError::InvalidManifest("bad position".to_string()))?;
            let lon = a.first().and_then(Value::as_f64).unwrap_or(f64::NAN);
            let lat = a.get(1).and_then(Value::as_f64).unwrap_or(f64::NAN);
            if !lon.is_finite() || !lat.is_finite() {
                return Err(CoreError::InvalidManifest(
                    "non-finite position".to_string(),
                ));
            }
            Ok([lon, lat])
        })
        .collect()
}

fn strip(
    line: &[[f64; 2]],
    offset: f64,
    half_width: f64,
    origin: [f64; 2],
) -> (Vec<[f64; 3]>, Vec<u32>) {
    let east_scale = 111_320.0 * origin[1].to_radians().cos();
    let north_scale = 110_540.0;
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    for pair in line.windows(2) {
        let a = [
            (pair[0][0] - origin[0]) * east_scale,
            (pair[0][1] - origin[1]) * north_scale,
        ];
        let b = [
            (pair[1][0] - origin[0]) * east_scale,
            (pair[1][1] - origin[1]) * north_scale,
        ];
        let dx = b[0] - a[0];
        let dy = b[1] - a[1];
        let length = (dx * dx + dy * dy).sqrt();
        if length <= 0.0 {
            continue;
        }
        let nx = -dy / length;
        let ny = dx / length;
        let base = vertices.len() as u32;
        vertices.push([
            a[0] + nx * (offset - half_width),
            a[1] + ny * (offset - half_width),
            0.01,
        ]);
        vertices.push([
            a[0] + nx * (offset + half_width),
            a[1] + ny * (offset + half_width),
            0.01,
        ]);
        vertices.push([
            b[0] + nx * (offset + half_width),
            b[1] + ny * (offset + half_width),
            0.01,
        ]);
        vertices.push([
            b[0] + nx * (offset - half_width),
            b[1] + ny * (offset - half_width),
            0.01,
        ]);
        indices.extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
    }
    (vertices, indices)
}

/// Road markings from a documented GeoJSON fixture; geometry is inferred from
/// the centreline unless the source explicitly records observed markings.
pub fn generate_markings(
    bytes: &[u8],
    source: &SourceDescriptor,
) -> Result<Vec<GeneratedGeometry>> {
    check_source(source)?;
    let root: Value = serde_json::from_slice(bytes)?;
    let features = root
        .get("features")
        .and_then(Value::as_array)
        .ok_or_else(|| {
            CoreError::InvalidManifest("marking fixture must be a FeatureCollection".to_string())
        })?;
    let mut out = Vec::new();
    for (index, feature) in features.iter().enumerate() {
        let geometry = feature
            .get("geometry")
            .ok_or_else(|| CoreError::InvalidManifest("missing geometry".to_string()))?;
        if geometry.get("type").and_then(Value::as_str) != Some("LineString") {
            return Err(CoreError::InvalidManifest(
                "marking needs LineString".to_string(),
            ));
        }
        let line = line_points(geometry)?;
        if line.len() < 2 {
            return Err(CoreError::InvalidManifest(
                "marking needs two positions".to_string(),
            ));
        }
        let origin = line[0];
        let (vertices, indices) = strip(&line, 0.0, 0.15, origin);
        let observed = feature
            .get("properties")
            .and_then(|p| p.get("observed"))
            .and_then(Value::as_bool)
            .unwrap_or(false);
        let mut parameters = std::collections::BTreeMap::new();
        parameters.insert("width_m".to_string(), "0.3".to_string());
        out.push(mesh(
            GeneratedKind::RoadSurface,
            &format!(
                "marking:{index}:{}",
                feature.get("id").and_then(Value::as_str).unwrap_or("none")
            ),
            vertices,
            indices,
            if observed {
                EvidenceKind::Measured
            } else {
                EvidenceKind::InferredByRule
            },
            source,
            parameters,
        )?);
    }
    Ok(out)
}

/// Sidewalks and curbs as thin strips offset from road centrelines.
pub fn generate_sidewalks(
    bytes: &[u8],
    source: &SourceDescriptor,
) -> Result<Vec<GeneratedGeometry>> {
    check_source(source)?;
    let root: Value = serde_json::from_slice(bytes)?;
    let features = root
        .get("features")
        .and_then(Value::as_array)
        .ok_or_else(|| {
            CoreError::InvalidManifest("road fixture must be a FeatureCollection".to_string())
        })?;
    let mut out = Vec::new();
    for (index, feature) in features.iter().enumerate() {
        let props = feature.get("properties").and_then(Value::as_object);
        if props.and_then(|p| p.get("highway")).is_none() {
            continue;
        }
        let geometry = feature
            .get("geometry")
            .ok_or_else(|| CoreError::InvalidManifest("missing geometry".to_string()))?;
        if geometry.get("type").and_then(Value::as_str) != Some("LineString") {
            continue;
        }
        let line = line_points(geometry)?;
        if line.len() < 2 {
            continue;
        }
        let origin = line[0];
        for (side, offset) in [("left", -5.5), ("right", 5.5)] {
            let (vertices, indices) = strip(&line, offset, 0.75, origin);
            let mut parameters = std::collections::BTreeMap::new();
            parameters.insert("side".to_string(), side.to_string());
            parameters.insert("offset_m".to_string(), offset.abs().to_string());
            out.push(mesh(
                GeneratedKind::RoadSurface,
                &format!("sidewalk:{index}:{side}"),
                vertices,
                indices,
                EvidenceKind::InferredByRule,
                source,
                parameters,
            )?);
        }
    }
    Ok(out)
}

/// Basic vegetation proxies: one crossed quad per tree point.
pub fn generate_vegetation(bytes: &[u8], source: &SourceDescriptor) -> Result<GeneratedGeometry> {
    check_source(source)?;
    let root: Value = serde_json::from_slice(bytes)?;
    let features = root
        .get("features")
        .and_then(Value::as_array)
        .ok_or_else(|| {
            CoreError::InvalidManifest("vegetation fixture must be a FeatureCollection".to_string())
        })?;
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    let origin = [3.3480_f64, 6.5985_f64];
    let east_scale = 111_320.0 * origin[1].to_radians().cos();
    let north_scale = 110_540.0;
    for feature in features {
        let geometry = feature
            .get("geometry")
            .ok_or_else(|| CoreError::InvalidManifest("missing geometry".to_string()))?;
        if geometry.get("type").and_then(Value::as_str) != Some("Point") {
            return Err(CoreError::InvalidManifest(
                "vegetation needs Point".to_string(),
            ));
        }
        let point = line_points(
            &json!({"coordinates": [geometry.get("coordinates").cloned().ok_or_else(|| CoreError::InvalidManifest("missing point".to_string()))?]}),
        )?;
        let x = (point[0][0] - origin[0]) * east_scale;
        let y = (point[0][1] - origin[1]) * north_scale;
        let base = vertices.len() as u32;
        vertices.extend_from_slice(&[
            [x - 1.0, y, 0.0],
            [x + 1.0, y, 0.0],
            [x + 1.0, y, 5.0],
            [x - 1.0, y, 5.0],
        ]);
        indices.extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
    }
    let mut parameters = std::collections::BTreeMap::new();
    parameters.insert(
        "proxy".to_string(),
        "single crossed quad per tree".to_string(),
    );
    mesh(
        GeneratedKind::Vegetation,
        "vegetation:proxy",
        vertices,
        indices,
        EvidenceKind::InferredByRule,
        source,
        parameters,
    )
}
