//! Versioned, engine-agnostic World IR (GE-0011, ADR-0002/ADR-0008).
//!
//! Structured metadata lives here; binary geometry, textures and indexes are
//! referenced by project-relative paths in `AssetRef`. No giant inline JSON
//! document and no embedded geometry payloads.

use crate::{CoreError, CrsMetadata, Result, validate_relative_path};
use serde::de::Error as DeError;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, json};
use sha2::{Digest, Sha256};

pub const CURRENT_SCHEMA_VERSION: u32 = 2;
pub const LEGACY_SCHEMA_VERSION: u32 = 1;

/// Stable, content-derived identity. Names alone are not stable across edits.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(transparent)]
pub struct StableId(String);

impl<'de> Deserialize<'de> for StableId {
    fn deserialize<D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> std::result::Result<Self, D::Error> {
        let raw = String::deserialize(deserializer)?;
        Self::new(raw).map_err(D::Error::custom)
    }
}

impl StableId {
    pub fn new(value: impl Into<String>) -> Result<Self> {
        let value = value.into();
        if value.is_empty() || value.len() > 256 || value.chars().any(char::is_control) {
            return Err(CoreError::InvalidManifest("invalid stable id".to_string()));
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for StableId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// Deterministic ID derived from a namespace and a source-stable key.
pub fn derive_id(namespace: &str, key: &str) -> StableId {
    let mut hasher = Sha256::new();
    hasher.update(namespace.as_bytes());
    hasher.update(b"/");
    hasher.update(key.as_bytes());
    StableId(format!("id-{:x}", hasher.finalize()))
}

/// How a value was obtained. Never collapse measured and inferred values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceKind {
    Measured,
    DerivedWithConfidence,
    InferredByRule,
    Unknown,
}

/// Source, licence and derivation record for one object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Provenance {
    pub source_id: String,
    pub licence_elected: String,
    pub attribution: String,
    #[serde(default)]
    pub publisher: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub version: Option<String>,
    #[serde(default)]
    pub licence_url: Option<String>,
    #[serde(default)]
    pub vintage: Option<String>,
    #[serde(default)]
    pub filters: Vec<String>,
    #[serde(default)]
    pub checksum: Option<String>,
    #[serde(default)]
    pub source_feature_ids: Vec<String>,
    #[serde(default)]
    pub derivation_chain: Vec<String>,
    #[serde(default)]
    pub derivative_database_status: Option<String>,
    #[serde(default)]
    pub display_obligations: Vec<String>,
    #[serde(default)]
    pub third_party_terms: Vec<String>,
    #[serde(default)]
    pub recorded_at_unix: Option<u64>,
}

/// Reference to a binary payload stored outside the metadata document.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AssetRef {
    pub asset_id: StableId,
    pub payload_path: String,
    pub content_hash: String,
    pub byte_size: u64,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Dimensions {
    #[serde(default)]
    pub height_m: Option<f64>,
    #[serde(default)]
    pub footprint_area_m2: Option<f64>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationState {
    #[default]
    NotRun,
    Passed,
    Warnings(Vec<String>),
    Failed(Vec<String>),
}

/// Local origin for precision-safe mesh maths (ADR-0008). Kept as f64.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LocalFrame {
    pub frame_id: String,
    pub origin_ecef: [f64; 3],
    /// Geodetic origin of the local metre frame, recorded for ENU conversion.
    #[serde(default)]
    pub origin_lon_lat: [f64; 2],
    #[serde(default)]
    pub origin_altitude_m: f64,
    #[serde(default)]
    pub epoch: Option<f64>,
}

/// WGS84 geodetic (degrees, metres) to ECEF in metres (ADR-0008).
pub fn geodetic_to_ecef(lon: f64, lat: f64, altitude_m: f64) -> [f64; 3] {
    const A: f64 = 6_378_137.0;
    const F: f64 = 1.0 / 298.257_223_563;
    let e2 = F * (2.0 - F);
    let latitude = lat.to_radians();
    let longitude = lon.to_radians();
    let sin_lat = latitude.sin();
    let cos_lat = latitude.cos();
    let normal = A / (1.0 - e2 * sin_lat * sin_lat).sqrt();
    [
        (normal + altitude_m) * cos_lat * longitude.cos(),
        (normal + altitude_m) * cos_lat * longitude.sin(),
        (normal * (1.0 - e2) + altitude_m) * sin_lat,
    ]
}

/// ECEF delta to local east/north/up given the frame's geodetic origin.
pub fn ecef_to_enu(point: [f64; 3], origin_lon: f64, origin_lat: f64) -> [f64; 3] {
    let latitude = origin_lat.to_radians();
    let longitude = origin_lon.to_radians();
    let (sin_lat, cos_lat) = (latitude.sin(), latitude.cos());
    let (sin_lon, cos_lon) = (longitude.sin(), longitude.cos());
    let east = -sin_lon * point[0] + cos_lon * point[1];
    let north = -sin_lat * cos_lon * point[0] - sin_lat * sin_lon * point[1] + cos_lat * point[2];
    let up = cos_lat * cos_lon * point[0] + cos_lat * sin_lon * point[1] + sin_lat * point[2];
    [east, north, up]
}

/// Local ENU metres for a geodetic point relative to a geodetic origin.
pub fn geodetic_to_enu(
    lon: f64,
    lat: f64,
    altitude_m: f64,
    origin_lon: f64,
    origin_lat: f64,
    origin_altitude_m: f64,
) -> [f64; 3] {
    let point = geodetic_to_ecef(lon, lat, altitude_m);
    let origin = geodetic_to_ecef(origin_lon, origin_lat, origin_altitude_m);
    ecef_to_enu(
        [
            point[0] - origin[0],
            point[1] - origin[1],
            point[2] - origin[2],
        ],
        origin_lon,
        origin_lat,
    )
}

impl LocalFrame {
    pub fn from_geodetic(lon: f64, lat: f64, altitude_m: f64, frame_id: &str) -> Self {
        Self {
            frame_id: frame_id.to_string(),
            origin_ecef: geodetic_to_ecef(lon, lat, altitude_m),
            origin_lon_lat: [lon, lat],
            origin_altitude_m: altitude_m,
            epoch: None,
        }
    }
}

impl Default for LocalFrame {
    fn default() -> Self {
        Self {
            frame_id: "default".to_string(),
            origin_ecef: [0.0, 0.0, 0.0],
            origin_lon_lat: [0.0, 0.0],
            origin_altitude_m: 0.0,
            epoch: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorldObject {
    pub id: StableId,
    pub object_type: String,
    /// Authoritative geodetic position: [longitude, latitude, height].
    pub geographic_position: [f64; 3],
    /// Position relative to the local frame origin, in metres.
    pub local_position: [f64; 3],
    pub evidence: EvidenceKind,
    pub provenance: Provenance,
    #[serde(default)]
    pub dimensions: Dimensions,
    #[serde(default)]
    pub generation_seed: Option<u64>,
    #[serde(default)]
    pub dependencies: Vec<StableId>,
    #[serde(default)]
    pub validation_state: ValidationState,
    #[serde(default)]
    pub payloads: Vec<AssetRef>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorldLayer {
    pub id: StableId,
    #[serde(default)]
    pub objects: Vec<WorldObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorldIr {
    pub schema_version: u32,
    pub world_id: StableId,
    pub crs: CrsMetadata,
    #[serde(default)]
    pub local_frame: LocalFrame,
    #[serde(default)]
    pub layers: Vec<WorldLayer>,
}

impl WorldIr {
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        Ok(serde_json::to_vec_pretty(self)?)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        // Check the version on the raw document before deserializing: a legacy
        // document is shape-incompatible and must produce a structured version
        // error, not a JSON shape error.
        let raw: Value = serde_json::from_slice(bytes)?;
        let found = version_of(&raw)?;
        if found != CURRENT_SCHEMA_VERSION {
            return Err(CoreError::UnsupportedSchemaVersion {
                found,
                supported: CURRENT_SCHEMA_VERSION,
            });
        }
        let ir: Self = serde_json::from_slice(bytes)?;
        ir.validate()?;
        Ok(ir)
    }

    /// Check stable IDs, dependencies and payload path containment.
    pub fn validate(&self) -> Result<()> {
        let mut ids = std::collections::BTreeSet::new();
        // StableId::new is also applied during deserialization; re-check here
        // so hand-constructed documents cannot bypass validation.
        StableId::new(self.world_id.as_str())?;
        ids.insert(self.world_id.as_str());
        for layer in &self.layers {
            StableId::new(layer.id.as_str())?;
            if !ids.insert(layer.id.as_str()) {
                return Err(CoreError::InvalidManifest("duplicate layer id".to_string()));
            }
            for object in &layer.objects {
                StableId::new(object.id.as_str())?;
                StableId::new(object.provenance.source_id.clone())?;
                for asset in &object.payloads {
                    StableId::new(asset.asset_id.as_str())?;
                }
                for dependency in &object.dependencies {
                    StableId::new(dependency.as_str())?;
                }
                if !ids.insert(object.id.as_str()) {
                    return Err(CoreError::InvalidManifest(format!(
                        "duplicate object id {}",
                        object.id
                    )));
                }
                for asset in &object.payloads {
                    validate_relative_path(std::path::Path::new(&asset.payload_path))?;
                }
            }
        }
        for layer in &self.layers {
            for object in &layer.objects {
                for dependency in &object.dependencies {
                    if !ids.contains(dependency.as_str()) {
                        return Err(CoreError::InvalidManifest(format!(
                            "unresolved dependency {dependency}"
                        )));
                    }
                }
            }
        }
        if self.local_frame.frame_id.is_empty() {
            return Err(CoreError::InvalidManifest(
                "local frame id is empty".to_string(),
            ));
        }
        Ok(())
    }
}

fn version_of(value: &Value) -> Result<u32> {
    value
        .get("schema_version")
        .and_then(Value::as_u64)
        .and_then(|v| u32::try_from(v).ok())
        .ok_or_else(|| CoreError::InvalidManifest("missing schema_version".to_string()))
}

fn object_maps(layers: &mut Value) -> Vec<&mut Map<String, Value>> {
    let mut out = Vec::new();
    if let Some(array) = layers.as_array_mut() {
        for layer in array.iter_mut() {
            if let Some(objects) = layer.get_mut("objects").and_then(Value::as_array_mut) {
                for object in objects.iter_mut() {
                    if let Some(map) = object.as_object_mut() {
                        out.push(map);
                    }
                }
            }
        }
    }
    out
}

/// Migrate a legacy v1 document to the current schema.
pub fn migrate_forward(bytes: &[u8]) -> Result<Vec<u8>> {
    let mut value: Value = serde_json::from_slice(bytes)?;
    let version = version_of(&value)?;
    if version == CURRENT_SCHEMA_VERSION {
        return Ok(bytes.to_vec());
    }
    if version != LEGACY_SCHEMA_VERSION {
        return Err(CoreError::UnsupportedSchemaVersion {
            found: version,
            supported: CURRENT_SCHEMA_VERSION,
        });
    }
    value["schema_version"] = json!(CURRENT_SCHEMA_VERSION);
    if let Some(layers) = value.get_mut("layers") {
        for object in object_maps(layers) {
            if let Some(height) = object.remove("height_m") {
                let dimensions = object.entry("dimensions").or_insert_with(|| json!({}));
                if !dimensions.is_object() {
                    // An invalid non-object dimensions value cannot carry a
                    // valid height; replace it rather than silently dropping
                    // the top-level height_m we just removed.
                    *dimensions = json!({});
                }
                if let Some(map) = dimensions.as_object_mut()
                    && map.get("height_m").is_none()
                {
                    // Never overwrite an already-migrated nested height.
                    map.insert("height_m".to_string(), height);
                }
            }
        }
    }
    if value.get("local_frame").is_none() {
        value["local_frame"] = json!({
            "frame_id": "default",
            "origin_ecef": [0.0, 0.0, 0.0],
            "epoch": null
        });
    }
    Ok(serde_json::to_vec_pretty(&value)?)
}

/// Migrate a current document back to v1 (for fixture round-trips and rollback).
pub fn migrate_backward(bytes: &[u8]) -> Result<Vec<u8>> {
    let mut value: Value = serde_json::from_slice(bytes)?;
    let version = version_of(&value)?;
    if version == LEGACY_SCHEMA_VERSION {
        return Ok(bytes.to_vec());
    }
    if version != CURRENT_SCHEMA_VERSION {
        return Err(CoreError::UnsupportedSchemaVersion {
            found: version,
            supported: CURRENT_SCHEMA_VERSION,
        });
    }
    value["schema_version"] = json!(LEGACY_SCHEMA_VERSION);
    if let Some(layers) = value.get_mut("layers") {
        for object in object_maps(layers) {
            // Keep dimensions intact so fields such as footprint_area_m2 are
            // not lost; also expose height_m at the top level for v1 readers.
            if let Some(dimensions) = object.get("dimensions")
                && let Some(map) = dimensions.as_object()
                && let Some(height) = map.get("height_m")
            {
                object.insert("height_m".to_string(), height.clone());
            }
        }
    }
    if let Some(root) = value.as_object_mut() {
        root.remove("local_frame");
    }
    Ok(serde_json::to_vec_pretty(&value)?)
}
