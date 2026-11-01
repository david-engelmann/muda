# secondbill

It meters the tax a fleet pays in rework, reverted diffs, redone tasks, and fix-forward commits, and bills it to the instruction, prompt, or agent that caused it, as an itemized monthly statement.

## What this is

Traces end at the session while outcomes live in git and the tracker, so no run can be priced by whether its work survived, and a fast run that generates tomorrow's revert looks identical to a fast run that shipped. Secondbill makes that join and issues the statement, line by line, down to causes like vague ticket titles costing more than the model spend. Every fleet gets two bills, the one the provider sends and this one. The boundary with its sibling is one sentence. Catchledger prices checks, secondbill prices work.

## Status

Seeded in October 2026. Local setup is in progress on the setup
branch. The remote seeding pass is waiting on a token access fix, so
work is banked locally until pushes clear. This repo is in phase 3
of the Retrospective Line and its build waits on the phase gate, the
main chat confirmation that the Gate Truth evidence pack schema has
merged and passed acceptance. Design work may proceed, build does
not start before the gate.

## Programme status

- Closed perfection rounds, 0 of 100. A round counts only when a
  full cycle of work, audit, review, red team, and approval has
  closed.
- Contract, stub in place, not yet met.
- Local gate, python3 scripts/codepoint_scan.py and, once tests
  exist, python3 -m pytest tests -q. CI runs the same commands.

## Licence

MIT. See LICENSE.
