use genesis_core::{
    CoreResult, GenerationRecord, StableId, derive_id, missing_objects, plan_regeneration,
};
use std::collections::BTreeMap;

fn id(key: &str) -> StableId {
    derive_id("object", key)
}

fn record(key: &str, hash: &str, deps: &[&str], seed: u64, version: &str) -> GenerationRecord {
    GenerationRecord {
        object_id: id(key),
        input_hash: hash.to_string(),
        seed,
        generator_version: version.to_string(),
        dependencies: deps.iter().map(|d| id(d)).collect(),
    }
}

fn inputs(entries: &[(&str, &str)]) -> BTreeMap<StableId, String> {
    entries
        .iter()
        .map(|(k, v)| (id(k), v.to_string()))
        .collect()
}

#[test]
fn unchanged_inputs_are_a_noop() -> CoreResult<()> {
    let previous = vec![record("building", "h1", &[], 42, "procedural-v0.1")];
    let plan = plan_regeneration(
        &previous,
        &inputs(&[("building", "h1")]),
        42,
        "procedural-v0.1",
    )?;
    assert!(plan.is_noop());
    assert_eq!(plan.unchanged, vec![id("building")]);
    assert!(plan.reasons.is_empty());
    Ok(())
}

#[test]
fn one_changed_object_regenerates_only_its_subgraph() -> CoreResult<()> {
    // road depends on building; sign depends on road; tree is independent.
    let previous = vec![
        record("building", "h1", &[], 42, "v1"),
        record("road", "h2", &["building"], 42, "v1"),
        record("sign", "h3", &["road"], 42, "v1"),
        record("tree", "h4", &[], 42, "v1"),
    ];
    let current = inputs(&[
        ("building", "h1-changed"),
        ("road", "h2"),
        ("sign", "h3"),
        ("tree", "h4"),
    ]);
    let plan = plan_regeneration(&previous, &current, 42, "v1")?;
    assert_eq!(
        plan.regenerate,
        vec![id("building"), id("road"), id("sign")]
    );
    assert_eq!(plan.unchanged, vec![id("tree")]);
    assert!(plan.reasons[&id("road").to_string()].starts_with("dependency "));
    assert!(plan.reasons[&id("road").to_string()].ends_with("changed"));
    assert!(plan.reasons[&id("sign").to_string()].starts_with("dependency "));
    Ok(())
}

#[test]
fn new_objects_and_seed_or_version_changes_are_detected() -> CoreResult<()> {
    let previous = vec![record("building", "h1", &[], 42, "v1")];
    let current = inputs(&[("building", "h1"), ("new", "h9")]);

    let plan = plan_regeneration(&previous, &current, 42, "v1")?;
    assert_eq!(plan.regenerate, vec![id("new")]);

    let plan = plan_regeneration(&previous, &inputs(&[("building", "h1")]), 99, "v1")?;
    assert_eq!(plan.regenerate, vec![id("building")]);
    assert_eq!(plan.reasons[&id("building").to_string()], "seed changed");

    let plan = plan_regeneration(&previous, &inputs(&[("building", "h1")]), 42, "v2")?;
    assert_eq!(
        plan.reasons[&id("building").to_string()],
        "generator version changed"
    );
    Ok(())
}

#[test]
fn removed_objects_are_reported_not_regenerated() -> CoreResult<()> {
    let previous = vec![
        record("building", "h1", &[], 42, "v1"),
        record("road", "h2", &[], 42, "v1"),
    ];
    let current = inputs(&[("building", "h1")]);
    let plan = plan_regeneration(&previous, &current, 42, "v1")?;
    assert!(plan.is_noop());
    assert_eq!(missing_objects(&previous, &current), vec![id("road")]);
    Ok(())
}

#[test]
fn duplicate_generation_records_are_structured_errors() {
    let previous = vec![
        record("building", "h1", &[], 42, "v1"),
        record("building", "h2", &[], 42, "v1"),
    ];
    assert!(plan_regeneration(&previous, &inputs(&[]), 42, "v1").is_err());
}
