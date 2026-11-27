//! Delivery-time LOD, collision proxies and compression allowlist (GE-0030).
//!
//! The source World IR stays uncompressed. This module produces deterministic
//! screening LODs, geometrically measured collision proxies and a licence-aware
//! codec allowlist. No codec encoder/decoder is bundled or invoked here;
//! engine-specific optimisation is out of scope.

use crate::{CoreError, GeneratedGeometry, Result, verify_mesh};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TargetProfile {
    DesktopPreview,
    DesktopGameReady,
}

impl TargetProfile {
    pub fn max_triangles_per_asset(&self) -> usize {
        match self {
            Self::DesktopPreview => 12,
            Self::DesktopGameReady => 64,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LodLevel {
    pub name: String,
    pub mesh: GeneratedGeometry,
    pub triangle_budget: usize,
    pub triangle_count: usize,
}

/// Build a deterministic triangle-sampled LOD chain. Each level keeps the
/// first triangle plus a fixed stride of the source triangles, remapping
/// indices to compact vertices. This is a *screening* LOD, not visually
/// faithful simplification; the limitation is recorded in the output.
pub fn generate_lod_chain(
    source: &GeneratedGeometry,
    target: TargetProfile,
) -> Result<Vec<LodLevel>> {
    let budget = target.max_triangles_per_asset();
    if budget == 0 {
        return Err(CoreError::InvalidManifest(
            "LOD budget must be positive".to_string(),
        ));
    }
    let total = source.indices.len() / 3;
    if total == 0 {
        return Err(CoreError::InvalidManifest(
            "source mesh has no triangles".to_string(),
        ));
    }
    let levels = [
        ("lod0", total.min(budget)),
        ("lod1", total.min(budget / 2).max(1)),
        ("lod2", total.min(budget / 4).max(1)),
    ];
    let mut chain = Vec::new();
    for (name, cap) in levels {
        let stride = if cap >= total { 1 } else { total.div_ceil(cap) };
        let mut kept = Vec::new();
        let mut triangle = 0;
        while triangle < total && kept.len() < cap {
            kept.push(triangle);
            triangle += stride;
        }
        let mut mapping = std::collections::BTreeMap::new();
        let mut indices = Vec::with_capacity(kept.len() * 3);
        for tri in kept {
            for offset in 0..3 {
                let original = source.indices[tri * 3 + offset];
                let next = match mapping.get(&original) {
                    Some(value) => *value,
                    None => {
                        let value = mapping.len() as u32;
                        mapping.insert(original, value);
                        value
                    }
                };
                indices.push(next);
            }
        }
        let mut vertices = vec![[0.0_f64; 3]; mapping.len()];
        for (original, compact) in &mapping {
            vertices[*compact as usize] = source.vertices[*original as usize];
        }
        let mut mesh = source.clone();
        mesh.asset_id = crate::derive_id("mesh", &format!("{}:{}", source.asset_id, name));
        mesh.vertices = vertices;
        mesh.indices = indices;
        mesh.parameters
            .insert("lod_level".to_string(), name.to_string());
        mesh.parameters.insert(
            "lod_method".to_string(),
            "deterministic triangle sampling; screening only".to_string(),
        );
        verify_mesh(&mesh)?;
        let triangle_count = mesh.indices.len() / 3;
        if triangle_count > budget {
            return Err(CoreError::InvalidManifest(format!(
                "{name} exceeds triangle budget {budget}"
            )));
        }
        chain.push(LodLevel {
            name: name.to_string(),
            mesh,
            triangle_budget: budget,
            triangle_count,
        });
    }
    Ok(chain)
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CollisionProxy {
    pub min: [f64; 3],
    pub max: [f64; 3],
    pub tolerance_m: f64,
    pub sample_count: usize,
    pub coverage: f64,
}

impl CollisionProxy {
    /// Fraction of surface samples contained in the proxy. A value below 1.0
    /// means the proxy does not enclose the source geometry.
    pub fn coverage_of(&self, mesh: &GeneratedGeometry) -> f64 {
        let samples = surface_samples(mesh);
        if samples.is_empty() {
            return 0.0;
        }
        let inside = samples
            .iter()
            .filter(|p| {
                (0..3).all(|axis| {
                    p[axis] >= self.min[axis] - self.tolerance_m
                        && p[axis] <= self.max[axis] + self.tolerance_m
                })
            })
            .count();
        inside as f64 / samples.len() as f64
    }
}

fn surface_samples(mesh: &GeneratedGeometry) -> Vec<[f64; 3]> {
    let mut samples = mesh.vertices.clone();
    for triangle in mesh.indices.as_chunks::<3>().0 {
        let a = mesh.vertices[triangle[0] as usize];
        let b = mesh.vertices[triangle[1] as usize];
        let c = mesh.vertices[triangle[2] as usize];
        samples.push([
            (a[0] + b[0] + c[0]) / 3.0,
            (a[1] + b[1] + c[1]) / 3.0,
            (a[2] + b[2] + c[2]) / 3.0,
        ]);
    }
    samples
}

/// Build a padded AABB collision proxy and measure coverage against the
/// source surface. Non-finite vertices are rejected rather than silently
/// producing an enclosing box.
pub fn build_collision_proxy(mesh: &GeneratedGeometry, tolerance_m: f64) -> Result<CollisionProxy> {
    verify_mesh(mesh)?;
    if !mesh.indices.len().is_multiple_of(3) {
        return Err(CoreError::InvalidManifest(
            "collision mesh index count is not a multiple of three".to_string(),
        ));
    }
    if !tolerance_m.is_finite() || tolerance_m < 0.0 {
        return Err(CoreError::InvalidManifest(
            "collision tolerance must be non-negative".to_string(),
        ));
    }
    let samples = surface_samples(mesh);
    if samples.is_empty() {
        return Err(CoreError::InvalidManifest(
            "collision proxy needs geometry".to_string(),
        ));
    }
    let mut min = [f64::INFINITY; 3];
    let mut max = [f64::NEG_INFINITY; 3];
    for point in &samples {
        for axis in 0..3 {
            if !point[axis].is_finite() {
                return Err(CoreError::InvalidManifest(
                    "non-finite collision sample".to_string(),
                ));
            }
            min[axis] = min[axis].min(point[axis]);
            max[axis] = max[axis].max(point[axis]);
        }
    }
    let proxy = CollisionProxy {
        min,
        max,
        tolerance_m,
        sample_count: samples.len(),
        coverage: 1.0,
    };
    let coverage = proxy.coverage_of(mesh);
    if coverage < 1.0 {
        return Err(CoreError::InvalidManifest(format!(
            "collision proxy covers only {coverage} of surface samples"
        )));
    }
    Ok(CollisionProxy { coverage, ..proxy })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeliveryCompression {
    None,
    Draco,
    Meshopt,
    Ktx2Basis,
}

/// Allowlist of codecs whose licences permit redistribution. No codec binary
/// is bundled by Core; this is a declaration for the delivery step only.
pub fn compression_allowlist() -> Vec<(DeliveryCompression, &'static str)> {
    vec![
        (DeliveryCompression::None, "no codec"),
        (DeliveryCompression::Draco, "Apache-2.0 encoder/decoder"),
        (DeliveryCompression::Meshopt, "MIT encoder/decoder"),
        (
            DeliveryCompression::Ktx2Basis,
            "Basis Apache-2.0 / Khronos KTX2 spec",
        ),
    ]
}

pub fn compression_is_allowed(codec: DeliveryCompression, codec_licence_permissive: bool) -> bool {
    codec_licence_permissive
        && compression_allowlist()
            .iter()
            .any(|(allowed, _)| *allowed == codec)
}
