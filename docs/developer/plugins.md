# Plugin SDK contract (GE-0025)

The product-side contract is intentionally separate from the throwaway `prototype/plugin-sandbox/` evidence used for GE-0005. Nothing from that prototype is imported.

- **WIT world** (`WIT_WORLD_SOURCE`, version `1.0.0`): imports only `log` and `progress` (rate/value bounds for progress hostcalls are a future requirement), exports `run`. Filesystem, sockets, process, environment, CLI and clock imports are absent by construction; `wit_world_is_deny_by_default()` guards the shape. Generated bindings (`wit-bindgen`) are **not** produced in this slice.
- **Manifest** (`PluginManifest`): semver version, WIT world version, SHA-256 artefact digest, capability grants (empty by default), exact `name@semver` dependency pins, and a canonical-field integrity seal. `verify()` rejects malformed or tampered manifests. The seal is SHA-256 over canonical fields — **tamper-evident, not an asymmetric signature**. Authentic signing and key management are follow-up work.
- **Activation** (`ActivationStore`): `stage` validates without changing the active plugin; `commit` swaps in one mutation and preserves the previous manifest; `rollback` restores it. Persistence across process restarts is **not** implemented.

## Scope limit

This is a contract and state machine exercised with adversarial manifest fixtures. No guest code is executed, no WASI 0.2 component is instantiated, no sandbox policy is enforced by an OS, and no asymmetric signature is verified. The GE-0005 prototype's WASIp1 runtime evidence remains the only sandbox evidence, and its network-policy denial remains UNVERIFIED. Do not claim a live plugin runtime or a signed marketplace here.

## Verification

`cargo test --workspace --locked` includes five plugin tests: deny-by-default WIT world, deny-by-default capability grants, adversarial manifest rejection (bad WIT version, non-semver, short digest, unpinned dependency, seal tampering), atomic activation with rollback to the previous manifest, and structured invalid-state/serialisation checks.
