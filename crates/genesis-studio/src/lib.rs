//! Studio scene snapshot seam (GE-0016).
//!
//! Genesis Core owns all domain state. This crate only projects Core's World IR
//! and generated geometry into a versioned, serializable snapshot that a
//! renderer can display. No CRS maths, provenance rules or reconstruction logic
//! live here, and none live in the React frontend either.
//!
//! The Tauri command wrappers are behind the `tauri-bindings` feature so the
//! default build stays headless and portable.

pub mod scene;

#[cfg(feature = "tauri-bindings")]
pub mod commands;

#[cfg(feature = "tauri-bindings")]
pub use commands::{export_scene, open_scene, select_scene_node};
pub use scene::{
    CapabilityReport, Result, SNAPSHOT_SCHEMA_VERSION, SceneError, SceneMesh, SceneNode,
    SceneNodeKind, SceneSnapshot, build_scene_snapshot, capability_report, scene_from_documents,
    with_selection,
};
