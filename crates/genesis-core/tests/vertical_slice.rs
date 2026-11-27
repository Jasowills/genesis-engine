//! GE-0020 — first end-to-end vertical slice, driven headlessly through Core.
//! Steps mirror spec §50: project → boundary → OSM → IR → procedural →
//! validation → GLB export → re-import. No UI dependency.

use genesis_core::{
    CrsMetadata, LicenceElection, LocalFrame, QualityProfile, SourceDescriptor, WorldIr,
    WorldLayer, create_project, derive_id, export_glb, generate_building_volumes,
    generate_road_surfaces, import_geojson, import_glb, import_osm_geojson, to_world_objects,
    validate_world,
};

const BOUNDARY: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/ikeja-boundary.geojson"
);
const OSM: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/ikeja-small.geojson"
);

fn source() -> SourceDescriptor {
    SourceDescriptor {
        id: "osm:fixture:ikeja".to_string(),
        publisher: "OpenStreetMap contributors".to_string(),
        url: "https://www.openstreetmap.org/copyright".to_string(),
        version: "2026-11-27".to_string(),
        licence: LicenceElection::Odbl10,
        attribution: "© OpenStreetMap contributors".to_string(),
        ..Default::default()
    }
}

fn crs() -> CrsMetadata {
    CrsMetadata {
        wkt2: Some("GEOGCRS[\"WGS 84\"]".to_string()),
        epoch: Some(2024.5),
        vertical_datum: Some("EGM2008".to_string()),
        ..CrsMetadata::default()
    }
}

fn sha256(bytes: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    format!("{:x}", Sha256::digest(bytes))
}

#[test]
fn first_vertical_slice_runs_end_to_end() -> Result<(), Box<dyn std::error::Error>> {
    let started = std::time::Instant::now();
    let root = std::env::temp_dir().join(format!("genesis-e2e-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);

    // 1-4: project create, boundary import, CRS validation.
    let project = create_project(&root, "Ikeja Study", crs())?;
    assert_eq!(project.manifest().name, "Ikeja Study");
    let boundary_bytes = std::fs::read(BOUNDARY)?;
    let boundary = import_geojson(&boundary_bytes, "Ikeja study area", &source(), &crs())?;
    assert_eq!(boundary.bbox, [3.3480, 6.5985, 3.3552, 6.6051]);

    // 5-6: OSM features → World IR objects.
    let osm_bytes = std::fs::read(OSM)?;
    let features = import_osm_geojson(&osm_bytes, &source(), &crs())?;
    assert_eq!(features.len(), 2);
    let objects = to_world_objects(&features, &source())?;
    let world = WorldIr {
        schema_version: genesis_core::CURRENT_SCHEMA_VERSION,
        world_id: derive_id("world", "ikeja-study"),
        crs: crs(),
        local_frame: LocalFrame::default(),
        layers: vec![WorldLayer {
            id: derive_id("layer", "osm-features"),
            objects,
        }],
    };
    let restored = WorldIr::from_bytes(&world.to_bytes()?)?;
    assert_eq!(restored, world);

    // 7-8: procedural geometry from the same sourced features.
    let buildings = generate_building_volumes(&features, &source(), 42)?;
    let roads = generate_road_surfaces(&features, &source(), 42, 8.0)?;
    assert_eq!((buildings.len(), roads.len()), (1, 1));

    // 9-10: validation with honest accuracy reporting.
    let mut meshes = buildings.clone();
    meshes.extend(roads.clone());
    let report = validate_world(
        &restored,
        QualityProfile::GameReady,
        &meshes,
        false,
        "e2e-1.0",
    );
    assert_eq!(report.geographic_accuracy_claim, "none");
    assert!(report.passed);

    // 11-12: GLB export and re-import.
    let exported = export_glb(&meshes, &restored, &[source()], &report)?;
    let imported = import_glb(&exported.bytes)?;
    assert_eq!(imported.mesh_count, 2);
    assert_eq!(imported.triangle_count, 14);
    assert!(
        imported
            .provenance
            .iter()
            .all(|p| p["provenance"]["attribution"] == "© OpenStreetMap contributors")
    );

    // 13: reproducible evidence record (no accuracy or competitor claim).
    let elapsed_ms = started.elapsed().as_secs_f64() * 1000.0;
    let report_json = serde_json::json!({
        "ticket": "GE-0020",
        "boundary": {"bbox": boundary.bbox, "source": "research/BENCHMARK_AREA.md (D-031)"},
        "inputs": {
            "boundary_sha256": sha256(&boundary_bytes),
            "osm_sha256": sha256(&osm_bytes)
        },
        "steps": ["project", "boundary", "crs", "osm", "world_ir", "procedural", "validation", "glb_export", "glb_reimport"],
        "mesh_count": imported.mesh_count,
        "triangle_count": imported.triangle_count,
        "geographic_accuracy_claim": report.geographic_accuracy_claim,
        "ground_truth_available": false,
        "elapsed_ms": elapsed_ms,
        "benchmark_comparisons": [],
        "benchmark_note": "No competitor outputs from the same frozen inputs; no accuracy claim. Cross-tool comparison deferred.",
        "platform": std::env::consts::OS,
        "architecture": std::env::consts::ARCH,
        "memory_and_disk_peak": "UNVERIFIED: no allocator/disk telemetry in this slice"
    });
    if let Ok(path) = std::env::var("GENESIS_SLICE_REPORT") {
        std::fs::write(path, serde_json::to_vec_pretty(&report_json)?)?;
    }
    println!("{}", serde_json::to_string_pretty(&report_json)?);

    let _ = std::fs::remove_dir_all(&root);
    Ok(())
}
