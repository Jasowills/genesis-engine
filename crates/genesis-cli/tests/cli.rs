//! GE-0022 integration test: every E2E step is reachable headlessly through the CLI.

use genesis_core::import_glb;
use serde_json::Value;
use std::{path::Path, process::Command};

const BOUNDARY: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/ikeja-boundary.geojson"
);
const OSM: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/ikeja-small.geojson"
);

fn run(args: &[&str]) -> Value {
    let output = Command::new(env!("CARGO_BIN_EXE_genesis"))
        .args(args)
        .output()
        .expect("run genesis CLI");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        output.status.success(),
        "CLI failed: {args:?}\nstdout: {stdout}\nstderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let value: Value = serde_json::from_str(&stdout).expect("machine-readable JSON output");
    assert_eq!(value["schema_version"], 1);
    assert_eq!(value["status"], "ok");
    value
}

#[test]
fn cli_drives_the_full_slice_headlessly() {
    let root = std::env::temp_dir().join(format!("genesis-cli-e2e-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    let project = root.join("project");
    let project_s = project.to_str().unwrap();
    let glb = root.join("world.glb");
    let glb_s = glb.to_str().unwrap();

    let created = run(&["project", project_s, "Ikeja CLI", "--datum", "EGM2008"]);
    assert_eq!(created["data"]["schema_version"], 1);
    let boundary = run(&["boundary", project_s, BOUNDARY, "Ikeja study area"]);
    assert_eq!(boundary["data"]["bbox"][0], 3.348);
    let features = run(&["features", project_s, OSM]);
    assert_eq!(features["data"]["feature_count"], 2);
    let generated = run(&["generate", project_s, "42", "--road-width", "8"]);
    assert_eq!(generated["data"]["mesh_count"], 2);
    let validated = run(&["validate", project_s, "--profile", "game_ready"]);
    assert_eq!(validated["data"]["accuracy_claim"], "none");
    assert_eq!(validated["data"]["passed"], true);
    let exported = run(&["export", project_s, glb_s]);
    assert_eq!(exported["data"]["gltf_version"], "2.0");
    assert_eq!(exported["data"]["mesh_count"], 2);

    let scene = run(&["scene", project_s]);
    assert_eq!(scene["data"]["mesh_count"], 2);
    assert_eq!(scene["data"]["ids"].as_array().unwrap().len(), 2);

    let imported = import_glb(&std::fs::read(&glb).unwrap()).unwrap();
    assert_eq!(imported.mesh_count, 2);
    assert_eq!(imported.triangle_count, 14);
    assert_eq!(
        imported.provenance[0]["provenance"]["attribution"],
        "© OpenStreetMap contributors"
    );

    let job = run(&["job", project_s, "job-1"]);
    assert_eq!(job["data"]["status"], "unsupported");

    assert!(Path::new(&glb).exists());
    assert!(project.join("validation/report.json").exists());
    assert!(project.join("exports").is_dir());
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn invalid_profile_is_a_structured_error() {
    let output = Command::new(env!("CARGO_BIN_EXE_genesis"))
        .args(["validate", "/nonexistent", "--profile", "bogus"])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let value: Value = serde_json::from_slice(&output.stderr).unwrap();
    assert_eq!(value["status"], "error");
    assert_eq!(value["schema_version"], 1);
}
