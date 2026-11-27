use genesis_core::{
    CoreResult, DeliveryCompression, EvidenceKind, GeneratedGeometry, GeneratedKind,
    LicenceElection, Provenance, SourceDescriptor, TargetProfile, build_collision_proxy,
    compression_allowlist, compression_is_allowed, derive_id, generate_lod_chain, verify_mesh,
};

fn mesh() -> GeneratedGeometry {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    for row in 0..8 {
        for col in 0..8 {
            vertices.push([col as f64, row as f64, (row + col) as f64 * 0.01]);
        }
    }
    for row in 0..7 {
        for col in 0..7 {
            let a = (row * 8 + col) as u32;
            let b = a + 1;
            let c = a + 8;
            let d = c + 1;
            indices.extend_from_slice(&[a, b, c, b, d, c]);
        }
    }
    GeneratedGeometry {
        asset_id: derive_id("mesh", "lod-fixture"),
        kind: GeneratedKind::BuildingVolume,
        vertices,
        indices,
        evidence: EvidenceKind::Measured,
        generation_seed: 42,
        generator_version: "procedural-v0.1".to_string(),
        origin_lon_lat: [3.35, 6.60],
        source_object_ids: Vec::new(),
        provenance: Provenance::from(&SourceDescriptor {
            id: "local:lod".to_string(),
            publisher: "Genesis test".to_string(),
            url: "local".to_string(),
            licence: LicenceElection::Other("test".to_string()),
            attribution: "test".to_string(),
            ..Default::default()
        }),
        parameters: Default::default(),
    }
}

#[test]
fn lod_chain_respects_named_budget_and_preserves_source_frame() -> CoreResult<()> {
    let source = mesh();
    let chain = generate_lod_chain(&source, TargetProfile::DesktopPreview)?;
    assert_eq!(chain.len(), 3);
    for level in &chain {
        assert!(level.triangle_count <= level.triangle_budget);
        assert!(level.triangle_count > 0);
        assert_eq!(level.mesh.origin_lon_lat, source.origin_lon_lat);
        assert_eq!(level.mesh.generation_seed, source.generation_seed);
        assert_eq!(level.mesh.provenance, source.provenance);
        assert_eq!(level.mesh.source_object_ids, source.source_object_ids);
        assert_eq!(level.mesh.parameters["lod_level"], level.name);
        verify_mesh(&level.mesh)?;
    }
    assert!(chain[0].triangle_count > chain[1].triangle_count);
    assert!(chain[1].triangle_count > chain[2].triangle_count);
    let again = generate_lod_chain(&source, TargetProfile::DesktopPreview)?;
    assert_eq!(chain, again, "LOD generation must be deterministic");
    assert!(
        generate_lod_chain(&source, TargetProfile::DesktopGameReady)?[0].triangle_count
            > chain[0].triangle_count
    );
    Ok(())
}

#[test]
fn collision_proxy_encloses_source_and_measures_coverage() -> CoreResult<()> {
    let source = mesh();
    let proxy = build_collision_proxy(&source, 0.01)?;
    assert_eq!(proxy.coverage, 1.0);
    assert_eq!(
        proxy.sample_count,
        source.vertices.len() + source.indices.len() / 3
    );
    assert!(proxy.min[0] <= 0.0 && proxy.max[0] >= 7.0);
    let mut smaller = proxy.clone();
    smaller.max = [0.0, 0.0, 0.0];
    assert!(
        smaller.coverage_of(&source).unwrap() < 1.0,
        "shrunk proxy must report incomplete coverage"
    );
    assert!(build_collision_proxy(&source, -0.1).is_err());
    Ok(())
}

#[test]
fn compression_allowlist_is_licence_aware() {
    let allowlist = compression_allowlist();
    assert_eq!(allowlist.len(), 4);
    assert!(
        allowlist
            .iter()
            .any(|(c, note)| *c == DeliveryCompression::Draco && note.contains("Apache"))
    );
    assert!(compression_is_allowed(DeliveryCompression::Meshopt, true));
    assert!(!compression_is_allowed(DeliveryCompression::Meshopt, false));
    assert!(!compression_is_allowed(DeliveryCompression::None, false));
}

#[test]
fn invalid_mesh_indices_are_errors_not_panics() -> CoreResult<()> {
    let mut bad = mesh();
    bad.indices[0] = 999;
    assert!(generate_lod_chain(&bad, TargetProfile::DesktopPreview).is_err());
    let proxy = build_collision_proxy(&mesh(), 0.01)?;
    assert!(proxy.coverage_of(&bad).is_err());
    Ok(())
}
