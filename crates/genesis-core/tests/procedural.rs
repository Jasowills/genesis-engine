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

const CW_FOOTPRINT: &[u8] = br#"{
  "type": "FeatureCollection",
  "features": [{
    "type": "Feature",
    "properties": {"osm_id": "way/1", "building": "yes", "height": "12"},
    "geometry": {"type": "Polygon", "coordinates": [[
      [3.3480, 6.6051], [3.3552, 6.6051],
      [3.3552, 6.5985], [3.3480, 6.5985], [3.3480, 6.6051]
    ]]}
  }]
}"#;

#[test]
fn clockwise_footprints_generate_identical_outward_meshes() -> CoreResult<()> {
    let ccw = import_osm_geojson(FIXTURE, &source(), &crs())?;
    let cw = import_osm_geojson(CW_FOOTPRINT, &source(), &crs())?;
    let a = generate_building_volumes(&ccw, &source(), 7)?;
    let b = generate_building_volumes(&cw, &source(), 7)?;
    assert_eq!(a, b, "ring winding must be normalised before indexing");
    Ok(())
}

#[test]
fn origin_and_identity_are_stable_and_linked_to_world_objects() -> CoreResult<()> {
    let two = br#"{
      "type": "FeatureCollection",
      "features": [
        {"type":"Feature","properties":{"osm_id":"way/b","building":"yes","height":"12"},
         "geometry":{"type":"Polygon","coordinates":[[[3.10,6.10],[3.12,6.10],[3.12,6.12],[3.10,6.12],[3.10,6.10]]]}},
        {"type":"Feature","properties":{"osm_id":"way/a","building":"yes","height":"9"},
         "geometry":{"type":"Polygon","coordinates":[[[3.20,6.20],[3.22,6.20],[3.22,6.22],[3.20,6.22],[3.20,6.20]]]}}
      ]
    }"#;
    let features = import_osm_geojson(two, &source(), &crs())?;
    let mut reversed = features.clone();
    reversed.reverse();
    let a = generate_building_volumes(&features, &source(), 42)?;
    let b = generate_building_volumes(&reversed, &source(), 42)?;
    assert_eq!(a, b, "input reordering must not change the frame origin");
    assert_eq!(a.len(), 2);
    assert_eq!(a[0].source_object_ids.len(), 1);
    assert_eq!(a[0].origin_lon_lat, [3.10, 6.10]);
    assert_eq!(
        a[0].asset_id.as_str(),
        genesis_core::derive_id("mesh", "building:way/a:42:procedural-v0.1").as_str()
    );
    Ok(())
}

#[test]
fn verify_mesh_rejects_degenerate_and_invalid_meshes() -> CoreResult<()> {
    let features = import_osm_geojson(FIXTURE, &source(), &crs())?;
    let mut mesh = generate_building_volumes(&features, &source(), 7)?.remove(0);
    verify_mesh(&mesh)?;
    mesh.indices[0] = mesh.indices[1];
    assert!(
        matches!(verify_mesh(&mesh), Err(CoreError::InvalidManifest(m)) if m.contains("degenerate"))
    );
    mesh.indices[0] = 99_999;
    assert!(
        matches!(verify_mesh(&mesh), Err(CoreError::InvalidManifest(m)) if m.contains("out of range"))
    );
    mesh.indices[0] = 0;
    mesh.vertices[0][0] = f64::NAN;
    assert!(
        matches!(verify_mesh(&mesh), Err(CoreError::InvalidManifest(m)) if m.contains("non-finite"))
    );
    Ok(())
}

#[test]
fn levels_are_derived_and_height_with_units_is_not_measured() -> CoreResult<()> {
    let levels = br#"{"type":"FeatureCollection","features":[{"type":"Feature","properties":{"osm_id":"way/9","building":"yes","building:levels":"4"},"geometry":{"type":"Polygon","coordinates":[[[3.0,6.0],[3.2,6.0],[3.2,6.2],[3.0,6.2],[3.0,6.0]]]}}]}"#;
    let features = import_osm_geojson(levels, &source(), &crs())?;
    let mesh = generate_building_volumes(&features, &source(), 7)?.remove(0);
    assert_eq!(mesh.evidence, EvidenceKind::DerivedWithConfidence);
    assert_eq!(mesh.parameters["height_source"], "osm:levels*3m");
    assert_eq!(mesh.parameters["height_m"], "12.000");

    let units = br#"{"type":"FeatureCollection","features":[{"type":"Feature","properties":{"osm_id":"way/10","building":"yes","height":"12 m"},"geometry":{"type":"Polygon","coordinates":[[[3.0,6.0],[3.2,6.0],[3.2,6.2],[3.0,6.2],[3.0,6.0]]]}}]}"#;
    let features = import_osm_geojson(units, &source(), &crs())?;
    let mesh = generate_building_volumes(&features, &source(), 7)?.remove(0);
    assert_ne!(
        mesh.evidence,
        EvidenceKind::Measured,
        "unparsed height must not be claimed as measured"
    );
    assert_eq!(mesh.parameters["height_source"], "procedural:default");
    Ok(())
}

#[test]
fn cross_feature_way_junction_limitation_is_documented() -> CoreResult<()> {
    let two_ways = br#"{
      "type": "FeatureCollection",
      "features": [
        {"type":"Feature","properties":{"osm_id":"way/20","highway":"residential"},"geometry":{"type":"LineString","coordinates":[[3.0,6.0],[3.01,6.0]]}},
        {"type":"Feature","properties":{"osm_id":"way/21","highway":"residential"},"geometry":{"type":"LineString","coordinates":[[3.01,6.0],[3.01,6.01]]}}
      ]
    }"#;
    let features = import_osm_geojson(two_ways, &source(), &crs())?;
    let mesh = generate_road_surfaces(&features, &source(), 7, 8.0)?.remove(0);
    assert_eq!(
        mesh.vertices.len(),
        8,
        "separate ways are not miter-merged yet"
    );
    assert!(mesh.parameters["junction"].contains("deferred"));
    Ok(())
}
