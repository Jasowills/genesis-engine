//! Structural validation gates and evidence records (GE-0018).
//!
//! Every metric carries its method, version, parameters and limitations. No
//! geographic accuracy claim is emitted without independent checkpoints.

use crate::{GeneratedGeometry, WorldIr};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QualityProfile {
    Preview,
    GameReady,
    HighFidelity,
    SurveyReference,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetricRecord {
    pub name: String,
    pub value: f64,
    pub unit: String,
    pub method: String,
    pub tool_version: String,
    pub parameters: BTreeMap<String, String>,
    pub limitations: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValidationReport {
    pub schema_version: u32,
    pub world_id: String,
    /// Hash of the exact mesh set this report validates.
    pub subject_hash: String,
    pub profile: QualityProfile,
    pub metrics: Vec<MetricRecord>,
    pub geographic_accuracy_claim: String,
    pub checkpoints_available: bool,
    pub passed: bool,
    pub warnings: Vec<String>,
}

fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn normal(vertices: &[[f64; 3]], triangle: &[u32]) -> Option<[f64; 3]> {
    let a = vertices.get(triangle[0] as usize)?;
    let b = vertices.get(triangle[1] as usize)?;
    let c = vertices.get(triangle[2] as usize)?;
    let n = cross(sub(*b, *a), sub(*c, *a));
    let length = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
    if length <= f64::EPSILON || !length.is_finite() {
        None
    } else {
        Some([n[0] / length, n[1] / length, n[2] / length])
    }
}

/// Structural checks on a generated mesh. Self-intersection detection is
/// explicitly NOT implemented in this slice and is recorded as a limitation.
pub fn validate_mesh(mesh: &GeneratedGeometry, tool_version: &str) -> Vec<MetricRecord> {
    let mut zero_area = 0.0;
    let mut edge_counts: BTreeMap<(u32, u32), u32> = BTreeMap::new();
    let mut orientation_conflicts = 0.0;
    let mut normals: Vec<Option<[f64; 3]>> = Vec::new();
    for triangle in mesh.indices.as_chunks::<3>().0.iter() {
        let n = normal(&mesh.vertices, triangle);
        if n.is_none() {
            zero_area += 1.0;
        }
        normals.push(n);
        for i in 0..3 {
            let a = triangle[i];
            let b = triangle[(i + 1) % 3];
            let key = (a.min(b), a.max(b));
            *edge_counts.entry(key).or_insert(0) += 1;
        }
    }
    for ((a, b), count) in &edge_counts {
        if *count == 2 {
            // Shared edge should be traversed in opposite directions.
            let mut forward = 0;
            let mut backward = 0;
            for (index, triangle) in mesh.indices.as_chunks::<3>().0.iter().enumerate() {
                for i in 0..3 {
                    if triangle[i] == *a && triangle[(i + 1) % 3] == *b {
                        if normals[index].is_some_and(|n| n[2] >= 0.0) {
                            forward += 1;
                        } else {
                            backward += 1;
                        }
                    }
                }
            }
            if forward > 0 && backward > 0 {
                orientation_conflicts += 1.0;
            }
        }
    }
    let non_manifold = edge_counts.values().filter(|count| **count > 2).count() as f64;
    let boundary = edge_counts.values().filter(|count| **count == 1).count() as f64;

    let mut base = BTreeMap::new();
    base.insert("asset_id".to_string(), mesh.asset_id.to_string());
    base.insert(
        "generator_version".to_string(),
        mesh.generator_version.clone(),
    );

    let records = vec![
        MetricRecord {
            name: "zero_area_faces".to_string(),
            value: zero_area,
            unit: "faces".to_string(),
            method: "cross-product magnitude of triangle edges".to_string(),
            tool_version: tool_version.to_string(),
            parameters: base.clone(),
            limitations: vec![
                "near-degenerate slivers require a numeric threshold (not applied)".to_string(),
            ],
        },
        MetricRecord {
            name: "non_manifold_edges".to_string(),
            value: non_manifold,
            unit: "edges".to_string(),
            method: "count edges used by more than two triangles".to_string(),
            tool_version: tool_version.to_string(),
            parameters: base.clone(),
            limitations: vec!["open surfaces are expected to have boundary edges".to_string()],
        },
        MetricRecord {
            name: "boundary_edges".to_string(),
            value: boundary,
            unit: "edges".to_string(),
            method: "count edges used by exactly one triangle".to_string(),
            tool_version: tool_version.to_string(),
            parameters: base.clone(),
            limitations: vec!["not an error for road surfaces".to_string()],
        },
        MetricRecord {
            name: "normal_orientation_conflicts".to_string(),
            value: orientation_conflicts,
            unit: "shared-edges".to_string(),
            method: "up-direction heuristic on shared edges (coarse)".to_string(),
            tool_version: tool_version.to_string(),
            parameters: base.clone(),
            limitations: vec!["does not replace a full adjacency-consistency test".to_string()],
        },
        MetricRecord {
            name: "self_intersections".to_string(),
            value: -1.0,
            unit: "pairs".to_string(),
            method: "not implemented in this slice".to_string(),
            tool_version: tool_version.to_string(),
            parameters: base.clone(),
            limitations: vec![
                "UNVERIFIED: requires triangle-triangle intersection tests".to_string(),
            ],
        },
        MetricRecord {
            name: "triangle_count".to_string(),
            value: (mesh.indices.len() / 3) as f64,
            unit: "triangles".to_string(),
            method: "index count / 3".to_string(),
            tool_version: tool_version.to_string(),
            parameters: base,
            limitations: vec!["no target-engine budget has been calibrated".to_string()],
        },
    ];
    records
}

/// Hash the exact mesh set a report applies to, so an export cannot reuse a
/// report that validated different geometry.
pub fn mesh_subject_hash(meshes: &[GeneratedGeometry]) -> String {
    let bytes = serde_json::to_vec(meshes).unwrap_or_default();
    format!("{:x}", Sha256::digest(bytes))
}

/// Validate world structure and emit an honest quality report.
pub fn validate_world(
    world: &WorldIr,
    profile: QualityProfile,
    meshes: &[GeneratedGeometry],
    checkpoints_available: bool,
    tool_version: &str,
) -> ValidationReport {
    let mut warnings = Vec::new();
    let mut objects = 0.0;
    let mut missing_provenance = 0.0;
    let mut unknown_evidence = 0.0;
    for layer in &world.layers {
        for object in &layer.objects {
            objects += 1.0;
            if object.provenance.source_id.is_empty()
                || object.provenance.attribution.is_empty()
                || object.provenance.licence_elected.is_empty()
            {
                missing_provenance += 1.0;
            }
            if object.evidence == crate::EvidenceKind::Unknown {
                unknown_evidence += 1.0;
            }
        }
    }
    if missing_provenance > 0.0 {
        warnings.push("objects with incomplete provenance".to_string());
    }
    if unknown_evidence > 0.0 {
        warnings.push("objects with unknown evidence kind".to_string());
    }

    let mut metrics = vec![
        MetricRecord {
            name: "object_count".to_string(),
            value: objects,
            unit: "objects".to_string(),
            method: "World IR layer traversal".to_string(),
            tool_version: tool_version.to_string(),
            parameters: BTreeMap::new(),
            limitations: vec!["no completeness claim without reference data".to_string()],
        },
        MetricRecord {
            name: "missing_provenance".to_string(),
            value: missing_provenance,
            unit: "objects".to_string(),
            method: "source/licence/attribution presence check".to_string(),
            tool_version: tool_version.to_string(),
            parameters: BTreeMap::new(),
            limitations: Vec::new(),
        },
    ];
    for mesh in meshes {
        metrics.extend(validate_mesh(mesh, tool_version));
    }

    let accuracy = if checkpoints_available {
        "reference data present but ASPRS reporting not implemented in this slice"
    } else {
        "none"
    };
    if profile == QualityProfile::SurveyReference && !checkpoints_available {
        warnings.push("survey_reference requested without independent checkpoints".to_string());
    }

    ValidationReport {
        schema_version: 1,
        world_id: world.world_id.to_string(),
        subject_hash: mesh_subject_hash(meshes),
        profile,
        metrics,
        geographic_accuracy_claim: accuracy.to_string(),
        checkpoints_available,
        passed: missing_provenance == 0.0
            && !(profile == QualityProfile::SurveyReference && !checkpoints_available),
        warnings,
    }
}

/// Export gate: fail on structural errors or an unsupported survey claim.
pub fn check_export_ready(report: &ValidationReport) -> Result<(), String> {
    if report.schema_version != 1 {
        return Err(format!(
            "unsupported validation report schema version {}",
            report.schema_version
        ));
    }
    if report
        .metrics
        .iter()
        .any(|m| m.name == "missing_provenance" && m.value > 0.0)
    {
        return Err("objects have incomplete provenance".to_string());
    }
    if report.profile == QualityProfile::SurveyReference && !report.checkpoints_available {
        return Err("survey_reference requires independent checkpoints".to_string());
    }
    if !report.passed {
        return Err("validation report did not pass (check provenance and profile)".to_string());
    }
    if report
        .metrics
        .iter()
        .any(|m| m.name == "zero_area_faces" && m.value > 0.0)
    {
        return Err("mesh has zero-area faces".to_string());
    }
    if report
        .metrics
        .iter()
        .any(|m| m.name == "non_manifold_edges" && m.value > 0.0)
    {
        return Err("mesh has non-manifold edges".to_string());
    }
    Ok(())
}

/// Collect distinct source identities present in a report's world record.
pub fn referenced_sources(world: &WorldIr) -> BTreeSet<String> {
    world
        .layers
        .iter()
        .flat_map(|layer| layer.objects.iter())
        .map(|object| object.provenance.source_id.clone())
        .collect()
}
