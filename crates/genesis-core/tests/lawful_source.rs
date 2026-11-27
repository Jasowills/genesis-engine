use genesis_core::{
    CURRENT_SCHEMA_VERSION, CoreError, CoreResult, CrsMetadata, EvidenceKind, LicenceElection,
    LocalFrame, SourceDescriptor, SourcePurpose, attribution_sidecar, check_source, derive_id,
    enforce_sources, prepare_import,
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
        licence_url: None,
        source_feature_ids: vec!["way/123".to_string()],
        derivation_chain: vec!["clip".to_string()],
    }
}

#[test]
fn allowed_source_passes_and_every_denied_pattern_is_rejected() {
    assert!(check_source(&osm()).is_ok());

    let blocked = [
        ("Google", "https://maps.googleapis.com/maps/api/tile"),
        ("Google", "https://tile.googleapis.com/v1/3dtiles/root.json"),
        ("Google Maps Platform", "https://maps.google.com/maps"),
        ("Google Maps", "https://example.test/tiles"),
        ("Google", "https://streetview.example.test/pano"),
        ("Google", "https://example.test/street-view?pano=1"),
        (
            "Google",
            "https://example.test/photorealistic-3d-tiles/root",
        ),
        ("Google", "https://example.test/Photorealistic 3D Tiles"),
        ("Cesium ion", "https://ion.cesium.com/assets/123"),
        ("Cesium", "https://cesium.com/ion/credits"),
        ("Cesium", "https://assets.cesium.com/123/tileset.json"),
        ("Cesium", "https://api.cesium.com/v1/assets"),
        ("Cesium", "https://example.test/cesium-ion-content"),
    ];
    for (publisher, url) in blocked {
        let mut source = osm();
        source.id = format!("blocked:{publisher}:{url}");
        source.publisher = publisher.to_string();
        source.url = url.to_string();
        assert!(
            matches!(check_source(&source), Err(CoreError::SourceDenied(_))),
            "source not denied: {publisher} / {url}"
        );
    }

    // Deny patterns also apply to id/version/attribution/filters, not just URL.
    let mut by_id = osm();
    by_id.id = "google-maps-photorealistic-3d-tiles".to_string();
    assert!(matches!(
        check_source(&by_id),
        Err(CoreError::SourceDenied(_))
    ));

    // Case folding works.
    let mut upper = osm();
    upper.publisher = "GOOGLE MAPS PLATFORM".to_string();
    assert!(matches!(
        check_source(&upper),
        Err(CoreError::SourceDenied(_))
    ));

    // Open-standards software remains allowed.
    let mut cesiumjs = osm();
    cesiumjs.id = "cesiumjs:software".to_string();
    cesiumjs.publisher = "CesiumJS".to_string();
    cesiumjs.url = "https://github.com/CesiumGS/cesium".to_string();
    cesiumjs.licence = LicenceElection::Apache20;
    assert!(check_source(&cesiumjs).is_ok());
}

#[test]
fn restricted_licence_variants_are_denied_and_empty_custom_licence_is_invalid() {
    for text in [
        "CC BY-NC-SA 4.0",
        "Attribution-NonCommercial 4.0 International",
        "research/evaluation only",
        "research only",
    ] {
        let mut source = osm();
        source.licence = LicenceElection::Other(text.to_string());
        assert!(
            matches!(check_source(&source), Err(CoreError::SourceDenied(_))),
            "licence not denied: {text}"
        );
    }
    let mut empty = osm();
    empty.licence = LicenceElection::Other("  ".to_string());
    assert!(matches!(
        check_source(&empty),
        Err(CoreError::InvalidManifest(_))
    ));
}

#[test]
fn purpose_is_recorded_but_not_a_gate_in_this_release() {
    let mut training = osm();
    training.purpose = SourcePurpose::ModelTraining;
    assert!(
        check_source(&training).is_ok(),
        "purpose must not bypass the hard deny"
    );

    let mut basemap = osm();
    basemap.publisher = "Cesium ion".to_string();
    basemap.purpose = SourcePurpose::VisualBasemapOnly;
    assert!(matches!(
        check_source(&basemap),
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

fn world_with(provenance: Provenance) -> WorldIr {
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
    WorldIr {
        schema_version: CURRENT_SCHEMA_VERSION,
        world_id: derive_id("world", "ikeja"),
        crs: CrsMetadata::default(),
        local_frame: LocalFrame::default(),
        layers: vec![genesis_core::WorldLayer {
            id: derive_id("layer", "roads"),
            objects: vec![object],
        }],
    }
}

#[test]
fn prepare_import_and_sidecar_preserve_full_provenance() -> CoreResult<()> {
    let source = osm();
    let provenance = prepare_import(&source)?;
    assert_eq!(provenance.licence_elected, "ODbL-1.0");
    assert_eq!(
        provenance.publisher.as_deref(),
        Some("OpenStreetMap contributors")
    );
    assert_eq!(
        provenance.url.as_deref(),
        Some("https://www.openstreetmap.org/copyright")
    );
    assert_eq!(
        provenance.licence_url.as_deref(),
        Some("https://opendatacommons.org/licenses/odbl/1-0/")
    );
    assert_eq!(provenance.source_feature_ids, vec!["way/123".to_string()]);
    assert_eq!(provenance.derivation_chain, vec!["clip".to_string()]);

    let world = world_with(provenance);
    let restored = WorldIr::from_bytes(&world.to_bytes()?)?;
    let sidecar = attribution_sidecar(&restored, &[source])?;
    let value: Value = serde_json::from_slice(&sidecar)?;
    let record = &value["records"][0];
    assert_eq!(record["attribution"], "© OpenStreetMap contributors");
    assert_eq!(record["licence_elected"], "ODbL-1.0");
    assert_eq!(record["source_id"], "osm:extract:ikeja");
    assert_eq!(record["filters"][0], "bbox=ikeja");
    assert_eq!(record["derivation_chain"][0], "clip");
    assert_eq!(record["evidence"], "measured");
    Ok(())
}

#[test]
fn export_is_gated_and_sidecar_keeps_distinct_sources() -> CoreResult<()> {
    let june = osm();
    let mut july = osm();
    july.id = "osm:extract:ikeja-july".to_string();
    july.vintage = Some("2026-07".to_string());
    july.checksum = Some("def456".to_string());

    let mut world = world_with((&june).into());
    let second = genesis_core::WorldObject {
        id: derive_id("object", "building-2"),
        provenance: (&july).into(),
        ..world.layers[0].objects[0].clone()
    };
    world.layers[0].objects.push(second);

    let sidecar = attribution_sidecar(&world, &[june.clone(), july.clone()])?;
    let value: Value = serde_json::from_slice(&sidecar)?;
    assert_eq!(value["records"].as_array().map(Vec::len), Some(2));

    // A denied source cannot be used for export.
    let mut bad = osm();
    bad.publisher = "Cesium ion".to_string();
    assert!(matches!(
        attribution_sidecar(&world, &[bad]),
        Err(CoreError::SourceDenied(_))
    ));

    // An object whose source is not registered cannot be exported.
    assert!(matches!(
        enforce_sources(&world, &[june]),
        Err(CoreError::InvalidManifest(_))
    ));
    Ok(())
}

#[test]
fn obfuscated_deny_patterns_are_still_rejected() {
    let mut encoded = osm();
    encoded.url = "https://maps%2Egoogleapis%2Ecom/maps".to_string();
    assert!(matches!(
        check_source(&encoded),
        Err(CoreError::SourceDenied(_))
    ));

    let mut unicode_space = osm();
    unicode_space.publisher = "Cesium\u{202f}ion".to_string();
    assert!(matches!(
        check_source(&unicode_space),
        Err(CoreError::SourceDenied(_))
    ));

    let mut hyphen = osm();
    hyphen.publisher = "Cesium\u{2011}ion".to_string();
    assert!(matches!(
        check_source(&hyphen),
        Err(CoreError::SourceDenied(_))
    ));

    let mut licence = osm();
    licence.licence = LicenceElection::Other("non\u{00a0}commercial".to_string());
    assert!(matches!(
        check_source(&licence),
        Err(CoreError::SourceDenied(_))
    ));
}
