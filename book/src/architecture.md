# Muda Technical Architecture

<p align="center">
  <img src="media/architecture.svg" alt="muda technical architecture" width="920">
</p>

Session telemetry (`flighttape`) ends the moment an agent exits: a 30-second run whose commit gets reverted tomorrow morning looks identical in session logs to a 30-second run whose commit ships cleanly for a year. `muda` closes that loop on the Git side by reading reachable history from `HEAD`, classifying rework inside one stated UTC calendar month, and charging every item back to the commit that caused it.

## 1. Zero-Dependency Design

`muda` compiles with **zero crate dependencies** (`[dependencies]` is empty) and invokes the system `git` binary with scrubbed redirect environment variables (`GIT_DIR`, `GIT_WORK_TREE`, `GIT_INDEX_FILE`, `GIT_OBJECT_DIRECTORY`, `GIT_ALTERNATE_OBJECT_DIRECTORIES`, `GIT_NAMESPACE`, `GIT_PREFIX`, `GIT_CEILING`), `GIT_GRAFT_FILE=/dev/null`, and `--no-replace-objects --no-optional-locks` to read commit metadata and touched paths deterministically.

## 2. The Three Rework Classes & Cause Joins

| Class | Detection Signal in Window `[from, to)` | Cause Join Scope | When Unresolvable |
| :--- | :--- | :--- | :--- |
| **`revert`** | Standard `Revert "..."` subject or `This reverts commit <sha>` trailer | Whole reachable DAG (across prior months) | Recorded in `unjoined[]` with explicit reason |
| **`fix-forward`** | Commit trailer `Fixes: <sha>` naming an earlier commit | Whole reachable DAG (across prior months) | Recorded in `unjoined[]` with explicit reason |
| **`redone`** | Normalized commit subject repeats an earlier commit with overlapping file paths | Earlier commits **within the same stated month** | Not classified as redone across month boundaries |

## 3. Exact Integer Minor-Unit Pricing (`--rates`)

Floating-point currency math introduces rounding drift (`0.1 + 0.2 != 0.3`). `muda` parses decimal rate strings (`"10.00"`, `"2.50"`) into exact integer minor units, multiplies joined item counts per class and per cause without loss of precision, and formats exact decimal totals alongside raw item counts. Unjoined items are never silently priced and never dropped.

## 4. Repository Write Guard

Pointing `-o` anywhere inside the target Git repository being read (including via symlinks or relative traversal) exits `1` with `error, muda writes nothing inside the repository it reads`. Valid `-o` writes use a CSPRNG-named sibling temporary file and atomic rename.

## 5. Position in the Retrospective Line

<p align="center">
  <img src="media/retrospective-arc.svg" alt="muda in the Retrospective Line" width="920">
</p>
