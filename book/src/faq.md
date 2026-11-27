# Muda Frequently Asked Questions

### Why shouldn't I run `cargo install muda` or `pip install muda` from public registries?
Both `crates.io/crates/muda` (a Tauri GUI menu bar library) and `pypi.org/project/muda` (a musical data augmentation library) are unrelated third-party packages. Install `muda` via `install.sh` from `david-engelmann/muda` GitHub Releases, or from source via `cargo install --git https://github.com/david-engelmann/muda` (until `muda-cli` is published on `crates.io`).

### Why did my CI job miss causes from last month?
GitHub Actions `actions/checkout@v4` defaults to `fetch-depth: 1` (a single shallow commit). Always configure `fetch-depth: 0` when running `muda statement` in CI so `revert` and `fix-forward` trailers can join against earlier commits across month boundaries.

### How are commits near midnight on the 1st of the month bucketed?
`muda` buckets commits strictly by **author timestamp in UTC** inside `[YYYY-MM-01T00:00:00Z, next-month-01T00:00:00Z)`. A commit authored at `2026-01-31T20:30:00-05:00` equals `2026-02-01T01:30:00Z` and is billed on the `2026-02` statement.
