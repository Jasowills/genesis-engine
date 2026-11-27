//! External worker contract and normalised job events (GE-0023).
//!
//! This is a supervision framework, not a specific tool integration. It never
//! invokes a shell, never inherits environment variables, and maps worker
//! errors to canonical codes. Actual process spawning/kill-tree behaviour is a
//! follow-up; the fake worker here exercises the event and cancellation seam.

use crate::CoreError;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, time::Instant};

pub const JOB_RECORD_SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum JobState {
    Proposed,
    Queued,
    Running { stage: String, progress: f32 },
    Completed,
    Failed { code: String },
    Cancelled,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum JobEvent {
    StageStarted {
        id: String,
        expected_steps: u32,
    },
    Progress {
        stage: String,
        progress: f32,
        units: String,
    },
    Log {
        level: String,
        message: String,
    },
    Checkpoint {
        id: String,
        hash: String,
    },
    Done {
        output_manifest: String,
    },
    Error {
        code: String,
        provider_error: String,
    },
    Cancelled {
        stage: String,
        reason: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkerDescriptor {
    pub name: String,
    pub version: String,
    /// argv-only: the first element is the executable, never a shell string.
    pub argv: Vec<String>,
    pub working_directory: String,
    /// Must be empty: credentials must not be inherited by workers.
    pub environment: BTreeMap<String, String>,
    pub memory_limit_bytes: u64,
    pub timeout_ms: u64,
}

impl WorkerDescriptor {
    pub fn validate(&self) -> Result<(), CoreError> {
        if self.name.is_empty() || self.version.is_empty() || self.argv.is_empty() {
            return Err(CoreError::InvalidManifest(
                "worker needs name, version and argv".to_string(),
            ));
        }
        if !self.environment.is_empty() {
            return Err(CoreError::InvalidManifest(
                "worker environment must be empty (no inherited credentials)".to_string(),
            ));
        }
        if self.working_directory.is_empty()
            || self.working_directory.starts_with('/')
            || self.working_directory.contains("..")
        {
            return Err(CoreError::InvalidManifest(
                "worker working directory must be a safe project-relative path".to_string(),
            ));
        }
        if self.memory_limit_bytes == 0 || self.timeout_ms == 0 {
            return Err(CoreError::InvalidManifest(
                "worker needs memory and time limits".to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JobRecord {
    pub schema_version: u32,
    pub job_type: String,
    pub job_version: String,
    pub input_hashes: Vec<String>,
    pub seed: u64,
    pub parameters: BTreeMap<String, String>,
    pub worker: WorkerDescriptor,
    pub state: JobState,
    pub events: Vec<JobEvent>,
    pub output_manifest: Option<String>,
}

impl JobRecord {
    pub fn new(
        job_type: &str,
        job_version: &str,
        seed: u64,
        worker: WorkerDescriptor,
    ) -> Result<Self, CoreError> {
        worker.validate()?;
        Ok(Self {
            schema_version: JOB_RECORD_SCHEMA_VERSION,
            job_type: job_type.to_string(),
            job_version: job_version.to_string(),
            input_hashes: Vec::new(),
            seed,
            parameters: BTreeMap::new(),
            worker,
            state: JobState::Proposed,
            events: Vec::new(),
            output_manifest: None,
        })
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, CoreError> {
        Ok(serde_json::to_vec_pretty(self)?)
    }
}

/// Remove obvious credential values from provider text before persisting it.
/// This is a bounded best-effort helper, not a guarantee against all secrets.
pub fn redact(text: &str) -> String {
    let mut out = String::with_capacity(text.len().min(512));
    for word in text.split_whitespace().take(64) {
        let key = word
            .trim_matches(|c: char| !c.is_ascii_alphanumeric())
            .to_ascii_lowercase();
        if ["token", "key", "secret", "password", "apikey"].contains(&key.as_str()) {
            out.push_str("[REDACTED] ");
        } else {
            out.push_str(word);
            out.push(' ');
        }
    }
    out.trim_end().to_string()
}

/// Minimal worker interface. Implementations must not own domain rules.
pub trait Worker {
    fn descriptor(&self) -> &WorkerDescriptor;
    fn stages(&self) -> Vec<String>;
    fn run_stage(&self, stage: &str) -> Result<String, String>;
}

pub struct Supervisor {
    cancelled: std::sync::Arc<std::sync::atomic::AtomicBool>,
}

impl Default for Supervisor {
    fn default() -> Self {
        Self::new()
    }
}

impl Supervisor {
    pub fn new() -> Self {
        Self {
            cancelled: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
        }
    }

    pub fn token(&self) -> std::sync::Arc<std::sync::atomic::AtomicBool> {
        self.cancelled.clone()
    }

    pub fn cancel(&self) {
        self.cancelled
            .store(true, std::sync::atomic::Ordering::SeqCst);
    }

    /// Drive a worker through its stages, emitting normalised events. Failure,
    /// cancellation and timeout all produce deterministic terminal states.
    pub fn execute<W: Worker>(&self, job: &mut JobRecord, worker: &W) -> Result<(), CoreError> {
        worker.descriptor().validate()?;
        if matches!(
            job.state,
            JobState::Completed | JobState::Failed { .. } | JobState::Cancelled
        ) {
            return Err(CoreError::InvalidManifest(
                "terminal job cannot be re-executed".to_string(),
            ));
        }
        let stages = worker.stages();
        job.state = JobState::Queued;
        let started = Instant::now();

        for (index, stage) in stages.iter().enumerate() {
            if self.cancelled.load(std::sync::atomic::Ordering::SeqCst) {
                job.state = JobState::Cancelled;
                job.events.push(JobEvent::Cancelled {
                    stage: stage.clone(),
                    reason: "cancelled by caller".to_string(),
                });
                return Ok(());
            }
            job.state = JobState::Running {
                stage: stage.clone(),
                progress: index as f32 / stages.len() as f32,
            };
            job.events.push(JobEvent::StageStarted {
                id: stage.clone(),
                expected_steps: stages.len() as u32,
            });
            job.events.push(JobEvent::Log {
                level: "info".to_string(),
                message: redact(&format!("stage {stage} started")),
            });
            job.events.push(JobEvent::Progress {
                stage: stage.clone(),
                progress: index as f32 / stages.len() as f32,
                units: "fraction_of_stages".to_string(),
            });

            match worker.run_stage(stage) {
                Ok(hash) => job.events.push(JobEvent::Checkpoint {
                    id: stage.clone(),
                    hash,
                }),
                Err(provider_error) => {
                    job.state = JobState::Failed {
                        code: "WorkerStageFailed".to_string(),
                    };
                    job.events.push(JobEvent::Error {
                        code: "WorkerStageFailed".to_string(),
                        provider_error: redact(&provider_error),
                    });
                    return Ok(());
                }
            }

            if started.elapsed().as_millis() as u64 > worker.descriptor().timeout_ms {
                job.state = JobState::Failed {
                    code: "TimedOut".to_string(),
                };
                job.events.push(JobEvent::Error {
                    code: "TimedOut".to_string(),
                    provider_error: format!("exceeded {} ms", worker.descriptor().timeout_ms),
                });
                return Ok(());
            }
        }

        let manifest = format!("manifest:{}:{}", job.job_type, job.seed);
        job.output_manifest = Some(manifest.clone());
        job.state = JobState::Completed;
        job.events.push(JobEvent::Done {
            output_manifest: manifest,
        });
        Ok(())
    }
}
