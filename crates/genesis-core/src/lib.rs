//! Genesis Core — project model and portable local layout (GE-0010).
//!
//! This is the minimal headless foundation for the first vertical slice. The UI
//! must never own this state; Studio, CLI and SDK all call these functions.

pub mod lawful_source;
pub mod world_ir;

pub use lawful_source::{
    LicenceElection, SourceDescriptor, SourcePurpose, attribution_sidecar, check_source,
};
pub use world_ir::{
    AssetRef, CURRENT_SCHEMA_VERSION, Dimensions, EvidenceKind, LEGACY_SCHEMA_VERSION, LocalFrame,
    Provenance, StableId, ValidationState, WorldIr, WorldLayer, WorldObject, derive_id,
    migrate_backward, migrate_forward,
};

use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeSet,
    fmt, fs, io,
    path::{Component, Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

pub const SUPPORTED_SCHEMA_VERSION: u32 = 1;
pub const MANIFEST_PATH: &str = "metadata/project.json";
pub const LAYOUT_DIRECTORIES: [&str; 7] = [
    "metadata",
    "worlds",
    "payloads",
    "sources",
    "jobs",
    "validation",
    "exports",
];

fn default_unit() -> String {
    "metre".to_string()
}
fn default_up_axis() -> String {
    "Z".to_string()
}

/// Coordinate reference system metadata that must travel with the project.
/// Authoritative coordinates are high precision; the local frame origin is
/// recorded by the World IR (ADR-0008).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CrsMetadata {
    pub wkt2: Option<String>,
    pub projjson: Option<String>,
    pub epoch: Option<f64>,
    pub vertical_datum: Option<String>,
    #[serde(default = "default_unit")]
    pub unit: String,
    #[serde(default = "default_up_axis")]
    pub up_axis: String,
}

impl Default for CrsMetadata {
    fn default() -> Self {
        Self {
            wkt2: None,
            projjson: None,
            epoch: None,
            vertical_datum: None,
            unit: default_unit(),
            up_axis: default_up_axis(),
        }
    }
}

/// Portable project manifest. Contains no inline geometry: binary payloads are
/// referenced by relative paths (ADR-0002).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectManifest {
    pub schema_version: u32,
    pub id: String,
    pub name: String,
    pub created_at_unix: u64,
    pub crs: CrsMetadata,
    pub layout: Vec<String>,
}

#[derive(Debug)]
pub enum CoreError {
    Io(io::Error),
    Json(serde_json::Error),
    UnsupportedSchemaVersion { found: u32, supported: u32 },
    InvalidManifest(String),
    PathEscapesProject(String),
    SourceDenied(String),
    AlreadyExists(PathBuf),
}

impl fmt::Display for CoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(e) => write!(f, "I/O error: {e}"),
            Self::Json(e) => write!(f, "manifest JSON error: {e}"),
            Self::UnsupportedSchemaVersion { found, supported } => write!(
                f,
                "unsupported project schema version {found} (this build supports {supported})"
            ),
            Self::InvalidManifest(msg) => write!(f, "invalid project manifest: {msg}"),
            Self::PathEscapesProject(path) => write!(f, "path escapes project root: {path}"),
            Self::SourceDenied(reason) => {
                write!(f, "source denied by lawful-source policy: {reason}")
            }
            Self::AlreadyExists(path) => write!(f, "project already exists at {}", path.display()),
        }
    }
}

impl std::error::Error for CoreError {}
impl From<io::Error> for CoreError {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}
impl From<serde_json::Error> for CoreError {
    fn from(e: serde_json::Error) -> Self {
        Self::Json(e)
    }
}

pub type Result<T> = std::result::Result<T, CoreError>;
/// Explicit alias used by downstream tests and adapters.
pub type CoreResult<T> = Result<T>;

/// A project opened from a portable directory. All paths are relative to `root`.
#[derive(Debug, Clone)]
pub struct Project {
    root: PathBuf,
    manifest: ProjectManifest,
}

impl Project {
    pub fn root(&self) -> &Path {
        &self.root
    }
    pub fn manifest(&self) -> &ProjectManifest {
        &self.manifest
    }

    /// Resolve a project-relative path, rejecting traversal and absolute paths.
    pub fn resolve(&self, relative: &Path) -> Result<PathBuf> {
        validate_relative_path(relative)?;
        Ok(self.root.join(relative))
    }

    /// Persist the manifest atomically (temp file then rename).
    pub fn save(&self) -> Result<()> {
        let manifest_path = self.root.join(MANIFEST_PATH);
        let bytes = serde_json::to_vec_pretty(&self.manifest)?;
        atomic_write(&manifest_path, &bytes)
    }
}

/// Reject absolute paths, drive prefixes and `..` components.
pub fn validate_relative_path(path: &Path) -> Result<()> {
    if path.as_os_str().is_empty() {
        return Err(CoreError::PathEscapesProject("(empty path)".to_string()));
    }
    // Cross-platform containment: a Windows drive prefix or backslash path is
    // a Prefix component only on Windows, so check the raw string everywhere.
    let raw = path.to_string_lossy();
    let bytes = raw.as_bytes();
    if raw.starts_with('/')
        || raw.contains('\\')
        || (bytes.len() >= 2 && bytes[1] == b':' && bytes[0].is_ascii_alphabetic())
    {
        return Err(CoreError::PathEscapesProject(raw.to_string()));
    }
    for component in path.components() {
        match component {
            Component::Normal(_) | Component::CurDir => {}
            Component::ParentDir | Component::RootDir | Component::Prefix(_) => {
                return Err(CoreError::PathEscapesProject(path.display().to_string()));
            }
        }
    }
    Ok(())
}

/// Create a new project directory tree and manifest.
pub fn create_project(root: &Path, name: &str, crs: CrsMetadata) -> Result<Project> {
    if name.trim().is_empty() {
        return Err(CoreError::InvalidManifest(
            "name must not be empty".to_string(),
        ));
    }
    if crs.unit.trim().is_empty() || crs.up_axis.trim().is_empty() {
        return Err(CoreError::InvalidManifest(
            "CRS unit and up-axis must be recorded".to_string(),
        ));
    }
    if let Some(epoch) = crs.epoch
        && !epoch.is_finite()
    {
        return Err(CoreError::InvalidManifest(
            "CRS epoch must be finite".to_string(),
        ));
    }
    let manifest_path = root.join(MANIFEST_PATH);
    if manifest_path.exists() {
        return Err(CoreError::AlreadyExists(root.to_path_buf()));
    }
    for dir in LAYOUT_DIRECTORIES {
        fs::create_dir_all(root.join(dir))?;
    }
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let manifest = ProjectManifest {
        schema_version: SUPPORTED_SCHEMA_VERSION,
        id: format!("ge-{}-{timestamp}", sanitise_id(name)),
        name: name.to_string(),
        created_at_unix: timestamp,
        crs,
        layout: LAYOUT_DIRECTORIES.iter().map(|d| d.to_string()).collect(),
    };
    let project = Project {
        root: root.to_path_buf(),
        manifest,
    };
    project.save()?;
    Ok(project)
}

/// Open a project from a directory. Never consult a global cache.
pub fn open_project(root: &Path) -> Result<Project> {
    let manifest_path = root.join(MANIFEST_PATH);
    if !manifest_path.is_file() {
        return Err(CoreError::InvalidManifest(format!(
            "missing {}",
            MANIFEST_PATH
        )));
    }
    let bytes = fs::read(&manifest_path)?;
    let manifest: ProjectManifest = serde_json::from_slice(&bytes)?;
    if manifest.schema_version != SUPPORTED_SCHEMA_VERSION {
        return Err(CoreError::UnsupportedSchemaVersion {
            found: manifest.schema_version,
            supported: SUPPORTED_SCHEMA_VERSION,
        });
    }
    if manifest.name.trim().is_empty() || manifest.id.trim().is_empty() {
        return Err(CoreError::InvalidManifest(
            "name and id must not be empty".to_string(),
        ));
    }
    if manifest.crs.unit.trim().is_empty() || manifest.crs.up_axis.trim().is_empty() {
        return Err(CoreError::InvalidManifest(
            "CRS unit and up-axis must be recorded".to_string(),
        ));
    }
    let expected: BTreeSet<&str> = LAYOUT_DIRECTORIES.iter().copied().collect();
    let mut seen = BTreeSet::new();
    for entry in &manifest.layout {
        validate_relative_path(Path::new(entry))?;
        if !seen.insert(entry.as_str()) {
            return Err(CoreError::InvalidManifest(
                "layout directory list contains duplicates".to_string(),
            ));
        }
        if !root.join(entry).is_dir() {
            return Err(CoreError::InvalidManifest(format!(
                "layout directory missing on disk: {entry}"
            )));
        }
    }
    let actual: BTreeSet<&str> = seen;
    if expected != actual {
        return Err(CoreError::InvalidManifest(
            "layout directory list does not match the supported layout".to_string(),
        ));
    }
    Ok(Project {
        root: root.to_path_buf(),
        manifest,
    })
}

fn sanitise_id(name: &str) -> String {
    let mut out: String = name
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
        .collect();
    out.truncate(32);
    if out.is_empty() {
        "project".to_string()
    } else {
        out
    }
}

fn atomic_write(path: &Path, bytes: &[u8]) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let tmp = path.with_extension("json.tmp");
    fs::write(&tmp, bytes)?;
    match fs::rename(&tmp, path) {
        Ok(()) => Ok(()),
        Err(_e) if cfg!(windows) && path.exists() => {
            // Windows rename cannot replace an existing file. Preserve the
            // only valid manifest as a backup, then restore it if the
            // replacement rename fails. Never delete the original first.
            let backup = path.with_extension("json.bak");
            let _ = fs::remove_file(&backup);
            fs::rename(path, &backup)?;
            if let Err(e) = fs::rename(&tmp, path) {
                let _ = fs::rename(&backup, path);
                return Err(CoreError::Io(e));
            }
            let _ = fs::remove_file(&backup);
            Ok(())
        }
        Err(e) => Err(CoreError::Io(e)),
    }
}
