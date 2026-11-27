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
const SIDEWALKS: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/ikeja-sidewalks.geojson"
);

fn osm_source() -> SourceDescriptor {
    SourceDescriptor {
        id: "osm:fixture:ikeja".to_string(),
        publisher: "OpenStreetMap contributors".to_string(),
        url: "https://www.openstreetmap.org/copyright".to_string(),
        version: "2026-11-27".to_string(),
        licence: LicenceElection::Odbl10,
        attribution: "© OpenStreetMap contributors".to_string(),
        ..Default::default()
    }
}

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
    assert_eq!(
        mesh.evidence,
        EvidenceKind::InferredByRule,
        "synthetic DEM must not be labelled measured"
    );
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
    bad["origin_lon_lat"][0] = serde_json::json!(null);
    assert!(DemGrid::from_bytes(&serde_json::to_vec(&bad).unwrap()).is_err());
}

#[test]
fn secondary_layers_have_separate_fixtures_and_evidence() -> CoreResult<()> {
    let markings = generate_markings(&std::fs::read(MARKINGS)?, &source())?;
    assert_eq!(markings.len(), 1);
    assert_eq!(markings[0].evidence, EvidenceKind::InferredByRule);
    assert_eq!(markings[0].parameters["width_m"], "0.3");
    assert_eq!(markings[0].origin_lon_lat, [3.3485, 6.5990]);
    assert!(normal_z(&markings[0]) > 0.0, "markings must face +Z");

    let sidewalks = generate_sidewalks(&std::fs::read(SIDEWALKS)?, &osm_source())?;
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
    assert_eq!(vegetation.origin_lon_lat, [3.3490, 6.6000]);
    Ok(())
}

#[test]
fn layer_generators_apply_source_gate() {
    let mut denied = source();
    denied.publisher = "Cesium ion".to_string();
    assert!(check_source(&denied).is_err());
    assert!(generate_markings(&std::fs::read(MARKINGS).unwrap(), &denied).is_err());
    assert!(generate_sidewalks(&std::fs::read(SIDEWALKS).unwrap(), &denied).is_err());
    assert!(generate_vegetation(&std::fs::read(VEGETATION).unwrap(), &denied).is_err());
    let grid = DemGrid::from_bytes(&std::fs::read(DEM).unwrap()).unwrap();
    assert!(generate_terrain(&grid, &denied).is_err());
}

fn normal_z(mesh: &genesis_core::GeneratedGeometry) -> f64 {
    let t = &mesh.indices[0..3];
    let a = mesh.vertices[t[0] as usize];
    let b = mesh.vertices[t[1] as usize];
    let c = mesh.vertices[t[2] as usize];
    (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0])
}

#[test]
fn observed_marking_is_measured_and_sidewalks_keep_osm_attribution() -> CoreResult<()> {
    let observed = br#"{"type":"FeatureCollection","features":[{"type":"Feature","id":"mark/observed","properties":{"observed":true},"geometry":{"type":"LineString","coordinates":[[3.0,6.0],[3.01,6.0]]}}]}"#;
    let marks = generate_markings(observed, &source())?;
    assert_ne!(
        marks[0].evidence,
        EvidenceKind::Measured,
        "a synthetic source cannot make observed=true measured"
    );
    let sidewalks = generate_sidewalks(&std::fs::read(SIDEWALKS)?, &osm_source())?;
    assert!(
        sidewalks
            .iter()
            .all(|m| m.provenance.attribution == "© OpenStreetMap contributors")
    );
    assert!(normal_z(&sidewalks[0]) > 0.0, "sidewalks must face +Z");
    Ok(())
}

fn surveyed_source() -> SourceDescriptor {
    SourceDescriptor {
        id: "survey:ikeja".to_string(),
        publisher: "Licensed survey".to_string(),
        url: "https://example.invalid/survey".to_string(),
        version: "2026-11-27".to_string(),
        licence: LicenceElection::Other("survey-licence".to_string()),
        attribution: " surveyed ground truth with checkpoints ".to_string(),
        ..Default::default()
    }
}

#[test]
fn declared_survey_sources_may_produce_measured_evidence() -> CoreResult<()> {
    let mut grid = DemGrid::from_bytes(&std::fs::read(DEM)?)?;
    grid.attribution = "surveyed ground truth DEM with checkpoints".to_string();
    let terrain = generate_terrain(&grid, &surveyed_source())?;
    assert_eq!(terrain.evidence, EvidenceKind::Measured);
    let observed = br#"{"type":"FeatureCollection","features":[{"type":"Feature","id":"m/1","properties":{"observed":true},"geometry":{"type":"LineString","coordinates":[[3.0,6.0],[3.01,6.0]]}}]}"#;
    let marks = generate_markings(observed, &surveyed_source())?;
    assert_eq!(marks[0].evidence, EvidenceKind::Measured);
    Ok(())
}

#[test]
fn direct_dem_construction_is_validated() -> CoreResult<()> {
    let mut grid = DemGrid::from_bytes(&std::fs::read(DEM)?)?;
    grid.elevations_m.clear();
    let result = generate_terrain(&grid, &source());
    assert!(matches!(
        result,
        Err(genesis_core::CoreError::InvalidManifest(_))
    ));
    Ok(())
}

#[test]
fn explicitly_unmeasured_sources_never_produce_measured_evidence() -> CoreResult<()> {
    let mut src = source();
    src.attribution = "DEM not measured by any survey".to_string();
    let mut grid = DemGrid::from_bytes(&std::fs::read(DEM)?)?;
    grid.attribution = "not measured".to_string();
    let terrain = generate_terrain(&grid, &src)?;
    assert_ne!(terrain.evidence, EvidenceKind::Measured);
    let observed = br#"{"type":"FeatureCollection","features":[{"type":"Feature","id":"m/2","properties":{"observed":true},"geometry":{"type":"LineString","coordinates":[[3.0,6.0],[3.01,6.0]]}}]}"#;
    let marks = generate_markings(observed, &src)?;
    assert_ne!(marks[0].evidence, EvidenceKind::Measured);
    Ok(())
}

#[test]
fn vegetation_keeps_fixture_feature_ids() -> CoreResult<()> {
    let vegetation = generate_vegetation(&std::fs::read(VEGETATION)?, &source())?;
    assert!(
        vegetation
            .provenance
            .source_feature_ids
            .contains(&"tree/1".to_string())
    );
    Ok(())
}
