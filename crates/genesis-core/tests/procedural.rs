use genesis_core::{
    CoreError, CoreResult, CrsMetadata, EvidenceKind, GeneratedKind, LicenceElection,
    SourceDescriptor, generate_building_volumes, generate_road_surfaces, import_osm_geojson,
    verify_mesh,
};

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

const FIXTURE: &[u8] = br#"{
  "type": "FeatureCollection",
  "features": [
    {
      "type": "Feature",
      "properties": {"osm_id": "way/1", "building": "yes", "height": "12"},
      "geometry": {"type": "Polygon", "coordinates": [[
        [3.3480, 6.5985], [3.3552, 6.5985],
        [3.3552, 6.6051], [3.3480, 6.6051], [3.3480, 6.5985]
      ]]}
    },
    {
      "type": "Feature",
      "properties": {"osm_id": "way/2", "highway": "residential", "lanes": "2"},
      "geometry": {"type": "LineString", "coordinates": [[3.3485, 6.5990], [3.3520, 6.6000], [3.3545, 6.6040]]}
    }
  ]
}"#;

const FOOTPRINT_ONLY: &[u8] = br#"{
  "type": "FeatureCollection",
  "features": [{
    "type": "Feature",
    "properties": {"osm_id": "way/3", "building": "yes"},
    "geometry": {"type": "Polygon", "coordinates": [[
      [3.0, 6.0], [3.2, 6.0], [3.2, 6.2], [3.0, 6.0]
    ]]}
  }]
}"#;

#[test]
fn same_inputs_and_seed_produce_identical_output() -> CoreResult<()> {
    let features = import_osm_geojson(FIXTURE, &source(), &crs())?;
    let a = generate_building_volumes(&features, &source(), 42)?;
    let b = generate_building_volumes(&features, &source(), 42)?;
    assert_eq!(a, b);
    let ra = generate_road_surfaces(&features, &source(), 42, 8.0)?;
    let rb = generate_road_surfaces(&features, &source(), 42, 8.0)?;
    assert_eq!(ra, rb);
    assert_eq!(ra[0].generator_version, "procedural-v0.1");
    Ok(())
}

#[test]
fn unsupported_attributes_are_inferred_and_named_rng_is_recorded() -> CoreResult<()> {
    let measured_features = import_osm_geojson(FIXTURE, &source(), &crs())?;
    let measured = generate_building_volumes(&measured_features, &source(), 7)?;
    assert_eq!(measured[0].evidence, EvidenceKind::Measured);
    assert_eq!(measured[0].parameters["height_source"], "osm:height");
    assert_eq!(measured[0].parameters["height_m"], "12.000");

    let features = import_osm_geojson(FOOTPRINT_ONLY, &source(), &crs())?;
    let inferred = generate_building_volumes(&features, &source(), 7)?;
    assert_eq!(inferred[0].evidence, EvidenceKind::InferredByRule);
    assert_eq!(
        inferred[0].parameters["height_source"],
        "procedural:default"
    );
    assert!(inferred[0].generation_seed == 7);
    assert!(
        inferred[0]
            .provenance
            .derivation_chain
            .iter()
            .any(|s| s.contains("chacha8"))
    );

    // A different seed can change the inferred default, deterministically.
    let other = generate_building_volumes(&features, &source(), 99)?;
    assert_eq!(other, generate_building_volumes(&features, &source(), 99)?);
    Ok(())
}

#[test]
fn generated_assets_carry_provenance_and_valid_meshes() -> CoreResult<()> {
    let features = import_osm_geojson(FIXTURE, &source(), &crs())?;
    let buildings = generate_building_volumes(&features, &source(), 42)?;
    let roads = generate_road_surfaces(&features, &source(), 42, 8.0)?;
    assert_eq!(buildings[0].kind, GeneratedKind::BuildingVolume);
    assert_eq!(roads[0].kind, GeneratedKind::RoadSurface);
    assert_eq!(buildings[0].provenance.licence_elected, "ODbL-1.0");
    assert_eq!(
        buildings[0].provenance.attribution,
        "© OpenStreetMap contributors"
    );
    assert_eq!(
        buildings[0].provenance.source_feature_ids,
        vec!["way/1".to_string()]
    );
    assert_eq!(
        roads[0].provenance.source_feature_ids,
        vec!["way/2".to_string()]
    );

    verify_mesh(&buildings[0])?;
    verify_mesh(&roads[0])?;
    // Four-corner ring: bottom+top rings; two road segments share an endpoint.
    assert_eq!(buildings[0].vertices.len(), 8);
    assert_eq!(roads[0].vertices.len(), 6);
    Ok(())
}

#[test]
fn generators_are_gated_and_reject_invalid_requests() {
    let mut denied = source();
    denied.publisher = "Cesium ion".to_string();
    let features = import_osm_geojson(FIXTURE, &source(), &crs()).unwrap();
    assert!(matches!(
        generate_building_volumes(&features, &denied, 42),
        Err(CoreError::SourceDenied(_))
    ));
    assert!(matches!(
        generate_road_surfaces(&features, &denied, 42, 8.0),
        Err(CoreError::SourceDenied(_))
    ));
    assert!(matches!(
        generate_road_surfaces(&features, &source(), 42, 0.0),
        Err(CoreError::InvalidManifest(_))
    ));
    assert!(
        generate_building_volumes(&[], &source(), 42)
            .unwrap()
            .is_empty()
    );
    assert!(
        generate_road_surfaces(&[], &source(), 42, 8.0)
            .unwrap()
            .is_empty()
    );
}
