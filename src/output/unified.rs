use std::io::Write;

use crate::diff::DiffChange;
use crate::error::RustydiffError;
use crate::output::Formatter;
use crate::record::Value;

pub struct UnifiedFormatter;

impl Formatter for UnifiedFormatter {
    fn write_change(
        &mut self,
        change: &DiffChange,
        out: &mut dyn Write,
    ) -> Result<(), RustydiffError> {
        match change {
            DiffChange::OnlyInLeft { record } => {
                writeln!(out, "- {}", record.raw).map_err(RustydiffError::Io)?;
            }
            DiffChange::OnlyInRight { record } => {
                writeln!(out, "+ {}", record.raw).map_err(RustydiffError::Io)?;
            }
            DiffChange::Modified {
                left,
                right,
                changed_fields,
            } => {
                writeln!(out, "@@ modified @@").map_err(RustydiffError::Io)?;
                for field in changed_fields {
                    let l = left
                        .fields
                        .get(field)
                        .map(value_display)
                        .unwrap_or_else(|| "-".to_string());
                    let r = right
                        .fields
                        .get(field)
                        .map(value_display)
                        .unwrap_or_else(|| "-".to_string());
                    writeln!(out, "-{field}={l}").map_err(RustydiffError::Io)?;
                    writeln!(out, "+{field}={r}").map_err(RustydiffError::Io)?;
                }
            }
        }
        out.flush().map_err(RustydiffError::Io)?;
        Ok(())
    }

    fn flush(&mut self, _out: &mut dyn Write) -> Result<(), RustydiffError> {
        Ok(())
    }
}

fn value_display(v: &Value) -> String {
    match v {
        Value::Str(s) => s.clone(),
        Value::Int(i) => i.to_string(),
        Value::Float(f) => f.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Null => "null".to_string(),
    }
}
