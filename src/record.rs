use std::collections::BTreeMap;
use std::fmt::Write as _;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Record {
    pub fields: BTreeMap<String, Value>,
    pub raw: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value {
    Str(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    Null,
}

impl Eq for Value {}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Str(a), Value::Str(b)) => a == b,
            (Value::Int(a), Value::Int(b)) => a == b,
            (Value::Float(a), Value::Float(b)) => a.to_bits() == b.to_bits(),
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Null, Value::Null) => true,
            _ => false,
        }
    }
}

impl Value {
    pub fn as_key_string(&self) -> String {
        match self {
            Value::Str(s) => s.clone(),
            Value::Int(i) => i.to_string(),
            Value::Float(f) => f.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Null => String::new(),
        }
    }
}

pub fn record_key(record: &Record, key_fields: &[String]) -> Result<String, String> {
    if key_fields.is_empty() {
        return Ok(fingerprint(record, &[]));
    }
    let mut parts = Vec::with_capacity(key_fields.len());
    for field in key_fields {
        match record.fields.get(field) {
            Some(value) => parts.push(value.as_key_string()),
            None => {
                return Err(format!("missing key field '{field}' in record"));
            }
        }
    }
    Ok(parts.join("\x1f"))
}

pub fn fingerprint(record: &Record, exclude: &[String]) -> String {
    let mut out = String::new();
    for (key, value) in &record.fields {
        if exclude.iter().any(|e| e == key) {
            continue;
        }
        let _ = write!(&mut out, "{key}=");
        write_value_canonical(&mut out, value);
        let _ = write!(&mut out, "\x1e");
    }
    out
}

fn write_value_canonical(out: &mut String, value: &Value) {
    match value {
        Value::Str(s) => {
            let _ = write!(out, "\"{s}\"");
        }
        Value::Int(i) => {
            let _ = write!(out, "{i}");
        }
        Value::Float(f) => {
            let _ = write!(out, "{f}");
        }
        Value::Bool(b) => {
            let _ = write!(out, "{b}");
        }
        Value::Null => {
            out.push_str("null");
        }
    }
}

pub fn changed_fields(left: &Record, right: &Record) -> Vec<String> {
    let mut keys: Vec<String> = left
        .fields
        .keys()
        .chain(right.fields.keys())
        .cloned()
        .collect();
    keys.sort();
    keys.dedup();
    keys.into_iter()
        .filter(|k| left.fields.get(k) != right.fields.get(k))
        .collect()
}
