use genesis_core::{
    CoreResult, DemGrid, EvidenceKind, LicenceElection, SourceDescriptor, check_source,
    generate_markings, generate_sidewalks, generate_terrain, generate_vegetation,
};

const DEM: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../fixtures/ikeja-dem.json");
const MARKINGS: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/ikeja-markings.geojson"
);
const VEGETATION: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/ikeja-vegetation.geojson"
);
const ROADS: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/ikeja-small.geojson"
);

fn source() -> SourceDescriptor {
    SourceDescriptor {
        id: "local:synthetic:layers".to_string(),
        publisher: "Genesis demo".to_string(),
        url: "local-fixture".to_string(),
        version: "2026-11-27".to_string(),
        licence: LicenceElection::Other("demo-synthetic".to_string()),
        attribution: "Synthetic demonstration fixture".to_string(),
        ..Default::default()
    }
}

#[test]
fn terrain_records_resolution_and_does_not_claim_upsampling() -> CoreResult<()> {
    let grid = DemGrid::from_bytes(&std::fs::read(DEM)?)?;
    let mesh = generate_terrain(&grid, &source())?;
    assert_eq!(mesh.vertices.len(), 9);
    assert_eq!(mesh.indices.len(), 24);
    assert!(mesh.parameters["resolution_claim"].contains("no upsampling"));
    assert_eq!(mesh.parameters["source_resolution_m"], "30");
    assert_eq!(mesh.parameters["vertical_accuracy_m"], "12");
    assert_eq!(mesh.evidence, EvidenceKind::Measured);
    assert_eq!(
        mesh.provenance.attribution,
        "Synthetic demonstration fixture"
    );
    Ok(())
}

#[test]
fn invalid_dem_is_rejected() {
    let mut bad: serde_json::Value = serde_json::from_slice(&std::fs::read(DEM).unwrap()).unwrap();
    bad["source_resolution_m"] = serde_json::json!(0);
    assert!(DemGrid::from_bytes(&serde_json::to_vec(&bad).unwrap()).is_err());
    bad["source_resolution_m"] = serde_json::json!(30);
    bad["elevations_m"][0][0] = serde_json::json!(f64::NAN);
    assert!(DemGrid::from_bytes(&serde_json::to_vec(&bad).unwrap()).is_err());
}

#[test]
fn secondary_layers_have_separate_fixtures_and_evidence() -> CoreResult<()> {
    let markings = generate_markings(&std::fs::read(MARKINGS)?, &source())?;
    assert_eq!(markings.len(), 1);
    assert_eq!(markings[0].evidence, EvidenceKind::InferredByRule);
    assert_eq!(markings[0].parameters["width_m"], "0.3");

    let sidewalks = generate_sidewalks(&std::fs::read(ROADS)?, &source())?;
    assert_eq!(
        sidewalks.len(),
        2,
        "one sidewalk strip per side of the road"
    );
    assert!(
        sidewalks
            .iter()
            .all(|m| m.evidence == EvidenceKind::InferredByRule)
    );

    let vegetation = generate_vegetation(&std::fs::read(VEGETATION)?, &source())?;
    assert_eq!(vegetation.vertices.len(), 4);
    assert_eq!(vegetation.indices.len(), 6);
    assert_eq!(vegetation.evidence, EvidenceKind::InferredByRule);
    Ok(())
}

#[test]
fn layer_generators_apply_source_gate() {
    let mut denied = source();
    denied.publisher = "Cesium ion".to_string();
    assert!(check_source(&denied).is_err());
    assert!(generate_markings(&std::fs::read(MARKINGS).unwrap(), &denied).is_err());
    assert!(generate_sidewalks(&std::fs::read(ROADS).unwrap(), &denied).is_err());
    assert!(generate_vegetation(&std::fs::read(VEGETATION).unwrap(), &denied).is_err());
    let grid = DemGrid::from_bytes(&std::fs::read(DEM).unwrap()).unwrap();
    assert!(generate_terrain(&grid, &denied).is_err());
}
