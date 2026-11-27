use genesis_core::{
    CURRENT_SCHEMA_VERSION, CoreError, CoreResult, CrsMetadata, CrsMetadata as Crs, EvidenceKind,
    LicenceElection, LocalFrame, SourceDescriptor, attribution_sidecar, derive_id,
    import_osm_geojson, to_world_objects,
};
use genesis_core::{WorldIr, WorldLayer};
use serde_json::Value;

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
    Crs {
        wkt2: Some("GEOGCRS[\"WGS 84\"]".to_string()),
        epoch: Some(2024.5),
        vertical_datum: Some("EGM2008".to_string()),
        ..Crs::default()
    }
}

fn fixture() -> &'static [u8] {
    br#"{
      "type": "FeatureCollection",
      "features": [
        {
          "type": "Feature",
          "properties": {
            "osm_id": "way/123", "version": 3, "timestamp": "2026-10-01T12:00:00Z",
            "building": "yes", "height": "12", "roof:shape": "gabled"
          },
          "geometry": {
            "type": "Polygon",
            "coordinates": [[
              [3.3480, 6.5985], [3.3552, 6.5985],
              [3.3552, 6.6051], [3.3480, 6.6051], [3.3480, 6.5985]
            ]]
          }
        },
        {
          "type": "Feature",
          "properties": {
            "osm_id": "way/456", "version": "2", "timestamp": "2026-09-01T08:00:00Z",
            "highway": "residential", "lanes": "2"
          },
          "geometry": {
            "type": "LineString",
            "coordinates": [[3.3485, 6.5990], [3.3520, 6.6000], [3.3545, 6.6040]]
          }
        },
        {
          "type": "Feature",
          "properties": {"osm_id": "node/789", "amenity": "bench"},
          "geometry": {"type": "Point", "coordinates": [3.35, 6.60]}
        }
      ]
    }"#
}

#[test]
fn features_import_with_stable_ids_versions_and_evidence() -> CoreResult<()> {
    let features = import_osm_geojson(fixture(), &source(), &crs())?;
    assert_eq!(features.len(), 2, "non-target features are skipped");
    assert_eq!(features[0].source_feature_id, "way/123");
    assert_eq!(features[0].version, Some(3));
    assert_eq!(
        features[0].timestamp.as_deref(),
        Some("2026-10-01T12:00:00Z")
    );
    assert_eq!(
        features[0].evidence,
        EvidenceKind::Measured,
        "explicit height tag"
    );
    assert_eq!(features[1].source_feature_id, "way/456");
    assert_eq!(features[1].version, Some(2));
    assert_eq!(features[1].evidence, EvidenceKind::InferredByRule);
    assert_eq!(features[1].tags.get("lanes").map(String::as_str), Some("2"));
    Ok(())
}

#[test]
fn features_become_world_objects_with_provenance() -> CoreResult<()> {
    let features = import_osm_geojson(fixture(), &source(), &crs())?;
    let objects = to_world_objects(&features, &source())?;
    assert_eq!(objects.len(), 2);
    assert_eq!(objects[0].id, derive_id("object", "osm:pbf:ikeja:way/123"));
    assert_eq!(objects[0].object_type, "building");
    assert_eq!(objects[1].object_type, "road");
    assert_eq!(
        objects[0].provenance.source_feature_ids,
        vec!["way/123".to_string()]
    );
    assert_eq!(objects[0].provenance.licence_elected, "ODbL-1.0");
    assert_eq!(
        objects[0].provenance.attribution,
        "© OpenStreetMap contributors"
    );
    assert_eq!(
        objects[0].provenance.vintage.as_deref(),
        Some("2026-10-01T12:00:00Z")
    );
    assert!(
        objects[0]
            .provenance
            .derivation_chain
            .iter()
            .any(|s| s == "tag:height=12")
    );
    assert!(
        objects[1]
            .provenance
            .derivation_chain
            .iter()
            .any(|s| s == "tag:lanes=2")
    );
    assert_eq!(objects[0].evidence, EvidenceKind::Measured);
    assert_eq!(objects[1].evidence, EvidenceKind::InferredByRule);

    let world = WorldIr {
        schema_version: CURRENT_SCHEMA_VERSION,
        world_id: derive_id("world", "ikeja"),
        crs: crs(),
        local_frame: LocalFrame::default(),
        layers: vec![WorldLayer {
            id: derive_id("layer", "osm-features"),
            objects,
        }],
    };
    let restored = WorldIr::from_bytes(&world.to_bytes()?)?;
    let sidecar = attribution_sidecar(&restored, &[source()])?;
    let value: Value = serde_json::from_slice(&sidecar)?;
    assert_eq!(value["records"][0]["source_id"], "osm:pbf:ikeja");
    assert_eq!(value["records"][0]["licence_elected"], "ODbL-1.0");
    Ok(())
}

#[test]
fn denied_source_and_malformed_geometry_are_rejected() {
    let mut denied = source();
    denied.publisher = "Cesium ion".to_string();
    assert!(matches!(
        import_osm_geojson(fixture(), &denied, &crs()),
        Err(CoreError::SourceDenied(_))
    ));

    let unclosed = br#"{"type":"FeatureCollection","features":[{"type":"Feature","properties":{"building":"yes"},"geometry":{"type":"Polygon","coordinates":[[[3.0,6.0],[3.1,6.0],[3.1,6.1]]]}}]}"#;
    assert!(matches!(
        import_osm_geojson(unclosed, &source(), &crs()),
        Err(CoreError::InvalidManifest(_))
    ));

    let short_line = br#"{"type":"FeatureCollection","features":[{"type":"Feature","properties":{"highway":"residential"},"geometry":{"type":"LineString","coordinates":[[3.0,6.0]]}}]}"#;
    assert!(matches!(
        import_osm_geojson(short_line, &source(), &crs()),
        Err(CoreError::InvalidManifest(_))
    ));
}
