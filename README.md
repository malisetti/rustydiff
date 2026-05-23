# rustydiff

Structured-diff CLI for log streams. Compare two JSON-lines or logfmt files, align records by a key field (or full-record fingerprint), and emit **OnlyInLeft**, **OnlyInRight**, or **Modified** changes.

## Install

```bash
cargo install --path .
# or
cargo build --release && cp target/release/rustydiff ~/.local/bin/
```

## Quickstart

```bash
# JSON-lines, align on `id`, JSONL output (default)
rustydiff left.jsonl right.jsonl --key id

# Side-by-side view
rustydiff left.jsonl right.jsonl --key id --format side-by-side

# Unified-style field deltas
rustydiff left.log right.log --format unified
```

## Query & format reference

| Flag | Description |
|------|-------------|
| `left`, `right` | Input log files (JSON-lines or logfmt per line, auto-detected) |
| `--key FIELDS` | Comma-separated fields for alignment (default: full-record fingerprint) |
| `--format` | `jsonl` (default), `side-by-side`, or `unified` |
| `--verbose` | Warn on skipped bad lines (stderr) |

**Exit codes** (grep-style): `0` no diff, `1` differences found, `2` error.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.
