use genesis_core::{
    CoreResult, CrsMetadata, EvidenceKind, GeneratedGeometry, GeneratedKind, LicenceElection,
    LocalFrame, Provenance, QualityProfile, SourceDescriptor, ValidationReport, WorldIr,
    WorldLayer, WorldObject, check_export_ready, derive_id, referenced_sources, validate_mesh,
    validate_world,
};

fn mesh(vertices: Vec<[f64; 3]>, indices: Vec<u32>) -> GeneratedGeometry {
    GeneratedGeometry {
        asset_id: derive_id("mesh", "test"),
        kind: GeneratedKind::BuildingVolume,
        vertices,
        indices,
        evidence: EvidenceKind::Measured,
        generation_seed: 42,
        generator_version: "procedural-v0.1".to_string(),
        origin_lon_lat: [3.0, 6.0],
        source_object_ids: vec![derive_id("object", "way/1")],
        provenance: Provenance::from(&SourceDescriptor {
            id: "osm:test".to_string(),
            publisher: "OpenStreetMap contributors".to_string(),
            url: "https://www.openstreetmap.org/copyright".to_string(),
            licence: LicenceElection::Odbl10,
            attribution: "© OpenStreetMap contributors".to_string(),
            ..Default::default()
        }),
        parameters: Default::default(),
    }
}

fn good_mesh() -> GeneratedGeometry {
    mesh(
        vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]],
        vec![0, 1, 2],
    )
}

fn world() -> WorldIr {
    let provenance = Provenance::from(&SourceDescriptor {
        id: "osm:test".to_string(),
        publisher: "OpenStreetMap contributors".to_string(),
        url: "https://www.openstreetmap.org/copyright".to_string(),
        licence: LicenceElection::Odbl10,
        attribution: "© OpenStreetMap contributors".to_string(),
        ..Default::default()
    });
    WorldIr {
        schema_version: 1,
        world_id: derive_id("world", "test"),
        crs: CrsMetadata::default(),
        local_frame: LocalFrame::default(),
        layers: vec![WorldLayer {
            id: derive_id("layer", "test"),
            objects: vec![WorldObject {
                id: derive_id("object", "way/1"),
                object_type: "building".to_string(),
                geographic_position: [3.0, 6.0, 0.0],
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
fn mesh_gates_are_machine_readable_and_detect_defects() -> CoreResult<()> {
    let records = validate_mesh(&good_mesh(), "test-1.0");
    assert!(
        records
            .iter()
            .any(|m| m.name == "zero_area_faces" && m.value == 0.0)
    );
    assert!(
        records
            .iter()
            .any(|m| m.name == "triangle_count" && m.value == 1.0)
    );
    let self_intersections = records
        .iter()
        .find(|m| m.name == "self_intersections")
        .unwrap();
    assert!(
        self_intersections
            .limitations
            .iter()
            .any(|s| s.contains("UNVERIFIED"))
    );
    assert!(
        records
            .iter()
            .all(|m| !m.tool_version.is_empty() && !m.method.is_empty())
    );

    let degenerate = mesh(
        vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [2.0, 0.0, 0.0]],
        vec![0, 1, 2],
    );
    let records = validate_mesh(&degenerate, "test-1.0");
    assert_eq!(
        records
            .iter()
            .find(|m| m.name == "zero_area_faces")
            .unwrap()
            .value,
        1.0
    );

    let non_manifold = mesh(
        vec![
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.5, 0.5, 0.5],
            [0.5, -0.5, 0.5],
        ],
        vec![0, 1, 2, 0, 1, 3, 0, 1, 4],
    );
    let records = validate_mesh(&non_manifold, "test-1.0");
    assert_eq!(
        records
            .iter()
            .find(|m| m.name == "non_manifold_edges")
            .unwrap()
            .value,
        1.0
    );
    Ok(())
}

#[test]
fn no_accuracy_claim_without_checkpoints() -> CoreResult<()> {
    let meshes = vec![good_mesh()];
    let report = validate_world(
        &world(),
        QualityProfile::Preview,
        &meshes,
        false,
        "test-1.0",
    );
    assert_eq!(report.geographic_accuracy_claim, "none");
    assert!(!report.checkpoints_available);
    assert!(report.passed);
    check_export_ready(&report).expect("export gate should pass");

    let with_checkpoints =
        validate_world(&world(), QualityProfile::Preview, &meshes, true, "test-1.0");
    assert!(
        with_checkpoints
            .geographic_accuracy_claim
            .contains("not implemented")
    );
    Ok(())
}

#[test]
fn survey_profile_is_rejected_without_checkpoints() {
    let report = validate_world(
        &world(),
        QualityProfile::SurveyReference,
        &[],
        false,
        "test-1.0",
    );
    assert!(!report.passed);
    assert_eq!(
        check_export_ready(&report),
        Err("survey_reference requires independent checkpoints".to_string())
    );
    let with_checkpoints = validate_world(
        &world(),
        QualityProfile::SurveyReference,
        &[],
        true,
        "test-1.0",
    );
    assert!(check_export_ready(&with_checkpoints).is_ok());
}

#[test]
fn report_round_trips_as_machine_readable_json() -> CoreResult<()> {
    let report = validate_world(
        &world(),
        QualityProfile::GameReady,
        &[good_mesh()],
        false,
        "test-1.0",
    );
    let bytes = serde_json::to_vec_pretty(&report)?;
    let restored: ValidationReport = serde_json::from_slice(&bytes)?;
    assert_eq!(restored, report);
    assert!(
        restored
            .metrics
            .iter()
            .any(|m| m.name == "missing_provenance" && m.value == 0.0)
    );
    Ok(())
}

#[test]
fn incomplete_provenance_is_flagged() {
    let mut w = world();
    w.layers[0].objects[0].provenance.attribution.clear();
    let report = validate_world(&w, QualityProfile::Preview, &[], false, "test-1.0");
    assert!(!report.passed);
    assert!(
        report
            .warnings
            .iter()
            .any(|s| s.contains("incomplete provenance"))
    );
    assert_eq!(
        report
            .metrics
            .iter()
            .find(|m| m.name == "missing_provenance")
            .unwrap()
            .value,
        1.0
    );
}

#[test]
fn unsupported_validation_report_schema_blocks_export() {
    let mut report = validate_world(
        &world(),
        QualityProfile::GameReady,
        &[good_mesh()],
        false,
        "test-1.0",
    );
    assert!(check_export_ready(&report).is_ok());
    report.schema_version = 999;
    let message = check_export_ready(&report).unwrap_err();
    assert!(message.contains("unsupported validation report schema version"));
}

#[test]
fn referenced_sources_collects_distinct_source_ids() {
    let w = world();
    let sources = referenced_sources(&w);
    assert_eq!(sources.len(), 1);
    assert!(sources.contains("osm:test"));
    let mut multi = w.clone();
    let mut second = multi.layers[0].objects[0].clone();
    second.id = derive_id("object", "way/2");
    second.provenance.source_id = "local:second".to_string();
    multi.layers[0].objects.push(second);
    assert_eq!(referenced_sources(&multi).len(), 2);
}
