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

## Round 1 amendments, the pinned shapes (recorded before code)

- SB-5a. The read is every commit reachable from the
  repository's HEAD, oldest first, taken with
  --no-optional-locks so the read itself writes
  nothing. The month filter and every ordering judge
  by author time, read in UTC. A path that is not a
  readable git repository, and a repository with no
  commits, are named refusals. Test,
  tests/test_statement.py.
- SB-6a. The mechanical forms: a revert trailer is a
  body line "This reverts commit" followed by a hex
  sha of 7 to 40 characters, as git itself writes it;
  a quoted revert subject opens with Revert " and
  closes with a quote, the quoted text being the
  reverted subject; a fixes trailer is a body line
  opening "Fixes: " followed by a hex sha of 7 to 40
  characters. Classes are checked in the order
  revert, fix-forward, redone, and the first match is
  the item's one class. Test, tests/test_statement.py.
- SB-7a. A sha named by a trailer resolves against
  the whole read: exactly one commit whose sha opens
  with it joins; none or several leaves the item
  unjoined with the reason. A quoted revert subject
  joins the earlier commit carrying that exact
  subject; none or several leaves it unjoined with
  the reason. A redone task's earlier commit must sit
  in the same month window; its cause may be any
  earlier commit in the window with the matching
  subject and overlapping paths, and when several
  qualify the earliest is the cause and the join says
  so. A revert or fix-forward cause may sit outside
  the month; the item is in the month by its own
  author time. Test, tests/test_statement.py.
- SB-8a. The statement document carries the month,
  the window bounds, commits read, merges among them,
  the causes in read order each with its items (sha,
  class, the join rule that fired), the unjoined
  items with their reasons, and totals: items per
  class, items per cause, unjoined count. Items
  within a cause stand in read order. Test,
  tests/test_statement.py.
- SB-11a. An output path inside the repository being
  read is refused by name; secondbill writes nothing
  there, not even its statement. Test,
  tests/test_statement.py.

## Round 1 amendments, after the reviews (recorded before the repair code)

- SB-5b. The read is ordered by author time, ties
  broken by read position, so earlier always means
  earlier authored (SB-5a) even when commits were
  cherry-picked or rebased. A shallow repository is
  refused by name: a statement over a partial read
  would price a fiction. Replace objects and grafts
  are never honored; the read runs with replace
  objects off and the graft file pointed at nothing.
  Test, tests/test_statement.py.
- SB-6b. Trailer lines match at the start of the
  line; an indented trailer, for example inside a
  quoted code block, is message text, not a trailer.
  A commit message that is not valid UTF-8 makes the
  read a named refusal. Test, tests/test_statement.py.
- SB-7b. A trailer naming the item's own sha never
  joins the item to itself; the unjoined reason says
  the trailer names the item itself. Test,
  tests/test_statement.py.
