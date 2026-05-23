mod detect;
mod json;
mod logfmt;

pub use detect::sniff;
pub use json::parse_json_line;
pub use logfmt::parse_logfmt_line;

use crate::error::RustydiffError;
use crate::record::Record;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineFormat {
    Json,
    Logfmt,
}

pub fn parse_auto(line: &str, format: LineFormat) -> Result<Record, RustydiffError> {
    match format {
        LineFormat::Json => parse_json_line(line),
        LineFormat::Logfmt => parse_logfmt_line(line),
    }
}

pub fn parse_line_skipping_bad(line: &str) -> Option<Record> {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return None;
    }
    let format = sniff(trimmed);
    parse_auto(trimmed, format).ok()
}
