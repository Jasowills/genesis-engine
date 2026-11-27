use genesis_core::{
    ActivationStore, Capability, CapabilityGrants, CoreError, LicenceElection, PluginManifest,
    SourceDescriptor, WIT_WORLD_SOURCE, wit_world_is_deny_by_default,
};

fn artifact_source() -> SourceDescriptor {
    SourceDescriptor {
        id: "local:plugin:fixture".to_string(),
        publisher: "Genesis test".to_string(),
        url: "local".to_string(),
        licence: LicenceElection::Other("test".to_string()),
        attribution: "test".to_string(),
        ..Default::default()
    }
}

fn manifest(name: &str, version: &str) -> PluginManifest {
    let mut m = PluginManifest {
        schema_version: genesis_core::MANIFEST_SCHEMA_VERSION,
        name: name.to_string(),
        version: version.to_string(),
        wit_world_version: genesis_core::WIT_WORLD_VERSION.to_string(),
        artifact_digest: "a".repeat(64),
        artifact_source: artifact_source(),
        grants: CapabilityGrants::default(),
        dependency_pins: vec!["core@1.0.0".to_string()],
        seal: genesis_core::IntegritySeal {
            algorithm: String::new(),
            digest: String::new(),
        },
    };
    m.seal();
    m
}

#[test]
fn wit_world_has_no_ambient_capabilities() {
    assert!(wit_world_is_deny_by_default());
    assert!(WIT_WORLD_SOURCE.contains("export run:"));
    for forbidden in ["wasi:", "filesystem", "sockets", "process", "environment"] {
        assert!(
            !WIT_WORLD_SOURCE.contains(forbidden),
            "forbidden WIT import: {forbidden}"
        );
    }
}

#[test]
fn capability_grants_are_deny_by_default() {
    let grants = CapabilityGrants::default();
    assert!(!grants.allows(Capability::HostLog));
    assert!(!grants.allows(Capability::HostProgress));
    let mut allowed = grants.clone();
    allowed.granted.insert(Capability::HostLog);
    assert!(allowed.allows(Capability::HostLog));
    assert!(!allowed.allows(Capability::HostProgress));
}

#[test]
fn evil_plugin_fixtures_are_rejected() {
    let mut bad = manifest("evil", "1.0.0");
    bad.wit_world_version = "99.0.0".to_string();
    assert!(matches!(bad.verify(), Err(CoreError::InvalidManifest(m)) if m.contains("WIT world")));

    let bad = manifest("evil", "not-semver");
    assert!(bad.verify().is_err());

    let mut bad = manifest("evil", "1.0.0");
    bad.artifact_digest = "short".to_string();
    assert!(bad.verify().is_err());

    let mut bad = manifest("evil", "1.0.0");
    bad.dependency_pins = vec!["core@latest".to_string()];
    assert!(
        matches!(bad.verify(), Err(CoreError::InvalidManifest(m)) if m.contains("dependency pins"))
    );

    // Tampering with granted capabilities invalidates the seal.
    let mut tampered = manifest("evil", "1.0.0");
    tampered.grants.granted.insert(Capability::HostLog);
    assert!(matches!(tampered.verify(), Err(CoreError::InvalidManifest(m)) if m.contains("seal")));

    let mut store = ActivationStore::default();
    assert!(matches!(
        store.stage(tampered),
        Err(CoreError::InvalidManifest(_))
    ));
    assert!(store.active().is_none());
}

#[test]
fn activation_is_atomic_and_rollback_restores_previous() {
    let mut store = ActivationStore::default();
    let first = manifest("first", "1.0.0");
    store.stage(first.clone()).unwrap();
    assert!(store.active().is_none(), "staging must not activate");
    store.commit().unwrap();
    assert_eq!(store.active(), Some(&first));
    assert!(store.previous().is_none());

    let second = manifest("second", "2.0.0");
    store.stage(second.clone()).unwrap();
    store.commit().unwrap();
    assert_eq!(store.active(), Some(&second));
    assert_eq!(store.previous(), Some(&first));

    store.rollback().unwrap();
    assert_eq!(store.active(), Some(&first));
    assert!(store.staged().is_none());
    assert!(store.previous().is_none());
}

#[test]
fn invalid_state_transitions_are_structured_errors() {
    let mut store = ActivationStore::default();
    assert!(
        matches!(store.commit(), Err(CoreError::InvalidManifest(m)) if m.contains("no staged"))
    );
    assert!(
        matches!(store.rollback(), Err(CoreError::InvalidManifest(m)) if m.contains("nothing to roll back"))
    );
    let good = manifest("good", "1.0.0");
    store.stage(good.clone()).unwrap();
    let bytes = good.to_bytes().unwrap();
    assert_eq!(
        serde_json::from_slice::<PluginManifest>(&bytes).unwrap(),
        good
    );
}

#[test]
fn staging_a_replacement_does_not_deactivate_the_committed_plugin() {
    let mut store = ActivationStore::default();
    let first = manifest("first", "1.0.0");
    store.stage(first.clone()).unwrap();
    store.commit().unwrap();
    let replacement = manifest("replacement", "2.0.0");
    store.stage(replacement).unwrap();
    store.rollback().unwrap();
    assert_eq!(store.active(), Some(&first));
    assert!(store.staged().is_none());
}

#[test]
fn malformed_semver_suffixes_are_rejected() {
    for bad in ["1.0.0-", "1.0.0+", "1.0.0-alpha..1"] {
        let m = manifest("bad", bad);
        assert!(m.verify().is_err(), "accepted invalid semver {bad}");
    }
    let good = manifest("good", "1.0.0-beta.1+build.5");
    assert!(good.verify().is_ok());
}

#[test]
fn denied_artifact_source_blocks_plugin_activation() {
    let mut denied = artifact_source();
    denied.publisher = "Cesium ion".to_string();
    let mut m = manifest("denied", "1.0.0");
    m.artifact_source = denied;
    m.seal();
    assert!(matches!(
        m.verify(),
        Err(genesis_core::CoreError::SourceDenied(_))
    ));
    let mut store = ActivationStore::default();
    assert!(matches!(
        store.stage(m),
        Err(genesis_core::CoreError::SourceDenied(_))
    ));
}
