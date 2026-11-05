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

Built in Round 1. The stub's SB-1 fixes this round's
scope: the git side only, one repo, one itemized
statement. The round also added the amendments below
(SB-5a to SB-11a before code, SB-5b, SB-6b, and SB-7b
after the reviews, before the repair code).

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

## Round 2. Priced statements (SB-13 to SB-16)

- SB-13. A statement may be priced. The caller
  supplies a rates file: a JSON object naming a
  currency, a string of three letters, and a rate
  for each of the three classes under per_item, each
  rate a string holding a non-negative decimal with
  at most two fraction digits. A card missing a
  class, naming an unknown class, or carrying a
  malformed currency or rate is refused by name.
  With no rates file, the statement is Round 1's,
  unchanged. Test, tests/test_statement.py.
- SB-14. Pricing is exact. Each amount is its count
  times its rate, computed in minor units by integer
  arithmetic, and rendered as a decimal string with
  exactly two fraction digits. The priced section's
  arithmetic closes: the class amounts sum to the
  total amount, and the cause amounts sum to the
  total amount. Test, tests/test_statement.py.
- SB-15. Only joined items are priced. Unjoined
  items appear exactly as in the counts statement,
  counts only, and the priced section states how
  many items were priced beside the statement's
  item total, so the difference is visible, never
  silent. Test, tests/test_statement.py.
- SB-16. The priced statement is the counts
  statement plus one section, priced, carrying the
  currency, the priced item count, per class the
  item count, the rate, and the amount, per cause
  the amount, and the total amount. Nothing else in
  the document changes, and determinism stands
  (SB-10). Test, tests/test_statement.py.

## Round 2 post-review amendments, recorded before the repairs

- SB-13a. A card is refused by name when any object
  in it repeats a key, a repetition after JSON string
  decoding included, and when its top level carries a
  key besides currency and per_item. A rate's integer
  part runs to at most thirty digits as written,
  leading zeros counted: past that the card is refused
  by name rather than priced, so no conversion or
  rendering inside the run can meet a numeral the
  interpreter refuses. The currency, rate, and month
  forms are ASCII and match to the very end of the
  string; a trailing newline is part of no form. The
  first implementation priced a duplicated key at its
  last value in silence, and an unbounded numeral met
  the interpreter's conversion limit as a traceback.
  Test, tests/test_statement.py.

## Round 3. The Rust port (SB-17 to SB-24)

- SB-17. The port reproduces the behavior of
  Rounds 1 and 2 exactly, the Python on main at
  918dc7e standing as the behavioral spec, its
  tests included. Every divergence found is
  repaired or named as a deviation in the round
  dossier; an unnamed divergence is a defect.
  Test, the ported suite and the differential
  harness.
- SB-18. The port is a library crate plus a thin
  binary. It takes no dependency: the git read is
  the standard library's subprocess, the rates
  reader stands in this crate, and no sibling
  crate is consumed, because secondbill reads a
  repository, not tapes. Tests are cargo tests.
  Test, tests/test_statement.rs.
- SB-19. The git read keeps its posture and its
  bytes: the same argv (the two --no- flags, -C,
  the caller's repository), the same environment
  scrubbed of the eight GIT_ variables with
  GIT_GRAFT_FILE at the null device, the same call
  order (the directory probe, the HEAD probe
  unchecked, the shallow probe, the two logs), and
  the same record surgery over the decoded output,
  splits on the unit and record separators
  included. A failure names the repository and
  carries git's own detail or "git failed"; output
  that is not UTF-8 is the SB-6b refusal. Test,
  tests/test_statement.rs.
- SB-20. The rates card is read by a strict
  scanner in this crate that decides in parse
  order: a duplicate key, decoded before it is
  compared, refuses by name at the close of its
  object; nesting past the reader's limit refuses
  as not JSON where the text reaches it; an
  integer literal past the interpreter's digit
  limit refuses as not JSON; NaN and the infinities
  read as floats, as the Python's reader takes
  them. The card's own validation then runs in the
  Python's order (unknown keys in document order,
  the currency rendered by repr in its refusal,
  per_item, the classes in their stated order, the
  rates). Test, tests/test_statement.rs.
- SB-21. Money never touches a float. Rates
  convert to minor units by integer arithmetic
  over the digit string (the thirty-digit bound of
  SB-13a stands as written), amounts multiply and
  add as integers, and rendering is the integer
  part, a point, and two fraction digits. Test,
  tests/test_statement.rs.
- SB-22. Rendering and writing keep their shapes:
  the document renders as json.dumps with an
  indent of two and ensure_ascii off, keys in
  insertion order, one trailing newline; the file
  write is the mkstemp shape (a sibling temp named
  with the file's name and a dot as prefix, mode
  0600, replaced on success, unlinked on failure),
  with no suffix; a failed write refuses by name;
  a failed stdout write prints its refusal and
  exits 1. The output guard stands: nothing is
  written inside the repository being read, the
  target resolved before the comparison. Test,
  tests/test_statement.rs.
- SB-23. The tests port file for file, fixtures
  included: the suite builds its git repositories
  with the same commits, dates, and messages on
  both sides of the port. A differential harness
  drives the whole command surface against the
  Python, comparing exit codes, streams, and files
  after every step. Test, the harness report in
  the round dossier.
- SB-24. The round dossier carries the port
  report: port or rewrite with reasons, the Rust
  and Python line shares measured on the landing
  tree, the bindings determination made from the
  tool's consumers, and every non-Rust remainder
  with its justification.
- SB-25. Round 4 settles the output guard's
  resolver against the oracle on the family the
  line worked through in its Round 4 rotation:
  the cancelled loop, the dangling family
  (single link and chains), the recursion band
  (the interpreter's walk budget, measured in
  unheard's round at 990 nested splices
  resolving and the 991st raising), and
  whether this oracle's guard wraps an exists()
  arm. Each shape is probed at the oracle CLI
  first, and the port takes the oracle's
  outcome, repairing in the settled forms
  (finished-path judgment, iterated splice,
  the walk budget) where it diverges.
- SB-26. The round's verification is the full
  local gate plus the differential harness at
  zero failures, the new cells included.
  CodeRabbit's advisory pass on Round 3's PR 4
  posted no findings (zero entries on the
  review endpoints), so no disposition is owed
  from it.
- SB-25a. Amendment, from the Round 4 red team.
  A relative output spelling is resolved against
  the working directory, and from a deleted
  working directory the interpreter's getcwd
  raises uncaught (the crash class). The port's
  resolver therefore fails the resolution when
  the cwd read fails on a relative spelling;
  an absolute spelling never consults the cwd
  and is unaffected. The amendment also carries
  the reviews' comment corrections: the module
  header counts two normalizers, and the
  path-forms prose states the guard's actual
  asymmetry (the target resolved, the roots
  entering as git's unresolved strings).
- SB-27. Round 5 discharges the write-seam
  probe filed for this crate after retroforge's
  Round 5. The tailed output spelling:
  retroforge's red team proved a pre-existing
  class in that crate (spellings ending in a
  separator or dot component write on the
  oracle and refuse on the port, because the
  port hands the raw spelling to rename while
  pathlib's construction drops the tail), and
  wallofwrong and bluepencil each convicted the
  class in their own shapes. This crate's guard
  and write seam are probed for the class in
  full: tailed spellings onto the protected
  roots and away, on names, links, and dangling
  links, with the guard's own posture (the
  resolve is uncaught and crash-class per SB-25,
  the roots stand as git's unresolved strings)
  deciding what faithful means here. The class
  boundary is measured before any repair is
  written.
- SB-28. Repairs follow the measurements under
  new clauses if the probe set expands; the
  differential harness gains the proven cells
  with control runs against the pre-repair
  binary failing exactly the new cells, and the
  standing harness stays clean.
- SB-29. README records 5 of 100 only in the
  closure commit written after this round's
  three reviews, fixes, and owner verification.
