use genesis_core::{
    CoreError, EstimateRange, EvidenceKind, EvidenceTier, HostCapabilities, QualityProfile,
    authorize_paid_operation, paid_operation, preflight,
};

fn capable() -> HostCapabilities {
    HostCapabilities {
        logical_cpus: 8,
        memory_gb: 16.0,
        gpu_available: true,
        cuda_available: false,
        paid_operations_enabled: true,
    }
}

#[test]
fn all_profiles_map_to_explicit_gates() {
    let preview = genesis_core::ProfilePolicy::for_profile(QualityProfile::Preview);
    let game = genesis_core::ProfilePolicy::for_profile(QualityProfile::GameReady);
    let high = genesis_core::ProfilePolicy::for_profile(QualityProfile::HighFidelity);
    let survey = genesis_core::ProfilePolicy::for_profile(QualityProfile::SurveyReference);
    assert_eq!(preview.min_evidence, EvidenceTier::Inferred);
    assert_eq!(game.min_evidence, EvidenceTier::Derived);
    assert_eq!(high.min_evidence, EvidenceTier::Observed);
    assert!(survey.checkpoints_required);
    assert!(survey.gates.iter().any(|g| g == "independent_checkpoints"));
    assert!(survey.gates.len() > preview.gates.len());
    assert!(!preview.permits(EvidenceKind::Unknown));
    assert!(preview.permits(EvidenceKind::InferredByRule));
    assert!(!game.permits(EvidenceKind::InferredByRule));
    assert!(!high.permits(EvidenceKind::DerivedWithConfidence));
}

#[test]
fn preflight_emits_only_ranges_with_confidence() {
    let report = preflight(QualityProfile::GameReady, &capable(), 100).unwrap();
    assert_eq!(report.schema_version, 1);
    assert!(report.duration.low < report.duration.high);
    assert!(report.peak_memory.low < report.peak_memory.high);
    assert_eq!(report.duration.confidence, "low-uncalibrated");
    assert!(
        report
            .duration
            .limitations
            .iter()
            .any(|s| s.contains("not a measured result"))
    );
    assert!(!report.warnings.iter().any(|w| w.contains("unsuitable")));
}

#[test]
fn capability_report_is_declared_not_probed() {
    let weak = HostCapabilities {
        logical_cpus: 1,
        memory_gb: 1.0,
        ..Default::default()
    };
    let report = preflight(QualityProfile::HighFidelity, &weak, 10).unwrap();
    assert!(!report.capability.cpu_suitable);
    assert!(!report.capability.gpu_suitable);
    assert!(report.capability.source.contains("no hardware probing"));
    assert!(
        report
            .warnings
            .iter()
            .any(|w| w.contains("CPU below minimum"))
    );
}

#[test]
fn paid_operations_require_explicit_consent() {
    let mut caps = capable();
    caps.paid_operations_enabled = false;
    assert!(authorize_paid_operation("segment", &caps, true).is_err());
    caps.paid_operations_enabled = true;
    match authorize_paid_operation("segment", &caps, false) {
        Err(CoreError::InvalidManifest(m)) => assert!(m.contains("explicit consent")),
        other => panic!("expected consent error, got {other:?}"),
    }
    assert_eq!(
        authorize_paid_operation("segment", &caps, true).unwrap(),
        "consent:segment:granted"
    );
    let cost = EstimateRange::new(0.5, 2.0, "USD", "provider list price").unwrap();
    let entry = paid_operation("segment", cost).unwrap();
    assert_eq!(entry["consent"], "required-before-execution");
    assert_eq!(entry["cost_low"], "0.5");
}

#[test]
fn estimate_bounds_are_validated() {
    assert!(EstimateRange::new(5.0, 1.0, "ms", "test").is_err());
    assert!(EstimateRange::new(f64::NAN, 1.0, "ms", "test").is_err());
    assert!(EstimateRange::new(0.0, 0.0, "ms", "test").is_err());
    assert!(preflight(QualityProfile::Preview, &capable(), 0).is_err());
}
