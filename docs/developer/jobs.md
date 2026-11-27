# External worker contract (GE-0023)

`WorkerDescriptor` carries argv-only data (no shell execution is implemented), a project-relative working-directory check, an **empty** environment (no inherited credentials), a memory cap and a timeout field. `JobRecord` records job type/version, input hashes, seed, parameters, worker descriptor, state and normalised events. `Supervisor::execute` drives stages and emits `StageStarted`, `Progress`, `Log`, `Checkpoint`, `Done`, `Error` or `Cancelled` events with canonical error codes.

A cancellation token checked between stages stops the job before the next stage. Provider failures map to `WorkerStageFailed`; timeouts are mapped to `TimedOut` **after** a stage returns — intra-stage preemption is not implemented. Terminal jobs cannot be re-executed. Provider text is passed through bounded `redact()` before persistence. All output is JSON-serialisable for the versioned job record contract; progress is a 0..1 fraction.

## Scope limit

This is the **supervision contract and state machine**, exercised with a fake worker. Actual OS process spawning, argv execution, CPU caps, kill-tree termination, per-OS sandbox profiles (Job Objects/seccomp/Apple sandbox), intra-stage timeout preemption, checkpoint resume and real worker version probes are follow-up work; do not claim they are implemented.

## Verification

`cargo test --workspace --locked` includes eight job tests: completion with checkpoints and progress logs, pre-start cancellation, provider-failure mapping, post-stage timeout mapping, terminal-state guard, redaction, and descriptor validation against identity/path/environment/memory/timeout errors.
