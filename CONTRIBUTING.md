# Contributing to Muda

Thank you for contributing to `muda`.

## Development Prerequisites

- Rust stable toolchain (`1.70` or newer) with `rustfmt` and `clippy`
- `git` on `PATH` (used by both the runtime history reader and integration test fixtures)
- Python `3.10+` (for `scripts/codepoint_scan.py` and JSON Schema validation)

## Local Gate

Every change must pass all four repository checks with zero warnings and zero failures before opening a pull request:

```console
$ python3 scripts/codepoint_scan.py
$ cargo fmt --check
$ cargo clippy --all-targets -- -D warnings
$ cargo test
```

When editing shell scripts or JSON files, also verify:

```console
$ sh -n install.sh
$ python3 -m json.tool examples/rates.json > /dev/null
$ python3 -m json.tool schema/rates.schema.json > /dev/null
$ python3 -m json.tool schema/statement.schema.json > /dev/null
```

## Style and Repository Rules

- **Zero external crate dependencies**: `muda` intentionally takes zero external crates in `Cargo.toml` (`Cargo.lock` contains only `muda`). Do not add third-party crates.
- **Zero em-dashes or en-dashes**: Never introduce `U+2014` or `U+2013` in any file. `python3 scripts/codepoint_scan.py` enforces this rule in CI.
- **Deterministic output and refusal pins**: Statement JSON key ordering, minor-unit pricing, `unjoined` reason strings, and CLI exit codes (`0`, `1`, `2`) are pinned by `tests/test_statement.rs`.
