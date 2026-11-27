# Selective regeneration (GE-0027)

`plan_regeneration(previous_records, current_inputs, seed, generator_version)` computes the minimal set of objects to rebuild. An object is dirty when its input hash changed, it is new, the seed or generator version changed, or a dependency is dirty (transitively). Results are ordered blockers-first and include a reason per object; unchanged stable IDs are returned separately. `missing_objects` reports deleted inputs without pretending they can be regenerated.

## Scope

This is a **planning seam**, not a full live rebuild of geometry. The caller applies the plan by running only the selected generators and replacing the corresponding payloads; that integration is a follow-up. Stable IDs come from Core `derive_id`, so unchanged objects keep their identity.

## Verification

`cargo test --workspace --locked` includes five tests: no-op, one changed building with transitive dependents, new/seed/version changes, removed objects, and duplicate-record errors.
