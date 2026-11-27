//! Pure, headless-testable projection of Core data into a scene snapshot.
//!
//! `build_scene_snapshot` is the single place where Core types are shaped for
//! the viewport. Everything returned here is data: the UI never derives IDs,
//! re-projects coordinates or applies provenance rules.

use genesis_core::{
    CoreError, EvidenceKind, GeneratedGeometry, GeneratedKind, Provenance, WorldIr, WorldObject,
    verify_mesh,
};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fmt};

/// Bumped only when the snapshot data contract changes incompatibly.
pub const SNAPSHOT_SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SceneNodeKind {
    BuildingVolume,
    RoadSurface,
}

impl SceneNodeKind {
    fn as_str(self) -> &'static str {
        match self {
            Self::BuildingVolume => "building_volume",
            Self::RoadSurface => "road_surface",
        }
    }
}

/// Triangle mesh in Core's local metre frame. No geodetic maths is performed
/// here or in the renderer.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SceneMesh {
    pub vertices: Vec<[f64; 3]>,
    pub indices: Vec<u32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SceneNode {
    /// Stable Core asset ID, carried through unchanged.
    pub id: String,
    /// Display text assembled in Rust, never in React.
    pub label: String,
    pub kind: SceneNodeKind,
    /// Whether Core recorded a source object for this mesh.
    pub selectable: bool,
    /// World IR local position in metres, copied without projection.
    pub translation: [f64; 3],
    /// Geodetic anchor recorded by Core for this mesh's own local frame.
    pub origin_lon_lat: [f64; 2],
    pub mesh: SceneMesh,
    pub evidence: EvidenceKind,
    /// Generator/procedure recorded by Core (display only).
    pub method: String,
    pub provenance: Provenance,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SceneSnapshot {
    pub snapshot_version: u32,
    pub world_id: String,
    pub frame_id: String,
    /// Selection metadata projected by Core; the UI owns interaction state.
    pub selected_node_id: Option<String>,
    pub nodes: Vec<SceneNode>,
}

/// Pure capability classification for graceful GPU degradation. The actual
/// webview probe happens in the frontend; this is the contract it reports into.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityReport {
    pub webgl2: bool,
    pub webgpu: bool,
    pub renderable: bool,
    pub reason: String,
}

/// WebGPU alone is not renderable through the WebGL-based R3F renderer.
pub fn capability_report(webgl2: bool, webgpu: bool) -> CapabilityReport {
    let (renderable, reason) = if webgl2 {
        (true, "webgl2")
    } else if webgpu {
        (false, "webgpu-detected-r3f-requires-webgl2")
    } else {
        (false, "no-webgl2-or-webgpu")
    };
    CapabilityReport {
        webgl2,
        webgpu,
        renderable,
        reason: reason.to_string(),
    }
}

#[derive(Debug)]
pub enum SceneError {
    Core(CoreError),
    UnknownSourceObject { mesh: String, object: String },
}

impl fmt::Display for SceneError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Core(error) => write!(f, "Core error: {error}"),
            Self::UnknownSourceObject { mesh, object } => {
                write!(f, "mesh {mesh} references unknown World IR object {object}")
            }
        }
    }
}

impl std::error::Error for SceneError {}

impl From<CoreError> for SceneError {
    fn from(error: CoreError) -> Self {
        Self::Core(error)
    }
}

impl From<serde_json::Error> for SceneError {
    fn from(error: serde_json::Error) -> Self {
        Self::Core(CoreError::Json(error))
    }
}

pub type Result<T> = std::result::Result<T, SceneError>;

fn node_for_mesh(
    mesh: &GeneratedGeometry,
    objects: &BTreeMap<&str, &WorldObject>,
) -> Result<SceneNode> {
    verify_mesh(mesh)?;
    let mut sources = Vec::with_capacity(mesh.source_object_ids.len());
    for id in &mesh.source_object_ids {
        let object =
            objects
                .get(id.as_str())
                .copied()
                .ok_or_else(|| SceneError::UnknownSourceObject {
                    mesh: mesh.asset_id.to_string(),
                    object: id.to_string(),
                })?;
        sources.push(object);
    }

    let kind = match mesh.kind {
        GeneratedKind::BuildingVolume => SceneNodeKind::BuildingVolume,
        GeneratedKind::RoadSurface => SceneNodeKind::RoadSurface,
    };
    let feature = mesh
        .provenance
        .source_feature_ids
        .first()
        .map_or("unknown-feature", String::as_str);
    let label = format!(
        "{} — {feature}",
        match kind {
            SceneNodeKind::BuildingVolume => "Building",
            SceneNodeKind::RoadSurface => "Road surface",
        }
    );

    Ok(SceneNode {
        id: mesh.asset_id.to_string(),
        label,
        kind,
        selectable: !sources.is_empty(),
        translation: sources
            .first()
            .map_or([0.0; 3], |object| object.local_position),
        origin_lon_lat: mesh.origin_lon_lat,
        mesh: SceneMesh {
            vertices: mesh.vertices.clone(),
            indices: mesh.indices.clone(),
        },
        evidence: mesh.evidence,
        method: format!("{}:{}", mesh.generator_version, kind.as_str()),
        provenance: mesh.provenance.clone(),
    })
}

/// The one pure function that builds a versioned, render-ready snapshot.
pub fn build_scene_snapshot(
    world: &WorldIr,
    meshes: &[GeneratedGeometry],
) -> Result<SceneSnapshot> {
    world.validate()?;
    let objects: BTreeMap<&str, &WorldObject> = world
        .layers
        .iter()
        .flat_map(|layer| layer.objects.iter())
        .map(|object| (object.id.as_str(), object))
        .collect();
    let mut nodes: Vec<SceneNode> = meshes
        .iter()
        .map(|mesh| node_for_mesh(mesh, &objects))
        .collect::<Result<_>>()?;
    // Stable ordering independent of generator output order.
    nodes.sort_by(|a, b| a.id.cmp(&b.id));

    Ok(SceneSnapshot {
        snapshot_version: SNAPSHOT_SCHEMA_VERSION,
        world_id: world.world_id.to_string(),
        frame_id: world.local_frame.frame_id.clone(),
        selected_node_id: None,
        nodes,
    })
}

/// Document seam used by the Tauri command: Core parses and validates both
/// documents; Studio only hands them to the pure projection.
pub fn scene_from_documents(world_bytes: &[u8], meshes_bytes: &[u8]) -> Result<SceneSnapshot> {
    let world = WorldIr::from_bytes(world_bytes)?;
    let meshes: Vec<GeneratedGeometry> =
        serde_json::from_slice(meshes_bytes).map_err(CoreError::from)?;
    build_scene_snapshot(&world, &meshes)
}

/// Pure selection update. An unknown ID clears selection rather than inventing
/// a node; the renderer receives only data that Core can account for.
pub fn with_selection(snapshot: &SceneSnapshot, node_id: Option<&str>) -> SceneSnapshot {
    let mut next = snapshot.clone();
    next.selected_node_id = node_id
        .filter(|id| snapshot.nodes.iter().any(|node| node.id == *id))
        .map(str::to_string);
    next
}

#[cfg(test)]
mod tests {
    use super::*;
    use genesis_core::{
        CURRENT_SCHEMA_VERSION, CrsMetadata, LicenceElection, SourceDescriptor, StableId,
        WorldLayer, derive_id,
    };

    fn source() -> SourceDescriptor {
        SourceDescriptor {
            id: "osm:test".to_string(),
            publisher: "OpenStreetMap contributors".to_string(),
            url: "https://www.openstreetmap.org/copyright".to_string(),
            version: "1".to_string(),
            licence: LicenceElection::Odbl10,
            attribution: "© OpenStreetMap contributors".to_string(),
            ..Default::default()
        }
    }

    fn world_and_mesh() -> (WorldIr, GeneratedGeometry) {
        let object_id = derive_id("object", "osm:test:way/1");
        let mut provenance = Provenance::from(&source());
        provenance.source_feature_ids = vec!["way/1".to_string()];
        let object = WorldObject {
            id: object_id.clone(),
            object_type: "building".to_string(),
            geographic_position: [3.35, 6.6, 0.0],
            local_position: [12.5, -4.0, 0.0],
            evidence: EvidenceKind::Measured,
            provenance: provenance.clone(),
            dimensions: Default::default(),
            generation_seed: None,
            dependencies: Vec::new(),
            validation_state: Default::default(),
            payloads: Vec::new(),
        };
        let world = WorldIr {
            schema_version: CURRENT_SCHEMA_VERSION,
            world_id: derive_id("world", "test"),
            crs: CrsMetadata::default(),
            local_frame: Default::default(),
            layers: vec![WorldLayer {
                id: derive_id("layer", "test"),
                objects: vec![object],
            }],
        };
        let mesh = GeneratedGeometry {
            asset_id: derive_id("mesh", "building:way/1"),
            kind: GeneratedKind::BuildingVolume,
            vertices: vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]],
            indices: vec![0, 1, 2],
            evidence: EvidenceKind::Measured,
            generation_seed: 7,
            generator_version: "procedural-v0.1".to_string(),
            origin_lon_lat: [3.3, 6.6],
            source_object_ids: vec![object_id],
            provenance,
            parameters: BTreeMap::new(),
        };
        (world, mesh)
    }

    #[test]
    fn snapshot_preserves_ids_provenance_evidence_and_presentation_fields() -> Result<()> {
        let (world, mesh) = world_and_mesh();
        let snapshot = build_scene_snapshot(&world, &[mesh.clone()])?;

        assert_eq!(snapshot.snapshot_version, SNAPSHOT_SCHEMA_VERSION);
        assert_eq!(snapshot.world_id, world.world_id.to_string());
        assert_eq!(snapshot.frame_id, world.local_frame.frame_id);
        assert!(snapshot.selected_node_id.is_none());
        assert_eq!(snapshot.nodes.len(), 1);

        let node = &snapshot.nodes[0];
        assert_eq!(node.id, mesh.asset_id.as_str());
        assert_eq!(node.provenance, mesh.provenance);
        assert_eq!(node.evidence, mesh.evidence);
        assert_eq!(node.origin_lon_lat, mesh.origin_lon_lat);
        assert_eq!(node.mesh.vertices, mesh.vertices);
        assert_eq!(node.mesh.indices, mesh.indices);
        assert_eq!(node.translation, [12.5, -4.0, 0.0]);
        assert!(node.selectable);
        assert!(node.label.contains("way/1"));
        assert!(node.method.contains("procedural-v0.1"));
        assert_eq!(node.kind, SceneNodeKind::BuildingVolume);
        assert_eq!(build_scene_snapshot(&world, &[mesh])?, snapshot);
        Ok(())
    }

    #[test]
    fn selection_is_pure_and_rejects_unknown_ids() -> Result<()> {
        let (world, mesh) = world_and_mesh();
        let snapshot = build_scene_snapshot(&world, &[mesh])?;
        let id = snapshot.nodes[0].id.clone();

        let selected = with_selection(&snapshot, Some(&id));
        assert_eq!(selected.selected_node_id.as_deref(), Some(id.as_str()));
        assert!(
            snapshot.selected_node_id.is_none(),
            "input snapshot is unchanged"
        );

        let cleared = with_selection(&selected, Some("id-does-not-exist"));
        assert!(cleared.selected_node_id.is_none());
        Ok(())
    }

    #[test]
    fn meshes_must_reference_known_world_objects() -> Result<()> {
        let (world, mut mesh) = world_and_mesh();
        mesh.source_object_ids = vec![StableId::new("id-missing")?];
        assert!(matches!(
            build_scene_snapshot(&world, &[mesh]),
            Err(SceneError::UnknownSourceObject { .. })
        ));
        Ok(())
    }

    #[test]
    fn document_seam_rejects_bad_input_and_round_trips() -> Result<()> {
        let (world, mesh) = world_and_mesh();
        let world_bytes = world.to_bytes()?;
        let mesh_bytes = serde_json::to_vec(&vec![mesh])?;
        let snapshot = scene_from_documents(&world_bytes, &mesh_bytes)?;
        assert_eq!(snapshot.nodes.len(), 1);

        let encoded = serde_json::to_vec(&snapshot)?;
        let restored: SceneSnapshot = serde_json::from_slice(&encoded)?;
        assert_eq!(restored, snapshot);

        assert!(matches!(
            scene_from_documents(b"not json", &mesh_bytes),
            Err(SceneError::Core(_))
        ));
        Ok(())
    }

    #[test]
    fn capability_report_degrades_without_webgl2() {
        assert_eq!(
            capability_report(true, false),
            CapabilityReport {
                webgl2: true,
                webgpu: false,
                renderable: true,
                reason: "webgl2".to_string(),
            }
        );
        let webgpu_only = capability_report(false, true);
        assert!(!webgpu_only.renderable);
        assert_eq!(webgpu_only.reason, "webgpu-detected-r3f-requires-webgl2");
        let none = capability_report(false, false);
        assert!(!none.renderable);
        assert_eq!(none.reason, "no-webgl2-or-webgpu");
    }
}
