use genesis_core::{
    CoreError, CoreResult, CrsMetadata, EvidenceKind, GeneratedGeometry, GeneratedKind,
    LicenceElection, LocalFrame, Provenance, QualityProfile, SourceDescriptor, WorldIr, WorldLayer,
    WorldObject, check_export_ready, derive_id, export_glb, format_gate_metric, import_glb,
    validate_world,
};
use serde_json::Value;

fn source() -> SourceDescriptor {
    SourceDescriptor {
        id: "osm:pbf:ikeja".to_string(),
        publisher: "OpenStreetMap contributors".to_string(),
        url: "https://www.openstreetmap.org/copyright".to_string(),
        licence: LicenceElection::Odbl10,
        attribution: "© OpenStreetMap contributors".to_string(),
        ..Default::default()
    }
}

fn mesh() -> GeneratedGeometry {
    GeneratedGeometry {
        asset_id: derive_id("mesh", "test-glb"),
        kind: GeneratedKind::BuildingVolume,
        vertices: vec![[0.0, 0.0, 0.0], [2.0, 0.0, 0.0], [0.0, 3.0, 0.0]],
        indices: vec![0, 1, 2],
        evidence: EvidenceKind::Measured,
        generation_seed: 42,
        generator_version: "procedural-v0.1".to_string(),
        origin_lon_lat: [3.35, 6.60],
        source_object_ids: vec![derive_id("object", "way/1")],
        provenance: Provenance::from(&source()),
        parameters: Default::default(),
    }
}

fn world() -> WorldIr {
    let mut provenance = Provenance::from(&source());
    provenance.source_feature_ids = vec!["way/1".to_string()];
    WorldIr {
        schema_version: 1,
        world_id: derive_id("world", "ikeja"),
        crs: CrsMetadata::default(),
        local_frame: LocalFrame::default(),
        layers: vec![WorldLayer {
            id: derive_id("layer", "buildings"),
            objects: vec![WorldObject {
                id: derive_id("object", "way/1"),
                object_type: "building".to_string(),
                geographic_position: [3.35, 6.60, 0.0],
                local_position: [0.0, 0.0, 0.0],
                evidence: EvidenceKind::Measured,
                provenance,
                dimensions: Default::default(),
                generation_seed: None,
                dependencies: Vec::new(),
                validation_state: Default::default(),
                payloads: Vec::new(),
            }],
        }],
    }
}

#[test]
fn glb_round_trip_preserves_geometry_and_provenance() -> CoreResult<()> {
    let meshes = vec![mesh()];
    let w = world();
    let report = validate_world(&w, QualityProfile::Preview, &meshes, false, "test-1.0");
    check_export_ready(&report).expect("gate should pass");
    let exported = export_glb(&meshes, &w, &[source()], &report)?;
    if let Ok(path) = std::env::var("GENESIS_GLB_FIXTURE") {
        std::fs::write(path, &exported.bytes)?;
    }
    assert_eq!(&exported.bytes[0..4], b"glTF");
    assert_eq!(
        u32::from_le_bytes(exported.bytes[4..8].try_into().unwrap()),
        2
    );

    let restored = import_glb(&exported.bytes)?;
    assert_eq!(restored.version, "2.0");
    assert_eq!(restored.mesh_count, 1);
    assert_eq!(restored.vertex_count, 3);
    assert_eq!(restored.triangle_count, 1);
    assert_eq!(restored.provenance.len(), 1);
    let genesis = &restored.provenance[0];
    assert_eq!(genesis["asset_id"], derive_id("mesh", "test-glb").as_str());
    assert_eq!(
        genesis["provenance"]["attribution"],
        "© OpenStreetMap contributors"
    );
    assert_eq!(genesis["provenance"]["licence_elected"], "ODbL-1.0");
    assert_eq!(genesis["generation_seed"], 42);

    let sidecar: Value = serde_json::from_slice(&exported.attribution_sidecar)?;
    assert_eq!(
        sidecar["records"][0]["attribution"],
        "© OpenStreetMap contributors"
    );
    assert!(exported.warnings.iter().any(|w| w.contains("LOD")));
    Ok(())
}

#[test]
fn export_is_gated_by_lawful_source_and_validation() -> CoreResult<()> {
    let meshes = vec![mesh()];
    let w = world();
    let good = validate_world(&w, QualityProfile::Preview, &meshes, false, "test-1.0");
    let mut denied = source();
    denied.publisher = "Cesium ion".to_string();
    assert!(matches!(
        export_glb(&meshes, &w, &[denied], &good),
        Err(CoreError::SourceDenied(_))
    ));

    let mut bad_mesh = mesh();
    bad_mesh.indices = vec![0, 1, 1];
    let bad_report = validate_world(&w, QualityProfile::Preview, &[bad_mesh], false, "test-1.0");
    assert!(matches!(
        export_glb(&meshes, &w, &[source()], &bad_report),
        Err(CoreError::InvalidManifest(_))
    ));
    Ok(())
}

#[test]
fn malformed_glb_is_rejected_and_format_metric_is_recorded() -> CoreResult<()> {
    assert!(matches!(
        import_glb(b"not a glb"),
        Err(CoreError::InvalidManifest(_))
    ));
    let mut truncated = export_glb(
        &[mesh()],
        &world(),
        &[source()],
        &validate_world(
            &world(),
            QualityProfile::Preview,
            &[mesh()],
            false,
            "test-1.0",
        ),
    )?
    .bytes;
    truncated.pop();
    assert!(matches!(
        import_glb(&truncated),
        Err(CoreError::InvalidManifest(_))
    ));
    let metric = format_gate_metric(0, "gltf-validator 2.x");
    assert_eq!(metric.value, 0.0);
    assert!(metric.limitations.iter().any(|s| s.contains("externally")));
    Ok(())
}
