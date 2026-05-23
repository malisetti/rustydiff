#![forbid(unsafe_code)]

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;
use rustydiff::output::{write_changes_stdout, OutputFormat};
use rustydiff::parser::parse_line_skipping_bad;
use rustydiff::record::Record;
use rustydiff::{diff_streams, RustydiffError};

#[derive(Debug, Parser)]
#[command(
    name = "rustydiff",
    version,
    about = "Diff structured log streams (JSON-lines or logfmt)"
)]
struct Cli {
    /// Left log file
    left: PathBuf,
    /// Right log file
    right: PathBuf,
    /// Comma-separated key fields for alignment (default: full-record fingerprint)
    #[arg(long, value_delimiter = ',')]
    key: Vec<String>,
    /// Output format: jsonl | side-by-side | unified
    #[arg(long, default_value = "jsonl")]
    format: String,
    /// Verbose diagnostics on stderr
    #[arg(long)]
    verbose: bool,
}

fn read_records(path: &PathBuf, verbose: bool) -> Result<Vec<Record>, RustydiffError> {
    let file = File::open(path).map_err(RustydiffError::Io)?;
    let reader = BufReader::new(file);
    let mut records = Vec::new();
    for (idx, line) in reader.lines().enumerate() {
        let line = line.map_err(RustydiffError::Io)?;
        match parse_line_skipping_bad(&line) {
            Some(record) => records.push(record),
            None => {
                if verbose && !line.trim().is_empty() {
                    eprintln!(
                        "warning: skipped unparseable line {} in {}",
                        idx + 1,
                        path.display()
                    );
                }
            }
        }
    }
    Ok(records)
}

fn run(cli: Cli) -> Result<u8, RustydiffError> {
    let format = OutputFormat::parse(&cli.format)?;
    let left = read_records(&cli.left, cli.verbose)?;
    let right = read_records(&cli.right, cli.verbose)?;
    let changes = diff_streams(left, right, &cli.key).map_err(RustydiffError::Argument)?;
    if !changes.is_empty() {
        write_changes_stdout(&changes, format)?;
    }
    Ok(if changes.is_empty() { 0 } else { 1 })
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match run(cli) {
        Ok(code) => ExitCode::from(code),
        Err(err) => {
            eprintln!("rustydiff: {err}");
            ExitCode::from(2)
        }
    }
}
