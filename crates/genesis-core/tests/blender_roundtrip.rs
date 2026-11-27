//! GE-0029: a Core-exported GLB survives a pinned Blender import/export cycle
//! and remains importable by Core with provenance intact. The Blender-side
//! probe (`tools/blender_roundtrip.py`) produced the fixture under
//! Blender 4.5.17 LTS; Core never links GPL code.

use genesis_core::{CoreResult, import_glb};
use serde_json::Value;

const GLB: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/ikeja-blender-roundtrip.glb"
);
const REPORT: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/ikeja-blender-roundtrip-report.json"
);

#[test]
fn blender_exported_glb_reimports_with_provenance() -> CoreResult<()> {
    let bytes = std::fs::read(GLB)?;
    assert_eq!(&bytes[0..4], b"glTF", "fixture must be a binary GLB");
    let imported = import_glb(&bytes)?;
    assert_eq!(imported.version, "2.0");
    assert_eq!(imported.mesh_count, 2);
    assert_eq!(imported.triangle_count, 14);
    assert_eq!(imported.provenance.len(), 2);
    for record in &imported.provenance {
        let genesis = record;
        assert!(genesis["asset_id"].as_str().is_some_and(|s| !s.is_empty()));
        assert_eq!(genesis["generator_version"], "procedural-v0.1");
        assert_eq!(
            genesis["provenance"]["attribution"],
            "© OpenStreetMap contributors"
        );
        assert_eq!(genesis["provenance"]["licence_elected"], "ODbL-1.0");
    }
    Ok(())
}

#[test]
fn pinned_blender_report_records_no_metadata_loss() -> CoreResult<()> {
    let report: Value = serde_json::from_slice(&std::fs::read(REPORT)?)?;
    assert_eq!(report["schema_version"], 1);
    assert_eq!(report["blender_version"], "4.5.17 LTS");
    assert_eq!(report["objects_before"], 2);
    assert_eq!(report["objects_after"], 2);
    assert_eq!(report["object_names_preserved"], true);
    assert_eq!(report["custom_properties_preserved"], true);
    assert_eq!(report["metadata_identical"], true);
    assert_eq!(report["metadata_loss"].as_array().unwrap().len(), 0);
    assert!(report["limitations"].as_array().unwrap().len() > 0);
    Ok(())
}
