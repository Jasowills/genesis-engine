use genesis_core::{
    CoreError, CoreResult, CrsMetadata, LicenceElection, SourceDescriptor, SourcePurpose,
    boundary_payload_path, import_geojson, validate_crs,
};

fn source() -> SourceDescriptor {
    SourceDescriptor {
        id: "osm:pbf:ikeja".to_string(),
        publisher: "OpenStreetMap contributors".to_string(),
        url: "https://www.openstreetmap.org/copyright".to_string(),
        version: "2026-11-27".to_string(),
        licence: LicenceElection::Odbl10,
        attribution: "© OpenStreetMap contributors".to_string(),
        purpose: SourcePurpose::ReconstructionInput,
        ..Default::default()
    }
}

fn crs() -> CrsMetadata {
    CrsMetadata {
        wkt2: Some("GEOGCRS[\"WGS 84\"]".to_string()),
        projjson: None,
        epoch: Some(2024.5),
        vertical_datum: Some("EGM2008".to_string()),
        unit: "metre".to_string(),
        up_axis: "Z".to_string(),
    }
}

fn polygon() -> &'static [u8] {
    br#"{
      "type": "FeatureCollection",
      "features": [{
        "type": "Feature",
        "properties": {"name": "Ikeja study area"},
        "geometry": {
          "type": "Polygon",
          "coordinates": [[
            [3.3480, 6.5985], [3.3552, 6.5985],
            [3.3552, 6.6051], [3.3480, 6.6051], [3.3480, 6.5985]
          ]]
        }
      }]
    }"#
}

#[test]
fn valid_boundary_imports_with_bbox_and_source_checksum() -> CoreResult<()> {
    let boundary = import_geojson(polygon(), "Ikeja study area", &source(), &crs())?;
    assert_eq!(boundary.bbox, [3.3480, 6.5985, 3.3552, 6.6051]);
    assert_eq!(boundary.rings.len(), 1);
    assert_eq!(boundary.rings[0].first(), boundary.rings[0].last());
    assert_eq!(boundary.rings[0].len(), 5);
    assert_eq!(boundary.source_id, "osm:pbf:ikeja");
    assert_eq!(boundary.source_checksum.len(), 64);

    let again = import_geojson(polygon(), "Ikeja study area", &source(), &crs())?;
    assert_eq!(boundary, again, "import must be deterministic");
    assert_eq!(
        boundary_payload_path("worlds/ikeja.json")?,
        "worlds/ikeja.json"
    );
    Ok(())
}

#[test]
fn malformed_geometry_is_a_structured_error() {
    let unclosed =
        br#"{"type":"Polygon","coordinates":[[[3.0,6.0],[3.1,6.0],[3.1,6.1],[3.0,6.1]]]}"#;
    assert!(matches!(
        import_geojson(unclosed, "Bad", &source(), &crs()),
        Err(CoreError::InvalidManifest(_))
    ));

    let short = br#"{"type":"Polygon","coordinates":[[[3.0,6.0],[3.1,6.0],[3.0,6.0]]]}"#;
    assert!(matches!(
        import_geojson(short, "Bad", &source(), &crs()),
        Err(CoreError::InvalidManifest(_))
    ));

    let out_of_range =
        br#"{"type":"Polygon","coordinates":[[[181.0,6.0],[3.1,6.0],[3.1,6.1],[181.0,6.0]]]}"#;
    assert!(matches!(
        import_geojson(out_of_range, "Bad", &source(), &crs()),
        Err(CoreError::InvalidManifest(_))
    ));

    let point = br#"{"type":"Point","coordinates":[3.3,6.6]}"#;
    assert!(matches!(
        import_geojson(point, "Bad", &source(), &crs()),
        Err(CoreError::InvalidManifest(_))
    ));
}

#[test]
fn crs_validation_requires_datum_and_reference() -> CoreResult<()> {
    validate_crs(&crs())?;
    let mut missing_ref = crs();
    missing_ref.wkt2 = None;
    missing_ref.projjson = None;
    assert!(matches!(
        validate_crs(&missing_ref),
        Err(CoreError::InvalidManifest(_))
    ));

    let mut missing_datum = crs();
    missing_datum.vertical_datum = None;
    assert!(matches!(
        validate_crs(&missing_datum),
        Err(CoreError::InvalidManifest(_))
    ));

    let mut bad_epoch = crs();
    bad_epoch.epoch = Some(f64::NAN);
    assert!(matches!(
        validate_crs(&bad_epoch),
        Err(CoreError::InvalidManifest(_))
    ));
    Ok(())
}

#[test]
fn denied_source_and_antimeridian_are_rejected() {
    let mut denied = source();
    denied.publisher = "Google Maps Platform".to_string();
    assert!(matches!(
        import_geojson(polygon(), "Bad", &denied, &crs()),
        Err(CoreError::SourceDenied(_))
    ));

    let crossing = br#"{"type":"Polygon","coordinates":[[[179.0,-16.0],[-179.0,-16.0],[-179.0,-15.0],[179.0,-15.0],[179.0,-16.0]]]}"#;
    assert!(matches!(
        import_geojson(crossing, "Crossing", &source(), &crs()),
        Err(CoreError::InvalidManifest(_))
    ));

    assert!(boundary_payload_path("../outside.json").is_err());
}

#[test]
fn crs_mismatch_and_missing_epoch_are_rejected() -> CoreResult<()> {
    validate_crs(&crs())?;
    let mut no_epoch = crs();
    no_epoch.epoch = None;
    assert!(matches!(
        validate_crs(&no_epoch),
        Err(CoreError::InvalidManifest(_))
    ));

    let mut projected = crs();
    projected.wkt2 = Some("PROJCRS[\"Web Mercator / metres\"]".to_string());
    assert!(matches!(
        validate_crs(&projected),
        Err(CoreError::InvalidManifest(_))
    ));
    assert!(matches!(
        import_geojson(polygon(), "Ikeja", &source(), &projected),
        Err(CoreError::InvalidManifest(_))
    ));
    Ok(())
}

#[test]
fn known_answer_checksum_and_id_are_deterministic() -> CoreResult<()> {
    let boundary = import_geojson(polygon(), "Ikeja study area", &source(), &crs())?;
    assert_eq!(
        boundary.source_checksum,
        "ea577545cd60a3afe265e9ab29bc0707d44e9c2ecebc3296499a91d7e9a38399"
    );
    assert_eq!(
        boundary.id.as_str(),
        genesis_core::derive_id(
            "boundary",
            "Ikeja study area:ea577545cd60a3afe265e9ab29bc0707d44e9c2ecebc3296499a91d7e9a38399"
        )
        .as_str()
    );
    Ok(())
}

#[test]
fn additional_invalid_documents_are_rejected() {
    let two_features = br#"{"type":"FeatureCollection","features":[
        {"type":"Feature","geometry":{"type":"Polygon","coordinates":[[[3.0,6.0],[3.1,6.0],[3.1,6.1],[3.0,6.0]]]}},
        {"type":"Feature","geometry":{"type":"Polygon","coordinates":[[[3.0,6.0],[3.1,6.0],[3.1,6.1],[3.0,6.0]]]}}
    ]}"#;
    assert!(matches!(
        import_geojson(two_features, "Bad", &source(), &crs()),
        Err(CoreError::InvalidManifest(_))
    ));

    let no_rings = br#"{"type":"Polygon","coordinates":[]}"#;
    assert!(matches!(
        import_geojson(no_rings, "Bad", &source(), &crs()),
        Err(CoreError::InvalidManifest(_))
    ));

    let degenerate =
        br#"{"type":"Polygon","coordinates":[[[3.3,6.6],[3.3,6.6],[3.3,6.6],[3.3,6.6]]]}"#;
    assert!(matches!(
        import_geojson(degenerate, "Bad", &source(), &crs()),
        Err(CoreError::InvalidManifest(_))
    ));

    let bogus_type = br#"{"type":"Bogus","geometry":{"type":"Polygon","coordinates":[[[3.0,6.0],[3.1,6.0],[3.1,6.1],[3.0,6.0]]]}}"#;
    assert!(matches!(
        import_geojson(bogus_type, "Bad", &source(), &crs()),
        Err(CoreError::InvalidManifest(_))
    ));

    let huge =
        br#"{"type":"Polygon","coordinates":[[[1e999,6.0],[3.1,6.0],[3.1,6.1],[1e999,6.0]]]}"#;
    assert!(matches!(
        import_geojson(huge, "Bad", &source(), &crs()),
        Err(CoreError::Json(_))
    ));

    assert!(matches!(
        import_geojson(polygon(), "  ", &source(), &crs()),
        Err(CoreError::InvalidManifest(_))
    ));
}

#[test]
fn bare_feature_and_holes_are_supported() -> CoreResult<()> {
    let bare = br#"{"type":"Feature","geometry":{"type":"Polygon","coordinates":[[[3.0,6.0],[3.1,6.0],[3.1,6.1],[3.0,6.0]]]}}"#;
    let boundary = import_geojson(bare, "Bare", &source(), &crs())?;
    assert_eq!(boundary.rings.len(), 1);

    let with_hole = br#"{"type":"Polygon","coordinates":[
        [[3.0,6.0],[3.2,6.0],[3.2,6.2],[3.0,6.2],[3.0,6.0]],
        [[3.1,6.1],[3.15,6.1],[3.15,6.15],[3.1,6.1]]
    ]}"#;
    let boundary = import_geojson(with_hole, "Hole", &source(), &crs())?;
    assert_eq!(boundary.rings.len(), 2);
    Ok(())
}

#[test]
fn source_gate_runs_before_geometry_parsing() {
    let mut denied = source();
    denied.licence = LicenceElection::Other("CC BY-NC-SA 4.0".to_string());
    assert!(matches!(
        import_geojson(polygon(), "Bad", &denied, &crs()),
        Err(CoreError::SourceDenied(_))
    ));
    assert!(matches!(
        import_geojson(br#"not json"#, "Bad", &denied, &crs()),
        Err(CoreError::SourceDenied(_))
    ));
}
