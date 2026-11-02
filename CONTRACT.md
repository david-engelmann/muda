# secondbill Contract, stub

The full Contract is the first build act of this repo phase. This
stub records the obligations the setup already knows it owes. Each
obligation becomes executable before any feature code that depends
on it lands.

- SB-1. The first slice is the git side only, reverts, redone tasks, and fix forward commits, for one repo, as one itemized statement.
- SB-2. Every item is billed to the instruction, prompt, or agent that caused it, with the join stated and auditable.
- SB-3. Catchledger prices checks, secondbill prices work. The subjects never merge.
- SB-4. Every fixture committed to this repo is synthetic.

## Round 1. The git side statement (SB-5 to SB-12)

Contract preparation only in this pass. The build opens
when this repo's turn in the line's build order arrives.
The stub's SB-1 fixes this round's scope: the git side
only, one repo, one itemized statement.

- SB-5. The input is one git repository, read through its
  history with a stated field set: sha, parents, author
  time, subject, body, and the paths each commit touched.
  A repository that cannot be read is a named refusal.
  Merge commits are never items; the statement counts
  them among the commits read and says so. Test,
  tests/test_statement.py.
- SB-6. Item classes are mechanical, three only. A revert
  carries a this-reverts-commit trailer or a subject that
  quotes the subject it reverts. A fix-forward carries a
  fixes trailer naming the commit it repairs. A redone
  task is a non-merge commit whose subject matches an
  earlier commit's subject in the window and whose paths
  overlap that commit's paths. A commit matching no class
  is not an item. Test, tests/test_statement.py.
- SB-7. Every item is billed to a cause, and the join is
  stated on the item itself: a revert bills to the commit
  it reverts, a fix-forward to the commit its trailer
  names, a redone task to the earlier commit it repeats.
  An item whose cause cannot be named from the record is
  listed as unjoined with the reason, never silently
  billed and never dropped. Test,
  tests/test_statement.py.
- SB-8. The statement covers one stated calendar month.
  Items are grouped by cause commit; each item names its
  class, its own sha, its cause's sha, and the join rule
  that fired. The statement totals items per class and
  per cause, and states the window and the commits read.
  Test, tests/test_statement.py.
- SB-9. Round 1 prices in counts only: items, per class
  and per cause. No currency figure appears, because no
  rate has been stated; priced statements are a later
  round's work. Test, tests/test_statement.py.
- SB-10. The statement is deterministic: the same
  repository state and month produce byte-identical
  output, commits taken in the order the history read
  states. Test, tests/test_statement.py.
- SB-11. Secondbill never writes to the repository it
  reads. Its only output is the statement, to stdout or
  a named path, written atomically, and every failure is
  a named error with exit 1. Test,
  tests/test_statement.py.
- SB-12. Test fixtures are synthetic git histories built
  by script at test time, per SB-4. No real repository's
  history is committed to this repo. Test,
  tests/test_statement.py.
