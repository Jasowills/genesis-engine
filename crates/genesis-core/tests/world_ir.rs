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
        vintage: Some("2026-11-27".to_string()),
        filters: vec!["bbox=ikeja".to_string()],
        checksum: Some("abc123".to_string()),
        source_feature_ids: vec!["way/123".to_string()],
        derivation_chain: vec!["clip".to_string(), "extrude".to_string()],
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
    let original = ir();
    let bytes = original.to_bytes()?;
    let restored = WorldIr::from_bytes(&bytes)?;
    let a = &restored.local_frame.origin_ecef;
    let b = &original.local_frame.origin_ecef;
    let error = ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2) + (a[2] - b[2]).powi(2)).sqrt();
    assert!(error < 1e-9, "local-frame origin lost precision: {error} m");
    assert_eq!(
        restored.layers[0].objects[0].local_position,
        [123.456, -45.678, 12.345]
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
    assert!(
        restored["layers"][0]["objects"][0]
            .get("dimensions")
            .is_none()
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
