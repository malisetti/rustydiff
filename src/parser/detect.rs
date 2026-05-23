use super::LineFormat;

pub fn sniff(line: &str) -> LineFormat {
    let trimmed = line.trim_start();
    if trimmed.starts_with('{') || trimmed.starts_with('[') {
        LineFormat::Json
    } else {
        LineFormat::Logfmt
    }
}
