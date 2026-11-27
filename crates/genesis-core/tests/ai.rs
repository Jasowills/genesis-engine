use genesis_core::{
    CancelToken, CoreError, DepthRequest, ExternalProviderStub, FakeAdapter, InferenceAdapter,
    LicenceElection, ModelDescriptor, ModelLicence, SegmentationRequest, SourceDescriptor,
};

fn source() -> SourceDescriptor {
    SourceDescriptor {
        id: "osm:fixture:ikeja".to_string(),
        publisher: "OpenStreetMap contributors".to_string(),
        url: "https://www.openstreetmap.org/copyright".to_string(),
        licence: LicenceElection::Odbl10,
        attribution: "© OpenStreetMap contributors".to_string(),
        ..Default::default()
    }
}

fn model() -> ModelDescriptor {
    ModelDescriptor {
        id: "demo:permit-check".to_string(),
        version: "1.0.0".to_string(),
        licence: ModelLicence::Apache20,
        weight_licence: ModelLicence::Apache20,
        execution_provider_priority: vec!["CPUExecutionProvider".to_string()],
        settings: Default::default(),
        expected_sha256: "b".repeat(64),
        commercial_use_permitted: true,
    }
}

#[test]
fn typed_outputs_record_per_run_evidence() {
    let adapter = FakeAdapter::new(model()).unwrap();
    let seg = adapter
        .segment(
            &SegmentationRequest {
                source_object_id: "building/1".to_string(),
                source: source(),
                width_px: 2,
                height_px: 3,
            },
            &CancelToken::default(),
        )
        .unwrap();
    assert_eq!(seg.result.source_object_id, "building/1");
    assert_eq!(seg.result.mask_runs, vec![0, 6]);
    assert_eq!(seg.evidence.model_id, "demo:permit-check");
    assert_eq!(seg.evidence.adapter, "fake-deterministic");
    assert!(seg.evidence.completed);
    assert!(
        seg.evidence
            .limitations
            .iter()
            .any(|l| l.contains("no real model inference"))
    );

    let depth = adapter
        .depth(
            &DepthRequest {
                source_object_id: "building/1".to_string(),
                source: source(),
                width_px: 2,
                height_px: 2,
            },
            &CancelToken::default(),
        )
        .unwrap();
    assert_eq!(depth.result.depth_m, vec![1.0; 4]);
    assert_eq!(depth.evidence.model_version, "1.0.0");
    assert_eq!(depth.evidence.model_licence, ModelLicence::Apache20);
}

#[test]
fn cancellation_is_demonstrated_on_both_ports() {
    let adapter = FakeAdapter::new(model()).unwrap();
    let token = CancelToken::default();
    token.cancel();
    assert!(token.is_cancelled());
    assert!(matches!(
        adapter.segment(&SegmentationRequest {
            source_object_id: "building/1".to_string(), source: source(), width_px: 1, height_px: 1,
        }, &token),
        Err(CoreError::Cancelled(m)) if m.contains("cancelled")
    ));
    assert!(matches!(
        adapter.depth(&DepthRequest {
            source_object_id: "building/1".to_string(), source: source(), width_px: 1, height_px: 1,
        }, &token),
        Err(CoreError::Cancelled(m)) if m.contains("cancelled")
    ));
}

#[test]
fn non_commercial_weights_are_rejected() {
    let mut bad = model();
    bad.licence = ModelLicence::NonCommercial;
    assert!(
        matches!(FakeAdapter::new(bad), Err(CoreError::LicenseBlocked(m)) if m.contains("non-commercial"))
    );
    let mut bad = model();
    bad.weight_licence = ModelLicence::NonCommercial;
    assert!(
        matches!(FakeAdapter::new(bad), Err(CoreError::LicenseBlocked(m)) if m.contains("non-commercial"))
    );
    let mut bad = model();
    bad.commercial_use_permitted = false;
    assert!(FakeAdapter::new(bad).is_err());
    assert!(!ModelLicence::Unknown.commercial_use_permitted());
}

#[test]
fn external_provider_never_calls_out_by_default() {
    let stub = ExternalProviderStub::new(model(), None).unwrap();
    let req = SegmentationRequest {
        source_object_id: "x".to_string(),
        source: source(),
        width_px: 1,
        height_px: 1,
    };
    assert!(
        matches!(stub.segment(&req, &CancelToken::default()), Err(CoreError::ProviderUnavailable(m)) if m.contains("no consent token"))
    );
    let stub_with_consent =
        ExternalProviderStub::new(model(), Some("consent:x:granted".to_string())).unwrap();
    assert!(
        matches!(stub_with_consent.segment(&req, &CancelToken::default()), Err(CoreError::ProviderUnavailable(m)) if m.contains("transport not implemented"))
    );
}

#[test]
fn invalid_requests_and_descriptors_are_rejected() {
    let adapter = FakeAdapter::new(model()).unwrap();
    let bad = SegmentationRequest {
        source_object_id: String::new(),
        source: source(),
        width_px: 0,
        height_px: 1,
    };
    assert!(adapter.segment(&bad, &CancelToken::default()).is_err());
    let mut bad_model = model();
    bad_model.expected_sha256 = "short".to_string();
    assert!(bad_model.validate().is_err());
    bad_model.expected_sha256 = "c".repeat(64);
    bad_model.version.clear();
    assert!(bad_model.validate().is_err());
}

#[test]
fn oversized_requests_are_rejected_without_overflow() {
    let adapter = FakeAdapter::new(model()).unwrap();
    let result = adapter.segment(
        &SegmentationRequest {
            source_object_id: "huge".to_string(),
            source: source(),
            width_px: u32::MAX,
            height_px: u32::MAX,
        },
        &CancelToken::default(),
    );
    assert!(matches!(result, Err(CoreError::InvalidManifest(m)) if m.contains("exceeds")));
}

#[test]
fn denied_source_blocks_inference_before_any_run() {
    let adapter = FakeAdapter::new(model()).unwrap();
    let mut denied = source();
    denied.publisher = "Google Maps".to_string();
    let request = SegmentationRequest {
        source_object_id: "building/1".to_string(),
        source: denied,
        width_px: 2,
        height_px: 2,
    };
    assert!(matches!(
        adapter.segment(&request, &CancelToken::default()),
        Err(CoreError::SourceDenied(_))
    ));
}
