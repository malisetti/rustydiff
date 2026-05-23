use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::record::{changed_fields, record_key, Record};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum DiffChange {
    OnlyInLeft { record: Record },
    OnlyInRight { record: Record },
    Modified {
        left: Record,
        right: Record,
        changed_fields: Vec<String>,
    },
}

pub fn diff_streams<I, J>(
    left_iter: I,
    right_iter: J,
    key_fields: &[String],
) -> Result<Vec<DiffChange>, String>
where
    I: IntoIterator<Item = Record>,
    J: IntoIterator<Item = Record>,
{
    let mut left_map: BTreeMap<String, Record> = BTreeMap::new();
    for record in left_iter {
        let key = record_key(&record, key_fields)?;
        left_map.insert(key, record);
    }
    let mut right_map: BTreeMap<String, Record> = BTreeMap::new();
    for record in right_iter {
        let key = record_key(&record, key_fields)?;
        right_map.insert(key, record);
    }

    let mut changes = Vec::new();
    let mut all_keys: Vec<String> = left_map
        .keys()
        .chain(right_map.keys())
        .cloned()
        .collect();
    all_keys.sort();
    all_keys.dedup();

    for key in all_keys {
        match (left_map.get(&key), right_map.get(&key)) {
            (Some(left), Some(right)) => {
                if left != right {
                    let fields = changed_fields(left, right);
                    changes.push(DiffChange::Modified {
                        left: left.clone(),
                        right: right.clone(),
                        changed_fields: fields,
                    });
                }
            }
            (Some(left), None) => {
                changes.push(DiffChange::OnlyInLeft {
                    record: left.clone(),
                });
            }
            (None, Some(right)) => {
                changes.push(DiffChange::OnlyInRight {
                    record: right.clone(),
                });
            }
            (None, None) => {}
        }
    }
    Ok(changes)
}
