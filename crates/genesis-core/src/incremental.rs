//! Selective regeneration planning (GE-0027).
//!
//! Given the generation records from the previous run and the current input
//! hashes, compute exactly which objects must be rebuilt. Unchanged objects
//! keep their stable IDs and hashes. This is a planning seam: the caller then
//! runs only the selected generators.

use crate::{CoreError, Result, StableId};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, VecDeque};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GenerationRecord {
    pub object_id: StableId,
    pub input_hash: String,
    pub seed: u64,
    pub generator_version: String,
    #[serde(default)]
    pub dependencies: Vec<StableId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegenerationPlan {
    pub schema_version: u32,
    pub regenerate: Vec<StableId>,
    pub unchanged: Vec<StableId>,
    pub reasons: BTreeMap<String, String>,
}

impl RegenerationPlan {
    pub fn is_noop(&self) -> bool {
        self.regenerate.is_empty()
    }
}

/// Compute the minimal regeneration set.
///
/// An object regenerates when its own input hash changed, it is new, the run
/// seed or generator version changed, or any dependency regenerates. This is
/// transitive: dependents of dependents are included exactly once.
pub fn plan_regeneration(
    previous: &[GenerationRecord],
    current_inputs: &BTreeMap<StableId, String>,
    seed: u64,
    generator_version: &str,
) -> Result<RegenerationPlan> {
    let mut by_id = BTreeMap::new();
    for record in previous {
        if by_id.insert(record.object_id.clone(), record).is_some() {
            return Err(CoreError::InvalidManifest(format!(
                "duplicate generation record for {}",
                record.object_id
            )));
        }
    }

    let mut dirty: BTreeSet<StableId> = BTreeSet::new();
    let mut reasons: BTreeMap<String, String> = BTreeMap::new();

    for (object_id, hash) in current_inputs {
        match by_id.get(object_id) {
            None => {
                reasons.insert(object_id.to_string(), "new object".to_string());
                dirty.insert(object_id.clone());
            }
            Some(old) => {
                if old.input_hash != *hash {
                    reasons.insert(object_id.to_string(), "input hash changed".to_string());
                    dirty.insert(object_id.clone());
                }
                if old.seed != seed {
                    reasons
                        .entry(object_id.to_string())
                        .or_insert_with(|| "seed changed".to_string());
                    dirty.insert(object_id.clone());
                }
                if old.generator_version != generator_version {
                    reasons
                        .entry(object_id.to_string())
                        .or_insert_with(|| "generator version changed".to_string());
                    dirty.insert(object_id.clone());
                }
            }
        }
    }

    // Removed objects do not need regeneration; they must be deleted by the
    // caller, which is reported separately via `missing_objects`.
    let mut queue: VecDeque<StableId> = dirty.iter().cloned().collect();
    while let Some(changed) = queue.pop_front() {
        for record in previous {
            if record.dependencies.contains(&changed) && dirty.insert(record.object_id.clone()) {
                reasons.insert(
                    record.object_id.to_string(),
                    format!("dependency {changed} changed"),
                );
                queue.push_back(record.object_id.clone());
            }
        }
    }

    // Dependents come after their dependencies (blockers first), matching the
    // frontier workflow used by the ticket system.
    let mut regenerate: Vec<StableId> = dirty.iter().cloned().collect();
    regenerate.sort_by_key(|object_id| {
        (
            dependency_depth(object_id, &by_id, &mut BTreeSet::new()),
            object_id.clone(),
        )
    });
    let unchanged: Vec<StableId> = current_inputs
        .keys()
        .filter(|id| !dirty.contains(*id))
        .cloned()
        .collect();

    Ok(RegenerationPlan {
        schema_version: 1,
        regenerate,
        unchanged,
        reasons,
    })
}

fn dependency_depth(
    object_id: &StableId,
    by_id: &BTreeMap<StableId, &GenerationRecord>,
    visiting: &mut BTreeSet<StableId>,
) -> usize {
    if !visiting.insert(object_id.clone()) {
        return 0; // cycle guard
    }
    let depth = by_id
        .get(object_id)
        .map(|record| {
            record
                .dependencies
                .iter()
                .map(|dep| 1 + dependency_depth(dep, by_id, visiting))
                .max()
                .unwrap_or(0)
        })
        .unwrap_or(0);
    visiting.remove(object_id);
    depth
}

/// Objects present in the previous run but absent from current inputs.
pub fn missing_objects(
    previous: &[GenerationRecord],
    current_inputs: &BTreeMap<StableId, String>,
) -> Vec<StableId> {
    previous
        .iter()
        .filter(|record| !current_inputs.contains_key(&record.object_id))
        .map(|record| record.object_id.clone())
        .collect()
}
