use serde::{Deserialize, Serialize};

use crate::record::Record;

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

/// Stub for TDD — replaced in the implementation commit.
pub fn diff_streams<I, J>(
    left_iter: I,
    right_iter: J,
    key_fields: &[String],
) -> Result<Vec<DiffChange>, String>
where
    I: IntoIterator<Item = Record>,
    J: IntoIterator<Item = Record>,
{
    let _ = (left_iter, right_iter, key_fields);
    Ok(Vec::new())
}
