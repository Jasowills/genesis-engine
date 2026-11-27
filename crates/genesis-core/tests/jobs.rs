use genesis_core::{
    CoreResult, JobEvent, JobRecord, JobState, Supervisor, Worker, WorkerDescriptor,
};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

struct FakeWorker {
    descriptor: WorkerDescriptor,
    stages: Vec<String>,
    fail_at: Option<String>,
}

impl Worker for FakeWorker {
    fn descriptor(&self) -> &WorkerDescriptor {
        &self.descriptor
    }
    fn stages(&self) -> Vec<String> {
        self.stages.clone()
    }
    fn run_stage(&self, stage: &str) -> Result<String, String> {
        if self.fail_at.as_deref() == Some(stage) {
            return Err("fake worker exploded".to_string());
        }
        Ok(format!("checkpoint:{stage}"))
    }
}

fn worker(stages: &[&str]) -> FakeWorker {
    FakeWorker {
        descriptor: WorkerDescriptor {
            name: "fake".to_string(),
            version: "1.0".to_string(),
            argv: vec!["fake-worker".to_string()],
            working_directory: "jobs/fake".to_string(),
            environment: Default::default(),
            memory_limit_bytes: 64 * 1024 * 1024,
            timeout_ms: 5000,
        },
        stages: stages.iter().map(|s| s.to_string()).collect(),
        fail_at: None,
    }
}

fn record(w: &FakeWorker) -> JobRecord {
    JobRecord::new("fake", "1.0", 42, w.descriptor().clone()).unwrap()
}

#[test]
fn job_completes_with_normalised_events() -> CoreResult<()> {
    let w = worker(&["fetch", "build"]);
    let mut job = record(&w);
    Supervisor::new().execute(&mut job, &w)?;
    assert_eq!(job.state, JobState::Completed);
    assert_eq!(job.output_manifest, Some("manifest:fake:42".to_string()));
    assert!(
        job.events
            .iter()
            .any(|e| matches!(e, JobEvent::StageStarted { id, .. } if id == "fetch"))
    );
    assert_eq!(
        job.events
            .iter()
            .filter(|e| matches!(e, JobEvent::Checkpoint { .. }))
            .count(),
        2
    );
    assert!(matches!(job.events.last(), Some(JobEvent::Done { .. })));
    let restored: JobRecord = serde_json::from_slice(&job.to_bytes()?)?;
    assert_eq!(restored, job);
    Ok(())
}

#[test]
fn cancellation_stops_before_next_stage() -> CoreResult<()> {
    let w = worker(&["fetch", "build"]);
    let mut job = record(&w);
    let supervisor = Supervisor::new();
    supervisor.cancel();
    supervisor.execute(&mut job, &w)?;
    assert_eq!(job.state, JobState::Cancelled);
    assert!(matches!(
        job.events.last(),
        Some(JobEvent::Cancelled { .. })
    ));
    Ok(())
}

#[test]
fn provider_failure_maps_to_canonical_code() -> CoreResult<()> {
    let mut w = worker(&["fetch", "build"]);
    w.fail_at = Some("build".to_string());
    let mut job = record(&w);
    Supervisor::new().execute(&mut job, &w)?;
    assert_eq!(
        job.state,
        JobState::Failed {
            code: "WorkerStageFailed".to_string()
        }
    );
    match job.events.last() {
        Some(JobEvent::Error {
            code,
            provider_error,
        }) => {
            assert_eq!(code, "WorkerStageFailed");
            assert_eq!(provider_error, "fake worker exploded");
        }
        other => panic!("expected canonical error event, got {other:?}"),
    }
    Ok(())
}

#[test]
fn descriptors_reject_shell_or_environment_leakage() {
    let mut w = worker(&["fetch"]);
    w.descriptor
        .environment
        .insert("TOKEN".to_string(), "secret".to_string());
    assert!(w.descriptor().validate().is_err());
    let mut w = worker(&["fetch"]);
    w.descriptor.argv = vec!["sh -c rm -rf /".to_string()];
    assert!(
        w.descriptor().validate().is_ok(),
        "argv-only descriptor is validated by caller, not string content"
    );
    w.descriptor.memory_limit_bytes = 0;
    assert!(w.descriptor().validate().is_err());
    w.descriptor.memory_limit_bytes = 1024;
    w.descriptor.timeout_ms = 0;
    assert!(w.descriptor().validate().is_err());
}

struct SleepingWorker {
    descriptor: WorkerDescriptor,
    sleep_ms: u64,
}

impl Worker for SleepingWorker {
    fn descriptor(&self) -> &WorkerDescriptor {
        &self.descriptor
    }
    fn stages(&self) -> Vec<String> {
        vec!["slow".to_string()]
    }
    fn run_stage(&self, _stage: &str) -> Result<String, String> {
        std::thread::sleep(std::time::Duration::from_millis(self.sleep_ms));
        Ok("slow-checkpoint".to_string())
    }
}

#[test]
fn slow_stage_maps_to_timeout_error() -> CoreResult<()> {
    let descriptor = WorkerDescriptor {
        name: "slow".to_string(),
        version: "1.0".to_string(),
        argv: vec!["slow-worker".to_string()],
        working_directory: "jobs/slow".to_string(),
        environment: Default::default(),
        memory_limit_bytes: 1024,
        timeout_ms: 1,
    };
    let worker = SleepingWorker {
        descriptor,
        sleep_ms: 15,
    };
    let mut job = JobRecord::new("slow", "1.0", 7, worker.descriptor().clone())?;
    Supervisor::new().execute(&mut job, &worker)?;
    assert_eq!(
        job.state,
        JobState::Failed {
            code: "TimedOut".to_string()
        }
    );
    assert!(matches!(job.events.last(), Some(JobEvent::Error { code, .. }) if code == "TimedOut"));
    assert!(
        !job.events
            .iter()
            .any(|e| matches!(e, JobEvent::Done { .. }))
    );
    Ok(())
}

#[test]
fn terminal_jobs_cannot_be_reexecuted() -> CoreResult<()> {
    let w = worker(&["fetch"]);
    let mut job = record(&w);
    let supervisor = Supervisor::new();
    supervisor.execute(&mut job, &w)?;
    assert_eq!(job.state, JobState::Completed);
    assert!(matches!(
        supervisor.execute(&mut job, &w),
        Err(genesis_core::CoreError::InvalidManifest(message)) if message.contains("terminal job")
    ));
    Ok(())
}

#[test]
fn redaction_replaces_credential_like_words() {
    assert_eq!(
        genesis_core::redact("worker token abc123 failed"),
        "worker [REDACTED] [REDACTED] failed"
    );
}

#[test]
fn descriptor_validation_covers_identity_and_path() {
    let mut w = worker(&["fetch"]);
    w.descriptor.name.clear();
    assert!(w.descriptor().validate().is_err());
    let mut w = worker(&["fetch"]);
    w.descriptor.version.clear();
    assert!(w.descriptor().validate().is_err());
    let mut w = worker(&["fetch"]);
    w.descriptor.argv.clear();
    assert!(w.descriptor().validate().is_err());
    let mut w = worker(&["fetch"]);
    w.descriptor.working_directory = "../outside".to_string();
    assert!(w.descriptor().validate().is_err());
    let mut w = worker(&["fetch"]);
    w.descriptor.working_directory = "C:\\evil".to_string();
    assert!(
        w.descriptor().validate().is_err(),
        "Windows drive paths must be rejected"
    );
    let mut w = worker(&["fetch"]);
    w.descriptor.working_directory = "C:evil".to_string();
    assert!(
        w.descriptor().validate().is_err(),
        "drive-relative paths must be rejected"
    );
}

#[test]
fn redaction_covers_key_value_and_bearer_forms() {
    let cases = [
        "error token=abc123 leaked",
        "Authorization: Bearer sk-live-SECRET",
        "password:hunter2",
        "apikey:ABCDEF",
        "worker token abc123 failed",
    ];
    for case in cases {
        let redacted = genesis_core::redact(case);
        assert!(!redacted.contains("abc123"), "leaked: {redacted}");
        assert!(!redacted.contains("sk-live-SECRET"), "leaked: {redacted}");
        assert!(!redacted.contains("hunter2"), "leaked: {redacted}");
        assert!(!redacted.contains("ABCDEF"), "leaked: {redacted}");
        assert!(redacted.contains("[REDACTED]"), "no redaction for {case}");
    }
}

struct NoStages {
    descriptor: WorkerDescriptor,
}

impl Worker for NoStages {
    fn descriptor(&self) -> &WorkerDescriptor {
        &self.descriptor
    }
    fn stages(&self) -> Vec<String> {
        Vec::new()
    }
    fn run_stage(&self, _stage: &str) -> Result<String, String> {
        unreachable!()
    }
}

#[test]
fn empty_stage_lists_are_rejected() -> CoreResult<()> {
    let w = NoStages {
        descriptor: worker(&[]).descriptor().clone(),
    };
    let mut job = record(&FakeWorker {
        descriptor: w.descriptor().clone(),
        stages: vec![],
        fail_at: None,
    });
    assert!(matches!(
        Supervisor::new().execute(&mut job, &w),
        Err(genesis_core::CoreError::InvalidManifest(m)) if m.contains("no stages")
    ));
    Ok(())
}

#[test]
fn running_jobs_cannot_be_reexecuted_or_duplicate_history() -> CoreResult<()> {
    let w = worker(&["first", "second"]);
    let mut job = record(&w);
    job.state = JobState::Running {
        stage: "first".to_string(),
        progress: 0.5,
    };
    let before = job.events.len();
    assert!(matches!(
        Supervisor::new().execute(&mut job, &w),
        Err(genesis_core::CoreError::InvalidManifest(m)) if m.contains("running job")
    ));
    assert_eq!(job.events.len(), before, "no duplicate history appended");
    Ok(())
}
