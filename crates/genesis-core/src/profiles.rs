//! Named quality profiles and honest preflight estimates (GE-0024).
//!
//! Every estimate is a range with a confidence label and limitations — a
//! single number is never presented as a guarantee. Hardware suitability is
//! computed from caller-declared capabilities; no hardware probing is done
//! here. Paid operations are gated on explicit consent and this module never
//! invokes an external service.

use crate::{CoreError, EvidenceKind, QualityProfile, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Explicit, named gate policy. Values are policy thresholds, not measurements.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProfilePolicy {
    pub profile: QualityProfile,
    pub description: String,
    /// Maximum allowed geometric deviation in metres. `None` means not applicable.
    pub max_deviation_m: Option<f64>,
    /// Minimum evidence tier required for generated geometry.
    pub min_evidence: EvidenceTier,
    /// Whether independent checkpoints are mandatory.
    pub checkpoints_required: bool,
    /// Whether a documented source resolution is mandatory.
    pub source_resolution_required: bool,
    pub gates: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceTier {
    Observed,
    Derived,
    Inferred,
    /// Unknown evidence never satisfies a gate.
    Unknown,
}

impl ProfilePolicy {
    /// Fixed project-defined policy. No number here is a measured performance
    /// claim; the gate thresholds are explicit configuration.
    pub fn for_profile(profile: QualityProfile) -> Self {
        match profile {
            QualityProfile::Preview => Self {
                profile,
                description: "Interactive preview; inferred geometry permitted".to_string(),
                max_deviation_m: Some(25.0),
                min_evidence: EvidenceTier::Inferred,
                checkpoints_required: false,
                source_resolution_required: false,
                gates: vec![
                    "structural_mesh_validation".to_string(),
                    "lawful_source_gate".to_string(),
                ],
            },
            QualityProfile::GameReady => Self {
                profile,
                description: "Game-ready asset; derived geometry permitted".to_string(),
                max_deviation_m: Some(5.0),
                min_evidence: EvidenceTier::Derived,
                checkpoints_required: false,
                source_resolution_required: true,
                gates: vec![
                    "structural_mesh_validation".to_string(),
                    "lawful_source_gate".to_string(),
                    "source_resolution_recorded".to_string(),
                    "gltf_validator_zero_errors".to_string(),
                ],
            },
            QualityProfile::HighFidelity => Self {
                profile,
                description: "High-fidelity asset; observed geometry required for primary surfaces"
                    .to_string(),
                max_deviation_m: Some(1.0),
                min_evidence: EvidenceTier::Observed,
                checkpoints_required: false,
                source_resolution_required: true,
                gates: vec![
                    "structural_mesh_validation".to_string(),
                    "lawful_source_gate".to_string(),
                    "source_resolution_recorded".to_string(),
                    "gltf_validator_zero_errors".to_string(),
                    "observed_primary_surfaces".to_string(),
                ],
            },
            QualityProfile::SurveyReference => Self {
                profile,
                description: "Survey reference; independent checkpoints mandatory".to_string(),
                max_deviation_m: Some(0.1),
                min_evidence: EvidenceTier::Observed,
                checkpoints_required: true,
                source_resolution_required: true,
                gates: vec![
                    "structural_mesh_validation".to_string(),
                    "lawful_source_gate".to_string(),
                    "source_resolution_recorded".to_string(),
                    "gltf_validator_zero_errors".to_string(),
                    "observed_primary_surfaces".to_string(),
                    "independent_checkpoints".to_string(),
                ],
            },
        }
    }

    pub fn permits(&self, evidence: EvidenceKind) -> bool {
        let tier = match evidence {
            EvidenceKind::Measured => EvidenceTier::Observed,
            EvidenceKind::DerivedWithConfidence => EvidenceTier::Derived,
            EvidenceKind::Unknown => EvidenceTier::Unknown,
            EvidenceKind::InferredByRule => EvidenceTier::Inferred,
        };
        tier <= self.min_evidence
    }
}

/// A bounded range, never a guaranteed scalar.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EstimateRange {
    pub low: f64,
    pub high: f64,
    pub unit: String,
    pub confidence: String,
    pub method: String,
    pub limitations: Vec<String>,
}

impl EstimateRange {
    pub fn new(low: f64, high: f64, unit: &str, method: &str) -> Result<Self> {
        if !low.is_finite() || !high.is_finite() || low < 0.0 || high <= low {
            return Err(CoreError::InvalidManifest(
                "estimate needs finite, ordered non-negative bounds".to_string(),
            ));
        }
        Ok(Self {
            low,
            high,
            unit: unit.to_string(),
            confidence: "low-uncalibrated".to_string(),
            method: method.to_string(),
            limitations: vec![
                "Uncalibrated estimate; not a measured result".to_string(),
                "Runner hardware and input size dominate; measure before committing".to_string(),
            ],
        })
    }
}

/// Caller-declared host capabilities. No probing happens in this module.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct HostCapabilities {
    pub logical_cpus: u32,
    pub memory_gb: f64,
    pub gpu_available: bool,
    pub cuda_available: bool,
    /// True only when the operator has opted into paid operations.
    pub paid_operations_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CapabilityReport {
    pub cpu_suitable: bool,
    pub gpu_suitable: bool,
    pub reasons: Vec<String>,
    /// Honesty marker: values are caller-declared, not detected.
    pub source: String,
}

impl HostCapabilities {
    pub fn report(&self, profile: QualityProfile) -> CapabilityReport {
        let policy = ProfilePolicy::for_profile(profile);
        let cpu_suitable = self.logical_cpus >= 2 && self.memory_gb >= 4.0;
        let gpu_suitable = match profile {
            QualityProfile::Preview => true,
            QualityProfile::GameReady
            | QualityProfile::HighFidelity
            | QualityProfile::SurveyReference => {
                self.gpu_available && self.logical_cpus >= 4 && self.memory_gb >= 8.0
            }
        };
        let mut reasons = vec![format!(
            "declared logical_cpus={}, memory_gb={}",
            self.logical_cpus, self.memory_gb
        )];
        if !cpu_suitable {
            reasons.push("CPU below minimum: need >=2 logical CPUs and >=4 GiB".to_string());
        }
        if !gpu_suitable && policy.profile != QualityProfile::Preview {
            reasons.push(format!(
                "GPU path needs declared gpu_available, >=4 logical CPUs and >=8 GiB for {}",
                profile_label(profile)
            ));
        }
        CapabilityReport {
            cpu_suitable,
            gpu_suitable,
            reasons,
            source: "caller-declared; no hardware probing performed".to_string(),
        }
    }
}

fn profile_label(profile: QualityProfile) -> &'static str {
    match profile {
        QualityProfile::Preview => "preview",
        QualityProfile::GameReady => "game-ready",
        QualityProfile::HighFidelity => "high-fidelity",
        QualityProfile::SurveyReference => "survey-reference",
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PreflightReport {
    pub schema_version: u32,
    pub profile: QualityProfile,
    pub gates: Vec<String>,
    pub duration: EstimateRange,
    pub peak_memory: EstimateRange,
    pub capability: CapabilityReport,
    pub paid_operations: Vec<String>,
    pub warnings: Vec<String>,
}

/// Build a preflight report. Every number is a range; no cost or timing is a
/// promise. `paid_operations` are listed but never executed here.
pub fn preflight(
    profile: QualityProfile,
    capabilities: &HostCapabilities,
    object_count: u32,
) -> Result<PreflightReport> {
    if object_count == 0 {
        return Err(CoreError::InvalidManifest(
            "preflight needs at least one object".to_string(),
        ));
    }
    let policy = ProfilePolicy::for_profile(profile);
    let n = object_count as f64;
    let per_object_ms = match profile {
        QualityProfile::Preview => 2.0,
        QualityProfile::GameReady => 8.0,
        QualityProfile::HighFidelity => 30.0,
        QualityProfile::SurveyReference => 120.0,
    };
    let duration = EstimateRange::new(
        n * per_object_ms * 0.6,
        n * per_object_ms * 2.5,
        "ms",
        "object-count heuristic (uncalibrated)",
    )?;
    let per_object_mb = match profile {
        QualityProfile::Preview => 0.05,
        QualityProfile::GameReady => 0.15,
        QualityProfile::HighFidelity => 0.5,
        QualityProfile::SurveyReference => 1.5,
    };
    let peak_memory = EstimateRange::new(
        n * per_object_mb * 0.7,
        n * per_object_mb * 2.0,
        "MiB",
        "object-count heuristic (uncalibrated)",
    )?;
    let capability = capabilities.report(profile);
    let mut warnings = Vec::new();
    if !capability.cpu_suitable {
        warnings.push("CPU below minimum for the selected profile".to_string());
    }
    if !capability.gpu_suitable && profile != QualityProfile::Preview {
        warnings.push("GPU path unsuitable for the selected profile".to_string());
    }
    if profile == QualityProfile::SurveyReference {
        warnings.push("Survey-reference claims require independent checkpoints".to_string());
    }
    Ok(PreflightReport {
        schema_version: 1,
        profile,
        gates: policy.gates,
        duration,
        peak_memory,
        capability,
        paid_operations: Vec::new(),
        warnings,
    })
}

/// Explicit consent gate for a paid operation. Returns a token that the caller
/// must pass to the eventual (separate, approved) execution path.
pub fn authorize_paid_operation(
    name: &str,
    capabilities: &HostCapabilities,
    consent_granted: bool,
) -> Result<String> {
    if name.trim().is_empty() {
        return Err(CoreError::InvalidManifest(
            "paid operation needs a name".to_string(),
        ));
    }
    if !capabilities.paid_operations_enabled {
        return Err(CoreError::InvalidManifest(
            "paid operations are disabled in host capabilities".to_string(),
        ));
    }
    if !consent_granted {
        return Err(CoreError::InvalidManifest(
            "explicit consent required before a paid operation".to_string(),
        ));
    }
    Ok(format!("consent:{name}:granted"))
}

/// Metadata for a named paid operation, carried in the preflight report so the
/// operator can see it before consenting.
pub fn paid_operation(
    name: &str,
    estimated_cost: EstimateRange,
) -> Result<BTreeMap<String, String>> {
    let mut entry = BTreeMap::new();
    entry.insert("name".to_string(), name.to_string());
    entry.insert("cost_low".to_string(), estimated_cost.low.to_string());
    entry.insert("cost_high".to_string(), estimated_cost.high.to_string());
    entry.insert("unit".to_string(), estimated_cost.unit);
    entry.insert(
        "consent".to_string(),
        "required-before-execution".to_string(),
    );
    Ok(entry)
}
