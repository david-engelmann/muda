<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="media/lockup-dark.svg">
    <img src="media/lockup-light.svg" alt="muda" width="760">
  </picture>
</p>

<p align="center">
  <img src="media/money-shot.gif" alt="muda monthly rework statement showcase" width="880">
</p>

<p align="center">
  <img src="media/architecture.svg" alt="muda Git DAG rework billing architecture" width="900">
</p>

# Muda Operator and Library Guide

`muda` is a zero-dependency Rust CLI and library (`muda statement`) that audits a Git repository's commit history and emits a deterministic monthly rework statement as JSON. While `catchledger` prices checks, `muda` prices work by attributing reverts, fix-forwards, and redone tasks back to the earlier commits that caused them.

---

## 1. Installation (Avoiding the `crates.io` Name Collision)

> [!IMPORTANT]
> Do **not** run bare `cargo install muda`. On `crates.io`, the `muda` package name is occupied by Tauri's desktop menu-bar library (`tauri-apps/muda`), which has no binary target and is unrelated to this tool. Always install via `install.sh` from GitHub Releases or via `cargo install --git` / `cargo install --path .` from `david-engelmann/muda`.

### Option A: Prebuilt Binary via `install.sh` (Recommended)

`install.sh` downloads the matching release archive from `https://github.com/david-engelmann/muda/releases`, verifies its SHA-256 digest against `SHA256SUMS`, and installs `muda` into `$HOME/.local/bin` (override with `MUDA_INSTALL_DIR` and `MUDA_VERSION`):

```console
$ ./install.sh
```

### Option B: Build from Source (`git` or local checkout)

`muda` takes zero external crates; only a Rust toolchain and `git` on `PATH` are required:

```console
$ cargo install --git https://github.com/david-engelmann/muda
```

Or from a local clone:

```console
$ git clone https://github.com/david-engelmann/muda
$ cd muda
$ cargo install --path .
```

---

## 2. Running `muda` in GitHub Actions (`fetch-depth: 0` Required)

`muda` inspects `git rev-parse --is-shallow-repository` and **refuses shallow clones** (`the repository at <path> is shallow; a statement needs the whole history`), because a truncated commit graph produces false `unjoined` rework causes whenever a revert or fix-forward references an earlier commit outside the shallow horizon.

Because `actions/checkout@v4` defaults to `fetch-depth: 1` (shallow clone), always pass `fetch-depth: 0` in CI workflows:

```yaml
name: monthly-rework-audit

permissions:
  contents: read

on:
  schedule:
    - cron: "0 6 1 * *"
  workflow_dispatch:

jobs:
  rework-statement:
    permissions:
      contents: read
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0
          persist-credentials: false
      - name: Install muda
        run: |
          curl -fsSL https://raw.githubusercontent.com/david-engelmann/muda/main/install.sh | sh
          echo "$HOME/.local/bin" >> "$GITHUB_PATH"
      - name: Generate monthly rework statement
        run: |
          MONTH="$(date -u -d 'last month' +%Y-%m)"
          muda statement . --month "$MONTH" --rates examples/rates.json -o /tmp/rework-statement.json
          cat /tmp/rework-statement.json
```

Note that `-o` writes to `/tmp/rework-statement.json` outside the checked-out worktree because `muda` refuses any `-o` path inside the repository it reads (`error, muda writes nothing inside the repository it reads`).

---

## 3. History Read and UTC Month Window Rules

1. **Reachability from `HEAD`**: `muda` reads every commit reachable from the checked-out `HEAD` (`git log --reverse`), with `GIT_DIR`, `GIT_WORK_TREE`, `GIT_INDEX_FILE`, `GIT_OBJECT_DIRECTORY`, `GIT_ALTERNATE_OBJECT_DIRECTORIES`, `GIT_NAMESPACE`, `GIT_PREFIX`, ceiling variables, grafts, and replace-objects scrubbed so ambient environment variables cannot redirect the audit. Unmerged side branches are not read unless checked out.
2. **Author-time ordering**: Commits are sorted stably by UTC author timestamp (`%aI` parsed to UTC epoch microseconds; ties keep `git log --reverse` topological read order).
3. **UTC calendar month window (`--month YYYY-MM`)**:
   - `--month` must be seven ASCII characters `YYYY-MM` with `MM` in `01..=12` (boundary months `0000-01..0000-12` and `9999-12` are outside the supported window range).
   - The window is half-open in UTC: `[YYYY-MM-01T00:00:00Z, next-month-01T00:00:00Z)`, reported in the output under `"window": {"from": "YYYY-MM-01", "to": "YYYY-MM-01"}`.
   - Because timestamps are evaluated in UTC, a commit authored at `2026-01-31T20:00:00-05:00` has UTC instant `2026-02-01T01:00:00Z` and belongs to `2026-02`, not `2026-01`.
4. **Merge commits**: Commits with two or more parents (`parents.len() > 1`) are counted in `commits_read` and `merges_read`, and can still be named as the target of a trailer if ever referenced, but a merge commit itself is **never** classified as a rework item even if its message carries a `Fixes:` or revert trailer.

---

## 4. Rework Classification Grammars and Precedence

For each non-merge commit inside the requested UTC month window, `muda` tests four join rules in strict priority order. The first rule that matches classifies the commit:

### Priority 1: `revert` via Trailer (`"join": "revert trailer"`)

Scans body lines (split on Python `splitlines()` boundaries) in order for the first line matching:

```text
This reverts commit <7..40 lowercase hex>[.]
```

Exact grammar rules:

- Must start at column 0 (any leading space or tab treats the line as quoted prose, not a trailer).
- Trailing whitespace at the end of the line is stripped before matching.
- Prefix must be exact case `"This reverts commit "`.
- The SHA prefix must be `7` to `40` **lowercase** ASCII hex characters (`0-9`, `a-f`), optionally followed by a single `.` and nothing else.
- Resolves against **all** commits in the read (including out-of-month commits), excluding the commit's own SHA.

### Priority 2: `revert` via Quoted Subject (`"join": "revert subject"`)

If no revert trailer matched, tests whether the commit subject starts with `Revert "` and ends with `"` (with at least one character inside the quotes):

```text
Revert "<original commit subject>"
```

- Resolves `<original commit subject>` by exact string equality against **earlier** commits (`earlier_all`, across month boundaries).
- Joins when **exactly one** earlier commit carries that exact subject; otherwise lands in `unjoined`.

### Priority 3: `fix-forward` via Trailer (`"join": "fixes trailer"`)

Scans body lines in order for the first line matching:

```text
Fixes: <7..40 lowercase hex>
```

Exact grammar rules:

- Must start at column 0 (unindented), with trailing whitespace stripped.
- Prefix must be exact case `"Fixes: "` (capital `F`, colon, single space).
- The SHA prefix must be `7` to `40` **lowercase** ASCII hex characters (`0-9`, `a-f`) with **zero** trailing characters (no trailing period, no `(#42)` suffix).
- Variant spellings such as `fixes: abc1234`, `Fixes: ABC1234`, or `Fixes #42` are ignored rather than counted.
- Resolves against **all** commits in the read (across month boundaries), excluding the commit's own SHA.

### Priority 4: `redone` via Same-Month Subject and Path Overlap

If none of the explicit revert or fix-forward rules matched, checks earlier non-merge commits **inside the same UTC month window** (`earlier_window`):

- `candidate.subject == commit.subject` (exact string match), **and**
- `candidate.paths` and `commit.paths` share at least one touched file path.
- If one earlier commit in the month matches: `"class": "redone"`, `"join": "redone task"`, billed to that commit.
- If multiple earlier commits in the month match: `"class": "redone"`, `"join": "redone task, earliest of several matches"`, billed to the **earliest** matching commit in the month.
- Tasks whose first attempt occurred in an earlier calendar month are not classified as `redone`.

---

## 5. Complete Reference of `unjoined` Reason Strings

When a commit matches a `revert` or `fix-forward` rule but cannot be attributed to a single cause SHA, `muda` never drops it and never guesses. Instead, it appends an entry to `"unjoined"` (and increments `"totals.items"`, `"totals.per_class[<class>]"`, and `"totals.unjoined"`) with one of five exact `"reason"` strings:

| Class | Join | `reason` String | When It Fires |
| --- | --- | --- | --- |
| `revert` or `fix-forward` | `revert trailer` or `fixes trailer` | `"the trailer names no commit in the read, <prefix>"` | Zero commits in the reachable history start with `<prefix>`. |
| `revert` or `fix-forward` | `revert trailer` or `fixes trailer` | `"the trailer names several commits in the read, <prefix>"` | Two or more distinct commits start with `<prefix>` (ambiguous short SHA). |
| `revert` or `fix-forward` | `revert trailer` or `fixes trailer` | `"the trailer names the item itself, <prefix>"` | `<prefix>` matches only the rework commit's own SHA. |
| `revert` | `revert subject` | `"no earlier commit carries the quoted subject"` | Zero earlier commits have a subject equal to the string inside `Revert "..."`. |
| `revert` | `revert subject` | `"several earlier commits carry the quoted subject"` | Two or more earlier commits share the exact quoted subject (use `git revert` with its `"This reverts commit <sha>."` trailer to disambiguate). |

Note that `redone` items never appear in `unjoined` because multiple matches resolve deterministically to the earliest matching commit in the month.

---

## 6. Rates Card and Exact Minor-Unit Pricing (`--rates`)

Passing `--rates <card.json>` (see `examples/rates.json` and `schema/rates.schema.json`) appends a `"priced"` object to the statement JSON:

```json
{
  "currency": "USD",
  "per_item": {
    "revert": "15.00",
    "fix-forward": "7.50",
    "redone": "4.00"
  }
}
```

Validation and arithmetic rules:

1. **Strict JSON object**: UTF-8 (optional UTF-8 BOM stripped), no duplicate keys anywhere in the document (`the rates file names <key> twice`), and only top-level keys `"currency"` and `"per_item"` allowed.
2. **Currency**: Exact 3-letter ASCII string `[A-Za-z]{3}` (`"USD"`, `"EUR"`, `"GBP"`).
3. **Per-item decimal strings**: `"per_item"` must price all three classes (`"revert"`, `"fix-forward"`, `"redone"`) and no unknown classes. Each rate must be a JSON **string** (not a JSON float) of `1` to `30` ASCII integer digits and an optional `.` followed by `1` or `2` ASCII fractional digits (`"10"`, `"10.5"`, `"10.50"`, `"0.00"`).
4. **Exact integer arithmetic**: Rates are converted to exact integer minor units (cents) as digit strings of arbitrary precision, multiplied and summed without floating-point rounding.
5. **Joined items only**: Only items in `"causes"` are priced (`"items_priced"`). Items in `"unjoined"` remain visible in `"totals"` and `"unjoined"` at zero currency charge until their cause is disambiguated. Both `per_class` amounts and `per_cause` amounts are guaranteed to sum identically to `"priced.total"`.

Machine-readable Draft 2020-12 JSON Schemas for both documents live at `schema/rates.schema.json` and `schema/statement.schema.json`.

---

## 7. Rust Library Overview and Exit Codes

`muda` exposes its history reader, statement builder, rate-card parser, and JSON emitter as a zero-dependency Rust library:

- `muda::history::{read_history, Commit, Fail, worktree_root, git_dir}`
- `muda::statement::{build_statement, load_rates_bytes, price_statement, Rates}`
- `muda::emit::{dumps_document, Mode}`
- `muda::value::{Text, Value}`

```rust,ignore
use muda::emit::{dumps_document, Mode};
use muda::history::read_history;
use muda::statement::{build_statement, load_rates_bytes, price_statement};

fn run_audit(repo_path: &str, month: &str, card_bytes: &[u8]) -> Result<Vec<u8>, muda::history::Fail> {
    let repo_units: Vec<u16> = repo_path.encode_utf16().collect();
    let month_units: Vec<u16> = month.encode_utf16().collect();
    let commits = read_history(&repo_units)?;
    let statement = build_statement(&commits, &month_units)?;
    let rates = load_rates_bytes(card_bytes)?;
    let priced = price_statement(statement, &rates);
    Ok(dumps_document(&priced, Mode::Strict).expect("valid UTF-8 statement"))
}
```

### CLI Exit Codes

| Exit Code | Meaning |
| --- | --- |
| `0` | Statement emitted cleanly to stdout or `-o <path>`, or `-h` / `--help` printed help. |
| `1` | Refusal (`error, <reason>` on stderr, such as shallow repo, invalid `--month`, invalid `--rates`, or `-o` inside the read repo) or unrecoverable runtime failure. |
| `2` | CLI argument parse error (`argparse`-compatible usage and error diagnostic on stderr). |

---

## Shell Completions

Static shell completions ship under `completions/` (and inside GitHub Release archives):

```sh
source completions/muda.bash                       # Bash
fpath=(./completions $fpath) && compinit           # Zsh
cp completions/muda.fish ~/.config/fish/completions/  # Fish
. ./completions/muda.ps1                           # PowerShell
```

