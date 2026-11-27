# Quality profiles and preflight estimates (GE-0024)

Four named profiles map to explicit gates (`ProfilePolicy::for_profile`): **Preview**, **Game-ready**, **High-fidelity**, **Survey-reference**. Each records minimum evidence tier, maximum deviation threshold, whether independent checkpoints and source resolution are required, and the exact gate list. Unknown evidence satisfies no gate. Survey-reference cannot be claimed without independent checkpoints.

`preflight(profile, capabilities, object_count)` returns a `PreflightReport` where duration and peak memory are **bounded ranges** with `confidence: "low-uncalibrated"` and explicit limitations. No single guaranteed number is emitted. Capability suitability is computed from **caller-declared** `HostCapabilities` and labelled `caller-declared; no hardware probing performed`. This module performs no hardware detection, no timing run and no external call.

`authorize_paid_operation` requires both `paid_operations_enabled` in host capabilities and an explicit `consent_granted` flag; it returns a consent token only, never executes or charges anything. `PreflightReport::add_paid_operation` attaches a named `EstimateRange` to the report for display before consent. The entry retains cost bounds, unit, confidence, method and limitations, plus `consent: required-before-execution`. `cuda_available` is informational: GPU suitability is vendor-neutral and does not depend on a specific backend.

## Scope limit

These are **policy and estimate contracts**, not calibrated benchmarks. Cost and performance assertions remain estimates until measured on real fixtures. The paid-consent gate guards a future execution path; no payment provider is integrated here.

## Verification

`cargo test --workspace --locked` includes six profile tests: explicit gate mapping across all four profiles (Unknown evidence rejected by every profile, observed accepted by every profile, deviation/resolution/checkpoint thresholds), range-only preflight output with survey warning, paid-cost metadata retention in the report, declared-not-probed capability reporting, paid consent gating (including empty name), and estimate bound validation.
