use genesis_core::{
    CoreError, EstimateRange, EvidenceKind, EvidenceTier, HostCapabilities, ProfilePolicy,
    QualityProfile, authorize_paid_operation, preflight,
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

fn all_profiles() -> [QualityProfile; 4] {
    [
        QualityProfile::Preview,
        QualityProfile::GameReady,
        QualityProfile::HighFidelity,
        QualityProfile::SurveyReference,
    ]
}

#[test]
fn all_profiles_map_to_explicit_gates() {
    let preview = ProfilePolicy::for_profile(QualityProfile::Preview);
    let game = ProfilePolicy::for_profile(QualityProfile::GameReady);
    let high = ProfilePolicy::for_profile(QualityProfile::HighFidelity);
    let survey = ProfilePolicy::for_profile(QualityProfile::SurveyReference);

    assert_eq!(preview.min_evidence, EvidenceTier::Inferred);
    assert_eq!(game.min_evidence, EvidenceTier::Derived);
    assert_eq!(high.min_evidence, EvidenceTier::Observed);
    assert_eq!(survey.min_evidence, EvidenceTier::Observed);
    assert_eq!(preview.max_deviation_m, Some(25.0));
    assert_eq!(game.max_deviation_m, Some(5.0));
    assert_eq!(high.max_deviation_m, Some(1.0));
    assert_eq!(survey.max_deviation_m, Some(0.1));
    assert!(!preview.source_resolution_required);
    assert!(
        game.source_resolution_required
            && high.source_resolution_required
            && survey.source_resolution_required
    );
    assert!(
        !preview.checkpoints_required && !game.checkpoints_required && !high.checkpoints_required
    );
    assert!(survey.checkpoints_required);
    assert!(survey.gates.iter().any(|g| g == "independent_checkpoints"));
    assert!(survey.gates.len() > high.gates.len() && high.gates.len() > game.gates.len());

    // Unknown evidence never satisfies any profile; observed always does.
    for profile in all_profiles() {
        let policy = ProfilePolicy::for_profile(profile);
        assert!(
            !policy.permits(EvidenceKind::Unknown),
            "Unknown must fail {profile:?}"
        );
        assert!(
            policy.permits(EvidenceKind::Measured),
            "Measured must pass {profile:?}"
        );
    }
    assert!(preview.permits(EvidenceKind::InferredByRule));
    assert!(!game.permits(EvidenceKind::InferredByRule));
    assert!(game.permits(EvidenceKind::DerivedWithConfidence));
    assert!(!high.permits(EvidenceKind::DerivedWithConfidence));
}

#[test]
fn preflight_emits_only_ranges_with_confidence() {
    for profile in all_profiles() {
        let report = preflight(profile, &capable(), 100).unwrap();
        assert_eq!(report.schema_version, 1);
        assert_eq!(report.profile, profile);
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
    let survey = preflight(QualityProfile::SurveyReference, &capable(), 100).unwrap();
    assert!(survey.warnings.iter().any(|w| w.contains("checkpoints")));
}

#[test]
fn paid_cost_estimates_keep_their_honesty_metadata() {
    let mut report = preflight(QualityProfile::GameReady, &capable(), 10).unwrap();
    let cost = EstimateRange::new(0.5, 2.0, "USD", "provider list price").unwrap();
    report.add_paid_operation("segment", &cost).unwrap();
    let entry = &report.paid_operations[0];
    assert_eq!(entry["name"], "segment");
    assert_eq!(entry["cost_low"], "0.5");
    assert_eq!(entry["cost_high"], "2");
    assert_eq!(entry["confidence"], "low-uncalibrated");
    assert_eq!(entry["method"], "provider list price");
    assert!(entry["limitations"].contains("Uncalibrated estimate"));
    assert_eq!(entry["consent"], "required-before-execution");
    assert!(report.add_paid_operation(" ", &cost).is_err());
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
    match authorize_paid_operation("", &caps, true) {
        Err(CoreError::InvalidManifest(m)) => assert!(m.contains("needs a name")),
        other => panic!("expected name error, got {other:?}"),
    }
    assert_eq!(
        authorize_paid_operation("segment", &caps, true).unwrap(),
        "consent:segment:granted"
    );
}

#[test]
fn estimate_bounds_are_validated() {
    assert!(EstimateRange::new(5.0, 1.0, "ms", "test").is_err());
    assert!(EstimateRange::new(f64::NAN, 1.0, "ms", "test").is_err());
    assert!(EstimateRange::new(0.0, 0.0, "ms", "test").is_err());
    assert!(preflight(QualityProfile::Preview, &capable(), 0).is_err());
}
