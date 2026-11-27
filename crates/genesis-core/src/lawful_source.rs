//! Lawful-source gate and provenance enforcement (GE-0012, ADR-0007).
//!
//! Rights-blocked sources are rejected in code, not merely documented. Open
//! standards software (for example CesiumJS or 3D Tiles) remains usable with
//! Genesis-owned content. Every import and export must go through this module.

use crate::{CoreError, Result, WorldIr, world_ir::Provenance};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LicenceElection {
    #[default]
    Mit,
    Odbl10,
    CcBy40,
    CdlaPermissive20,
    Apache20,
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

    pub fn url(&self) -> Option<String> {
        match self {
            Self::Odbl10 => Some("https://opendatacommons.org/licenses/odbl/1-0/".to_string()),
            Self::CcBy40 => Some("https://creativecommons.org/licenses/by/4.0/".to_string()),
            Self::CdlaPermissive20 => Some("https://cdla.dev/permissive-2-0/".to_string()),
            Self::Apache20 => Some("https://www.apache.org/licenses/LICENSE-2.0".to_string()),
            Self::Mit => Some("https://opensource.org/license/mit".to_string()),
            Self::Other(_) => None,
        }
    }
}

/// Recorded for reproducibility. `ModelTraining` is currently **informational**:
/// sources that pass the gate are accepted for all recorded purposes; a future
/// ticket may gate it separately (no behaviour is claimed here).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourcePurpose {
    #[default]
    ReconstructionInput,
    VisualBasemapOnly,
    ModelTraining,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
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
    #[serde(default)]
    pub licence_url: Option<String>,
    #[serde(default)]
    pub source_feature_ids: Vec<String>,
    #[serde(default)]
    pub derivation_chain: Vec<String>,
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
        if matches!(&self.licence, LicenceElection::Other(text) if text.trim().is_empty()) {
            return Err(CoreError::InvalidManifest(
                "custom licence election must not be empty".to_string(),
            ));
        }
        Ok(())
    }
}

/// Targeted deny patterns: hosts, provider identifiers and exact product names.
/// A bare `cesium` or `3d tiles` pattern is deliberately absent so open
/// CesiumJS / 3D Tiles software remains permitted.
const DENIED_PATTERNS: [&str; 15] = [
    "maps.googleapis.com",
    "tile.googleapis.com",
    "maps.google.com",
    "google maps",
    "street view",
    "street-view",
    "streetview",
    "photorealistic 3d tiles",
    "photorealistic-3d-tiles",
    "cesium ion",
    "cesium-ion",
    "cesium.com/ion/",
    "ion.cesium.com",
    "assets.cesium.com",
    "api.cesium.com",
];

/// Check a source before it can enter import, inference or export.
pub fn check_source(source: &SourceDescriptor) -> Result<()> {
    source.validate()?;
    let haystack = format!(
        "{} {} {} {} {} {}",
        source.publisher,
        source.url,
        source.id,
        source.version,
        source.attribution,
        source.filters.join(" ")
    )
    .to_lowercase();
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
        for forbidden in [
            "non-commercial",
            "noncommercial",
            "by-nc",
            "research only",
            "research/evaluation",
            "evaluation only",
        ] {
            if lower.contains(forbidden) {
                return Err(CoreError::SourceDenied(format!(
                    "source '{}' has a restricted licence ('{forbidden}')",
                    source.id
                )));
            }
        }
    }
    Ok(())
}

/// Mandatory import path: gate first, then produce provenance. Adapters must
/// call this instead of constructing `Provenance` directly from raw sources.
pub fn prepare_import(source: &SourceDescriptor) -> Result<Provenance> {
    check_source(source)?;
    Ok(source.into())
}

impl From<&SourceDescriptor> for Provenance {
    fn from(source: &SourceDescriptor) -> Self {
        Self {
            source_id: source.id.clone(),
            licence_elected: source.licence.as_text(),
            attribution: source.attribution.clone(),
            publisher: Some(source.publisher.clone()),
            url: Some(source.url.clone()),
            version: Some(source.version.clone()),
            licence_url: source.licence_url.clone().or_else(|| source.licence.url()),
            vintage: source.vintage.clone(),
            filters: source.filters.clone(),
            checksum: source.checksum.clone(),
            source_feature_ids: source.source_feature_ids.clone(),
            derivation_chain: source.derivation_chain.clone(),
            derivative_database_status: None,
            display_obligations: Vec::new(),
            third_party_terms: Vec::new(),
            recorded_at_unix: None,
        }
    }
}

/// Mandatory pre-export check: every registered source is lawful and every
/// object in the world traces back to one of those sources.
pub fn enforce_sources(world: &WorldIr, sources: &[SourceDescriptor]) -> Result<()> {
    let mut registered = std::collections::BTreeSet::new();
    for source in sources {
        check_source(source)?;
        registered.insert(source.id.as_str());
    }
    for layer in &world.layers {
        for object in &layer.objects {
            if !registered.contains(object.provenance.source_id.as_str()) {
                return Err(CoreError::InvalidManifest(format!(
                    "object {} has unregistered source '{}'",
                    object.id, object.provenance.source_id
                )));
            }
        }
    }
    Ok(())
}

/// Build a machine-readable attribution sidecar from a World IR. This is the
/// export path: it enforces the source gate and preserves full source identity.
pub fn attribution_sidecar(world: &WorldIr, sources: &[SourceDescriptor]) -> Result<Vec<u8>> {
    enforce_sources(world, sources)?;
    let mut records: Vec<serde_json::Value> = Vec::new();
    let mut seen = std::collections::BTreeSet::new();
    for layer in &world.layers {
        for object in &layer.objects {
            let p = &object.provenance;
            let key = format!(
                "{}|{}|{}|{}|{}",
                p.source_id,
                p.licence_elected,
                p.attribution,
                p.vintage.as_deref().unwrap_or(""),
                p.checksum.as_deref().unwrap_or("")
            );
            if seen.insert(key) {
                records.push(json!({
                    "source_id": p.source_id,
                    "publisher": p.publisher,
                    "url": p.url,
                    "version": p.version,
                    "licence_elected": p.licence_elected,
                    "licence_url": p.licence_url,
                    "attribution": p.attribution,
                    "vintage": p.vintage,
                    "filters": p.filters,
                    "checksum": p.checksum,
                    "source_feature_ids": p.source_feature_ids,
                    "derivation_chain": p.derivation_chain,
                    "evidence": object.evidence,
                    "world_id": world.world_id.as_str()
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
