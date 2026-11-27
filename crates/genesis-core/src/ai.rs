//! Swappable AI adapter contract for segmentation and depth (GE-0026).
//!
//! Domain code depends only on these typed ports, never on an inference
//! runtime. This slice provides the contract, evidence records, cancellation
//! and a deterministic fake adapter. Real ONNX inference and named model
//! weights are deliberately NOT loaded (owner cost/licence decision pending);
//! see GE-0006. No external provider call is made by default.

use crate::CoreError;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModelLicence {
    Apache20,
    Mit,
    Bsd3Clause,
    CcBy40,
    /// Any licence that forbids commercial use is rejected.
    NonCommercial,
    Unknown,
}

impl ModelLicence {
    pub fn commercial_use_permitted(&self) -> bool {
        matches!(
            self,
            Self::Apache20 | Self::Mit | Self::Bsd3Clause | Self::CcBy40
        )
    }
}

/// Metadata only. No weight bytes are loaded, deserialised or executed here.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelDescriptor {
    pub id: String,
    pub version: String,
    pub licence: ModelLicence,
    /// SHA-256 of the intended weight file. Checksum verification of real
    /// weights is a follow-up; this field records the expected value only.
    pub expected_sha256: String,
    pub commercial_use_permitted: bool,
}

impl ModelDescriptor {
    pub fn validate(&self) -> Result<(), CoreError> {
        if self.id.trim().is_empty() || self.version.trim().is_empty() {
            return Err(CoreError::InvalidManifest(
                "model needs id and version".to_string(),
            ));
        }
        if !self.commercial_use_permitted || !self.licence.commercial_use_permitted() {
            return Err(CoreError::LicenseBlocked(
                "non-commercial model weights are not permitted".to_string(),
            ));
        }
        if self.expected_sha256.len() != 64
            || !self.expected_sha256.bytes().all(|b| b.is_ascii_hexdigit())
        {
            return Err(CoreError::InvalidManifest(
                "expected weight digest must be sha256 hex".to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SegmentationRequest {
    pub source_object_id: String,
    pub width_px: u32,
    pub height_px: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SegmentationResult {
    pub source_object_id: String,
    /// Run-length encoded mask intervals: alternating start/length pairs.
    pub mask_runs: Vec<u32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DepthRequest {
    pub source_object_id: String,
    pub width_px: u32,
    pub height_px: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DepthResult {
    pub source_object_id: String,
    /// Row-major depth values in metres.
    pub depth_m: Vec<f32>,
}

/// Per-run evidence. Records exactly which model/version produced an output,
/// so results can never be attributed to an unnamed model.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InferenceEvidence {
    pub model_id: String,
    pub model_version: String,
    pub model_licence: ModelLicence,
    pub adapter: String,
    pub parameters: std::collections::BTreeMap<String, String>,
    pub completed: bool,
    pub limitations: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SegmentationOutput {
    pub result: SegmentationResult,
    pub evidence: InferenceEvidence,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DepthOutput {
    pub result: DepthResult,
    pub evidence: InferenceEvidence,
}

#[derive(Debug, Clone, Default)]
pub struct CancelToken {
    flag: Arc<AtomicBool>,
}

impl CancelToken {
    pub fn cancel(&self) {
        self.flag.store(true, Ordering::SeqCst);
    }
    pub fn is_cancelled(&self) -> bool {
        self.flag.load(Ordering::SeqCst)
    }
}

/// Typed port. Capability code must depend on this trait, not on a runtime.
pub trait InferenceAdapter {
    fn name(&self) -> &str;
    fn model(&self) -> &ModelDescriptor;
    fn segment(
        &self,
        request: &SegmentationRequest,
        token: &CancelToken,
    ) -> Result<SegmentationOutput, CoreError>;
    fn depth(&self, request: &DepthRequest, token: &CancelToken) -> Result<DepthOutput, CoreError>;
}

fn evidence(model: &ModelDescriptor, adapter: &str) -> InferenceEvidence {
    let mut parameters = std::collections::BTreeMap::new();
    parameters.insert(
        "expected_weight_sha256".to_string(),
        model.expected_sha256.clone(),
    );
    InferenceEvidence {
        model_id: model.id.clone(),
        model_version: model.version.clone(),
        model_licence: model.licence.clone(),
        adapter: adapter.to_string(),
        parameters,
        completed: true,
        limitations: vec![
            "Deterministic fixture output; no real model inference".to_string(),
            "Weight checksum not verified against a loaded artefact".to_string(),
        ],
    }
}

fn check_request(id: &str, width: u32, height: u32) -> Result<(), CoreError> {
    if id.trim().is_empty() || width == 0 || height == 0 {
        return Err(CoreError::InvalidManifest(
            "inference request needs an object id and non-zero dimensions".to_string(),
        ));
    }
    Ok(())
}

/// Deterministic in-process adapter used to exercise the contract. It emits
/// typed outputs with complete evidence and honours cancellation between
/// operations. It performs no inference.
pub struct FakeAdapter {
    model: ModelDescriptor,
}

impl FakeAdapter {
    pub fn new(model: ModelDescriptor) -> Result<Self, CoreError> {
        model.validate()?;
        Ok(Self { model })
    }
}

impl InferenceAdapter for FakeAdapter {
    fn name(&self) -> &str {
        "fake-deterministic"
    }

    fn model(&self) -> &ModelDescriptor {
        &self.model
    }

    fn segment(
        &self,
        request: &SegmentationRequest,
        token: &CancelToken,
    ) -> Result<SegmentationOutput, CoreError> {
        check_request(
            &request.source_object_id,
            request.width_px,
            request.height_px,
        )?;
        if token.is_cancelled() {
            return Err(CoreError::Cancelled("segmentation cancelled".to_string()));
        }
        Ok(SegmentationOutput {
            result: SegmentationResult {
                source_object_id: request.source_object_id.clone(),
                mask_runs: vec![0, request.width_px * request.height_px],
            },
            evidence: evidence(&self.model, self.name()),
        })
    }

    fn depth(&self, request: &DepthRequest, token: &CancelToken) -> Result<DepthOutput, CoreError> {
        check_request(
            &request.source_object_id,
            request.width_px,
            request.height_px,
        )?;
        if token.is_cancelled() {
            return Err(CoreError::Cancelled("depth cancelled".to_string()));
        }
        let count = request.width_px as usize * request.height_px as usize;
        Ok(DepthOutput {
            result: DepthResult {
                source_object_id: request.source_object_id.clone(),
                depth_m: vec![1.0; count],
            },
            evidence: evidence(&self.model, self.name()),
        })
    }
}

/// External provider stub. Paid calls are disabled unless the caller supplies
/// a consent token obtained from the GE-0024 consent gate. No network client
/// exists in this crate.
pub struct ExternalProviderStub {
    model: ModelDescriptor,
    consent_token: Option<String>,
}

impl ExternalProviderStub {
    pub fn new(model: ModelDescriptor, consent_token: Option<String>) -> Result<Self, CoreError> {
        model.validate()?;
        Ok(Self {
            model,
            consent_token,
        })
    }
}

impl InferenceAdapter for ExternalProviderStub {
    fn name(&self) -> &str {
        "external-provider-stub"
    }
    fn model(&self) -> &ModelDescriptor {
        &self.model
    }

    fn segment(
        &self,
        _request: &SegmentationRequest,
        _token: &CancelToken,
    ) -> Result<SegmentationOutput, CoreError> {
        Err(CoreError::ProviderUnavailable(
            if self.consent_token.is_none() {
                "external provider disabled: no consent token".to_string()
            } else {
                "external provider transport not implemented".to_string()
            },
        ))
    }

    fn depth(
        &self,
        _request: &DepthRequest,
        _token: &CancelToken,
    ) -> Result<DepthOutput, CoreError> {
        Err(CoreError::ProviderUnavailable(
            if self.consent_token.is_none() {
                "external provider disabled: no consent token".to_string()
            } else {
                "external provider transport not implemented".to_string()
            },
        ))
    }
}
