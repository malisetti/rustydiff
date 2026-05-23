mod jsonl;
mod side_by_side;
mod unified;

use std::io::{self, Write};

pub use jsonl::JsonlFormatter;
pub use side_by_side::SideBySideFormatter;
pub use unified::UnifiedFormatter;

use crate::diff::DiffChange;
use crate::error::RustydiffError;

pub trait Formatter {
    fn write_change(
        &mut self,
        change: &DiffChange,
        out: &mut dyn Write,
    ) -> Result<(), RustydiffError>;
    fn flush(&mut self, out: &mut dyn Write) -> Result<(), RustydiffError>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Jsonl,
    SideBySide,
    Unified,
}

impl OutputFormat {
    pub fn parse(s: &str) -> Result<Self, RustydiffError> {
        match s {
            "jsonl" => Ok(Self::Jsonl),
            "side-by-side" => Ok(Self::SideBySide),
            "unified" => Ok(Self::Unified),
            other => Err(RustydiffError::Argument(format!(
                "unknown format '{other}'; expected jsonl, side-by-side, or unified"
            ))),
        }
    }

    pub fn build_formatter(self) -> Box<dyn Formatter> {
        match self {
            Self::Jsonl => Box::new(JsonlFormatter),
            Self::SideBySide => Box::new(SideBySideFormatter),
            Self::Unified => Box::new(UnifiedFormatter),
        }
    }
}

pub fn write_changes(
    changes: &[DiffChange],
    format: OutputFormat,
    out: &mut dyn Write,
) -> Result<(), RustydiffError> {
    let mut formatter = format.build_formatter();
    for change in changes {
        formatter.write_change(change, out)?;
    }
    formatter.flush(out)?;
    Ok(())
}

pub fn write_changes_stdout(
    changes: &[DiffChange],
    format: OutputFormat,
) -> Result<(), RustydiffError> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    write_changes(changes, format, &mut handle)
}
