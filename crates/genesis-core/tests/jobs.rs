use genesis_core::{
    CoreResult, JobEvent, JobRecord, JobState, Supervisor, Worker, WorkerDescriptor,
};

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
