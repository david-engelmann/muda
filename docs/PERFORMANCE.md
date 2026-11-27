# Muda Performance & Scale Benchmarks

<p align="center">
  <img src="media/benchmark-chart.svg" alt="muda benchmark profile" width="920">
</p>

Measured from a zero-dependency thin-LTO symbol-stripped `--release` build (`599.6 KiB` binary) on **Apple M3 Max** (macOS `14.6`, `rustc 1.99.1 (58cf8f9c3 2026-10-13) (Homebrew)`), median of 5 runs over synthetic Git histories with `--rates examples/rates.json`.

## Monthly Statement Latency Across Git History Sizes

| Commits Reachable from `HEAD` | Tracked Modules | `muda statement` Latency | Effective Commit Scan Rate |
| ---: | ---: | ---: | ---: |
| `50` | `10` | `1631.77 ms` | `31 commits/s` |
| `200` | `10` | `2180.49 ms` | `92 commits/s` |
| `500` | `10` | `1101.31 ms` | `454 commits/s` |
