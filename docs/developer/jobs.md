# External worker contract (GE-0023)

`WorkerDescriptor` carries argv-only execution, a restricted working directory, an **empty** environment (no inherited credentials), a memory cap and a timeout. `JobRecord` records job type/version, input hashes, seed, parameters, worker descriptor, state and normalised events. `Supervisor::execute` drives stages and emits `StageStarted`, `Progress`, `Log`, `Checkpoint`, `Done`, `Error` or `Cancelled` events with canonical error codes.

A cancellation token checked between stages stops the job before the next stage. Provider failures map to `WorkerStageFailed`; timeouts map to `TimedOut`. All output is JSON-serialisable for the versioned job record contract.

## Scope limit

This is the **supervision contract and state machine**, exercised with a fake worker. Actual OS process spawning, kill-tree termination, per-OS sandbox profiles (Job Objects/seccomp/Apple sandbox) and real worker version probes are follow-up work; do not claim they are implemented.

## Verification

`cargo test --workspace --locked` includes four job tests: completion with checkpoints, cancellation, provider-failure mapping and descriptor validation against environment/memory/timeout errors.
