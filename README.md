# secondbill

It meters the tax a fleet pays in rework, reverted diffs, redone tasks, and fix-forward commits, and bills it to the cause, as an itemized monthly statement. Catchledger prices checks, secondbill prices work.

## What this is

Traces end at the session while outcomes live in git, so no run can be priced by whether its work survived, and a fast run that generates tomorrow's revert looks identical to a fast run that shipped. Secondbill makes that join on the git side. It reads one repository's whole history and, for one stated calendar month, finds the rework: commits that revert an earlier commit, commits that fix one forward, and tasks done twice. Every item is billed to the commit that caused it, with the join stated on the item itself. An item whose cause cannot be named from the record is listed as unjoined with the reason, never silently billed and never dropped.

Round 1 prices in counts only, items per class and per cause, because no rate has been stated. The causes here are commits; joining causes onward to the instruction, prompt, or agent behind the commit, and pricing the counts, is later work.

## Status

Round 1 is built and in review on its branch. Setup landed earlier in October 2026.

## Programme status

- Closed perfection rounds, 1 of 100. A round counts only when a
  full cycle of work, audit, review, red team, and approval has
  closed.
- Contract, SB-1 to SB-12 as amended, met for the Round 1 scope.
- Local gate, python3 scripts/codepoint_scan.py and
  python3 -m pytest tests -q. CI runs the same commands.

## Licence

MIT. See LICENSE.
