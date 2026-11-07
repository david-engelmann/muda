# muda

It meters the tax a fleet pays in rework, reverted diffs, redone tasks, and fix-forward commits, and bills it to the cause, as an itemized monthly statement. Catchledger prices checks, muda prices work.

## What this is

Traces end at the session while outcomes live in git, so no run can be priced by whether its work survived, and a fast run that generates tomorrow's revert looks identical to a fast run that shipped. Muda makes that join on the git side. It reads one repository's whole history and, for one stated calendar month, finds the rework: commits that revert an earlier commit, commits that fix one forward, and tasks done twice. Every item is billed to the commit that caused it, with the join stated on the item itself. An item whose cause cannot be named from the record is listed as unjoined with the reason, never silently billed and never dropped.

The statement itself prices in counts only, items per class and per cause. Round 2 adds pricing on top: given a rates card naming a currency and a rate for each class, the statement gains a priced section, exact in minor units, counting only the joined items, with the unpriced remainder visible beside the totals. The causes here are commits; joining causes onward to the instruction, prompt, or agent behind the commit is later work.

## Status

Round 1 is closed. The statement command reads one
repository's history and bills a month of rework to its
causes in counts.

Round 2 is closed. A rates card prices the statement:
one added section, exact integer amounts, joined items
only. Setup landed earlier in October 2026.

Round 3 is closed. The tool is a Rust library crate
with a thin binary and no dependencies, reproducing
the Python of Rounds 1 and 2 byte for byte across the
whole command surface.

## Programme status

- Closed perfection rounds, 6 of 100. A round counts only when a
  full cycle of work, audit, review, red team, and approval has
  closed.
- Contract, SB-1 to SB-12 as amended, met for the Round 1 scope.
  SB-13 to SB-16 with amendment SB-13a, written ahead of
  the code and met in Round 2. SB-17 to SB-24, the port
  obligations, written ahead of the code and met in
  Round 3.
- Local gate, python3 scripts/codepoint_scan.py,
  cargo fmt --check, cargo clippy --all-targets with
  warnings denied, and cargo test. CI runs the same
  commands.

## Licence

MIT. See LICENSE.
