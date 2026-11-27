use genesis_core::{
    AssetRef, CURRENT_SCHEMA_VERSION, CoreError, CoreResult, CrsMetadata, Dimensions, EvidenceKind,
    LEGACY_SCHEMA_VERSION, LocalFrame, Provenance, StableId, ValidationState, WorldIr, WorldLayer,
    WorldObject, derive_id, migrate_backward, migrate_forward,
};
use serde_json::{Value, json};

fn provenance() -> Provenance {
    Provenance {
        source_id: "osm:pbf:fixture".to_string(),
        licence_elected: "ODbL-1.0".to_string(),
        attribution: "© OpenStreetMap contributors".to_string(),
        publisher: Some("OpenStreetMap contributors".to_string()),
        url: Some("https://www.openstreetmap.org/copyright".to_string()),
        version: Some("2026-11-27".to_string()),
        licence_url: Some("https://opendatacommons.org/licenses/odbl/1-0/".to_string()),
        vintage: Some("2026-11-27".to_string()),
        filters: vec!["bbox=ikeja".to_string()],
        checksum: Some("abc123".to_string()),
        source_feature_ids: vec!["way/123".to_string()],
        derivation_chain: vec!["clip".to_string(), "extrude".to_string()],
        derivative_database_status: Some("possible".to_string()),
        display_obligations: vec!["credit in viewport".to_string()],
        third_party_terms: Vec::new(),
        recorded_at_unix: Some(1_764_192_000),
    }
}

fn object(id: StableId, evidence: EvidenceKind) -> WorldObject {
    WorldObject {
        id,
        object_type: "building".to_string(),
        geographic_position: [3.3516, 6.6018, 40.0],
        local_position: [123.456, -45.678, 12.345],
        evidence,
        provenance: provenance(),
        dimensions: Dimensions {
            height_m: Some(12.5),
            footprint_area_m2: None,
        },
        generation_seed: Some(42),
        dependencies: vec![],
        validation_state: ValidationState::NotRun,
        payloads: vec![AssetRef {
            asset_id: derive_id("asset", "building-1"),
            payload_path: "payloads/building-1.glb".to_string(),
            content_hash: "def456".to_string(),
            byte_size: 1024,
        }],
    }
}

fn ir() -> WorldIr {
    let building = object(derive_id("object", "building-1"), EvidenceKind::Measured);
    let road = object(derive_id("object", "road-1"), EvidenceKind::InferredByRule);
    WorldIr {
        schema_version: CURRENT_SCHEMA_VERSION,
        world_id: derive_id("world", "ikeja-fixture"),
        crs: CrsMetadata::default(),
        local_frame: LocalFrame {
            frame_id: "ikeja-local".to_string(),
            origin_ecef: [6_325_680.944472361, 364_736.4683773493, 728_214.141724354],
            origin_lon_lat: [3.35, 6.6],
            origin_altitude_m: 0.0,
            epoch: Some(2024.5),
        },
        layers: vec![
            WorldLayer {
                id: derive_id("layer", "buildings"),
                objects: vec![building],
            },
            WorldLayer {
                id: derive_id("layer", "roads"),
                objects: vec![road],
            },
        ],
    }
}

#[test]
fn schema_round_trips_losslessly() -> CoreResult<()> {
    let original = ir();
    let bytes = original.to_bytes()?;
    let restored = WorldIr::from_bytes(&bytes)?;
    assert_eq!(restored, original);
    Ok(())
}

#[test]
fn golden_vector_f64_precision_is_preserved() -> CoreResult<()> {
    // Independent constants from GE-0003 (Ikeja origin computed by the
    // throwaway precision prototype), not values taken from this object.
    const GE0003_ORIGIN: [f64; 3] = [6_325_680.944472361, 364_736.4683773493, 728_214.141724354];
    let original = ir();
    assert_eq!(original.local_frame.origin_ecef, GE0003_ORIGIN);
    let bytes = original.to_bytes()?;
    let restored = WorldIr::from_bytes(&bytes)?;
    assert_eq!(restored.local_frame.origin_ecef, GE0003_ORIGIN);
    assert_eq!(
        restored.layers[0].objects[0].local_position,
        [123.456, -45.678, 12.345]
    );

    // GE-0003 contrast: at 1e7 m a 1 cm offset disappears in binary32.
    let global = 10_000_000.0_f64;
    let f32_delta = ((global + 0.01) as f32) as f64 - global;
    assert_eq!(f32_delta, 0.0);

    // Antimeridian golden vector survives the IR round trip unchanged.
    let mut near_dateline = original.clone();
    near_dateline.layers[0].objects[0].geographic_position = [179.999, -16.5, 12.0];
    let restored = WorldIr::from_bytes(&near_dateline.to_bytes()?)?;
    assert_eq!(
        restored.layers[0].objects[0].geographic_position,
        [179.999, -16.5, 12.0]
    );
    Ok(())
}

#[test]
fn stable_ids_are_deterministic_and_distinct() {
    assert_eq!(
        derive_id("object", "building-1"),
        derive_id("object", "building-1")
    );
    assert_ne!(
        derive_id("object", "building-1"),
        derive_id("object", "building-2")
    );
    assert_ne!(
        derive_id("layer", "building-1"),
        derive_id("object", "building-1")
    );
    assert!(StableId::new("").is_err());
}

#[test]
fn forward_and_backward_migrations_are_lossless() -> CoreResult<()> {
    let legacy = json!({
        "schema_version": LEGACY_SCHEMA_VERSION,
        "world_id": derive_id("world", "ikeja-fixture").as_str(),
        "crs": {"unit": "metre", "up_axis": "Z"},
        "layers": [{
            "id": derive_id("layer", "buildings").as_str(),
            "objects": [{
                "id": derive_id("object", "building-1").as_str(),
                "object_type": "building",
                "geographic_position": [3.3516, 6.6018, 40.0],
                "local_position": [0.0, 0.0, 0.0],
                "evidence": "measured",
                "provenance": {
                    "source_id": "osm:pbf:fixture",
                    "licence_elected": "ODbL-1.0",
                    "attribution": "© OpenStreetMap contributors"
                },
                "height_m": 12.5
            }]
        }]
    });
    let legacy_bytes = serde_json::to_vec(&legacy)?;

    let forward = migrate_forward(&legacy_bytes)?;
    assert!(
        migrate_forward(&forward)? == forward,
        "forward migration is idempotent"
    );
    let current = WorldIr::from_bytes(&forward)?;
    assert_eq!(current.layers[0].objects[0].dimensions.height_m, Some(12.5));

    let backward = migrate_backward(&forward)?;
    let restored: Value = serde_json::from_slice(&backward)?;
    assert_eq!(restored["schema_version"], json!(LEGACY_SCHEMA_VERSION));
    assert_eq!(restored["layers"][0]["objects"][0]["height_m"], json!(12.5));
    // v1 readers gain a top-level height_m; dimensions is retained so other
    // fields (e.g. footprint_area_m2) are never silently destroyed.
    assert_eq!(
        restored["layers"][0]["objects"][0]["dimensions"]["height_m"],
        json!(12.5)
    );
    assert!(restored.get("local_frame").is_none());
    assert_eq!(
        migrate_backward(&backward)?,
        backward,
        "backward migration is idempotent"
    );
    Ok(())
}

#[test]
fn unsupported_and_invalid_documents_are_structured_errors() -> CoreResult<()> {
    let mut bad_version = ir();
    bad_version.schema_version = 999;
    assert!(matches!(
        WorldIr::from_bytes(&bad_version.to_bytes()?),
        Err(CoreError::UnsupportedSchemaVersion { found: 999, .. })
    ));

    let mut bad_path = ir();
    bad_path.layers[0].objects[0].payloads[0].payload_path = "../outside.glb".to_string();
    assert!(matches!(
        WorldIr::from_bytes(&bad_path.to_bytes()?),
        Err(CoreError::PathEscapesProject(_))
    ));

    let mut bad_dependency = ir();
    let missing = derive_id("object", "does-not-exist");
    bad_dependency.layers[1].objects[0].dependencies = vec![missing];
    assert!(matches!(
        WorldIr::from_bytes(&bad_dependency.to_bytes()?),
        Err(CoreError::InvalidManifest(_))
    ));
    Ok(())
}

#[test]
fn migration_preserves_all_dimensions_and_does_not_overwrite_nested_height() -> CoreResult<()> {
    let mut current = ir();
    current.layers[0].objects[0].dimensions = Dimensions {
        height_m: Some(9.5),
        footprint_area_m2: Some(240.75),
    };
    let bytes = current.to_bytes()?;

    let backward = migrate_backward(&bytes)?;
    let v1: Value = serde_json::from_slice(&backward)?;
    assert_eq!(v1["layers"][0]["objects"][0]["height_m"], json!(9.5));
    assert_eq!(
        v1["layers"][0]["objects"][0]["dimensions"]["footprint_area_m2"],
        json!(240.75)
    );

    let forward = migrate_forward(&backward)?;
    let restored = WorldIr::from_bytes(&forward)?;
    assert_eq!(
        restored.layers[0].objects[0].dimensions,
        current.layers[0].objects[0].dimensions
    );

    // Raw v1 with both a stale top-level height and a nested height: nested
    // wins, because it is the already-migrated value.
    let mut raw: Value = serde_json::from_slice(&backward)?;
    raw["layers"][0]["objects"][0]["height_m"] = json!(99.0);
    let forward = migrate_forward(&serde_json::to_vec(&raw)?)?;
    let restored = WorldIr::from_bytes(&forward)?;
    assert_eq!(restored.layers[0].objects[0].dimensions.height_m, Some(9.5));
    Ok(())
}

#[test]
fn legacy_shape_document_gets_structured_version_error() -> CoreResult<()> {
    let legacy_shape = json!({
        "schema_version": 1,
        "world_id": "id-legacy",
        "crs": {"unit": "metre", "up_axis": "Z"},
        "layers": []
    });
    let bytes = serde_json::to_vec(&legacy_shape)?;
    // No local_frame field: serde would fail on shape before the version gate.
    assert!(matches!(
        WorldIr::from_bytes(&bytes),
        Err(CoreError::UnsupportedSchemaVersion { found: 1, .. })
    ));
    Ok(())
}

#[test]
fn stable_ids_are_validated_on_deserialization() -> CoreResult<()> {
    assert!(serde_json::from_str::<StableId>("\"\"").is_err());
    assert!(serde_json::from_str::<StableId>(&format!("\"{}\"", "x".repeat(300))).is_err());
    assert!(serde_json::from_str::<StableId>("\"bad\\nid\"").is_err());
    assert!(serde_json::from_str::<StableId>("\"valid-id\"").is_ok());
    Ok(())
}

#[test]
fn validation_rejects_duplicates_empty_frame_and_escaping_payload() -> CoreResult<()> {
    let mut duplicate = ir();
    duplicate.layers[1].id = duplicate.layers[0].id.clone();
    assert!(matches!(
        WorldIr::from_bytes(&duplicate.to_bytes()?),
        Err(CoreError::InvalidManifest(_))
    ));

    let mut empty_frame = ir();
    empty_frame.local_frame.frame_id.clear();
    assert!(matches!(
        WorldIr::from_bytes(&empty_frame.to_bytes()?),
        Err(CoreError::InvalidManifest(_))
    ));

    for evil in ["../x.glb", "/etc/x.glb", "C:\\x.glb"] {
        let mut bad = ir();
        bad.layers[0].objects[0].payloads[0].payload_path = evil.to_string();
        assert!(
            matches!(
                WorldIr::from_bytes(&bad.to_bytes()?),
                Err(CoreError::PathEscapesProject(_))
            ),
            "payload path not rejected: {evil}"
        );
    }
    Ok(())
}

#[test]
fn wgs84_enu_conversion_is_precision_safe() {
    use genesis_core::{geodetic_to_ecef, geodetic_to_enu};
    let origin_lon = 3.35;
    let origin_lat = 6.6;
    let ecef = geodetic_to_ecef(origin_lon, origin_lat, 0.0);
    assert!((ecef[0] - 6_325_320.576246685).abs() < 1e-6);
    assert!((ecef[1] - 370_254.2048726019).abs() < 1e-6);
    assert!((ecef[2] - 728_209.5442383343).abs() < 1e-6);

    let at_origin = geodetic_to_enu(origin_lon, origin_lat, 0.0, origin_lon, origin_lat, 0.0);
    assert!(at_origin.iter().all(|v| v.abs() < 1e-9));

    // A one-centimetre-class offset survives in the local frame.
    let offset = geodetic_to_enu(
        origin_lon + 1e-7,
        origin_lat,
        0.0,
        origin_lon,
        origin_lat,
        0.0,
    );
    assert!(offset[0] > 0.0 && offset[0] < 0.02);
    let frame = LocalFrame::from_geodetic(origin_lon, origin_lat, 0.0, "test-frame");
    assert_eq!(frame.origin_lon_lat, [origin_lon, origin_lat]);
    assert_eq!(frame.origin_ecef, ecef);
}
