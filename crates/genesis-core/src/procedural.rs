//! Deterministic procedural generation for the first vertical slice (GE-0015).
//!
//! Converts imported OSM centrelines/footprints into simple local-frame mesh
//! geometry. No LOD, collision, facade, bridge or sidewalk generation yet.

use crate::{
    CoreError, Result, SourceDescriptor, check_source,
    osm_ingest::{OsmFeature, OsmFeatureKind},
    world_ir::{EvidenceKind, Provenance, StableId},
};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};

pub const GENERATOR_VERSION: &str = "procedural-v0.1";
const INFERRED_HEIGHTS_M: [f64; 3] = [6.0, 9.0, 12.0];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GeneratedKind {
    RoadSurface,
    BuildingVolume,
    Terrain,
    Vegetation,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeneratedGeometry {
    pub asset_id: StableId,
    pub kind: GeneratedKind,
    /// Local ENU-ish coordinates in metres (x east, y north, z up).
    pub vertices: Vec<[f64; 3]>,
    pub indices: Vec<u32>,
    pub evidence: EvidenceKind,
    pub generation_seed: u64,
    pub generator_version: String,
    /// Geodetic origin of the local metre frame (longitude, latitude).
    pub origin_lon_lat: [f64; 2],
    /// World IR object IDs this geometry was generated from (GE-0014).
    pub source_object_ids: Vec<StableId>,
    pub provenance: Provenance,
    pub parameters: BTreeMap<String, String>,
}

/// Named, pinned RNG: ChaCha8 with a seed derived from the recorded run seed
/// and the object's stable key (never `StdRng`/`SmallRng`, which are not
/// portable across releases).
fn object_rng(seed: u64, key: &str) -> ChaCha8Rng {
    let digest = Sha256::digest(format!("chacha8/{seed}/{key}"));
    let mut bytes = [0u8; 8];
    bytes.copy_from_slice(&digest[..8]);
    ChaCha8Rng::seed_from_u64(u64::from_le_bytes(bytes))
}

/// Deterministic frame origin: lexicographically smallest coordinate across
/// all features, independent of input ordering or ring winding.
fn frame_origin(features: &[&OsmFeature]) -> [f64; 2] {
    features
        .iter()
        .flat_map(|f| f.coordinates.iter())
        .min_by(|a, b| a[0].total_cmp(&b[0]).then_with(|| a[1].total_cmp(&b[1])))
        .copied()
        .unwrap_or([0.0, 0.0])
}

fn to_local(lon: f64, lat: f64, origin_lon: f64, origin_lat: f64) -> [f64; 2] {
    let east = (lon - origin_lon) * 111_320.0 * origin_lat.to_radians().cos();
    let north = (lat - origin_lat) * 110_540.0;
    [east, north]
}

/// Vertex accumulator with rounded-key deduplication. Within one way, adjacent
/// segments share mitered left/right vertices; joins between separate ways are
/// intentionally deferred (documented limitation).
#[derive(Default)]
struct MeshBuilder {
    keys: BTreeMap<String, u32>,
    vertices: Vec<[f64; 3]>,
}

impl MeshBuilder {
    fn vertex(&mut self, point: [f64; 3]) -> Result<u32> {
        if point.iter().any(|v| !v.is_finite()) {
            return Err(CoreError::InvalidManifest(
                "non-finite generated vertex".to_string(),
            ));
        }
        let key = format!("{:.3},{:.3},{:.3}", point[0], point[1], point[2]);
        if let Some(index) = self.keys.get(&key) {
            return Ok(*index);
        }
        let index = u32::try_from(self.vertices.len())
            .map_err(|_| CoreError::InvalidManifest("mesh exceeds u32 index range".to_string()))?;
        self.keys.insert(key, index);
        self.vertices.push(point);
        Ok(index)
    }
}

/// Stable World IR object ID from GE-0014 for the source feature.
fn source_object_id(source: &SourceDescriptor, feature: &OsmFeature) -> StableId {
    crate::derive_id(
        "object",
        &format!("{}:{}", source.id, feature.source_feature_id),
    )
}

fn provenance_for(source: &SourceDescriptor, feature: &OsmFeature, seed: u64) -> Provenance {
    let mut provenance = Provenance::from(source);
    provenance.source_feature_ids = vec![feature.source_feature_id.clone()];
    provenance.version = feature
        .version
        .map(|v| v.to_string())
        .or(provenance.version);
    provenance.vintage = feature.timestamp.clone().or(provenance.vintage);
    provenance
        .derivation_chain
        .push(format!("{GENERATOR_VERSION}:seed={seed}:chacha8"));
    provenance
}

fn height_for(feature: &OsmFeature, seed: u64) -> (f64, EvidenceKind, String) {
    if let Some(height) = feature
        .tags
        .get("height")
        .and_then(|v| v.parse::<f64>().ok())
        && height.is_finite()
        && height > 0.0
    {
        return (height, EvidenceKind::Measured, "osm:height".to_string());
    }
    if let Some(levels) = feature
        .tags
        .get("building:levels")
        .and_then(|v| v.parse::<f64>().ok())
        && levels.is_finite()
        && levels > 0.0
    {
        return (
            levels * 3.0,
            EvidenceKind::DerivedWithConfidence,
            "osm:levels*3m".to_string(),
        );
    }
    let mut rng = object_rng(seed, &feature.source_feature_id);
    let height = INFERRED_HEIGHTS_M[rng.random_range(0..INFERRED_HEIGHTS_M.len())];
    (
        height,
        EvidenceKind::InferredByRule,
        "procedural:default".to_string(),
    )
}

/// Check that a generated mesh has finite vertices and in-range triangle indices.
pub fn verify_mesh(mesh: &GeneratedGeometry) -> Result<()> {
    if mesh.vertices.is_empty() || mesh.indices.is_empty() || !mesh.indices.len().is_multiple_of(3)
    {
        return Err(CoreError::InvalidManifest(
            "mesh has no triangles".to_string(),
        ));
    }
    if mesh
        .vertices
        .iter()
        .any(|v| v.iter().any(|c| !c.is_finite()))
    {
        return Err(CoreError::InvalidManifest(
            "mesh has non-finite vertices".to_string(),
        ));
    }
    for i in (0..mesh.indices.len()).step_by(3) {
        let triangle = &mesh.indices[i..i + 3];
        if triangle[0] == triangle[1] || triangle[1] == triangle[2] || triangle[0] == triangle[2] {
            return Err(CoreError::InvalidManifest(
                "mesh contains a degenerate triangle".to_string(),
            ));
        }
        if triangle
            .iter()
            .any(|index| *index as usize >= mesh.vertices.len())
        {
            return Err(CoreError::InvalidManifest(
                "mesh index out of range".to_string(),
            ));
        }
    }
    Ok(())
}

/// Extrude each building footprint into a simple prism.
pub fn generate_building_volumes(
    features: &[OsmFeature],
    source: &SourceDescriptor,
    seed: u64,
) -> Result<Vec<GeneratedGeometry>> {
    check_source(source)?;
    let mut buildings: Vec<&OsmFeature> = features
        .iter()
        .filter(|f| f.kind == OsmFeatureKind::Building)
        .collect();
    // Stable frame origin regardless of input ordering.
    buildings.sort_by(|a, b| a.source_feature_id.cmp(&b.source_feature_id));
    if buildings.is_empty() {
        return Ok(Vec::new());
    }
    let [origin_lon, origin_lat] = frame_origin(&buildings);
    let mut output = Vec::with_capacity(buildings.len());
    for feature in buildings {
        let (height, evidence, height_source) = height_for(feature, seed);
        let local: Vec<[f64; 2]> = feature
            .coordinates
            .iter()
            .map(|p| to_local(p[0], p[1], origin_lon, origin_lat))
            .collect();
        let unwrapped = if local.first() == local.last() {
            &local[..local.len() - 1]
        } else {
            &local[..]
        };
        // Normalise orientation: OSM ways have no winding guarantee, and a
        // clockwise ring would produce inward walls and a downward roof cap.
        let mut area2 = 0.0;
        for i in 0..unwrapped.len() {
            let a = unwrapped[i];
            let b = unwrapped[(i + 1) % unwrapped.len()];
            area2 += a[0] * b[1] - b[0] * a[1];
        }
        let mut ring = unwrapped.to_vec();
        if area2 < 0.0 {
            ring.reverse();
        }
        // Canonical start vertex makes winding-normalised output byte-identical
        // regardless of the input ring's starting point or direction.
        if let Some(min_index) = (0..ring.len()).min_by(|&i, &j| {
            ring[i][0]
                .total_cmp(&ring[j][0])
                .then_with(|| ring[i][1].total_cmp(&ring[j][1]))
        }) {
            ring.rotate_left(min_index);
        }
        if ring.len() < 3 {
            return Err(CoreError::InvalidManifest(
                "building ring has fewer than three vertices".to_string(),
            ));
        }
        let mut mesh = MeshBuilder::default();
        let mut bottom = Vec::with_capacity(ring.len());
        let mut top = Vec::with_capacity(ring.len());
        for point in &ring {
            bottom.push(mesh.vertex([point[0], point[1], 0.0])?);
            top.push(mesh.vertex([point[0], point[1], height])?);
        }
        let mut indices = Vec::with_capacity(ring.len() * 9);
        for i in 0..ring.len() {
            let next = (i + 1) % ring.len();
            indices.extend_from_slice(&[bottom[i], bottom[next], top[next]]);
            indices.extend_from_slice(&[bottom[i], top[next], top[i]]);
        }
        for i in 1..ring.len() - 1 {
            indices.extend_from_slice(&[top[0], top[i], top[i + 1]]);
        }
        let mut parameters = BTreeMap::new();
        parameters.insert("height_m".to_string(), format!("{height:.3}"));
        parameters.insert("height_source".to_string(), height_source);
        output.push(GeneratedGeometry {
            asset_id: crate::derive_id(
                "mesh",
                &format!(
                    "building:{}:{seed}:{GENERATOR_VERSION}",
                    feature.source_feature_id
                ),
            ),
            kind: GeneratedKind::BuildingVolume,
            vertices: mesh.vertices,
            indices,
            evidence,
            generation_seed: seed,
            generator_version: GENERATOR_VERSION.to_string(),
            origin_lon_lat: [origin_lon, origin_lat],
            source_object_ids: vec![source_object_id(source, feature)],
            provenance: provenance_for(source, feature, seed),
            parameters,
        });
    }
    Ok(output)
}

/// Build road surfaces from centrelines, deduplicating shared endpoints.
pub fn generate_road_surfaces(
    features: &[OsmFeature],
    source: &SourceDescriptor,
    seed: u64,
    width_m: f64,
) -> Result<Vec<GeneratedGeometry>> {
    check_source(source)?;
    if !width_m.is_finite() || width_m <= 0.0 {
        return Err(CoreError::InvalidManifest(
            "road width must be finite and positive".to_string(),
        ));
    }
    let width = width_m;
    let mut roads: Vec<&OsmFeature> = features
        .iter()
        .filter(|f| f.kind == OsmFeatureKind::Road)
        .collect();
    roads.sort_by(|a, b| a.source_feature_id.cmp(&b.source_feature_id));
    if roads.is_empty() {
        return Ok(Vec::new());
    }
    let [origin_lon, origin_lat] = frame_origin(&roads);
    let mut mesh = MeshBuilder::default();
    let mut indices = Vec::new();
    for feature in &roads {
        let local: Vec<[f64; 2]> = feature
            .coordinates
            .iter()
            .map(|p| to_local(p[0], p[1], origin_lon, origin_lat))
            .collect();
        if local.len() < 2 {
            continue;
        }
        // Per-vertex averaged (miter) normals: adjacent segments share the
        // same left/right vertices, so bends do not leave gaps.
        let mut normals: Vec<[f64; 2]> = Vec::with_capacity(local.len());
        let mut segment_normals: Vec<[f64; 2]> = Vec::with_capacity(local.len() - 1);
        for pair in local.windows(2) {
            let dx = pair[1][0] - pair[0][0];
            let dy = pair[1][1] - pair[0][1];
            let length = (dx * dx + dy * dy).sqrt();
            if !length.is_finite() || length <= 0.0 {
                segment_normals.push([0.0, 0.0]);
                continue;
            }
            segment_normals.push([-dy / length, dx / length]);
        }
        for i in 0..local.len() {
            let n = match (i.checked_sub(1), segment_normals.get(i)) {
                (None, Some(next)) => *next,
                (Some(prev), None) => segment_normals[prev],
                (Some(prev), Some(next)) => {
                    let sum = [
                        segment_normals[prev][0] + next[0],
                        segment_normals[prev][1] + next[1],
                    ];
                    let magnitude = (sum[0] * sum[0] + sum[1] * sum[1]).sqrt();
                    if magnitude > f64::EPSILON {
                        [sum[0] / magnitude, sum[1] / magnitude]
                    } else {
                        *next
                    }
                }
                (None, None) => continue,
            };
            normals.push(n);
        }
        if normals.len() != local.len() {
            continue;
        }
        let mut left = Vec::with_capacity(local.len());
        let mut right = Vec::with_capacity(local.len());
        for (point, normal) in local.iter().zip(normals.iter()) {
            let offset = [normal[0] * width / 2.0, normal[1] * width / 2.0];
            left.push(mesh.vertex([point[0] + offset[0], point[1] + offset[1], 0.0])?);
            right.push(mesh.vertex([point[0] - offset[0], point[1] - offset[1], 0.0])?);
        }
        for i in 0..local.len() - 1 {
            indices.extend_from_slice(&[left[i], right[i], right[i + 1]]);
            indices.extend_from_slice(&[left[i], right[i + 1], left[i + 1]]);
        }
    }
    if indices.is_empty() {
        return Err(CoreError::InvalidManifest(
            "no road surface geometry generated".to_string(),
        ));
    }
    let mut parameters = BTreeMap::new();
    parameters.insert("width_m".to_string(), format!("{width:.3}"));
    parameters.insert(
        "junction".to_string(),
        "per-feature-miter;cross-feature-junction-deferred".to_string(),
    );
    let mut provenance = Provenance::from(source);
    provenance
        .derivation_chain
        .push(format!("{GENERATOR_VERSION}:seed={seed}:road-surface"));
    provenance.source_feature_ids = roads
        .iter()
        .map(|f| f.source_feature_id.clone())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    Ok(vec![GeneratedGeometry {
        asset_id: crate::derive_id(
            "mesh",
            &format!("road:{seed}:{width:.3}:{GENERATOR_VERSION}"),
        ),
        kind: GeneratedKind::RoadSurface,
        vertices: mesh.vertices,
        indices,
        evidence: EvidenceKind::InferredByRule,
        generation_seed: seed,
        generator_version: GENERATOR_VERSION.to_string(),
        origin_lon_lat: [origin_lon, origin_lat],
        source_object_ids: roads.iter().map(|f| source_object_id(source, f)).collect(),
        provenance,
        parameters,
    }])
}
