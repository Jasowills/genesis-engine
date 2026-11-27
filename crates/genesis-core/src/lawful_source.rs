//! Lawful-source gate and provenance enforcement (GE-0012, ADR-0007).
//!
//! Rights-blocked sources are rejected in code, not merely documented. Open
//! standards software (for example CesiumJS or 3D Tiles) remains usable with
//! Genesis-owned content.

use crate::{CoreError, Result, WorldIr, world_ir::Provenance};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LicenceElection {
    Odbl10,
    CcBy40,
    CdlaPermissive20,
    Apache20,
    Mit,
    Other(String),
}

impl LicenceElection {
    pub fn as_text(&self) -> String {
        match self {
            Self::Odbl10 => "ODbL-1.0".to_string(),
            Self::CcBy40 => "CC-BY-4.0".to_string(),
            Self::CdlaPermissive20 => "CDLA-Permissive-2.0".to_string(),
            Self::Apache20 => "Apache-2.0".to_string(),
            Self::Mit => "MIT".to_string(),
            Self::Other(text) => text.clone(),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourcePurpose {
    #[default]
    ReconstructionInput,
    VisualBasemapOnly,
    ModelTraining,
}

/// A source dataset or provider with its licence election and attribution.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SourceDescriptor {
    pub id: String,
    pub publisher: String,
    pub url: String,
    pub version: String,
    pub licence: LicenceElection,
    pub attribution: String,
    #[serde(default)]
    pub vintage: Option<String>,
    #[serde(default)]
    pub filters: Vec<String>,
    #[serde(default)]
    pub checksum: Option<String>,
    #[serde(default)]
    pub purpose: SourcePurpose,
}

impl SourceDescriptor {
    pub fn validate(&self) -> Result<()> {
        if self.id.trim().is_empty()
            || self.publisher.trim().is_empty()
            || self.url.trim().is_empty()
            || self.attribution.trim().is_empty()
        {
            return Err(CoreError::InvalidManifest(
                "source id, publisher, url and attribution must be recorded".to_string(),
            ));
        }
        Ok(())
    }
}

/// Hard deny-list. These terms prohibit derivation, offline caching and AI
/// training on their content, so no purpose is accepted for them in v1.
const DENIED_PATTERNS: [&str; 9] = [
    "maps.googleapis.com",
    "maps.google.com",
    "google maps",
    "streetview",
    "photorealistic 3d tiles",
    "cesium ion",
    "ion.cesium.com",
    "cesium.com/ion/",
    "api.cesium.com",
];

/// Check a source before it can enter import, inference or export.
pub fn check_source(source: &SourceDescriptor) -> Result<()> {
    source.validate()?;
    let haystack = format!("{} {}", source.publisher, source.url).to_lowercase();
    for pattern in DENIED_PATTERNS {
        if haystack.contains(pattern) {
            return Err(CoreError::SourceDenied(format!(
                "source '{}' matches blocked pattern '{pattern}'",
                source.id
            )));
        }
    }
    if let LicenceElection::Other(text) = &source.licence {
        let lower = text.to_lowercase();
        if lower.contains("non-commercial")
            || lower.contains("by-nc")
            || lower.contains("research only")
        {
            return Err(CoreError::SourceDenied(format!(
                "source '{}' has a non-commercial licence",
                source.id
            )));
        }
    }
    Ok(())
}

impl From<&SourceDescriptor> for Provenance {
    fn from(source: &SourceDescriptor) -> Self {
        Self {
            source_id: source.id.clone(),
            licence_elected: source.licence.as_text(),
            attribution: source.attribution.clone(),
            vintage: source.vintage.clone(),
            filters: source.filters.clone(),
            checksum: source.checksum.clone(),
            source_feature_ids: Vec::new(),
            derivation_chain: Vec::new(),
        }
    }
}

/// Build a machine-readable attribution sidecar from a World IR.
/// Attribution and licence election survive IR → export.
pub fn attribution_sidecar(world: &WorldIr) -> Result<Vec<u8>> {
    let mut records: Vec<serde_json::Value> = Vec::new();
    let mut seen = std::collections::BTreeSet::new();
    for layer in &world.layers {
        for object in &layer.objects {
            let key = format!(
                "{}|{}",
                object.provenance.attribution, object.provenance.licence_elected
            );
            if seen.insert(key) {
                records.push(json!({
                    "attribution": object.provenance.attribution,
                    "licence_elected": object.provenance.licence_elected,
                    "source_id": object.provenance.source_id,
                    "vintage": object.provenance.vintage,
                    "checksum": object.provenance.checksum
                }));
            }
        }
    }
    Ok(serde_json::to_vec_pretty(&json!({
        "sidecar_schema_version": 1,
        "world_id": world.world_id.as_str(),
        "records": records
    }))?)
}
