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

/// Subcommand aliases map to canonical command names in the JSON envelope.
fn command_for<'a>(args: &[&'a str]) -> &'a str {
    match args[0] {
        "project" => "project_create",
        "boundary" => "boundary_import",
        "features" => "features_import",
        "job" => "job_status",
        "job-cancel" => "job_cancel",
        other => other,
    }
}

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
    assert_eq!(
        value["command"],
        command_for(args),
        "command envelope must match invocation"
    );
    assert!(
        value["status"] == "ok" || value["status"] == "unsupported",
        "unexpected status: {}",
        value["status"]
    );
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

    let created = run(&["project", project_s, "Ikeja CLI", "--datum", "EGM96"]);
    assert_eq!(created["data"]["schema_version"], 1);
    let boundary = run(&["boundary", project_s, BOUNDARY, "Ikeja study area"]);
    assert_eq!(boundary["data"]["bbox"][0], 3.348);
    let features = run(&[
        "features",
        project_s,
        OSM,
        "--source-id",
        "osm:fixture:ikeja",
        "--licence",
        "odbl",
        "--attribution",
        "© OpenStreetMap contributors",
    ]);
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
    let reproducibility = &scene["data"]["reproducibility"];
    assert_eq!(reproducibility["generator_versions"][0], "procedural-v0.1");
    assert_eq!(reproducibility["seeds"].as_array().unwrap().len(), 2);
    assert_eq!(reproducibility["source_ids"][0], "osm:fixture:ikeja");
    assert_eq!(reproducibility["subject_hash"].as_str().unwrap().len(), 64);

    let imported = import_glb(&std::fs::read(&glb).unwrap()).unwrap();
    assert_eq!(imported.mesh_count, 2);
    assert_eq!(imported.triangle_count, 14);
    assert_eq!(
        imported.provenance[0]["provenance"]["attribution"],
        "© OpenStreetMap contributors"
    );

    let job = run(&["job", project_s, "job-1"]);
    assert_eq!(job["status"], "unsupported");
    let cancel = run(&["job-cancel", project_s, "job-1"]);
    assert_eq!(cancel["status"], "unsupported");

    assert!(Path::new(&glb).exists());
    assert!(project.join("validation/report.json").exists());
    // CRS and provenance supplied at create/import time are persisted and reused.
    let manifest: Value =
        serde_json::from_slice(&std::fs::read(project.join("metadata/project.json")).unwrap())
            .unwrap();
    assert_eq!(manifest["crs"]["vertical_datum"], "EGM96");
    let stored_source: Value =
        serde_json::from_slice(&std::fs::read(project.join("metadata/source.json")).unwrap())
            .unwrap();
    assert_eq!(stored_source["attribution"], "© OpenStreetMap contributors");
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn invalid_profile_is_a_structured_error() {
    let root = std::env::temp_dir().join(format!("genesis-cli-profile-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    let project = root.join("project");
    let project_s = project.to_str().unwrap();
    run(&["project", project_s, "Profile test"]);
    let output = Command::new(env!("CARGO_BIN_EXE_genesis"))
        .args(["validate", project_s, "--profile", "bogus"])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let value: Value = serde_json::from_slice(&output.stderr).unwrap();
    assert_eq!(value["status"], "error");
    assert_eq!(value["schema_version"], 1);
    assert_eq!(value["error"]["kind"], "cli_error");
    assert!(
        value["error"]["message"]
            .as_str()
            .unwrap()
            .contains("unknown profile 'bogus'")
    );
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn failed_import_does_not_mutate_persisted_source() {
    let root = std::env::temp_dir().join(format!(
        "genesis-cli-source-{}-{}",
        std::process::id(),
        FAILED_IMPORT_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
    ));
    let _ = std::fs::remove_dir_all(&root);
    let project = root.join("project");
    let project_s = project.to_str().unwrap();
    run(&["project", project_s, "Atomic source"]);
    run(&[
        "features",
        project_s,
        OSM,
        "--source-id",
        "osm:fixture:ikeja",
        "--attribution",
        "ORIGINAL",
    ]);
    let failed = Command::new(env!("CARGO_BIN_EXE_genesis"))
        .args([
            "features",
            project_s,
            "/definitely/missing.geojson",
            "--source-id",
            "osm:fixture:ikeja",
            "--attribution",
            "CHANGED",
        ])
        .output()
        .unwrap();
    assert!(!failed.status.success());
    let stored: Value =
        serde_json::from_slice(&std::fs::read(project.join("metadata/source.json")).unwrap())
            .unwrap();
    assert_eq!(stored["attribution"], "ORIGINAL");
    let _ = std::fs::remove_dir_all(&root);
}

static FAILED_IMPORT_COUNTER: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

#[cfg(unix)]
#[test]
fn symlinked_source_destination_is_rejected() {
    use std::os::unix::fs::symlink;
    let root = std::env::temp_dir().join(format!("genesis-cli-symlink-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    let project = root.join("project");
    let project_s = project.to_str().unwrap();
    let outside = root.join("outside.geojson");
    std::fs::write(&outside, "original").unwrap();
    run(&["project", project_s, "Symlink test"]);
    std::fs::create_dir_all(project.join("sources")).unwrap();
    symlink(&outside, project.join("sources/boundary.geojson")).unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_genesis"))
        .args(["boundary", project_s, BOUNDARY, "Ikeja"])
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("symlink"));
    assert_eq!(std::fs::read_to_string(&outside).unwrap(), "original");
    let _ = std::fs::remove_dir_all(&root);
}

#[cfg(unix)]
#[test]
fn non_utf8_argument_does_not_panic() {
    use std::os::unix::ffi::OsStringExt;
    let arg = std::ffi::OsString::from_vec(vec![0xff, b'i', b'n', b'v', b'a', b'l', b'i', b'd']);
    let output = Command::new(env!("CARGO_BIN_EXE_genesis"))
        .arg(arg)
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !stderr.contains("panicked"),
        "non-UTF8 argv panicked: {stderr}"
    );
}

#[test]
fn export_without_a_passing_validation_report_is_a_structured_error() {
    let root = std::env::temp_dir().join(format!("genesis-cli-no-report-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    let project = root.join("project");
    let project_s = project.to_str().unwrap();
    run(&["project", project_s, "No report"]);
    run(&[
        "boundary",
        project_s,
        BOUNDARY,
        "Ikeja",
        "--source-id",
        "osm:fixture:ikeja",
        "--licence",
        "odbl",
        "--attribution",
        "© OpenStreetMap contributors",
    ]);
    run(&[
        "features",
        project_s,
        OSM,
        "--source-id",
        "osm:fixture:ikeja",
        "--licence",
        "odbl",
        "--attribution",
        "© OpenStreetMap contributors",
    ]);
    run(&["generate", project_s, "42"]);
    let output = Command::new(env!("CARGO_BIN_EXE_genesis"))
        .args([
            "export",
            project_s,
            root.join("world.glb").to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let value: Value = serde_json::from_slice(&output.stderr).unwrap();
    assert_eq!(value["status"], "error");
    assert_eq!(value["command"], "export");
    assert_eq!(value["error"]["kind"], "cli_error");
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn corrupt_world_ir_is_rejected_by_the_headless_path() {
    let root = std::env::temp_dir().join(format!("genesis-cli-ir-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    let project = root.join("project");
    let project_s = project.to_str().unwrap();
    run(&["project", project_s, "Corrupt IR"]);
    std::fs::create_dir_all(project.join("worlds")).unwrap();
    std::fs::write(
        project.join("worlds/world.json"),
        r#"{"schema_version":999,"world_id":"x","crs":{},"local_frame":{},"layers":[]}"#,
    )
    .unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_genesis"))
        .args(["scene", project_s])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let value: Value = serde_json::from_slice(&output.stderr).unwrap();
    assert_eq!(value["status"], "error");
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn features_only_project_uses_a_feature_derived_frame() {
    let root = std::env::temp_dir().join(format!("genesis-cli-no-boundary-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    let project = root.join("project");
    let project_s = project.to_str().unwrap();
    run(&["project", project_s, "No boundary"]);
    run(&[
        "features",
        project_s,
        OSM,
        "--source-id",
        "osm:fixture:ikeja",
        "--licence",
        "odbl",
        "--attribution",
        "© OpenStreetMap contributors",
    ]);
    run(&["generate", project_s, "42"]);
    let meshes: Value =
        serde_json::from_slice(&std::fs::read(project.join("payloads/meshes.json")).unwrap())
            .unwrap();
    let origin = meshes[0]["origin_lon_lat"].as_array().unwrap();
    assert_ne!(origin[0].as_f64(), Some(0.0));
    assert_ne!(origin[1].as_f64(), Some(0.0));
    let east = meshes[0]["vertices"][0][0].as_f64().unwrap();
    assert!(east.abs() < 1_000.0, "local metres expected, got {east}");
    let _ = std::fs::remove_dir_all(&root);
}
