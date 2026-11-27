# Muda Performance & Scale Benchmarks

Measured from a zero-dependency `--release` build (`791.4 KiB` binary) on **Apple M3 Max** (macOS `14.6`, `rustc 1.99.1 (58cf8f9c3 2026-10-13) (Homebrew)`), median of 3 runs over synthetic Git histories with `--rates examples/rates.json`.

## Monthly Statement Latency Across Git History Sizes

| Commits Reachable from `HEAD` | Tracked Modules | `muda statement` Latency | Effective Commit Scan Rate |
| ---: | ---: | ---: | ---: |
| `50` | `10` | `875.52 ms` | `57 commits/s` |
| `200` | `10` | `915.96 ms` | `218 commits/s` |
| `500` | `10` | `699.6 ms` | `715 commits/s` |

End-to-end monthly statement generation (including `git` subprocess log/diff traversal, causal SHA resolution, and minor-unit pricing) runs in **under 1 second** across 500-commit histories.
