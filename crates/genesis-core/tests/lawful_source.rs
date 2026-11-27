use genesis_core::{
    CoreError, CoreResult, CrsMetadata, EvidenceKind, LicenceElection, LocalFrame,
    SourceDescriptor, SourcePurpose, attribution_sidecar, check_source, derive_id,
    world_ir::{Provenance, WorldIr},
};
use serde_json::Value;

fn osm() -> SourceDescriptor {
    SourceDescriptor {
        id: "osm:extract:ikeja".to_string(),
        publisher: "OpenStreetMap contributors".to_string(),
        url: "https://www.openstreetmap.org/copyright".to_string(),
        version: "2026-11-27".to_string(),
        licence: LicenceElection::Odbl10,
        attribution: "© OpenStreetMap contributors".to_string(),
        vintage: Some("2026-11-27".to_string()),
        filters: vec!["bbox=ikeja".to_string()],
        checksum: Some("abc123".to_string()),
        purpose: SourcePurpose::ReconstructionInput,
    }
}

#[test]
fn allowed_source_passes_and_denied_sources_are_rejected() {
    assert!(check_source(&osm()).is_ok());

    for (publisher, url) in [
        (
            "Google Maps Platform",
            "https://maps.googleapis.com/maps/api/tile",
        ),
        ("Google", "https://streetview.example.test/pano"),
        ("Cesium ion", "https://ion.cesium.com/assets/123"),
        ("Cesium", "https://cesium.com/ion/credits"),
    ] {
        let mut source = osm();
        source.id = format!("blocked:{publisher}");
        source.publisher = publisher.to_string();
        source.url = url.to_string();
        assert!(
            matches!(check_source(&source), Err(CoreError::SourceDenied(_))),
            "source not denied: {publisher} / {url}"
        );
    }

    // Open-standards software with Genesis-owned content remains allowed.
    let mut cesiumjs = osm();
    cesiumjs.id = "cesiumjs:software".to_string();
    cesiumjs.publisher = "CesiumJS".to_string();
    cesiumjs.url = "https://github.com/CesiumGS/cesium".to_string();
    cesiumjs.licence = LicenceElection::Apache20;
    assert!(check_source(&cesiumjs).is_ok());
}

#[test]
fn non_commercial_weights_and_model_training_are_denied() {
    let mut source = osm();
    source.licence = LicenceElection::Other("CC BY-NC-SA 4.0".to_string());
    source.purpose = SourcePurpose::ModelTraining;
    assert!(matches!(
        check_source(&source),
        Err(CoreError::SourceDenied(_))
    ));
}

#[test]
fn malformed_source_is_structured_error() {
    let mut source = osm();
    source.attribution.clear();
    assert!(matches!(
        check_source(&source),
        Err(CoreError::InvalidManifest(_))
    ));
}

#[test]
fn provenance_survives_ir_and_export_sidecar() -> CoreResult<()> {
    let source = osm();
    check_source(&source)?;
    let provenance: Provenance = (&source).into();
    assert_eq!(provenance.licence_elected, "ODbL-1.0");
    assert_eq!(provenance.attribution, "© OpenStreetMap contributors");
    assert_eq!(provenance.filters, vec!["bbox=ikeja".to_string()]);

    let object = genesis_core::WorldObject {
        id: derive_id("object", "road-1"),
        object_type: "road".to_string(),
        geographic_position: [3.3516, 6.6018, 40.0],
        local_position: [0.0, 0.0, 0.0],
        evidence: EvidenceKind::Measured,
        provenance,
        dimensions: Default::default(),
        generation_seed: None,
        dependencies: Vec::new(),
        validation_state: Default::default(),
        payloads: Vec::new(),
    };
    let world = WorldIr {
        schema_version: genesis_core::CURRENT_SCHEMA_VERSION,
        world_id: derive_id("world", "ikeja"),
        crs: CrsMetadata::default(),
        local_frame: LocalFrame::default(),
        layers: vec![genesis_core::WorldLayer {
            id: derive_id("layer", "roads"),
            objects: vec![object],
        }],
    };

    let restored = WorldIr::from_bytes(&world.to_bytes()?)?;
    let sidecar = attribution_sidecar(&restored)?;
    let value: Value = serde_json::from_slice(&sidecar)?;
    let record = &value["records"][0];
    assert_eq!(record["attribution"], "© OpenStreetMap contributors");
    assert_eq!(record["licence_elected"], "ODbL-1.0");
    assert_eq!(record["source_id"], "osm:extract:ikeja");
    assert_eq!(value["world_id"], derive_id("world", "ikeja").as_str());
    Ok(())
}
