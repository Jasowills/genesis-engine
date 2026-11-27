//! GE-0016 integration seam: redistributable OSM-style fixture -> Core import
//! and procedural generation -> pure Studio snapshot. No Tauri or WebGL here.

use genesis_core::{
    CURRENT_SCHEMA_VERSION, CrsMetadata, EvidenceKind, GeneratedGeometry, LicenceElection,
    SourceDescriptor, WorldIr, WorldLayer, derive_id, generate_building_volumes,
    generate_road_surfaces, import_osm_geojson, to_world_objects,
};
use genesis_studio::{SceneSnapshot, build_scene_snapshot, scene_from_documents};

const FIXTURE: &[u8] = include_bytes!("../../../fixtures/ikeja-small.geojson");
const SEED: u64 = 7;

fn source() -> SourceDescriptor {
    SourceDescriptor {
        id: "osm:pbf:ikeja".to_string(),
        publisher: "OpenStreetMap contributors".to_string(),
        url: "https://www.openstreetmap.org/copyright".to_string(),
        version: "2026-11-27".to_string(),
        licence: LicenceElection::Odbl10,
        attribution: "© OpenStreetMap contributors".to_string(),
        ..Default::default()
    }
}

fn crs() -> CrsMetadata {
    CrsMetadata {
        wkt2: Some("GEOGCRS[\"WGS 84\"]".to_string()),
        epoch: Some(2024.5),
        vertical_datum: Some("EGM2008".to_string()),
        ..CrsMetadata::default()
    }
}

fn built() -> (WorldIr, Vec<GeneratedGeometry>) {
    let features = import_osm_geojson(FIXTURE, &source(), &crs()).expect("fixture imports");
    assert_eq!(features.len(), 2, "one building and one road");
    let objects = to_world_objects(&features, &source()).expect("objects convert");
    let world = WorldIr {
        schema_version: CURRENT_SCHEMA_VERSION,
        world_id: derive_id("world", "ikeja-small"),
        crs: crs(),
        local_frame: Default::default(),
        layers: vec![WorldLayer {
            id: derive_id("layer", "ikeja-small"),
            objects,
        }],
    };
    let mut meshes = generate_building_volumes(&features, &source(), SEED).expect("building mesh");
    meshes.extend(generate_road_surfaces(&features, &source(), SEED, 6.0).expect("road mesh"));
    (world, meshes)
}

#[test]
fn fixture_drives_snapshot_with_preserved_identity_and_provenance() {
    let (world, meshes) = built();
    let snapshot = build_scene_snapshot(&world, &meshes).expect("snapshot builds");

    assert_eq!(snapshot.world_id, world.world_id.as_str());
    assert_eq!(snapshot.nodes.len(), 2);
    assert!(snapshot.selected_node_id.is_none());

    let ids: Vec<&str> = snapshot.nodes.iter().map(|node| node.id.as_str()).collect();
    let mesh_ids: Vec<&str> = meshes.iter().map(|mesh| mesh.asset_id.as_str()).collect();
    assert_eq!(ids, mesh_ids, "stable Core mesh IDs are carried unchanged");
    assert!(
        ids.windows(2).all(|pair| pair[0] < pair[1]),
        "deterministic ordering"
    );

    for node in &snapshot.nodes {
        assert!(node.selectable);
        assert!(node.translation.iter().all(|value| value.is_finite()));
        assert!(
            node.mesh
                .vertices
                .iter()
                .all(|v| v.iter().all(|c| c.is_finite()))
        );
        assert!(
            node.mesh
                .indices
                .iter()
                .all(|index| (*index as usize) < node.mesh.vertices.len())
        );
        assert_eq!(node.provenance.source_id, "osm:pbf:ikeja");
        assert_eq!(node.provenance.licence_elected, "ODbL-1.0");
        assert_eq!(node.provenance.attribution, "© OpenStreetMap contributors");
        assert!(
            node.provenance
                .derivation_chain
                .iter()
                .any(|step| step.contains("procedural-v0.1"))
        );
    }

    let building = snapshot
        .nodes
        .iter()
        .find(|node| node.label.contains("way/101"))
        .expect("building node");
    let road = snapshot
        .nodes
        .iter()
        .find(|node| node.label.contains("way/102"))
        .expect("road node");
    assert_eq!(
        building.evidence,
        EvidenceKind::Measured,
        "explicit height tag"
    );
    assert_eq!(road.evidence, EvidenceKind::InferredByRule);
    assert_eq!(
        building.provenance.source_feature_ids,
        vec!["way/101".to_string()]
    );
    assert_eq!(
        road.provenance.source_feature_ids,
        vec!["way/102".to_string()]
    );
}

#[test]
fn snapshot_survives_versioned_document_round_trip() {
    let (world, meshes) = built();
    let world_bytes = world.to_bytes().expect("world serialises");
    let meshes_bytes = serde_json::to_vec(&meshes).expect("meshes serialise");

    let snapshot: SceneSnapshot = scene_from_documents(&world_bytes, &meshes_bytes).expect("seam");
    let encoded = serde_json::to_vec_pretty(&snapshot).expect("snapshot serialises");
    let restored: SceneSnapshot = serde_json::from_slice(&encoded).expect("snapshot deserialises");
    assert_eq!(restored, snapshot);
}
