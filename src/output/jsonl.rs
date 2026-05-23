use std::io::Write;

use serde_json::json;

use crate::diff::DiffChange;
use crate::error::RustydiffError;
use crate::output::Formatter;
pub struct JsonlFormatter;

impl Formatter for JsonlFormatter {
    fn write_change(
        &mut self,
        change: &DiffChange,
        out: &mut dyn Write,
    ) -> Result<(), RustydiffError> {
        let line = match change {
            DiffChange::OnlyInLeft { record } => {
                json!({"kind":"only_in_left","record":record})
            }
            DiffChange::OnlyInRight { record } => {
                json!({"kind":"only_in_right","record":record})
            }
            DiffChange::Modified {
                left,
                right,
                changed_fields,
            } => json!({
                "kind": "modified",
                "left": left,
                "right": right,
                "changed_fields": changed_fields,
            }),
        };
        serde_json::to_writer(&mut *out, &line).map_err(|e| {
            RustydiffError::Io(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                e.to_string(),
            ))
        })?;
        out.write_all(b"\n").map_err(RustydiffError::Io)?;
        out.flush().map_err(RustydiffError::Io)?;
        Ok(())
    }

    fn flush(&mut self, _out: &mut dyn Write) -> Result<(), RustydiffError> {
        Ok(())
    }
}
