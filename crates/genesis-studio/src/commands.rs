//! Thin Tauri command wrappers (GE-0016).
//!
//! Commands read files and call the same pure functions used by headless tests.
//! All parsing, validation and derivation remains in Genesis Core.

use crate::scene::{SceneSnapshot, scene_from_documents, with_selection};

fn command_error(context: impl std::fmt::Display, error: impl std::fmt::Display) -> String {
    format!("{context}: {error}")
}

/// Open a versioned scene document pair: `<path>` is a World IR JSON document
/// and `<stem>.meshes.json` holds Core-generated geometry.
#[tauri::command]
pub fn open_scene(path: String) -> std::result::Result<SceneSnapshot, String> {
    let document = std::path::Path::new(&path);
    let meshes_path = document.with_extension("meshes.json");
    let world_bytes = std::fs::read(document)
        .map_err(|error| command_error(format!("cannot read {}", document.display()), error))?;
    let meshes_bytes = std::fs::read(&meshes_path)
        .map_err(|error| command_error(format!("cannot read {}", meshes_path.display()), error))?;
    scene_from_documents(&world_bytes, &meshes_bytes)
        .map_err(|error| command_error("cannot build scene snapshot", error))
}

/// Export the versioned snapshot only. Core-owned deliverables (for example a
/// validated glTF bundle with attribution sidecar) use Core's export commands.
#[tauri::command]
pub fn export_scene(path: String, snapshot: SceneSnapshot) -> std::result::Result<(), String> {
    let bytes = serde_json::to_vec_pretty(&snapshot)
        .map_err(|error| command_error("cannot encode scene snapshot", error))?;
    std::fs::write(&path, bytes)
        .map_err(|error| command_error(format!("cannot write {path}"), error))
}

#[tauri::command]
pub fn select_scene_node(snapshot: SceneSnapshot, node_id: Option<String>) -> SceneSnapshot {
    with_selection(&snapshot, node_id.as_deref())
}
