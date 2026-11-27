# Quality profiles and preflight estimates (GE-0024)

Four named profiles map to explicit gates (`ProfilePolicy::for_profile`): **Preview**, **Game-ready**, **High-fidelity**, **Survey-reference**. Each records minimum evidence tier, maximum deviation threshold, whether independent checkpoints and source resolution are required, and the exact gate list. Unknown evidence satisfies no gate. Survey-reference cannot be claimed without independent checkpoints.

`preflight(profile, capabilities, object_count)` returns a `PreflightReport` where duration and peak memory are **bounded ranges** with `confidence: "low-uncalibrated"` and explicit limitations. No single guaranteed number is emitted. Capability suitability is computed from **caller-declared** `HostCapabilities` and labelled `caller-declared; no hardware probing performed`. This module performs no hardware detection, no timing run and no external call.

`authorize_paid_operation` requires both `paid_operations_enabled` in host capabilities and an explicit `consent_granted` flag; it returns a consent token only, never executes or charges anything. `paid_operation` records estimated cost bounds and `consent: required-before-execution` for display before consent.

## Scope limit

These are **policy and estimate contracts**, not calibrated benchmarks. Cost and performance assertions remain estimates until measured on real fixtures. The paid-consent gate guards a future execution path; no payment provider is integrated here.

## Verification

`cargo test --workspace --locked` includes five profile tests: explicit gate mapping across all four profiles (including evidence-tier rejection), range-only preflight output, declared-not-probed capability reporting, paid consent gating, and estimate bound validation.
