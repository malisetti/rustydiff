use std::collections::BTreeMap;

use regex::Regex;

use crate::error::RustydiffError;
use crate::record::{Record, Value};

pub fn parse_logfmt_line(line: &str) -> Result<Record, RustydiffError> {
    let raw = line.to_string();
    let re = Regex::new(r#"(\w+)=("(?:\\.|[^"\\])*"|[^\s]+)"#)
        .map_err(|e| RustydiffError::Parse(e.to_string()))?;
    let mut fields = BTreeMap::new();
    for cap in re.captures_iter(line) {
        let (Some(key_match), Some(val_match)) = (cap.get(1), cap.get(2)) else {
            continue;
        };
        let value = parse_logfmt_value(val_match.as_str())?;
        fields.insert(key_match.as_str().to_string(), value);
    }
    if fields.is_empty() && !line.trim().is_empty() {
        return Err(RustydiffError::Parse(
            "logfmt line has no key=value pairs".to_string(),
        ));
    }
    Ok(Record { fields, raw })
}

fn parse_logfmt_value(raw: &str) -> Result<Value, RustydiffError> {
    if raw == "true" {
        return Ok(Value::Bool(true));
    }
    if raw == "false" {
        return Ok(Value::Bool(false));
    }
    if raw == "null" {
        return Ok(Value::Null);
    }
    if let Some(quoted) = raw.strip_prefix('"').and_then(|s| s.strip_suffix('"')) {
        let unescaped = quoted.replace("\\\"", "\"").replace("\\\\", "\\");
        return Ok(Value::Str(unescaped));
    }
    if let Ok(i) = raw.parse::<i64>() {
        return Ok(Value::Int(i));
    }
    if let Ok(f) = raw.parse::<f64>() {
        return Ok(Value::Float(f));
    }
    Ok(Value::Str(raw.to_string()))
}
