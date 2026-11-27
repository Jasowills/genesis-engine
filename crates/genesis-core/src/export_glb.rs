//! Minimal glTF 2.0 / GLB interchange for the first vertical slice (GE-0019).
//!
//! This is a deliberately small writer/reader with an explicit extension
//! allowlist (currently empty). It carries provenance in `extras` and emits a
//! separate attribution sidecar. No materials, textures, LOD or collision are
//! exported; unsupported fields are reported, never silently dropped.

use crate::{
    CoreError, GeneratedGeometry, Result, SourceDescriptor, WorldIr, attribution_sidecar,
    enforce_sources,
};
use serde_json::{Value, json};

const GLB_MAGIC: u32 = 0x46546c67; // "glTF"
const GLB_VERSION: u32 = 2;
const JSON_CHUNK: u32 = 0x4e4f534a; // "JSON"
const BIN_CHUNK: u32 = 0x004e4942; // "BIN\0"
const EXTENSION_ALLOWLIST: [&str; 0] = [];

#[derive(Debug, Clone, PartialEq)]
pub struct GlbExport {
    pub bytes: Vec<u8>,
    pub attribution_sidecar: Vec<u8>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImportedGlb {
    pub version: String,
    pub mesh_count: usize,
    pub vertex_count: usize,
    pub triangle_count: usize,
    pub provenance: Vec<Value>,
}

fn push_u32(out: &mut Vec<u8>, value: u32) {
    out.extend_from_slice(&value.to_le_bytes());
}

fn pad_to_four(out: &mut Vec<u8>, pad: u8) {
    while !out.len().is_multiple_of(4) {
        out.push(pad);
    }
}

/// Export generated meshes to GLB after the lawful-source and validation gate.
pub fn export_glb(
    meshes: &[GeneratedGeometry],
    world: &WorldIr,
    sources: &[SourceDescriptor],
    report: &crate::ValidationReport,
) -> Result<GlbExport> {
    enforce_sources(world, sources)?;
    crate::check_export_ready(report).map_err(CoreError::InvalidManifest)?;
    if report.world_id != world.world_id.to_string() {
        return Err(CoreError::InvalidManifest(
            "validation report belongs to a different world".to_string(),
        ));
    }
    if report.subject_hash != crate::validation::mesh_subject_hash(meshes) {
        return Err(CoreError::InvalidManifest(
            "validation report does not match the meshes being exported".to_string(),
        ));
    }
    for mesh in meshes {
        let source_id = mesh.provenance.source_id.as_str();
        if !sources.iter().any(|source| source.id == source_id) {
            return Err(CoreError::InvalidManifest(format!(
                "mesh {} has unregistered source '{source_id}'",
                mesh.asset_id
            )));
        }
        if mesh.provenance.licence_elected.is_empty() || mesh.provenance.attribution.is_empty() {
            return Err(CoreError::InvalidManifest(format!(
                "mesh {} has incomplete provenance",
                mesh.asset_id
            )));
        }
    }

    let mut warnings = vec![
        "materials and textures are not exported in this slice".to_string(),
        "LOD chains, collision meshes and instancing are not exported".to_string(),
        "float64 vertex attributes are not used (draft glTF extension)".to_string(),
    ];
    if !EXTENSION_ALLOWLIST.is_empty() {
        warnings.push("extension allowlist is non-empty".to_string());
    }

    let mut binary = Vec::new();
    let mut accessors = Vec::new();
    let mut buffer_views = Vec::new();
    let mut gltf_meshes = Vec::new();
    let mut nodes = Vec::new();

    for (index, mesh) in meshes.iter().enumerate() {
        let position_offset = binary.len() as u32;
        let mut min = [f32::INFINITY; 3];
        let mut max = [f32::NEG_INFINITY; 3];
        for vertex in &mesh.vertices {
            for (axis, component) in vertex.iter().enumerate() {
                let value = *component as f32;
                min[axis] = min[axis].min(value);
                max[axis] = max[axis].max(value);
                binary.extend_from_slice(&value.to_le_bytes());
            }
        }
        pad_to_four(&mut binary, 0);
        let position_view = buffer_views.len() as u32;
        buffer_views.push(json!({
            "buffer": 0,
            "byteOffset": position_offset,
            "byteLength": (mesh.vertices.len() * 12) as u32,
            "target": 34962
        }));

        let mut indices_bytes = Vec::new();
        for index in &mesh.indices {
            indices_bytes.extend_from_slice(&index.to_le_bytes());
        }
        pad_to_four(&mut indices_bytes, 0);
        let index_offset = binary.len() as u32;
        binary.extend_from_slice(&indices_bytes);
        let index_view = buffer_views.len() as u32;
        buffer_views.push(json!({
            "buffer": 0,
            "byteOffset": index_offset,
            "byteLength": (mesh.indices.len() * 4) as u32,
            "target": 34963
        }));

        let position_accessor = accessors.len() as u32;
        accessors.push(json!({
            "bufferView": position_view,
            "componentType": 5126,
            "count": mesh.vertices.len(),
            "type": "VEC3",
            "min": min,
            "max": max,
        }));
        let index_accessor = accessors.len() as u32;
        accessors.push(json!({
            "bufferView": index_view,
            "componentType": 5125,
            "count": mesh.indices.len(),
            "type": "SCALAR",
        }));

        gltf_meshes.push(json!({
            "name": mesh.asset_id.to_string(),
            "primitives": [{
                "attributes": {"POSITION": position_accessor},
                "indices": index_accessor,
                "mode": 4
            }],
            "extras": {
                "genesis": {
                    "asset_id": mesh.asset_id.to_string(),
                    "generator_version": mesh.generator_version,
                    "generation_seed": mesh.generation_seed,
                    "evidence": mesh.evidence,
                    "source_object_ids": mesh.source_object_ids.iter().map(|id| id.to_string()).collect::<Vec<_>>(),
                    "origin_lon_lat": mesh.origin_lon_lat,
                    "provenance": mesh.provenance
                }
            }
        }));
        nodes.push(json!({"mesh": index, "name": mesh.asset_id.to_string()}));
    }

    let json_value = json!({
        "asset": {"version": "2.0", "generator": "genesis-core GE-0019"},
        "scene": 0,
        "scenes": [{"nodes": (0..nodes.len()).collect::<Vec<_>>()}],
        "nodes": nodes,
        "meshes": gltf_meshes,
        "accessors": accessors,
        "bufferViews": buffer_views,
        "buffers": [{"byteLength": binary.len() as u32}]
    });
    let mut json_bytes = serde_json::to_vec(&json_value)?;
    pad_to_four(&mut json_bytes, b' ');
    pad_to_four(&mut binary, 0);

    let total = 12 + 8 + json_bytes.len() + 8 + binary.len();
    let mut glb = Vec::with_capacity(total);
    push_u32(&mut glb, GLB_MAGIC);
    push_u32(&mut glb, GLB_VERSION);
    push_u32(&mut glb, total as u32);
    push_u32(&mut glb, json_bytes.len() as u32);
    push_u32(&mut glb, JSON_CHUNK);
    glb.extend_from_slice(&json_bytes);
    push_u32(&mut glb, binary.len() as u32);
    push_u32(&mut glb, BIN_CHUNK);
    glb.extend_from_slice(&binary);

    Ok(GlbExport {
        bytes: glb,
        attribution_sidecar: attribution_sidecar(world, sources)?,
        warnings,
    })
}

fn read_u32(bytes: &[u8], offset: usize) -> Result<u32> {
    let raw = bytes
        .get(offset..offset + 4)
        .ok_or_else(|| CoreError::InvalidManifest("truncated GLB".to_string()))?;
    Ok(u32::from_le_bytes(raw.try_into().map_err(|_| {
        CoreError::InvalidManifest("bad GLB word".to_string())
    })?))
}

/// Re-import a GLB produced by this writer and verify metadata survival.
pub fn import_glb(bytes: &[u8]) -> Result<ImportedGlb> {
    if read_u32(bytes, 0)? != GLB_MAGIC || read_u32(bytes, 4)? != GLB_VERSION {
        return Err(CoreError::InvalidManifest(
            "not a GLB 2.0 document".to_string(),
        ));
    }
    let declared_length = read_u32(bytes, 8)? as usize;
    if declared_length != bytes.len() {
        return Err(CoreError::InvalidManifest(
            "GLB length mismatch".to_string(),
        ));
    }
    let json_length = read_u32(bytes, 12)? as usize;
    if read_u32(bytes, 16)? != JSON_CHUNK {
        return Err(CoreError::InvalidManifest(
            "GLB first chunk is not JSON".to_string(),
        ));
    }
    let json_start: usize = 20;
    let json_end = json_start
        .checked_add(json_length)
        .ok_or_else(|| CoreError::InvalidManifest("GLB JSON length overflow".to_string()))?;
    let json: Value = serde_json::from_slice(
        bytes
            .get(json_start..json_end)
            .ok_or_else(|| CoreError::InvalidManifest("GLB JSON chunk truncated".to_string()))?,
    )?;
    let version = json
        .get("asset")
        .and_then(|a| a.get("version"))
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_string();
    if version != "2.0" {
        return Err(CoreError::InvalidManifest(
            "unsupported glTF version".to_string(),
        ));
    }
    let meshes = json
        .get("meshes")
        .and_then(Value::as_array)
        .ok_or_else(|| CoreError::InvalidManifest("GLB has no meshes".to_string()))?;
    let mut provenance = Vec::new();
    let mut vertex_count = 0usize;
    let mut triangle_count = 0usize;
    for mesh in meshes {
        let extras = mesh
            .get("extras")
            .and_then(|e| e.get("genesis"))
            .ok_or_else(|| CoreError::InvalidManifest("mesh has no Genesis extras".to_string()))?;
        provenance.push(extras.clone());
        let primitive = mesh
            .get("primitives")
            .and_then(Value::as_array)
            .and_then(|p| p.first())
            .ok_or_else(|| CoreError::InvalidManifest("mesh has no primitive".to_string()))?;
        let position_accessor = primitive
            .get("attributes")
            .and_then(|a| a.get("POSITION"))
            .and_then(Value::as_u64)
            .ok_or_else(|| CoreError::InvalidManifest("missing POSITION accessor".to_string()))?
            as usize;
        let count = json
            .get("accessors")
            .and_then(Value::as_array)
            .and_then(|a| a.get(position_accessor))
            .and_then(|a| a.get("count"))
            .and_then(Value::as_u64)
            .ok_or_else(|| {
                CoreError::InvalidManifest("POSITION accessor count missing".to_string())
            })?;
        let index_accessor = primitive
            .get("indices")
            .and_then(Value::as_u64)
            .ok_or_else(|| CoreError::InvalidManifest("missing indices accessor".to_string()))?
            as usize;
        let index_count = json
            .get("accessors")
            .and_then(Value::as_array)
            .and_then(|a| a.get(index_accessor))
            .and_then(|a| a.get("count"))
            .and_then(Value::as_u64)
            .ok_or_else(|| {
                CoreError::InvalidManifest("indices accessor count missing".to_string())
            })?;
        vertex_count += count as usize;
        triangle_count += index_count as usize / 3;
    }
    // Confirm BIN chunk exists and covers the declared buffer.
    let bin_header = json_end;
    if read_u32(bytes, bin_header + 4)? != BIN_CHUNK {
        return Err(CoreError::InvalidManifest(
            "GLB BIN chunk missing".to_string(),
        ));
    }
    let bin_length = read_u32(bytes, bin_header)? as usize;
    let expected = json
        .get("buffers")
        .and_then(Value::as_array)
        .and_then(|b| b.first())
        .and_then(|b| b.get("byteLength"))
        .and_then(Value::as_u64)
        .unwrap_or(0) as usize;
    if bin_length < expected {
        return Err(CoreError::InvalidManifest(
            "GLB BIN chunk too short".to_string(),
        ));
    }
    Ok(ImportedGlb {
        version,
        mesh_count: meshes.len(),
        vertex_count,
        triangle_count,
        provenance,
    })
}

/// Report a metric record for the format gate (validator result supplied by
/// the external glTF-Validator; this function records its outcome honestly).
pub fn format_gate_metric(validator_exit_code: i32, tool_version: &str) -> crate::MetricRecord {
    crate::MetricRecord {
        name: "gltf_validator_exit_code".to_string(),
        value: validator_exit_code as f64,
        unit: "exit_code".to_string(),
        method: "Khronos glTF-Validator".to_string(),
        tool_version: tool_version.to_string(),
        parameters: Default::default(),
        limitations: vec!["validator must be run externally; 0 means no errors".to_string()],
    }
}
