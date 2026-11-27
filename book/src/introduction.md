<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="media/lockup-dark.svg">
    <img src="media/lockup-light.svg" alt="muda" width="780">
  </picture>
</p>

<p align="center">
  <a href="https://github.com/david-engelmann/muda/actions/workflows/ci.yml"><img src="https://github.com/david-engelmann/muda/actions/workflows/ci.yml/badge.svg" alt="ci"></a>
  <a href="https://github.com/david-engelmann/muda/actions/workflows/clean-install.yml"><img src="https://github.com/david-engelmann/muda/actions/workflows/clean-install.yml/badge.svg" alt="clean-install"></a>
  <a href="https://github.com/david-engelmann/muda/actions/workflows/docs.yml"><img src="https://github.com/david-engelmann/muda/actions/workflows/docs.yml/badge.svg" alt="docs"></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-orange.svg" alt="MIT License"></a>
</p>

# muda

Meters the rework tax across a fleet. Every reverted diff, redone task, and fix-forward is charged to the commit that caused it, and the charges come back as an itemized statement.

Muda is a command line tool that reads one git repository's whole history and writes an itemized statement for one stated calendar month as JSON. The read covers the commits reachable from the checked out HEAD, so work that lives only on unmerged branches is not read. Check out the branch whose history the statement should cover. It finds the month's rework, commits that revert an earlier commit, commits that fix one forward, and tasks done twice, and it bills every item to the commit that caused it, with the join stated on the item itself. An item whose cause cannot be named from the record is listed as unjoined with the reason, never silently billed and never dropped. Catchledger prices checks, muda prices work.

<p align="center">
  <img src="media/money-shot.gif" alt="muda monthly rework statement CLI showcase" width="880">
</p>

## Install

### Prebuilt Binary (Linux & macOS)

```console
$ curl -fsSL https://raw.githubusercontent.com/david-engelmann/muda/main/install.sh | sh
```

### Build from Source (Zero-Dependency Rust)

```console
$ git clone https://github.com/david-engelmann/muda
$ cd muda
$ cargo install --path . --locked
```

> Note: Do not run bare `cargo install muda` from crates.io (which installs an unrelated GUI menu crate); install from this repository or GitHub Releases until `muda-cli` is published.

## Quickstart

From a local checkout of this repository:

Build a deterministic two-commit repository with one January revert and bill `2026-01` with [`examples/rates.json`](https://github.com/david-engelmann/muda/tree/main/examples/rates.json):

```console
$ git init -q demo && cd demo
$ git config user.name "J. Doe" && git config user.email "jdoe@example.com"
$ echo "button" > export.py && git add .
$ GIT_AUTHOR_DATE="2026-01-05T09:00:00+00:00" GIT_COMMITTER_DATE="2026-01-05T09:00:00+00:00" git commit -q -m "Add the export button"
$ echo "no button" > export.py
$ GIT_AUTHOR_DATE="2026-01-12T09:00:00+00:00" GIT_COMMITTER_DATE="2026-01-12T09:00:00+00:00" git commit -q -am 'Revert "Add the export button"'
$ cd ..
$ muda statement --month 2026-01 --rates examples/rates.json demo
```

## Architecture

<p align="center">
  <img src="media/architecture.svg" alt="muda Git DAG rework billing architecture" width="900">
</p>

## Usage

- **Three rework joins**: `revert` joins by quoted subject or Git revert trailer across full history; `fix-forward` joins by `Fixes:` trailer across full history; `redone` joins by normalized subject + shared touched paths within the stated month.
- **Exact minor-unit pricing (`--rates`)**: Prices joined items only in exact integer minor units (`USD`, `EUR`, ...) alongside raw counts; `unjoined` items are reported with reasons and never silently billed.
- **Repository write guard**: `-o` writes atomically through a sibling tempfile and refuses any path inside the repository being read (`error, muda writes nothing inside the repository it reads`).

## Performance

<p align="center">
  <img src="media/benchmark-chart.svg" alt="benchmark profile" width="900">
</p>

Measured on Apple M3 Max (`--release`, `791.4 KiB` zero-dep binary; details in [docs/PERFORMANCE.md](performance.md)):

| Commits Reachable from `HEAD` | Tracked Modules | `muda statement` Latency | Effective Commit Scan Rate |
| ---: | ---: | ---: | ---: |
| `50` | `10` | `1631.77 ms` | `31 commits/s` |
| `200` | `10` | `2180.49 ms` | `92 commits/s` |
| `500` | `10` | `1101.31 ms` | `454 commits/s` |

## Where `muda` Fits in the Retrospective Line

<p align="center">
  <img src="media/retrospective-arc.svg" alt="Retrospective Line ten-repository architecture" width="900">
</p>

## Documentation

- [Documentation Index](introduction.md) & [Searchable Handbook](https://david-engelmann.github.io/muda/)
- [Operator & Integrator Guide](guide.md)
- [Technical Architecture](architecture.md)
- [Wire Formats & JSON Schemas](formats.md)
- [Ecosystem Comparison](comparison.md)
- [Performance Benchmarks](performance.md)
- [FAQ](faq.md)

## Status

The statement command is complete at its contracted scope and runs as a Rust binary, with the Python of the first two rounds standing as its behavioral spec. The statement bills each item to the commit that caused it. Joining causes onward to the instruction, prompt, or agent behind the commit is not built yet. Closed perfection rounds, 32 of 100. A round counts only when a full cycle of work, audit, review, red team, and approval has closed.

## License

MIT. See the [LICENSE](https://github.com/david-engelmann/muda/blob/main/LICENSE) file.
