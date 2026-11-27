//! Versioned plugin SDK contract and deny-by-default activation (GE-0025).
//!
//! This is the **product-side contract**, not the throwaway `prototype/`
//! sandbox evidence and not a live WASI component runtime. It defines the WIT
//! world surface, manifest validation, capability grants and atomic
//! activation/rollback state. No guest code is executed here; generated
//! bindings, asymmetric signature verification and component execution are
//! explicitly follow-up work.

use crate::{CoreError, SourceDescriptor, check_source};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;

pub const MANIFEST_SCHEMA_VERSION: u32 = 1;
pub const WIT_WORLD_VERSION: &str = "1.0.0";

/// The only host functions a plugin may import. There is deliberately no
/// filesystem, network, process, environment or clock capability in this
/// world — they cannot be granted because they cannot be named.
pub const WIT_WORLD_SOURCE: &str = r#"package genesis:plugin@1.0.0;

world plugin {
    import log: func(level: string, message: string);
    import progress: func(stage: string, fraction: f32);
    export run: func(input: list<u8>) -> result<list<u8>, string>;
}
"#;

pub fn wit_world_is_deny_by_default() -> bool {
    let forbidden = [
        "wasi:",
        "filesystem",
        "sockets",
        "process",
        "environment",
        "cli:",
        "clock",
    ];
    !forbidden.iter().any(|word| WIT_WORLD_SOURCE.contains(word))
        && WIT_WORLD_SOURCE.contains("import log:")
        && WIT_WORLD_SOURCE.contains("import progress:")
        && WIT_WORLD_SOURCE.contains("export run:")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Capability {
    HostLog,
    HostProgress,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityGrants {
    /// Empty by default. Only host logging and bounded progress can be granted.
    pub granted: BTreeSet<Capability>,
}

impl CapabilityGrants {
    pub fn allows(&self, capability: Capability) -> bool {
        self.granted.contains(&capability)
    }
}

/// Tamper-evident integrity seal over canonical manifest fields. This is a
/// SHA-256 digest, NOT an asymmetric signature — authenticity and key
/// management are follow-up work and must not be claimed here.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntegritySeal {
    pub algorithm: String,
    pub digest: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PluginManifest {
    pub schema_version: u32,
    pub name: String,
    pub version: String,
    pub wit_world_version: String,
    /// SHA-256 of the guest artefact bytes.
    pub artifact_digest: String,
    /// Provenance of the guest artefact; S12 is checked before activation.
    pub artifact_source: SourceDescriptor,
    pub grants: CapabilityGrants,
    /// Exact dependency pins in `name@version` form.
    pub dependency_pins: Vec<String>,
    pub seal: IntegritySeal,
}

#[derive(Debug, Clone, Serialize)]
struct SealPayload<'a> {
    name: &'a str,
    version: &'a str,
    wit_world_version: &'a str,
    artifact_digest: &'a str,
    artifact_source: &'a SourceDescriptor,
    grants: &'a BTreeSet<Capability>,
    dependency_pins: &'a [String],
}

fn is_hex64(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|b| b.is_ascii_hexdigit())
}

fn is_semver(value: &str) -> bool {
    // SemVer 2.0.0: numeric core identifiers with no leading zeros, plus
    // validated prerelease/build identifiers.
    let suffix_pos = value.find(['-', '+']);
    let (core, rest) = match suffix_pos {
        Some(index) => (&value[..index], &value[index + 1..]),
        None => (value, ""),
    };
    let mut parts = core.split('.');
    let take = |p: Option<&str>| -> bool {
        p.is_some_and(|s| {
            !s.is_empty()
                && s.bytes().all(|b| b.is_ascii_digit())
                && !(s.len() > 1 && s.starts_with('0'))
        })
    };
    if !(take(parts.next()) && take(parts.next()) && take(parts.next()) && parts.next().is_none()) {
        return false;
    }
    if suffix_pos.is_none() {
        return true;
    }
    if rest.is_empty() || rest.ends_with('.') {
        return false;
    }
    rest.split(['-', '+']).all(|segment| {
        !segment.is_empty()
            && segment.split('.').all(|id| {
                !id.is_empty() && id.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-')
            })
    })
}

impl PluginManifest {
    /// Compute the seal digest over canonicalised fields. Callers seal the
    /// manifest after setting the artefact digest, grants and pins.
    pub fn seal_digest(&self) -> String {
        let payload = SealPayload {
            name: &self.name,
            version: &self.version,
            wit_world_version: &self.wit_world_version,
            artifact_digest: &self.artifact_digest,
            artifact_source: &self.artifact_source,
            grants: &self.grants.granted,
            dependency_pins: &self.dependency_pins,
        };
        let bytes = serde_json::to_vec(&payload).expect("SealPayload is serialisable");
        format!("{:x}", Sha256::digest(bytes))
    }

    pub fn seal(&mut self) {
        self.seal = IntegritySeal {
            algorithm: "sha256-canonical-fields".to_string(),
            digest: self.seal_digest(),
        };
    }

    pub fn verify(&self) -> Result<(), CoreError> {
        if self.schema_version != MANIFEST_SCHEMA_VERSION {
            return Err(CoreError::InvalidManifest(
                "unsupported manifest schema version".to_string(),
            ));
        }
        if self.name.trim().is_empty() || !is_semver(&self.version) {
            return Err(CoreError::InvalidManifest(
                "plugin needs a name and semver version".to_string(),
            ));
        }
        if self.wit_world_version != WIT_WORLD_VERSION {
            return Err(CoreError::InvalidManifest(format!(
                "unsupported WIT world version '{}'",
                self.wit_world_version
            )));
        }
        if !is_hex64(&self.artifact_digest) {
            return Err(CoreError::InvalidManifest(
                "artifact digest must be sha256 hex".to_string(),
            ));
        }
        check_source(&self.artifact_source)?;
        if !wit_world_is_deny_by_default() {
            return Err(CoreError::InvalidManifest(
                "WIT world exposes an ambient capability".to_string(),
            ));
        }
        if self.dependency_pins.iter().any(|pin| {
            let mut parts = pin.split('@');
            let name = parts.next().unwrap_or("");
            let version = parts.next().unwrap_or("");
            name.is_empty() || parts.next().is_some() || !is_semver(version)
        }) {
            return Err(CoreError::InvalidManifest(
                "dependency pins must be name@semver".to_string(),
            ));
        }
        if self.seal.algorithm != "sha256-canonical-fields"
            || self.seal.digest != self.seal_digest()
        {
            return Err(CoreError::InvalidManifest(
                "manifest seal does not match its fields".to_string(),
            ));
        }
        Ok(())
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, CoreError> {
        Ok(serde_json::to_vec_pretty(self)?)
    }
}

/// Atomic activation state machine. `stage` validates without changing the
/// active plugin; `commit` swaps in one mutation and preserves the previous
/// manifest; `rollback` restores it. Persistence across process restarts is
/// NOT implemented.
#[derive(Debug, Clone, Default)]
pub struct ActivationStore {
    active: Option<PluginManifest>,
    staged: Option<PluginManifest>,
    previous: Option<PluginManifest>,
}

impl ActivationStore {
    pub fn active(&self) -> Option<&PluginManifest> {
        self.active.as_ref()
    }
    pub fn staged(&self) -> Option<&PluginManifest> {
        self.staged.as_ref()
    }
    pub fn previous(&self) -> Option<&PluginManifest> {
        self.previous.as_ref()
    }

    pub fn stage(&mut self, manifest: PluginManifest) -> Result<(), CoreError> {
        manifest.verify()?;
        self.staged = Some(manifest);
        Ok(())
    }

    pub fn commit(&mut self) -> Result<(), CoreError> {
        let next = self
            .staged
            .take()
            .ok_or_else(|| CoreError::InvalidManifest("no staged plugin to commit".to_string()))?;
        next.verify()?;
        self.previous = self.active.replace(next);
        Ok(())
    }

    pub fn rollback(&mut self) -> Result<(), CoreError> {
        // A merely staged candidate is discarded without touching the
        // committed plugin. Only a committed activation can be rolled back.
        if self.staged.is_some() {
            self.staged = None;
            return Ok(());
        }
        let previous = self
            .previous
            .take()
            .ok_or_else(|| CoreError::InvalidManifest("nothing to roll back".to_string()))?;
        self.active = Some(previous);
        Ok(())
    }
}
