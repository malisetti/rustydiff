use std::collections::BTreeMap;

use serde_json::Value as JsonValue;

use crate::error::RustydiffError;
use crate::record::{Record, Value};

pub fn parse_json_line(line: &str) -> Result<Record, RustydiffError> {
    let raw = line.to_string();
    let value: JsonValue = serde_json::from_str(line)
        .map_err(|e| RustydiffError::Parse(format!("invalid JSON line: {e}")))?;
    let fields = json_value_to_fields(&value)?;
    Ok(Record { fields, raw })
}

fn json_value_to_fields(value: &JsonValue) -> Result<BTreeMap<String, Value>, RustydiffError> {
    match value {
        JsonValue::Object(map) => {
            let mut fields = BTreeMap::new();
            for (k, v) in map {
                fields.insert(k.clone(), json_scalar_to_value(v)?);
            }
            Ok(fields)
        }
        _ => Err(RustydiffError::Parse(
            "JSON line must be an object".to_string(),
        )),
    }
}

fn json_scalar_to_value(value: &JsonValue) -> Result<Value, RustydiffError> {
    match value {
        JsonValue::Null => Ok(Value::Null),
        JsonValue::Bool(b) => Ok(Value::Bool(*b)),
        JsonValue::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(Value::Int(i))
            } else if let Some(f) = n.as_f64() {
                Ok(Value::Float(f))
            } else {
                Err(RustydiffError::Parse("unsupported JSON number".to_string()))
            }
        }
        JsonValue::String(s) => Ok(Value::Str(s.clone())),
        JsonValue::Array(_) | JsonValue::Object(_) => Err(RustydiffError::Parse(
            "nested JSON values are not supported in records".to_string(),
        )),
    }
}
