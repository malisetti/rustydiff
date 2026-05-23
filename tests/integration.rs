use std::fs;
use std::process::Command;

use rustydiff::diff_streams;
use rustydiff::output::{write_changes, OutputFormat};
use rustydiff::parser::{parse_auto, parse_line_skipping_bad, LineFormat};
use rustydiff::record::{record_key, Record};

fn fixture(name: &str) -> String {
    let path = format!("tests/fixtures/{name}");
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("fixture {path}: {e}"))
}

fn parse_fixture(name: &str) -> Vec<Record> {
    fixture(name)
        .lines()
        .filter_map(parse_line_skipping_bad)
        .collect()
}

#[test]
fn only_in_left_and_right() {
    let left = parse_fixture("only_left.jsonl");
    let right = parse_fixture("only_right.jsonl");
    let changes = diff_streams(left, right, &["id".to_string()]).expect("diff");
    insta::assert_json_snapshot!(changes);
}

#[test]
fn modified_by_key() {
    let left = parse_fixture("modified_left.jsonl");
    let right = parse_fixture("modified_right.jsonl");
    let changes = diff_streams(left, right, &["id".to_string()]).expect("diff");
    assert_eq!(changes.len(), 1);
    match &changes[0] {
        rustydiff::DiffChange::Modified { changed_fields, .. } => {
            assert!(changed_fields.contains(&"l".to_string()))
        }
        other => panic!("expected modified, got {other:?}"),
    }
}

#[test]
fn output_formats_snapshot() {
    let left = parse_fixture("modified_left.jsonl");
    let right = parse_fixture("modified_right.jsonl");
    let changes = diff_streams(left, right, &["id".to_string()]).expect("diff");

    let mut jsonl = Vec::new();
    write_changes(&changes, OutputFormat::Jsonl, &mut jsonl).expect("jsonl");
    insta::assert_snapshot!("output_jsonl", String::from_utf8_lossy(&jsonl));

    let mut sxs = Vec::new();
    write_changes(&changes, OutputFormat::SideBySide, &mut sxs).expect("sxs");
    insta::assert_snapshot!("output_side_by_side", String::from_utf8_lossy(&sxs));

    let mut unified = Vec::new();
    write_changes(&changes, OutputFormat::Unified, &mut unified).expect("unified");
    insta::assert_snapshot!("output_unified", String::from_utf8_lossy(&unified));
}

#[test]
fn logfmt_auto_detect() {
    let line = r#"id=1 level=info msg="hello world""#;
    let fmt = rustydiff::parser::sniff(line);
    assert_eq!(fmt, LineFormat::Logfmt);
    let record = parse_auto(line, fmt).expect("parse");
    assert_eq!(
        record.fields.get("level").map(|v| format!("{v:?}")),
        Some("Str(\"info\")".to_string())
    );
}

#[test]
fn fingerprint_alignment_default_key() {
    let left = parse_fixture("fp_left.jsonl");
    let right = parse_fixture("fp_right.jsonl");
    let changes = diff_streams(left, right, &[]).expect("diff");
    assert_eq!(changes.len(), 2);
    let same = parse_fixture("modified_left.jsonl");
    let changes = diff_streams(same.clone(), same, &[]).expect("diff");
    assert!(changes.is_empty());
}

#[test]
fn cli_exit_codes() {
    let bin = env!("CARGO_BIN_EXE_rustydiff");
    let left = "tests/fixtures/modified_left.jsonl";
    let right = "tests/fixtures/modified_right.jsonl";
    let diff = Command::new(bin)
        .args([left, right, "--key", "id"])
        .output()
        .expect("run");
    assert_eq!(diff.status.code(), Some(1));

    let same = Command::new(bin)
        .args(["tests/fixtures/empty.jsonl", "tests/fixtures/empty.jsonl"])
        .output()
        .expect("run");
    assert_eq!(same.status.code(), Some(0));

    let err = Command::new(bin)
        .args(["/no/such/file", right])
        .output()
        .expect("run");
    assert_eq!(err.status.code(), Some(2));
}

#[test]
fn record_key_missing_field_errors() {
    let records = parse_fixture("modified_left.jsonl");
    let err = record_key(&records[0], &["missing".to_string()]);
    assert!(err.is_err());
}
