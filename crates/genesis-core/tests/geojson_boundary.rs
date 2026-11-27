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
