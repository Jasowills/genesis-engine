use genesis_core::{
    CoreError, CoreResult, CrsMetadata, LAYOUT_DIRECTORIES, MANIFEST_PATH,
    SUPPORTED_SCHEMA_VERSION, create_project, open_project, validate_relative_path,
};
use std::{
    fs,
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
};

static COUNTER: AtomicU64 = AtomicU64::new(0);

struct TestDir(PathBuf);

impl TestDir {
    fn new(tag: &str) -> Self {
        let n = COUNTER.fetch_add(1, Ordering::Relaxed);
        let dir =
            std::env::temp_dir().join(format!("genesis-ge0010-{tag}-{}-{n}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).expect("test dir");
        Self(dir)
    }
    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TestDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn crs() -> CrsMetadata {
    CrsMetadata {
        wkt2: Some("GEOGCRS[\"WGS 84\"]".to_string()),
        projjson: None,
        epoch: Some(2024.5),
        vertical_datum: Some("EGM2008".to_string()),
        ..CrsMetadata::default()
    }
}

#[test]
fn create_open_save_round_trip() -> CoreResult<()> {
    let dir = TestDir::new("roundtrip");
    let project = create_project(dir.path(), "Ikeja Demo", crs())?;
    for entry in LAYOUT_DIRECTORIES {
        assert!(dir.path().join(entry).is_dir(), "missing {entry}");
    }
    assert!(dir.path().join(MANIFEST_PATH).is_file());

    let reopened = open_project(dir.path())?;
    assert_eq!(reopened.manifest(), project.manifest());
    assert_eq!(reopened.manifest().name, "Ikeja Demo");
    assert_eq!(
        reopened.manifest().crs.vertical_datum.as_deref(),
        Some("EGM2008")
    );
    assert_eq!(reopened.manifest().schema_version, SUPPORTED_SCHEMA_VERSION);

    let resolved = reopened.resolve(Path::new("worlds/world.json"))?;
    assert_eq!(resolved, dir.path().join("worlds/world.json"));
    reopened.save()?;
    assert_eq!(open_project(dir.path())?.manifest(), project.manifest());
    Ok(())
}

#[test]
fn project_is_portable_after_move() -> CoreResult<()> {
    let dir = TestDir::new("move");
    let original = create_project(dir.path(), "Portable", crs())?;
    let moved = dir.path().with_extension("moved");
    let _ = fs::remove_dir_all(&moved);
    fs::rename(dir.path(), &moved)?;

    let reopened = open_project(&moved)?;
    assert_eq!(reopened.manifest(), original.manifest());
    assert!(moved.join("payloads").is_dir());
    assert!(
        !reopened
            .manifest()
            .layout
            .iter()
            .any(|p| Path::new(p).is_absolute())
    );

    fs::remove_dir_all(&moved)?;
    Ok(())
}

#[test]
fn unknown_schema_version_is_structured_error() -> CoreResult<()> {
    let dir = TestDir::new("version");
    create_project(dir.path(), "Versioned", crs())?;
    let mut manifest: serde_json::Value =
        serde_json::from_slice(&fs::read(dir.path().join(MANIFEST_PATH))?)?;
    manifest["schema_version"] = serde_json::json!(999);
    fs::write(
        dir.path().join(MANIFEST_PATH),
        serde_json::to_vec_pretty(&manifest)?,
    )?;

    match open_project(dir.path()) {
        Err(CoreError::UnsupportedSchemaVersion { found, supported }) => {
            assert_eq!(found, 999);
            assert_eq!(supported, SUPPORTED_SCHEMA_VERSION);
        }
        other => panic!("expected structured version error, got {other:?}"),
    }
    Ok(())
}

#[test]
fn malformed_manifest_is_structured_error() -> CoreResult<()> {
    let dir = TestDir::new("malformed");
    create_project(dir.path(), "Malformed", crs())?;
    fs::write(dir.path().join(MANIFEST_PATH), b"{ not json")?;
    assert!(matches!(open_project(dir.path()), Err(CoreError::Json(_))));

    fs::write(dir.path().join(MANIFEST_PATH), b"{\"schema_version\":1}")?;
    assert!(matches!(
        open_project(dir.path()),
        Err(CoreError::Json(_)) | Err(CoreError::InvalidManifest(_))
    ));

    fs::remove_file(dir.path().join(MANIFEST_PATH))?;
    assert!(matches!(
        open_project(dir.path()),
        Err(CoreError::InvalidManifest(_))
    ));
    Ok(())
}

#[test]
fn path_containment_rejects_traversal_and_absolute_paths() {
    assert!(validate_relative_path(Path::new("worlds/world.json")).is_ok());
    assert!(matches!(
        validate_relative_path(Path::new("../outside")),
        Err(CoreError::PathEscapesProject(_))
    ));
    assert!(matches!(
        validate_relative_path(Path::new("/etc/passwd")),
        Err(CoreError::PathEscapesProject(_))
    ));
    assert!(matches!(
        validate_relative_path(Path::new("C:secret")),
        Err(CoreError::PathEscapesProject(_))
    ));
    assert!(matches!(
        validate_relative_path(Path::new("")),
        Err(CoreError::PathEscapesProject(_))
    ));
}

#[test]
fn duplicate_project_and_empty_name_are_errors() -> CoreResult<()> {
    let dir = TestDir::new("duplicate");
    create_project(dir.path(), "First", crs())?;
    assert!(matches!(
        create_project(dir.path(), "Second", crs()),
        Err(CoreError::AlreadyExists(_))
    ));
    let other = TestDir::new("empty-name");
    assert!(matches!(
        create_project(other.path(), "  ", crs()),
        Err(CoreError::InvalidManifest(_))
    ));
    Ok(())
}
