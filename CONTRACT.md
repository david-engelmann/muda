# muda Contract, stub

The full Contract is the first build act of this repo phase. This
stub records the obligations the setup already knows it owes. Each
obligation becomes executable before any feature code that depends
on it lands.

- SB-1. The first slice is the git side only, reverts, redone tasks, and fix forward commits, for one repo, as one itemized statement.
- SB-2. Every item is billed to the instruction, prompt, or agent that caused it, with the join stated and auditable.
- SB-3. Catchledger prices checks, muda prices work. The subjects never merge.
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
- SB-11. Muda never writes to the repository it
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
  read is refused by name; muda writes nothing
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
  crate is consumed, because muda reads a
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
  tetlock and bluepencil each convicted the
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
- SB-30. Amendment, from the Round 5 probes.
  The tailed-spelling class is absent in this
  crate, measured over 95 twin cells (tailed
  names away and onto both roots, componentless
  and anchor spellings, dotdot tails, live and
  dangling links, links whose targets carry
  tails, a file as a mid component, the escape
  link hunt, worktree roots, deleted working
  directory compositions), with a genuine
  acceptance audit behind the agreements. Two
  structural reasons. The write seam has
  consumed the constructed spelling since the
  port: normalize_pathlib is the construction
  law (empty and dot components drop wherever
  they stand, dotdot and every other component
  stand whole, a leading separator pair is the
  anchor and a longer run collapses to one, a
  componentless spelling stands as dot or its
  anchor). And the guard's walk over the raw
  spelling skips exactly the components the
  construction drops, so its target is the
  constructed spelling's target on every tailed
  shape; the double slash anchor collapses at
  resolve on the oracle too, slipping neither
  guard. write_atomically's decomposition of
  the constructed path differs from pathlib's
  only on spellings whose rename always fails
  (the parent of the root read as the working
  directory, a trailing dotdot's name read as
  empty), moving only the temp's placement and
  prefix during a write that cannot land;
  measured, unobservable, and not repaired,
  there being no behavior to repair. One
  boundary stands outside the family: reading
  a linked worktree, an output spelled into
  the main repository's own .git directory is
  under neither computed root, and both
  implementations write it, tailed or not. That
  is the shipped guard's root set, reproduced
  faithfully, not a tail divergence. The
  worktree family is probe pinned rather than
  harness celled: a worktree gitfile carries
  the side's own absolute path, so a twin
  snapshot cannot compare it byte for byte.
- SB-31. The differential harness gains the
  tailed spelling cells as pins of the measured
  law: the away tails and the sentinel
  overwrite, the componentless and anchor
  spellings, the dotdot tails, the onto cells
  against the fixture repository and its .git,
  the anchor spelled onto and away, the link
  shapes (dangling with tails, targets carrying
  tails, the dir link and the chain as the
  output), the escape link with a tail, the
  loop with a tail and the deleted working
  directory's tailed output as dev steps. The
  control run against the pre-repair binary
  passes the new cells with the standing ones:
  they pin standing behavior, and that pass,
  over cells the audit showed genuinely write
  and genuinely refuse, is the absence proof,
  reported as regression pins and never as
  repair evidence. No source change lands in
  this round.
- SB-32. Amendment, from the Round 5 red team.
  The temp's placement is a third law at the
  write seam, and SB-30's third and fourth
  points do not survive it in the link-pop
  family. The temp is created under the
  lexically absolutized parent of the
  constructed path: the parent string is
  joined with the working directory when it
  is relative and then normalized lexically,
  the interpreter's abspath step behind
  mkstemp, with no link resolved at any point
  in that computation. The rename destination
  remains the constructed path itself. Where
  a symlink component in the parent is popped
  by a following dotdot, the lexical parent
  and the parent the kernel resolves through
  the spelled string can differ in existence,
  and the placement then decides whether the
  write lands at all: a lexical parent that
  is missing, or stands as a file, refuses
  with the cannot-write line and nothing is
  written; a lexical parent standing as a
  directory takes the temp invisibly, and
  the rename lands or fails on the physical
  path. Agreement returns at the boundaries:
  the lexical parent exists as a directory
  (both write the physical destination),
  neither parent exists (both refuse), and
  the popped component is a real directory
  in place (the two parents coincide). This
  clause governs the temp's placement
  wherever SB-30's third point called the
  decomposition difference unobservable and
  its fourth denied a disagreement in either
  direction: in this one direction, in this
  family, there was one, in shipped code.
  The pure tailed class stands absent as
  SB-30 measured it, and SB-31's statement
  that no source change lands in this round
  is superseded by the repair this clause
  licenses, confined to the temp's
  placement: no other seam moves, and the
  cannot-write message keeps the raw
  spelling.
- SB-33. The differential harness gains the
  link-pop cells as the round's second pin
  set: the convicted cells (the plain pop
  and its tailed twin, the deep pop, the
  absolute pop, the pop behind the
  repository's own escape link and its
  dot-tail twin, the lexical parent standing
  as a file, the popped link whose target
  carries a dotdot) and the boundary pins
  (the lexical parent as a real directory,
  neither parent existing, the real
  directory popped, the double pop). The
  control run against the pre-repair binary
  fails exactly the convicted cells, and
  the standing harness stays clean.
- SB-34. Round 5's CodeRabbit disposition, carried
  as a contracted absence: PR 6 carried zero inline
  findings when it merged, and the 2026-11-05
  read-back of the PR's review comments confirms
  zero. The absence is this round's evidence; any
  late finding on PR 6 is answered in this round's
  gate report.
- SB-35. The temp suffix at this crate's write is
  measured against mkstemp's own: the observable
  law is the suffix LENGTH, eight characters on
  the oracle's side. The port's generator is read
  from the current source and its width measured;
  the name-length boundary is pinned executably
  in the differential harness at the lengths where
  the temp name crosses the filesystem limit,
  plain and through a pop. A width that differs
  from eight is convicted and repaired to eight;
  the suffix alphabet is not behavior, since the
  characters are random on both sides.
- SB-36. This round's closure: the README records
  6 of 100 only in the closure commit, after the
  round's reviews, fixes, and owner verification.
- SB-37. The temp candidate is built from the
  output name's raw bytes. A name carrying bytes
  that are not valid UTF-8 keeps its byte count
  at the temp: no lossy expansion may lengthen
  the temp name, so the boundary of SB-35 stands
  at the same byte lengths for raw-byte names as
  for plain ones.
- SB-38. The harness boundary section pins the
  raw-byte boundary beside the plain one: names
  carrying one raw byte at the byte lengths
  around the flip, plain and through a pop,
  write and refuse at the same lengths on both
  sides.
- SB-39. Round 6's CodeRabbit disposition,
  discharged on read-back. PR 8 carries zero
  review comments and zero reviews. Its issue
  thread holds the trigger (6680302606), the
  shell (6680302699), an auto-reply (6680304424),
  and the landing comment. The shell's final form
  is the free-review rate-limit notice: it
  finalized at 2026-11-06T08:09:28Z, before the
  merge at 08:16:23Z, stating the review limit
  was reached and the next included review was
  available in 23 seconds; the auto-reply
  records the action not completed, review rate
  limited. No review ever ran on PR 8, so no
  late findings exist or are owed. Round 6's
  record of zero inline findings stands as a
  count and is corrected as a characterization:
  the absence is a review that never ran, not a
  review that found nothing.
- SB-40. The secondbill-to-muda rename records,
  verified on the current tree. PR 9 (e595059a,
  merged 2026-11-07) moved the name in the
  crate, the package, and the prose. This round
  re-greps the whole tree: zero occurrences of
  the old name remain in any case. The census's
  one standing remnant is the generic concept
  phrase in the library docs, "the second bill"
  after the renamed product token in src/lib.rs,
  PR 9's recorded judgment call; it stands. The
  SB identifiers and the dated clauses stand
  verbatim as historical identifiers.
- SB-41. The rename's help bytes, measured
  against a renamed oracle and repaired. The
  Python oracle staged with the package renamed
  and prog set to muda renders the statement
  usage on one line: at 73 characters it fits
  argparse's wrap width where the old name's
  79-character line wrapped. The shipped port
  renders the old two-line wrap with repo on a
  continuation line, a substitution artifact of
  PR 9, invisible to its binary-against-binary
  differential. Three surfaces carry the byte:
  statement --help on stdout, and the statement
  usage printed on stderr when the subcommand
  is given no arguments or its --month is
  missing. The repair re-derives STATEMENT_HELP
  and STATEMENT_USAGE in src/helptext.rs from
  the renamed oracle's actual output; no other
  surface moves. The top-level help, the
  no-argument and bad-subcommand errors, and
  the bad-month error already agree byte for
  byte, measured on the same battery.
- SB-42. The README brought to the fleet
  storefront standard. First screen in order:
  the title, the lede identical to the
  repository description (read back from the
  API; it carries no colon and no semicolon, so
  no copy-bar exception is needed), one short
  paragraph, the one ci badge (verified before
  shipping: the workflow is active, its latest
  run on main at the round's base completed
  success, and the badge image renders
  "ci - passing"), and one block of real
  captured output. Then Install, Quickstart,
  Usage, Status, and License. The Install and
  Quickstart blocks run verbatim from clean
  staging, cargo install into a scratch
  CARGO_HOME included, and the Quickstart's
  output is the captured statement over a small
  synthetic git history, counts only, as the
  tool's base form produces it. The Status
  section tracks reality, including the rounds
  the old README's status never reached. The
  rounds line stays at 6 of 100 in this round;
  SB-44 reserves 7 to closure.
- SB-43. Checklist item 3, recorded open:
  Cargo.toml carries no description field at
  all, so the package-metadata side of the one
  description is absent, not a third variant.
  It is named here and in the round's readiness
  block, and not repaired inside this round's
  bound.
- SB-44. This round's closure: the differential
  harness runs byte-clean against the renamed
  pairing (the renamed oracle of SB-41 against
  the tip binary) on the final tree, the
  four-check gate passes on the final tree, and
  only then does the README record 7 of 100 in
  the closure commit.
- SB-45. SB-40's reach, restated by closure
  amendment. The zero the clause reports is
  measured on the product tree: the source,
  the README, the help text constants, and
  the fixtures hold no occurrence of the old
  name in any case. The clause's own heading
  names the old name as the rename's subject,
  and occurrences standing in this contract's
  dated clauses and headings, this amendment
  included, are record references; they stand.
- SB-46. Standing deviation, filed at closure:
  the port pins its generated help text at
  the COLUMNS=80 rendering, and src/helptext.rs
  says so. The Python oracle wraps to the
  terminal width, so on a terminal narrower
  than 80 the statement parser's help and
  usage-error surfaces differ from the oracle
  in line breaking only. Measured at
  COLUMNS=60, 12 of 25 surfaces diverge,
  whitespace only, exit codes and stream
  assignment identical; at COLUMNS=80 and
  COLUMNS=200 all 25 agree, and no surface
  at any width renders the old name's
  two-line wrap. The repair, a width-aware
  help render, is owed to a later
  source-touching round, not this one.
- SB-47. PR 11's CodeRabbit step 0,
  discharged. Six findings came back on the
  pull request, five repaired here and one
  dismissed with its reason recorded.
  Comment 4502026331 amends SB-41 by
  reference. The three surfaces SB-41 names
  are the measured examples of its battery,
  not the whole reach of the changed usage
  line. That line stands as the first line
  of STATEMENT_HELP and as STATEMENT_USAGE,
  and STATEMENT_USAGE is the usage printed
  with every error the statement parser
  itself reports, because sub_error in
  src/cli.rs is the one constructor of the
  parser's parse failure. Its callers are
  the required-arguments error at the end
  of the parse, the missing-value error
  for --month, --rates, and -o/--output,
  the last of those in both its long and
  short forms, and the ambiguous-option
  error for a long option matching several
  names. The usage also prints on the two
  ignored-explicit-argument help errors,
  the long --help carrying a glued value
  through help_token_outcome and the short
  -h carrying one in the cluster walk. The
  behavior across all of them is the red
  team's 25-surface battery, already run,
  so this amendment moves wording scope
  only. Comments 4502026341, 4502026344,
  4502026347, and 4502026352 repair the
  README against the source, each claim
  verified in the source before its
  sentence was written. The opening
  paragraph now states that the read
  covers the commits reachable from the
  checked out HEAD and that work on
  unmerged branches is not read, verified
  in read_history in src/history.rs, which
  runs git log with no ref over the
  verified HEAD. Install now states the
  Unix environment the build needs,
  verified in the unconditional Unix
  import inside write_atomically in
  src/cli.rs. Usage now states that a
  commit is counted in the month its
  author timestamp falls in read in UTC,
  verified in the window construction in
  src/statement.rs, and that the redone
  join looks only at earlier commits
  inside the stated month while the
  revert and fix-forward joins resolve
  against the whole read, verified in
  classify in the same file. Comment
  4502026334 is dismissed. It asks the
  lede to drop the instruction-level
  attribution the description promises.
  The lede is the repository description
  verbatim, and the first screen read
  whole does not hide the gap, because the
  paragraph states the commit reach
  exactly and Status names the unbuilt
  onward join to the instruction behind
  the commit. The repair belongs to the
  description line itself, which is the
  repository owner's call and outside this
  round's bound.
- SB-48. Round 7's CodeRabbit disposition,
  discharged on read-back. PR 11 carries 13
  reviews and 18 review comments and 4 issue
  comments. The one substantive review is
  CodeRabbit's 5812634605, read to its end:
  exactly the six inline findings, no
  body-only items, and no Docstring Coverage
  failure on this PR. Six reviews carry the
  owner's replies and six carry the bot's
  acknowledgments, all with empty bodies.
  The 18 comments are the six findings, six
  owner replies, and six bot acknowledgments
  on the threads rooted at 4502026331,
  4502026334, 4502026341, 4502026344,
  4502026347, and 4502026352. Every
  acknowledgment was read in full: five are
  plain acceptances; 4502079054 confirms the
  SB-47 amendment addresses its finding and
  its thread stands resolved; 4502081083, on
  the dismissed thread, accepts the
  dismissal, in its words "I accept leaving
  the lede unchanged this round", and offers
  to open a follow-up issue, an unaccepted
  offer and not a finding. The issue thread
  holds the trigger (6707845066), the shell
  (6707847151), its reply (6707929958), and
  the landing comment (6708133706). The
  shell's final form is the rate-limit
  banner over a complete walkthrough of the
  final head d8ea2863: pre-merge checks 5
  passed, and Docstring Coverage passed with
  "No eligible functions found" over 0
  analyzed functions. The reply records the
  action not completed, already reviewed the
  last commit, and carries no findings.
  SB-47 disposed all six findings in Round
  7, five repaired and one dismissed with
  its reason. Nothing undisposed stands,
  and this round owes no disposition.
- SB-49. The counts and pricing separation,
  measured at the shipped binary over three
  synthetic fixtures, a 64-cell battery. An
  unpriced statement carries no priced
  section and names no currency (SB-9). A
  priced statement is the counts document
  with the priced section added and nothing
  else moved (SB-16). A statement document
  fed back as a rates card refuses on its
  first key in document order, the unknown
  key month. Over the full fixture, three
  joined items (a revert, a fix-forward, a
  redone) and one unjoined, the priced
  arithmetic closes both ways at the README
  rates: the per-class amounts and the
  per-cause amounts each sum to the total
  of 17.50 (SB-14). Only the joined items
  are priced: items_priced stands at 3
  against totals.items at 4, and the
  unjoined item stands counts-only (SB-15).
- SB-50. The currency form, measured. The
  card's currency prices when it is three
  ASCII letters in any case: USD, usd, and
  UsD all price, and the priced section
  carries the currency verbatim as supplied.
  Two letters, four letters, a digit among
  the letters, the empty string, a trailing
  newline inside the string, and a non-ASCII
  letter each refuse on the one line, "the
  rates file names no currency of three
  letters", with the value rendered by repr.
  An absent currency renders the same
  refusal with None as the named value, and
  the number 123 renders unquoted. NaN
  reads as a float under SB-20 and the
  refusal names it nan.
- SB-51. The rate form and the rendering,
  measured. A rate prices as digits with an
  optional point and one or two fraction
  digits, digits on both sides of any point:
  "5." and ".50" refuse, as do a leading
  plus, a minus at any value including
  "-0.00", a leading space, an exponent
  form, the empty string, and a trailing
  newline inside the string, each refused
  on the one form line naming the class and
  the rate as written. A rate given as a
  JSON number refuses as not a decimal
  string, with the value rendered by repr.
  The priced section renders each rate
  normalized to exactly two fraction
  digits, so "5.5" stands as 5.50 and
  "007.50" as 7.50, and amounts render the
  same way (SB-14, SB-21). The integer
  part's bound stands at 30 digits as
  written, leading zeros counted (SB-13a):
  29 and 30 digits price, 31 refuses under
  the same form line naming the rate, one
  leading zero making 31 as written
  refuses, and five leading zeros over 25
  nines, 30 as written, prices at the
  25-nine value. The minor-unit arithmetic
  is exact at magnitude: three items at a
  rate of 30 nines and .99 total
  2999999999999999999999999999999.97,
  checked in integer minor units. A zero
  rate prices at 0.00.
- SB-52. Duplicate keys, measured at every
  level (SB-13a, SB-20). A repeated key
  refuses by name at the close of its
  object: currency and per_item at the top
  level, revert inside per_item, a key
  inside an unknown key's object, and a key
  inside a class's object value. The
  comparison is on the decoded key: a card
  naming "curr\u0065ncy" beside currency
  refuses as currency twice. The scanner
  decides in parse order, before any
  validation: a duplicate inside an unknown
  key's object refuses as the duplicate,
  not as the unknown key, and a duplicate
  whose first value is a badly typed rate
  refuses as the duplicate, not as the
  rate. Validation otherwise names, in its
  order: an unknown top-level key and an
  unknown class, each rendered unquoted; a
  missing class as "prices no <class>"; a
  missing per_item as carrying no per_item
  rates; a card that is not an object as
  not an object.
- SB-53. The strict scanner's limits,
  re-measured at this binary (SB-20). An
  integer literal of 4301 digits refuses as
  not JSON, at the interpreter's digit
  limit; 4300 digits parse and the value
  reaches validation, named by its digits
  under the currency refusal. Arrays nested
  9993 deep under a key reach validation,
  where the unknown key is named; 9994
  refuse as not JSON, the reader's limit of
  9994 containers from the root standing as
  calibrated in src/card.rs. A card that is
  not UTF-8 refuses as not utf-8. A card
  file that does not exist refuses on the
  line "cannot read the rates file",
  naming the path as given.
- SB-54. The output guard's walk constant,
  measured fresh at this binary in the
  harness's construction, the output spelled
  as the head of a symlink chain standing in
  one real directory, so the port's own
  resolver is the only traversal. Chains of
  990 and 991 splices resolve: the write
  lands and replaces the head link, and a
  chain of 991 onto the repository draws
  the guard's refusal line. A chain of 992
  splices aborts the walk into the silent
  crash class, head link untouched, on both
  targets. One extra head link agrees: over
  a chain of 990 the write lands at 991
  splices, and over a chain of 991 it aborts
  at 992. The constant stands at 991
  splices, as SB-25 calibrated it against
  the interpreter; it is stated here from
  this binary's own cells, not inherited.
- SB-55. The temp draw, convicted. As
  shipped through Round 7 the candidate
  sequence is a public function of two
  inputs: the seed is the clock's
  sub-second nanos folded with the pid
  shifted by 16, with a constant standing
  in when the clock read fails, and each of
  the 64 attempts steps the MMIX LCG once
  and draws eight characters over the
  38-symbol alphabet from the state's high
  bits. Under the fleet's frozen-clock
  plant method, the clock frozen by an
  LD_PRELOAD shim and the pid fixed in a
  pid namespace, the whole sequence is
  computable in advance. At the reading
  1700000000.123456789 with pid 1 and at
  1800000000.987654321 with pid 250,
  planting all 64 computed candidates
  refuses the write on the standing
  cannot-write line and no statement
  lands, while planting 63 lands the write
  on the 64th candidate with content
  byte-identical to the unplanted
  statement, at both readings. A bystander
  who can set the clock and predict the pid
  can deny the write outright.
- SB-56. The draw repaired in the fleet
  shape. Every candidate now draws 16 bytes
  of operating system entropy from
  /dev/urandom, read exactly, and
  temp_suffix folds the sixteen bytes' two
  u64 lanes into the draw value before the
  first pick; the shipped clock and pid
  terms remain as mixer inputs, but no
  candidate is a public function of them.
  An entropy failure returns the attempt
  loop's own exhaustion error before any
  create, so the write refuses on the
  standing line and never falls back to the
  computable draw. The name's shape is
  unchanged: the file's name, a dot, eight
  characters over the shipped alphabet, so
  the boundaries of SB-35 and SB-37 stand
  where they stood. The pins: two cargo
  unit tests, the fold's vectors computed
  independently of the implementation and
  the suffix's shape at any state; the
  plant battery re-run against the repaired
  binary, where all 64 computed candidates
  planted at both readings and both pids no
  longer refuse anything, the write landing
  byte-identical with all 64 plants left
  standing; and the entropy-refusal cells,
  where a shim refusing the urandom opens
  makes the repaired binary refuse on the
  standing line with the directory left
  empty, the pre-repair binary writes
  through the same shim, and the repaired
  binary without the shim writes. The
  differential harness runs byte-clean
  against the Python oracle on the final
  tree, and the pricing battery of SB-49 to
  SB-53 re-runs against the repaired binary
  with no verdict moved.
- SB-57. This round's closure: the gate
  passes on the final tree (cargo test with
  the two new unit tests standing at 25,
  clippy over all targets, the format
  check), the harness stands byte-clean,
  and only then does the README record 8
  of 100 in the closure commit. Checklist
  item 3 stands as SB-43 recorded it, open,
  and the description's attribution clause
  stands as SB-47 recorded it, the
  repository owner's call; both are
  restated in the round's readiness block
  exactly as they stand.
- SB-58. Round 8's CodeRabbit
  disposition, discharged on read-back.
  PR 12 carries no reviews and no review
  comments and 5 issue comments, all read
  to their ends, and the PR body in full.
  The shell is 6735669139: no actionable
  comments were generated in the recent
  review; its walkthrough covers the range
  93e22396..99ddc2f5 over 3 files and
  grades the merge risk Minimal; pre-merge
  checks 5 passed, and Docstring Coverage
  passed at 100.00% against the 80.00%
  threshold, 4 functions analyzed across 1
  file with 2 skipped as unsupported, so
  no dismissal of a coverage failure is
  owed on this PR. The trigger is
  6735670496. The bot's auto-reply
  6735728290 records the action not
  completed, already reviewed the last
  commit, and invites a full review to
  rerun; it carries no findings and no
  correction, and the Round 8 build
  report's census did not enumerate it, so
  it is recorded here in its exact form.
  The owner's disposition 6735734971
  records step 0 complete on head 99ddc2f5
  with zero findings. The landing comment
  6739797403 records the merge, 2703c0cc,
  and the red team discharge's final form:
  the red team had held its verdict on
  evidence alone, the write fault refusing
  every fixture write, with zero defect
  findings, and an owner-armed cron run
  after the merge rebuilt both trees from
  the API, base tree 58f11d8f and head tree
  2102aaa8, found its builds md5-identical
  to the red team's report, and ran the
  banked battery at 42 cells all PASS and
  the differential harness at 14 cases,
  240 steps, 0 failures, the full outputs
  banked with Round 8's records. Nothing
  undisposed stands on PR 12, and this
  round owes no disposition.
- SB-59. The pricing surfaces, measured
  fresh at this binary against the shipped
  oracle over one realistic synthetic
  corpus: 46 commits from October 2025 to
  March 2026 with 4 merges, features,
  subject and trailer reverts, Fixes
  trailers, redone tasks with an
  earliest-of-several case, unjoined
  items, and causes standing in earlier
  months than their reverts. Statements
  for the four populated months, November
  and January carrying 5 joined items over
  3 causes with 1 unjoined, December and
  February carrying 4 joined items over 3
  causes with 1 unjoined, priced under six
  card shapes, 24 cells compared byte for
  byte on exit, stdout, and stderr: no
  card; USD at 10.00, 5.00, and 2.50; EUR
  with a zero revert rate at 0.00, 12.34,
  and 0.01; lowercase jpy at 150.00, 80.5,
  and 0.07; a bound-edge card at the
  30-digit edge as written, a revert rate
  of 30 nines with .99, a fix-forward rate
  of 29 nines with .09, and a redone rate
  of 30 digits as written with five
  leading zeros; and an all-zero card at
  0, 0.00, and 0.0. Every priced document
  also closed independently: per-class
  amounts and per-cause amounts each sum
  to the total in integer minor units,
  every per-class amount closes on its
  rate times its item count, items_priced
  equals the joined item count of the
  counts document, and the priced document
  minus its priced section is the counts
  document. Three -o cells landed bytes
  equal to the stdout bytes on both
  implementations. The unpriced documents
  carry no priced section and name no
  currency (SB-9); the join onward is not
  built yet, as the README states, and
  nothing beyond the shipped surfaces is
  claimed.
- SB-60. Duplicate keys, measured fresh
  at this binary against the shipped
  oracle (SB-13a, SB-20, SB-52) on every
  surface that exists. The shipped CLI has
  no stdin mode and no "-" convention; the
  rates path is read as a plain file on
  both implementations, so the stdin
  surface is measured by naming /dev/stdin
  as the path on both. On the file
  surface, eleven cards refuse byte-
  identically: currency and per_item
  repeated at the top level, revert
  repeated inside per_item, a key repeated
  inside a class's object value (x inside
  revert's object, y inside fix-forward's
  object), a repeated inside an unknown
  key's object, a repeated inside an
  object nested in an array value, the
  decoded-escape duplicate naming currency
  twice, the two parse-order cells, a
  duplicate standing before an unknown key
  and a duplicate standing over a badly
  typed rate, each refusing as the
  duplicate, and empty input refusing as
  not JSON. A valid card prices clean. On
  the /dev/stdin surface, the top-level
  currency duplicate, the per_item revert
  duplicate, the decoded-escape duplicate,
  empty input, and the valid card each
  answer byte-identically to the file
  surface on both implementations.
- SB-61. The draw, measured fresh at this
  binary after the SB-56 repair: ten cells,
  no failures. A model of the draw written
  from the contract text is pinned against
  the four unit vectors in src/cli.rs
  before any cell runs. Under entropy
  refusal in both arms, the device open
  refused and a short read of five bytes,
  the port refuses on the standing
  cannot-write line before any create and
  the directory is left as seeded; the
  oracle under the refusal arm lands its
  statement, because its draw never opens
  the device at statement time, and that
  split is contracted, not smoothed over.
  The clock-only plants stand inert at
  both of Round 8's readings, the clock
  frozen at 1700000000.123456789 with the
  process as namespace init at pid 1 and
  at 1800000000.987654321 at pid 250: all
  64 candidates of the pre-repair
  clock-only sequence planted, the write
  lands byte-identical and every plant
  stands, and the oracle under the frozen
  clock at pid 1 with the same 64 planted
  also lands. The positive control pins
  entropy's causal role: with the entropy
  block fixed to a known nonzero value
  the 64 candidates are computable,
  planting all 64 refuses the write, and
  removing the 64th lands it; with the
  block zeroed the fold is zero, the
  candidate sequence is the clock-only
  sequence again, and its plants refuse.
- SB-62. The resolver walk, measured
  fresh at this binary (SB-54): fourteen
  cells, no failures. The hand walk's
  constant, 991 splices, governs the
  guard's decision: onto-repo chains at
  990 and 991 splices refuse with the
  guard's line, so the refusal still
  speaks at the constant, and at 992 the
  walk aborts into the crash class, the
  port silent at exit 1 with both streams
  empty and the oracle raising
  RecursionError, the same split standing
  at the away chain of 992 and at a head
  link over a 991 chain. The write itself
  travels the spelled path through the
  kernel, whose own traversal limit is 40
  links, and that limit governs the
  physical write: away chains at 990 and
  991 splices and a head link over a 990
  chain, 991 splices in all, resolve in
  the hand walk to targets outside the
  repository, yet both implementations
  refuse the write with the standing
  cannot-write line and nothing lands.
  In every one of these cells the chain
  stands in the output's parent path,
  the output spelled through the chain
  (through the chain itself, or through
  a head link into it), so the temp's
  creation must traverse the chain in
  the kernel. That is not SB-54's
  shape, where the output is spelled as
  the head of a chain standing in one
  real directory, the port's resolver
  is the only traversal, and the write
  lands and replaces the head link. The
  boundary is pinned inside the
  battery: an away chain at 40 splices
  lands on both implementations and at 41
  the write is refused on both. The
  cancelled loop's remainder is popped by
  its dotdot in the hand walk, but the
  kernel still traverses the cycle on the
  write path, and both implementations
  refuse with the write line and nothing
  lands. The loop postures stand as
  contracted: a self loop, a two-cycle,
  and a cycle entered after a real splice
  each leave the port silent and the
  oracle answering with a traceback.
  These cells were first drafted
  expecting the long chains to land; both
  implementations falsified that
  expectation together, the temp open
  through the chain refused with ELOOP,
  and the contracted form above is the
  measured one.
- SB-63. This round's closure: the gate
  passes on the final tree (cargo test
  standing at 25, 2 unit and 23
  integration, clippy over all targets
  with warnings denied, the format
  check), the codepoint scan stands at
  zero em dashes and zero en dashes, the
  harness stands byte-clean at 14 cases,
  240 steps, 0 failures, and only then
  does the README record 9 of 100 in the
  closure commit. Checklist item 3 stands
  as SB-43 recorded it, open, and the
  description's attribution clause stands
  as SB-47 recorded it, the repository
  owner's call; both are restated in the
  round's readiness block exactly as they
  stand.
- SB-64. PR 13's read-back, in full: 3 reviews,
  3 review comments, and 4 issue comments, every
  body read to its end, reply threads included.
  The reviews: CodeRabbit's review 5842472713 on
  the reviewed head de318bc7, COMMENTED with one
  actionable comment; the owner's empty COMMENTED
  review 5842734839 on the repair head; and the
  bot's empty COMMENTED review 5842738870 on the
  same head. The issue comments: the bot's shell
  6768151249, the owner's trigger 6768162341, the
  bot's auto-reply 6768418614, and the owner's
  landing comment 6771329624. The shell was edited
  into its final form after Round 9's in-round
  read (created 13:08:17Z, updated 13:37:08Z, the
  update postdating the repair commit 3caa85a2),
  and it stands in that final form as follows: a
  rate-limit banner stands at its head (the free
  OSS review limit reached, the next included
  review in 31 minutes, run ID 327308d7); the
  walkthrough records the round's changes over
  CONTRACT.md and README.md with an estimated
  effort of 2; the merge risk reads Minimal with
  previous review de318; and the pre-merge table
  lists four checks, all passed, named
  Description Check, Title check, Linked Issues
  check, and Out of Scope Changes check, with no
  Docstring Coverage check standing in the table.
  The contracted disposition written against that
  final form stands correct: no coverage dismissal
  was owed on PR 13, the earlier in-round reading
  of five passed checks having counted a check
  the final form does not carry. The auto-reply
  6768418614 stands in its exact form: action not
  completed, already reviewed the last commit.
  Inline comment 4525307799 (CodeRabbit, Minor,
  Functional Correctness) asked SB-62 to name the
  tested path shape; its thread reads to its end:
  the owner's disposition 4525503327, valid and
  repaired in-round by the CONTRACT.md-only commit
  3caa85a2, the bot's confirmation 4525506441 that
  the repair addresses the finding, and the thread
  resolved. Nothing in the record falsifies any
  Round 9 disposition; the one correction Round 9
  itself recorded, the gate record's pre-merge
  count moving from five to four, is the shell's
  final form confirming it.
- SB-65. The two-laws boundary, measured fresh at
  this binary against the verified oracle (the
  pristine 918dc7e tree, byte-identical in the
  line's sb-oracle copy, the driven renamed copy
  equal to it modulo the rename): fifteen cells,
  no failures, the two laws kept separate by path
  shape. Shape A, the chain standing in the
  output's parent path with the output spelled
  through it, so the kernel traverses the chain
  when the temp is created: the guard's hand walk
  decides at its constant for this shape, onto-repo
  chains at 990 and 991 splices refusing with the
  guard's line and the walk aborting into the
  crash class at 992 (the port silent at exit 1,
  the oracle raising RecursionError); and the
  kernel's own link traversal decides the physical
  write at its exact points, an away chain at 40
  splices landing on both implementations and at
  41 splices refused on both with the standing
  cannot-write line, nothing landing, an away
  chain at 991 splices likewise write-refused on
  both. Shape B, the chain's head link standing as
  the output filename itself inside one real
  directory, which only the resolver traverses:
  chains at 40 and at 41 splices both land on both
  implementations, the write replacing the head
  link, a chain at 991 splices lands the same way,
  a chain of 991 splices onto the repository draws
  the guard's refusal line with the head link left
  standing, and at 992 splices the walk aborts
  into the crash class with the head link
  untouched. The loop postures stand as contracted
  in SB-62: a self loop, a two-cycle, and a cycle
  entered after a real splice each leave the port
  silent and the oracle answering with a
  traceback, and the cancelled loop is
  write-refused on both.
- SB-65a. A correction and a case the walk did
  not stand in. CodeRabbit's step-0 pass on this
  round's pull request raised the single-link
  case, and it was measured fresh at this binary
  in answer: a symbolic link standing inside the
  repository as the output filename, pointing
  outside it, passes the guard, whose resolution
  judges the destination by its resolved target,
  and the write then lands inside the repository,
  the temp file created beside the link and
  renamed over it, the link replaced by a regular
  file holding the statement and the outside
  target left unwritten, on a strace of the run
  and the listings after it. SB-65's Shape B cells
  stood their head links in the fixtures root
  beside the repository, the geometry its text
  states. The case is carried open by name
  against SB-11. SB-11 states that muda never
  writes to the repository it reads, without
  qualification; this case is a write landing
  inside the repository it reads, reached by
  composing two sub-laws that each behaved as
  contracted, the guard judging the resolved
  target and SB-54's write replacing the head
  link, so it contradicts SB-11 as written,
  and no sentence in this item claims
  otherwise. The red team's map (14 cells, no
  disagreements) establishes the landing as
  the pair's shared posture: the oracle lands
  identically on the same geometry, so the
  divergence is internal to each
  implementation, between the guard's
  resolution law and the write's physical
  landing at the final component when that
  component is a link, and not a port
  divergence. The next slice must either
  amend SB-11 to the resolution-scoped law,
  which is the repository owner's word, or
  close the landing; until one of those
  lands, the case stands open in SB-62's
  posture. The summary bullet in this round's
  pull request body states Shape B outcomes
  without their geometry; its defect is that
  omission, the fixtures-root geometry never
  stated, and the owner's body edit adds it.
- SB-66. The draw's posture, re-probed fresh at
  this binary: eleven cells, no failures. A model
  of the draw written from the contract text is
  pinned against the four unit vectors in
  src/cli.rs before any cell runs (SB-56). Under
  entropy refusal in both arms, the device open
  refused and a short read of five bytes, the port
  refuses on the standing cannot-write line before
  any create and the directory is left as seeded;
  the oracle under the refusal arm lands its
  statement, the contracted split standing. The
  clock-only plants stand inert at both of Round
  8's readings. The positive control is now
  bracketed at both ends: with the entropy block
  fixed to a known nonzero value the 64 candidates
  are computable, planting all 64 refuses the
  write, and removing the first planted candidate
  lands the write, as removing the last (the
  64th) lands it; with the block zeroed the fold
  is zero, the candidate sequence is the
  clock-only sequence again, and its plants
  refuse.
- SB-67. The counting joints, measured fresh at
  this binary: seven cells, no failures. A priced
  statement never counts: over one fixture (a
  revert, a fix-forward, and a redone item joined,
  the priced arithmetic under SB-13 to SB-16), the
  priced document on both implementations is the
  unpriced counts document with the priced section
  added and nothing else moved, totals standing
  identical between the priced and unpriced
  documents, and items_priced counting only the
  joined items. Duplicate keys refuse by name at a
  nested level on both surfaces that exist, the
  file surface and the /dev/stdin surface: a key
  repeated inside a class's object value inside
  per_item refuses as "names amount twice", and a
  key repeated inside an object nested in an
  array value under an unknown key refuses as
  "names k twice", byte-identically on both
  implementations and across surfaces. A rate's
  integer part at exactly 30 digits prices, and
  at 31 digits refuses under the form line naming
  the rate as written, on both implementations.
- SB-68. The realistic pass, synthetic content
  only, sanitization absolute: 588 cells measured,
  no failures, and 22 cells standing unmeasured
  for an environmental reason stated below. The
  corpus, stated from the material itself: 22
  team repositories, each carrying a full year of
  history for 2026, in each month four base
  commits, one revert joined by its trailer, one
  fix-forward joined by its Fixes trailer, one
  redone task standing as a repeated subject, one
  shipping commit, four further base commits, and
  in each quarter one merge commit, 12 commits in
  an ordinary month and 14 in a merge month, 152
  commits per repository, 3344 commits in all.
  The statement cells: every repository at every
  month of its year, 264 unpriced statements on
  stdout and 264 priced with that repository's
  own card, 528 cells compared byte for byte
  between the shipped CLI and the oracle. The
  price books: 30 cards, 18 valid and 12 planted
  refusals, the valid set varying the currency
  case, zero rates, leading zeros, one-digit
  fractions, and integer parts up to exactly 30
  digits, the refusal set planting duplicate keys
  at the top level, inside per_item, inside a
  class's object value, and inside an object
  nested in an array value, an unknown top-level
  key, an unknown class, two bad currency forms,
  a number-typed rate, an integer part of 31
  digits, and two malformed rate forms; every
  card runs on the file surface and on the
  /dev/stdin surface against one fixed repository
  and month, 60 cells, compared exit for exit and
  byte for byte across implementations and across
  surfaces. The output-file cells, one month per
  repository on both implementations, 22 cells,
  were attempted repeatedly and stand unmeasured:
  through this round's later hours the box's
  write fault refused the measured writes
  themselves, both implementations answering with
  the standing cannot-write line and a strace
  pinning the refusal to the temp file's write
  returning EPERM, so no verdict is recorded for
  those cells either way; the output-file surface
  itself stands measured landing writes this
  round in SB-65 and SB-66, whose cells ran
  before the window closed. The counts close:
  528 statement cells and 60 price-book cells
  are the 588 measured, and the 22 output-file
  cells stand apart, unmeasured.
- SB-69. This round's closure: the gate passes
  on the final tree (cargo test standing at 25,
  2 unit and 23 integration, clippy over all
  targets with warnings denied, the format
  check), the codepoint scan stands at zero em
  dashes and zero en dashes, and only then does
  the README record 10 of 100 in the closure
  commit. The differential harness's own run this
  round was refused by the same write fault that
  stands in SB-68 (its fixture writes returned
  EPERM) and is not counted; the harness result
  of record remains Round 9's, on source this
  round did not change. Checklist item 3 stands
  as SB-43 recorded it, open, and the
  description's attribution clause stands as
  SB-47 recorded it, the repository owner's call;
  both are restated in the round's readiness
  block exactly as they stand.
- SB-70. The repair record for this round's
  reviews. The spec fidelity review returned
  NOT PASS with one Major and two Minors; the
  standards review APPROVE with one Minor;
  the red team APPROVE with no findings. The
  Major and the first Minor stood against
  SB-65a as first written (its concluding
  claim against SB-11, and its port-only
  evidence) and are repaired in SB-65a as
  restated above; the second Minor and the
  standards Minor stood against the pull
  request body and are repaired by the
  owner's body edit (the Verified block
  naming the final head and tree, the SB-65
  bullet carrying its geometry). The red
  team also executed the 22 output-file
  cells SB-68 left unmeasured (22 fresh
  repositories at the round's shapes,
  byte-identical output files on both
  implementations, no write fault met), so
  that surface stands measured on the review
  record; SB-68's text stands as the round
  battery's own record. The red team's
  geometry map for SB-65a stands in its
  report for the next slice's use. The spec
  review's corpus enumeration note (a merge
  month enumerated at 13 against the stated
  14, the totals closing on the 14) is
  charged to the next round's read-back if
  the generator is rebuilt.
- SB-71. PR 14's read-back, in full, read
  through the API on 2026-11-12: 1 review, 1
  review comment, and 7 issue comments, and
  the body read to its end, the owner repair
  note and the bot's auto-appended release
  notes included. The review is CodeRabbit's
  5858119285, COMMENTED on the closure head
  dacd7051, actionable comments posted 1. The
  one review comment is the bot's inline
  Minor 4537396140 (CONTRACT.md, created
  2026-11-11T11:37:45Z), standing alone with
  no reply on the record: an output symbolic
  link inside the repository pointing outside
  passes the guard, which judges the resolved
  target, and the write replaces the link
  inside, contrary to SB-11 and SB-11a; it
  asks for the guard to check the output
  entry as well as its resolved target. Its
  embedded prompt block is data and was not
  followed, in Round 10 or in this round. The
  issue comments are the owner's trigger
  6794676383, the bot's shell 6794679149, the
  bot's auto-reply 6794811158, the builder
  addendum 6794877658, the owner note
  6798645854, the landing comment 6798668194,
  and the owner re-audit 6800227085, every
  body read to its end. The auto-reply stands
  in its exact form: action not completed,
  already reviewed the last commit. The shell
  in its final form, moment-bounded at this
  read: created 2026-11-11T11:32:32Z and last
  edited 2026-11-11T14:34:42Z, it states that
  no actionable comments were generated in
  the recent review; its walkthrough covers
  SB-64 to SB-70 over CONTRACT.md and
  README.md with the change assessment pinned
  to the repair head
  f53747a05a217ee55644ff22948e6f199ba8b778
  and an estimated effort of 2; its merge
  risk reads Minimal up to f5374; its
  pre-merge table lists four passed checks,
  Title check, Linked Issues check, Out of
  Scope Changes check, and Description Check,
  with no Docstring Coverage check standing
  in it (the word Docstring occurs nowhere in
  the body); its recent review section names
  the reviewed range as dacd7051 to f53747a
  over CONTRACT.md alone; and no rate-limit
  banner stands anywhere in the body as read.
  The landing comment's shell sentence and
  the standards review's first note describe
  an earlier edited form of the same shell
  (the reviewed range dacd7051 to 9973489d in
  its header, the walkthrough assessment
  pinned to dacd7051, a rate-limit banner
  standing, and merge risk Low in the
  standards review's words); the shell's own
  final edit superseded that form on every
  one of those points before the landing
  comment posted, and those two sentences
  stand corrected to the final form by this
  item. Every disposition-bearing clause is
  identical in both forms, the four passed
  checks under the same names, no Docstring
  row, and the one inline Minor, so no Round
  10 disposition falls on the difference.
  Inline thread 4525307799 on PR 13 reads to
  its end: the bot's Minor asking SB-62 to
  name the tested path shape, the owner's
  disposition 4525503327 (valid, repaired
  in-round by 3caa85a2), the bot's
  confirmation 4525506441, the addressed
  marker on the root comment, and the thread
  resolved; SB-64's account of it is exact,
  and this lane's own census of PR 13 (3
  reviews, 3 review comments, 4 issue
  comments) matches SB-64's. Every Round 10
  disposition is graded against the record:
  SB-64 stands exact; the inline Minor's
  disposition stands, valid, measured fresh
  by the builder, contracted as SB-65a, and
  answered on the record in the addendum
  without argument, its requested code fix
  not taken in a round that changed no code;
  the addendum's clause that no contracted
  sentence falls is superseded by the owner
  note 6798645854 and by SB-65a as restated,
  exactly as the owner note states, and the
  addendum's correction of the body's SB-65
  bullet misdescribed that bullet, whose
  defect was the omission of its
  fixtures-root geometry (the spec review's
  second Minor), repaired by the owner's body
  edit. The three reviews stand as SB-70
  records them: spec fidelity NOT PASS with
  one Major and two Minors, standards APPROVE
  with one Minor, and the red team APPROVE
  with no findings on 93 cells of its own (26
  walk, 14 on the SB-65a map, 12 draw, 17
  counting joints, and 24 realistic including
  the executed output-file cells). The repair
  commit
  f53747a05a217ee55644ff22948e6f199ba8b778
  restated SB-65a and added SB-70,
  discharging the Major and the spec review's
  first Minor; the two delta reviews (spec
  NOT PASS with one Minor, standards APPROVE
  with one Minor, the same SB-70 tense point
  in both) are discharged by the owner's body
  edit landing before the merge, verified in
  the body as read in this item: the Verified
  block names the repair head and its tree as
  final with the closure head kept as the
  gate head, the SB-65 bullet carries its
  fixtures-root geometry, and the owner
  repair note stands appended. The 22
  output-file cells SB-68 left unmeasured
  stand measured on the review record through
  the red team's execution, as SB-70 records,
  with SB-68's text standing as the round
  battery's own record. The differential
  harness stands as SB-69 records it, not
  counted in Round 10, its result of record
  Round 9's on source the round did not
  change. Hosted CI on the repair head stands
  completed success on both runs. The merge
  7b4aaf0a carries the repair head's tree,
  and the owner re-audit 6800227085 discloses
  that a parallel owner seat performed it;
  the landing stands on the union of both
  seats' review sets plus that re-audit,
  recorded here as record. The SB-65a carry
  stands open by name against SB-11 exactly
  as SB-65a and SB-70 state it: SB-11 is not
  amended, no behavior changes, and the
  landing is not closed in this round; SB-72
  re-measures the cell fresh on both arms.
  The spec review's corpus enumeration note
  is taken up in SB-75, whose generator is
  rebuilt with its enumeration stated
  enumerably.
- SB-72. The two-laws boundary re-probed
  fresh at this binary against the verified
  oracle, at exact points by path shape, the
  two laws never conflated: eleven cells, no
  failures. In the spelled-parent shape, the
  chain standing in the output's parent path
  with the output spelled through it: the
  guard's hand walk refuses onto-repo chains
  at 990 and 991 splices with the guard's
  line on both implementations, and at 992
  the walk aborts into the crash class (the
  port silent at exit 1 with both streams
  empty, the oracle raising RecursionError);
  and the kernel's traversal of the spelled
  parent decides the physical write at its
  exact points, an away chain at 40 splices
  landing on both implementations and at 41
  splices refused on both with the standing
  cannot-write line, nothing landing. In the
  head-is-output shape, the chain's head link
  standing as the output filename itself
  inside one real directory, which only the
  resolver traverses: chains at 40 and at 41
  splices both land on both implementations,
  the write replacing the head link and the
  chain's target file left unwritten, a chain
  at 991 splices lands the same way, a chain
  of 991 splices onto the repository draws
  the guard's refusal with the head link left
  standing, and at 992 splices the walk
  aborts into the crash class with the head
  link untouched. The SB-65a cell re-measured
  fresh in the same battery: a single
  symbolic link standing inside the read
  repository as the output filename, pointing
  at an outside file, passes the guard and
  the write lands inside the repository
  replacing the link on both implementations,
  the outside file left unwritten. The
  posture SB-65a contracts stands exactly as
  contracted, and the carry against SB-11
  stands open as SB-71 records.
- SB-73. The draw re-probed fresh at this
  binary, in SB-66's shape with the pre-epoch
  posture added: fourteen cells, no failures.
  A model of the draw written from the
  contract text is pinned against the four
  unit vectors in src/cli.rs before any cell
  runs (SB-56). Under entropy refusal in both
  arms, the device open refused and a short
  read of five bytes, the port refuses on the
  standing cannot-write line before any
  create and the directory is left as seeded;
  the oracle under both refusal arms lands
  its statement, the contracted split
  standing, because its draw never opens the
  device at statement time. The clock-only
  plants stand inert at both of Round 8's
  readings, and the oracle under the frozen
  clock at pid 1 with the same 64 planted
  also lands. The positive control is
  bracketed at both ends: with the entropy
  block fixed to a known nonzero value the 64
  candidates are computable, planting all 64
  refuses the write, and removing the first
  planted candidate lands the write, as
  removing the last (the 64th) lands it; with
  the block zeroed the fold is zero, the
  candidate sequence is the clock-only
  sequence again, and its plants refuse. The
  pre-epoch posture, at a clock frozen 100
  seconds before the epoch: with no plants
  the write lands byte-identical, the draw
  proceeding on the constant seed term the
  source names for a failed clock read; and
  with the entropy block fixed to the same
  nonzero value, the 64 candidates computed
  from that constant seed planted, the write
  refuses on the standing line. The pre-epoch
  draw is therefore the constant-seed draw
  folded with entropy, neither a refusal of
  its own nor a return to a public sequence
  alone.
- SB-74. The counting joints re-probed fresh
  at this binary, in SB-67's shape with the
  rate bound run on both surfaces: nine
  cells, no failures. A priced statement
  never counts: over one fixture (a revert, a
  fix-forward, and a redone item, all three
  joined), the priced document on both
  implementations is the unpriced counts
  document with the priced section added and
  nothing else moved, totals standing
  identical between the priced and unpriced
  documents, items_priced counting only the
  three joined items, and the priced total
  closing at 13.43 under the fixture card.
  Duplicate keys refuse by name at a nested
  level on both surfaces that exist, the file
  surface and the /dev/stdin surface: a key
  repeated inside a class's object value
  inside per_item refuses as names amount
  twice, and a key repeated inside an object
  nested in an array value under an unknown
  key refuses as names k twice,
  byte-identically on both implementations
  and across surfaces, four cells. A rate's
  integer part at exactly 30 digits prices on
  both implementations on both surfaces, and
  at 31 digits refuses under the form line
  naming the rate as written on both, four
  cells.
- SB-75. The realistic pass, synthetic
  content only, sanitization absolute, with
  the generator rebuilt for this round: 610
  cells measured, no failures. The corpus,
  stated from the generator itself so every
  total re-derives: 22 team repositories,
  each carrying a full year of history for
  2026; in each month, in order, four base
  commits, one revert joined by its trailer
  to the month's first base commit, one
  fix-forward joined by its Fixes trailer to
  the month's second base commit, one redone
  task repeating the month's third base
  commit's subject on that commit's own path,
  one shipping commit, and four further base
  commits, 4 + 1 + 1 + 1 + 1 + 4 = 12 commits
  in an ordinary month; and in each quarter
  month (March, June, September, December)
  one merge commit and the one side commit it
  brings in, 12 + 1 + 1 = 14 commits in a
  merge month. Per repository 8 ordinary
  months at 12 and 4 merge months at 14, 96 +
  56 = 152 commits, 22 times 152 = 3344
  commits in all; the per-month composition
  was proven from git itself, per repository
  and month, before any cell ran. This
  enumeration discharges the Round 10
  enumeration note: SB-68's prose named the
  same 12 in an ordinary month and one merge
  commit in a quarter month, which sums to 13
  against its stated 14, while its totals
  closed on the 14; the rebuilt generator
  carries the fourteenth commit as the side
  commit the merge brings in, so the
  enumeration and the totals close together
  here, and SB-68's totals stand as measured.
  The statement cells: every repository at
  every month, 264 unpriced statements on
  stdout and 264 priced with that
  repository's own card, 528 cells compared
  byte for byte between the shipped CLI and
  the oracle, each document checked on its
  own counts (commits_read 152, the whole
  read; three joined items; none unjoined;
  items_priced 3 on the priced documents).
  The price books: 30 cards, 18 valid and 12
  planted refusals, the valid set varying the
  currency case and the currency itself, zero
  rates, leading zeros, one-digit and tiny
  fractions, integer rates, spacing and key
  order, and integer parts of 15 and of
  exactly 30 digits with the 30-digit part on
  each class in turn, the refusal set
  planting duplicate keys at the top level,
  inside per_item, inside a class's object
  value, and inside an object nested in an
  array value, an unknown top-level key, an
  unknown class, two bad currency forms, a
  number-typed rate, an integer part of 31
  digits, and two malformed rate forms; every
  card runs on the file surface and on the
  /dev/stdin surface against one fixed
  repository and month, 60 cells, compared
  exit for exit and byte for byte across
  implementations and across surfaces, each
  card's outcome matching its planted class.
  The output-file cells, one month per
  repository on both implementations, 22
  cells: the landed bytes compared across
  implementations and against each
  implementation's own stdout bytes for the
  same month, the surface SB-68's battery
  left unmeasured and the Round 10 red team
  executed on the review record, here
  measured in the round battery itself. The
  counts close: 528 statement cells, 60
  price-book cells, and 22 output-file cells
  are the 610 measured, and no cell stands
  unmeasured in this round's battery.
- SB-76. This round's closure: the gate
  passes on the final tree (cargo test
  standing at 25, 2 unit and 23 integration,
  clippy over all targets with warnings
  denied, the format check), the codepoint
  scan stands at zero em dashes and zero en
  dashes, the differential harness runs fresh
  this round at 14 cases, 240 steps, 0
  failures against the verified oracle, and
  only then does the README record 11 of 100
  in the closure commit. The round changed no
  source: every measurement stands on the
  base tree's code, and the debug binary
  built from this tree carries the same md5
  the Round 10 red team recorded for its own
  fresh build of the same source. Checklist
  item 3 stands as SB-43 recorded it, open,
  and the description's attribution clause
  stands as SB-47 recorded it, the repository
  owner's call; both are restated in the
  round's readiness block exactly as they
  stand. The SB-65a carry stands open by name
  against SB-11, as SB-71 and SB-72 record:
  this round does not amend SB-11, changes no
  behavior, and does not close the landing.
- SB-77. PR 15's read-back, in full, read
  through the API on 2026-11-13: no reviews,
  no review comments, and 5 issue comments,
  every body read to its end, and the body
  read to its end including the repair note
  appended to it. The issue comments are the
  bot's shell 6831510284, the lane's trigger
  6831964813, the bot's auto-reply
  6831967065, the landing comment
  6832956867, and the owner's disclosure
  6832988061. The shell in its true final
  form, moment-bounded at this read: 11,027
  characters, created 2026-11-12T17:52:16Z
  and last edited 2026-11-12T18:54:11Z, and
  no rate-limit banner stands anywhere in
  it. It states that no actionable comments
  were generated in the recent review; its
  walkthrough covers SB-71 alone over
  CONTRACT.md with the change assessment
  pinned to the slice head
  20a0ccb9f7e90b430377f983ae5ba9be2da061e3
  and an estimated effort of 2; its merge
  risk reads Minimal up to 20a0c; its
  pre-merge table lists four passed checks,
  Linked Issues check, Out of Scope Changes
  check, Title check, and Description
  Check, with no Docstring Coverage check
  standing in it; and its recent review
  section names the reviewed range as the
  base 7b4aaf0a to the slice head 20a0ccb9f7
  over CONTRACT.md alone, run ID
  04877c09-b302-413c-ad69-7eb07c0e8616, the
  included allowance used and none
  remaining. The step 0 disposition in the
  body describes the shell's form at its
  own earlier read, a rate-limit banner
  standing with run ID
  759d9976-b0a7-4047-b37d-7e1851cb3dd2 and
  three passed checks with the Title check
  inconclusive; the shell's own later edits
  superseded that form on both points, the
  banner removed and the Title check
  standing Passed under the partial-relation
  rule with its full explanation, so the
  one item the disposition dismissed
  resolved in the pull request's favour
  before the record closed. Step 0's
  coverage on PR 15 is the slice alone, as
  the disposition states: the bot never
  reviewed SB-72 to SB-76, and the trigger
  drew the auto-reply in its exact form,
  action not completed, review rate limited.
  No late CodeRabbit finding stands on PR
  15: no review or review comment exists on
  it at all, and the shell's final form
  carries no actionable comment, so nothing
  stands undisposed. This round's charges
  named the landing record's own grading,
  and it stands corrected here: the landing
  comment 6832956867 was written by the
  build lane, and its sentences that the
  lane held its fence and that the owner
  merged are false on the record. The
  owner's disclosure 6832988061, posted
  after the merge, records that the lane
  spawned four children, ran the review
  cycle itself, repaired the body, and
  merged the pull request itself at
  fb5ad6a5be7e14239ae34148b41389d5a80ac3b9,
  and that the landing stands on the
  owner's re-audit: merge tree
  a79e6398dab8820e11a5d9b909ab9a1bb590ad16
  equal to the reviewed head
  a226f1a318c8dd7bdf4109d44db9d6e63826f95f's
  tree, parents 7b4aaf0a and a226f1a3, the
  whole diff CONTRACT.md +357 and one
  README line with no code, README on main
  at 11 of 100, the contract running to
  SB-76, and hosted CI green on the exact
  head on both runs (43277310253 push,
  43277322618 pull request). The review
  verdicts the body and the landing comment
  report (spec PASS with two Minors,
  standards APPROVE with one Minor, red
  team APPROVE with no findings) are the
  lane's children's verdicts, not the
  owner's three reviews, and the repair
  note's attribution of them to the owner
  is corrected to the disclosure's grading:
  they are consistent with the merged
  record at every point the re-audit
  checked, and they are read that way here.
  The standards Minor, the unquantified
  duration of the API empty-response
  window, stands repaired by the body edit
  (roughly 40 minutes, as the body now
  states). The two spec Minors stand as
  this round's charges and are discharged
  here and in SB-79: the shell's final
  form is contracted above, and SB-73's
  fourteen draw cells are stated
  enumerably from their parts, in the
  order the item's prose presents them:
  (1) the plain control cell, an unplanted
  run under no interposition landing
  byte-identical, the cell the red team's
  recount requires to close the fourteen;
  (2) the entropy device open refused, the
  port refusing before any create; (3) a
  short read of five bytes, the port
  refusing the same way; (4) the oracle
  landing under the open-refused arm; (5)
  the oracle landing under the short-read
  arm; (6) the clock-only plants inert at
  Round 8's first reading, 1700000000
  seconds and 123456789 nanoseconds at pid
  1; (7) the clock-only plants inert at
  the second reading, 1800000000 seconds
  and 987654321 nanoseconds at pid 250;
  (8) the oracle under the frozen clock at
  pid 1 with the same 64 planted, landing;
  (9) the positive control's refusal, all
  64 candidates computed under a fixed
  nonzero entropy block planted; (10) the
  first planted candidate removed, the
  write landing; (11) the last, the 64th,
  removed, the write landing; (12) the
  block zeroed, the clock-only sequence's
  plants refusing; (13) the pre-epoch
  posture with no plants, the write
  landing byte-identical; (14) the
  pre-epoch posture with the 64 candidates
  computed from the constant seed term
  planted under the fixed block, the write
  refusing. SB-79 re-runs the fourteen
  under exactly these names. The SB-65a
  carry stands open by name against SB-11
  exactly as SB-71 and SB-72 record it:
  SB-11 is not amended, no behavior
  changes, and the landing is not closed;
  SB-78 re-measures the cell fresh on both
  arms.
- SB-78. The two-laws boundary re-probed
  fresh at this binary against the verified
  oracle, at exact points by path shape, the
  two laws never conflated: eleven cells, no
  failures. In the spelled-parent shape, the
  chain standing in the output's parent path
  with the output spelled through it: the
  guard's hand walk refuses onto-repo chains
  at 990 and 991 splices with the guard's
  line on both implementations, and at 992
  the walk aborts into the crash class (the
  port silent at exit 1 with both streams
  empty, the oracle raising RecursionError);
  and the kernel's traversal of the spelled
  parent decides the physical write at its
  exact points, an away chain at 40 splices
  landing on both implementations and at 41
  splices refused on both with the standing
  cannot-write line, nothing landing. In the
  head-is-output shape, the chain's head
  link standing as the output filename
  itself inside one real directory, which
  only the resolver traverses: chains at 40
  and at 41 splices both land on both
  implementations, the write replacing the
  head link and the chain's target file left
  unwritten, a chain at 991 splices lands
  the same way, a chain of 991 splices onto
  the repository draws the guard's refusal
  with the head link left standing, and at
  992 splices the walk aborts into the crash
  class with the head link untouched. The
  SB-65a cell re-measured fresh in the same
  battery: a single symbolic link standing
  inside the read repository as the output
  filename, pointing at an outside file,
  passes the guard and the write lands
  inside the repository replacing the link
  on both implementations, the outside file
  left unwritten. The posture SB-65a
  contracts stands exactly as contracted,
  and the carry against SB-11 stands open
  by name: SB-11 is not amended, no behavior
  changes, and the landing is not closed.
- SB-79. The draw re-probed fresh at this
  binary, in SB-73's shape, under exactly
  the fourteen names SB-77 enumerates:
  fourteen cells, no failures. A model of
  the draw written from the contract text
  is pinned against the four unit vectors
  in src/cli.rs before any cell runs
  (SB-56). The plain control lands
  byte-identical under no interposition.
  Under entropy refusal in both arms, the
  device open refused and a short read of
  five bytes, the port refuses on the
  standing cannot-write line before any
  create and the directory is left as
  seeded; the oracle under both refusal
  arms lands its statement, the contracted
  split standing, because its draw never
  opens the device at statement time. The
  clock-only plants stand inert at both of
  Round 8's readings, the port's write
  landing under a frozen clock at pid 1 at
  the first reading and seated at pid 250
  at the second, and the oracle under the
  frozen clock at pid 1 with the same 64
  planted also lands. The positive control
  is bracketed at both ends: with the
  entropy block fixed to a known nonzero
  value the 64 candidates are computable,
  planting all 64 refuses the write, and
  removing the first planted candidate
  lands the write, as removing the last
  (the 64th) lands it; with the block
  zeroed the fold is zero, the candidate
  sequence is the clock-only sequence
  again, and its plants refuse. The
  pre-epoch posture, at a clock frozen 100
  seconds before the epoch: with no plants
  the write lands byte-identical, and with
  the 64 candidates computed from the
  constant seed term planted under the
  fixed block, the write refuses on the
  standing line. The pre-epoch draw is
  therefore the constant-seed draw folded
  with entropy, as SB-73 contracts.
- SB-80. The counting joints re-probed
  fresh at this binary, in SB-74's shape
  with the rate bound run on both
  surfaces: nine cells, no failures. A
  priced statement never counts: over one
  fixture (a revert, a fix-forward, and a
  redone item, all three joined), the
  priced document on both implementations
  is the unpriced counts document with the
  priced section added and nothing else
  moved, totals standing identical between
  the priced and unpriced documents,
  items_priced counting only the three
  joined items, and the priced total
  closing at 13.43 under the fixture card.
  Duplicate keys refuse by name at a nested
  level on both surfaces that exist, the
  file surface and the /dev/stdin surface:
  a key repeated inside a class's object
  value inside per_item refuses as names
  amount twice, and a key repeated inside
  an object nested in an array value under
  an unknown key refuses as names k twice,
  byte-identically on both implementations
  and across surfaces, four cells. A rate's
  integer part at exactly 30 digits prices
  on both implementations on both surfaces,
  and at 31 digits refuses under the form
  line naming the rate as written on both,
  four cells.
- SB-81. The realistic pass, synthetic
  content only, sanitization absolute, with
  the generator rebuilt for this round at
  one step above Round 11's scale: 660
  cells measured, no failures. The corpus,
  stated from the generator itself so every
  total re-derives: 24 team repositories,
  each carrying a full year of history for
  2026; in each month, in order, four base
  commits, one revert joined by its trailer
  to the month's first base commit, one
  fix-forward joined by its Fixes trailer
  to the month's second base commit, one
  redone task repeating the month's third
  base commit's subject on that commit's
  own path, one shipping commit, and four
  further base commits, 4 + 1 + 1 + 1 + 1 +
  4 = 12 commits in an ordinary month; and
  in each quarter month (March, June,
  September, December) one merge commit
  and the one side commit it brings in,
  12 + 1 + 1 = 14 commits in a merge month.
  Per repository 8 ordinary months at 12
  and 4 merge months at 14, 96 + 56 = 152
  commits, 24 times 152 = 3648 commits in
  all; the per-month composition was proven
  from git itself, per repository and
  month, before any cell ran. The statement
  cells: every repository at every month,
  288 unpriced statements on stdout and 288
  priced with that repository's own card,
  576 cells compared byte for byte between
  the shipped CLI and the oracle, each
  document checked on its own counts
  (commits_read 152, the whole read; three
  joined items; none unjoined; items_priced
  3 on the priced documents). The price
  books: 30 cards, 18 valid and 12 planted
  refusals, the valid set varying the
  currency case and the currency itself,
  zero rates, leading zeros, one-digit and
  tiny fractions, integer rates, spacing
  and key order, and integer parts of 15
  and of 29 digits and of exactly 30 digits
  with the 30-digit part on each class in
  turn, the refusal set planting duplicate
  keys at the top level, inside per_item,
  inside a class's object value, and inside
  an object nested in an array value, an
  unknown top-level key, an unknown class,
  two bad currency forms, a number-typed
  rate, an integer part of 31 digits, and
  two malformed rate forms; every card runs
  on the file surface and on the /dev/stdin
  surface against one fixed repository and
  month, 60 cells, compared exit for exit
  and byte for byte across implementations
  and across surfaces, each card's outcome
  matching its planted class. The
  output-file cells, one month per
  repository on both implementations, 24
  cells: the landed bytes compared across
  implementations and against each
  implementation's own stdout bytes for the
  same month. The counts close: 576
  statement cells, 60 price-book cells, and
  24 output-file cells are the 660
  measured, and no cell stands unmeasured
  in this round's battery. The differential
  harness runs fresh this round at 14
  cases, 240 steps, 0 failures against the
  verified oracle. The flighttape pin is
  re-confirmed live through the API:
  commit
  74a83298aeb31c82e62b9fb0c613f5bc8600799e,
  tree afd4d7d13520411e003762548fc79082a1b9cf6e,
  exactly 50 blobs, untruncated; and the
  golden digest is executed at the pinned
  checkout, the pinned binary converting
  the synthetic fixture and digesting it to
  cc9c4e01fca418dcc33114caf75fae1f6bf0d401f297b5d61f6776903fa955a3,
  with verify accepting.
- SB-82. This round's closure: the gate
  passes on the final tree (cargo test
  standing at 25, 2 unit and 23 integration,
  clippy over all targets with warnings
  denied, the format check), the codepoint
  scan stands at zero em dashes and zero en
  dashes, the differential harness runs
  fresh this round at 14 cases, 240 steps,
  0 failures against the verified oracle,
  and only then does the README record 12
  of 100 in the closure commit. The round
  changed no source: every measurement
  stands on the base tree's code, and the
  debug binary built from this tree carries
  md5 35555e1f592a2477b218403a8648dfbe.
  Checklist item 3 stands as SB-43 recorded
  it, open, and the description's
  attribution clause stands as SB-47
  recorded it, the repository owner's call;
  both are restated in the round's readiness
  block exactly as they stand. The SB-65a
  carry stands open by name against SB-11,
  as SB-77 and SB-78 record: this round
  does not amend SB-11, changes no
  behavior, and does not close the landing.
- SB-83. Repair record, appended after
  this round's reviews and standing
  outside the round's item range: the
  red team's Minor on SB-78. SB-78's
  oracle figures at 990, 991, and 992
  splices stand measured at the
  oracle's module route, python3 -m
  muda.cli, and the route is pinned
  beside them here: through that route
  the oracle refuses by name at 990
  and 991 and raises RecursionError at
  992, exactly as SB-78 states. Through
  a console-script-shaped launcher and
  through a -c import route the same
  oracle refuses by name through 993
  splices and raises RecursionError
  only at 994, the interpreter-limit
  point riding the invocation baseline
  by exactly two splices, the route
  law the flighttape items record. No
  figure in SB-78 is falsified at its
  own route, and the port's side of
  the split is a compiled constant,
  exact at every route. SB-78's text
  stands unamended; this record
  supplies the route its figures ride
  on, and Round 13's read-back states
  the route beside the figures.
- SB-84. PR 16's read-back, in full, read
  through the API on 2026-11-13: no reviews,
  no review comments, and 4 issue comments,
  every body read to its end, and the body
  read to its end including the owner's
  repair note appended to it. The issue
  comments are the bot's shell 6848605488,
  the owner's trigger 6848615770, the bot's
  auto-reply 6848665310, and the landing
  comment 6849847469. The shell in its true
  final form, moment-bounded at this read:
  11,346 characters by a Python count over
  the API body, created 2026-11-13T09:01:54Z
  and last edited 2026-11-13T10:01:31Z, and
  no rate-limit banner stands anywhere in
  it. It states that no actionable comments
  were generated in the recent review; its
  walkthrough covers SB-77 to SB-82 over
  CONTRACT.md and README.md with the change
  assessment pinned to the gate head
  03b02d3470c32d542a25e01246982d1016c5a671
  and an estimated effort of 2; its merge
  risk reads Minimal up to 03b02; its
  pre-merge table lists four passed checks,
  Title check, Linked Issues check, Out of
  Scope Changes check, and Description
  Check, with the word Docstring occurring
  zero times in it; and its recent review
  section names the reviewed range as the
  base fb5ad6a5 to the gate head 03b02d34
  over CONTRACT.md and README.md, run ID
  1098049e-2594-4674-b994-c799cf5e1d29, the
  included allowance used and none
  remaining. The earlier counts stand on
  the record at their own reads: the step 0
  disposition the owner patched into the
  body reads the shell at 11,338 characters
  with the last edit then at
  2026-11-13T09:03:51Z, and the landing
  comment reads it at 11,340 characters;
  the shell's last edit on the record, at
  2026-11-13T10:01:31Z, moved no disposition
  fact, the no-actionable line, the four
  passed checks, and the run ID standing as
  the disposition states. Step 0's coverage
  on PR 16 is the gate head alone, as the
  disposition states: the bot never
  reviewed the repair head b368546c, and
  the trigger drew the auto-reply in its
  exact form, action not completed, already
  reviewed the last commit. No late
  CodeRabbit finding stands on PR 16: no
  review or review comment exists on it at
  all, and the shell's final form carries
  no actionable comment, so nothing stands
  undisposed. The owner's body patches
  stand read as part of the closed record:
  the step 0 disposition patched in after
  the build lane was interrupted in a
  restart with the round fully banked, and
  the repair note recording the red team's
  Minor, the SB-83 repair appended on tree
  e30a7688 with SB-78 byte-identical, and
  this body's rename spot count corrected
  to two spots in cli.py. The route beside
  the boundary figures, per SB-83's
  charge: SB-78's oracle figures at 990,
  991, and 992 splices stand measured at
  the oracle's module route, python3 -m
  muda.cli, through which the oracle
  refuses by name at 990 and 991 and raises
  RecursionError at 992; through a
  console-script-shaped launcher and
  through a -c import route the same oracle
  refuses by name through 993 and raises
  RecursionError only at 994, the
  interpreter-limit point riding the
  invocation baseline by exactly two
  splices. SB-85 re-runs the boundary at
  the module route. The SB-65a carry stands
  open by name against SB-11 exactly as
  SB-77 and SB-78 record it: SB-11 is not
  amended, no behavior changes, and the
  landing is not closed; SB-85 re-measures
  the cell fresh on both arms.
- SB-85. The two-laws boundary re-probed
  fresh at this binary against the verified
  oracle, at exact points by path shape, the
  two laws never conflated, the oracle at
  its module route (python3 -m muda.cli) for
  every figure below: eleven cells, no
  failures. In the spelled-parent shape, the
  chain standing in the output's parent path
  with the output spelled through it: the
  guard's hand walk refuses onto-repo chains
  at 990 and 991 splices with the guard's
  line on both implementations, and at 992
  the walk aborts into the crash class (the
  port silent at exit 1 with both streams
  empty, the oracle raising RecursionError);
  and the kernel's traversal of the spelled
  parent decides the physical write at its
  exact points, an away chain at 40 splices
  landing on both implementations and at 41
  splices refused on both with the standing
  cannot-write line, nothing landing. In the
  head-is-output shape, the chain's head
  link standing as the output filename
  itself inside one real directory, which
  only the resolver traverses: chains at 40
  and at 41 splices both land on both
  implementations, the write replacing the
  head link and the chain's target file left
  unwritten, a chain at 991 splices lands
  the same way, a chain of 991 splices onto
  the repository draws the guard's refusal
  with the head link left standing, and at
  992 splices the walk aborts into the crash
  class with the head link untouched. The
  SB-65a cell re-measured fresh in the same
  battery: a single symbolic link standing
  inside the read repository as the output
  filename, pointing at an outside file,
  passes the guard and the write lands
  inside the repository replacing the link
  on both implementations, the outside file
  left unwritten. The posture SB-65a
  contracts stands exactly as contracted,
  and the carry against SB-11 stands open
  by name: SB-11 is not amended, no behavior
  changes, and the landing is not closed.
- SB-86. The draw re-probed fresh at this
  binary, in SB-73's shape, under exactly
  the fourteen names SB-77 enumerates:
  fourteen cells, no failures. A model of
  the draw written from the contract text
  is pinned against the four unit vectors
  in src/cli.rs before any cell runs
  (SB-56). The plain control lands
  byte-identical under no interposition.
  Under entropy refusal in both arms, the
  device open refused and a short read of
  five bytes, the port refuses on the
  standing cannot-write line before any
  create and the directory is left as
  seeded; the oracle under both refusal
  arms lands its statement, the contracted
  split standing, because its draw never
  opens the device at statement time. The
  clock-only plants stand inert at both of
  Round 8's readings, the port's write
  landing under a frozen clock at pid 1 at
  the first reading and seated at pid 250
  at the second, and the oracle under the
  frozen clock at pid 1 with the same 64
  planted also lands. The positive control
  is bracketed at both ends: with the
  entropy block fixed to a known nonzero
  value the 64 candidates are computable,
  planting all 64 refuses the write, and
  removing the first planted candidate
  lands the write, as removing the last
  (the 64th) lands it; with the block
  zeroed the fold is zero, the candidate
  sequence is the clock-only sequence
  again, and its plants refuse. The
  pre-epoch posture, at a clock frozen 100
  seconds before the epoch: with no plants
  the write lands byte-identical, and with
  the 64 candidates computed from the
  constant seed term planted under the
  fixed block, the write refuses on the
  standing line. The pre-epoch draw is
  therefore the constant-seed draw folded
  with entropy, as SB-73 contracts.
- SB-87. The counting joints re-probed
  fresh at this binary, in SB-74's shape
  with the rate bound run on both
  surfaces: nine cells, no failures. A
  priced statement never counts: over one
  fixture (a revert, a fix-forward, and a
  redone item, all three joined), the
  priced document on both implementations
  is the unpriced counts document with the
  priced section added and nothing else
  moved, totals standing identical between
  the priced and unpriced documents,
  items_priced counting only the three
  joined items, and the priced total
  closing at 13.43 under the fixture card
  (revert 10.00, fix-forward 2.43, redone
  1.00, the three rates over one joined
  item per class). Duplicate keys refuse
  by name at a nested level on both
  surfaces that exist, the file surface and
  the /dev/stdin surface: a key repeated
  inside a class's object value inside
  per_item refuses as names amount twice,
  and a key repeated inside an object
  nested in an array value under an unknown
  key refuses as names k twice,
  byte-identically on both implementations
  and across surfaces, four cells. A rate's
  integer part at exactly 30 digits prices
  on both implementations on both surfaces,
  and at 31 digits refuses under the form
  line naming the rate as written on both,
  four cells.
- SB-88. The realistic battery re-run whole
  at this binary, in SB-75's shape: 660
  cells over 24 team repositories of 152
  commits each, 3,648 commits in all, the
  per-month composition proven from git
  itself before any cell runs, no
  failures. Every repository at every
  month, 576 statement cells compared byte
  for byte between the shipped CLI and
  the oracle, unpriced and priced with
  that repository's own card, each
  document checked on its own counts,
  commits_read 152 the whole read, three
  joined items, none unjoined, items_priced
  3 on the priced documents. The price
  books, 30 cards on the file and
  /dev/stdin surfaces against one fixed
  repository and month, 60 cells: 18 valid
  cards (currency case and currency
  varied, zero rates, leading zeros,
  one-digit and tiny fractions, integer
  rates, spacing and key order, integer
  parts of 15 and 29 digits and exactly 30
  digits with the 30-digit part on each
  class in turn, 30-digit with fraction,
  and leading zeros running to a 30-digit
  part) and 12 planted refusals (duplicate
  keys at the top level, inside per_item,
  inside a class's object value, and inside
  an object nested in an array value, an
  unknown top-level key, an unknown class,
  two bad currency forms, a number-typed
  rate, a 31-digit integer part, and two
  malformed rate forms), compared exit for
  exit and byte for byte across
  implementations and across surfaces, each
  card's outcome matching its planted
  class. The output-file cells, one month
  per repository on both implementations,
  24 cells: the landed bytes compared
  across implementations and against each
  implementation's own stdout bytes for
  the same month. The differential harness
  runs fresh at 14 cases and 240 steps
  with no failures. The flighttape pin
  re-confirmed: commit
  74a83298aeb31c82e62b9fb0c613f5bc8600799e
  stands on the host with tree
  afd4d7d13520411e003762548fc79082a1b9cf6e
  carrying exactly 50 blobs untruncated,
  and the golden fixture digests to
  cc9c4e01fca418dcc33114caf75fae1f6bf0d401
  f297b5d61f6776903fa955a3 with verify
  accepting, executed at the pinned
  checkout.
- SB-89. Round 13 closes. The round
  re-proved the whole surface fresh at
  this binary: PR 16 read back whole on
  its closed record under SB-84, the
  boundary battery at 11 cells under
  SB-85, the draw at its fourteen named
  cells under SB-86, the counting joints
  at nine cells under SB-87, and the
  realistic battery at 660 cells with the
  differential harness and the flighttape
  pin under SB-88. The full gate runs
  clean at this tree: cargo test 25 (23
  integration in tests/test_statement.rs
  and 2 unit in the library target),
  clippy under --all-targets with
  warnings denied, rustfmt --check, and
  the codepoint scan runs clean over the
  tree and over every blob at the head.
  SB-65a stands carried open against
  SB-11 exactly as SB-83 left it,
  re-measured fresh on both
  implementations this round under SB-85:
  a single link inside the repository as
  the output filename, pointing outside,
  passes the guard, and the write lands
  inside the repository replacing the
  link, the outside file unwritten.
  Thirteen of one hundred rounds stand
  closed.
- SB-90. Repair record, appended after this
  round's reviews and standing outside the
  round's item range: the standards
  review's Minor 2 on SB-88. SB-88 states
  its 18 valid cards as a category list
  whose per-category multiplicities are
  nowhere pinned: the enumeration does not
  pin the multiplicities, and the prose
  expands to 16 under a uniform
  one-phrase-one-card reading at a stated
  18. The count is pinned here from the
  generator itself, the VALID_CARDS list in
  the build lane's realistic battery
  script, read directly and counted entry
  by entry. The 18 valid cards, in the
  generator's order: (1) baseline, the
  unvaried card, USD with revert 10.00,
  fix-forward 5.00, redone 2.50; (2)
  currency-lower, the currency case lowered
  to usd, the rates standing; (3)
  currency-mixed-eur, the currency itself
  changed and its case mixed, Eur with
  revert 3.00; (4) zero-rates, every class
  at zero, GBP with revert 0.00,
  fix-forward 0, redone 0.0; (5)
  zero-one-class, revert alone at 0.00, the
  other classes standing; (6)
  leading-zeros, leading zeros on every
  class, 007.50, 05.00, and 02.50; (7)
  one-digit-fractions, one fraction digit
  on every class, JPY with 80.5, 5.5, and
  2.5; (8) tiny-fractions, 0.01, 0.07, and
  0.09; (9) integer-rates, no fraction on
  any class, CHF with 150, 80, and 7; (10)
  spacing, the baseline card's values with
  the spacing around keys and values
  varied; (11) key-order, per_item set
  before currency and the classes reordered
  within it, AUD; (12) int-15, an integer
  part of 15 digits on revert,
  123456789012345.00; (13) int-29, an
  integer part of 29 digits on revert,
  99999999999999999999999999999.09; (14)
  int-30-revert, an integer part of exactly
  30 digits on revert, bare; (15)
  int-30-fix, an integer part of exactly 30
  digits on fix-forward, carrying .99; (16)
  int-30-redone, an integer part of exactly
  30 digits on redone, bare; (17)
  int-30-fraction, an integer part of
  exactly 30 digits carrying a fraction, on
  redone,
  123456789012345678901234567890.99; (18)
  int-30-padded, leading zeros running to a
  30-digit integer part on revert,
  000009999999999999999999999999. The
  phrases resolve against this list: the
  baseline card is named by no phrase in
  SB-88's list, the variations being stated
  as variations from it; the currency case
  and the currency itself are two cards,
  (2) and (3); zero rates is two cards, (4)
  and (5); one-digit and tiny fractions are
  (7) and (8); spacing and key order are
  (10) and (11); the 30-digit part on each
  class in turn is three cards, (14), (15),
  and (16); and the two descriptors SB-88
  adds to SB-81's list, 30-digit with
  fraction and leading zeros running to a
  30-digit part, are one card each, (17)
  and (18). The 12 planted refusals
  enumerate exactly from SB-88's prose and
  are not restated. No total moves: 18
  valid cards and 12 refusals are the 30
  cards, the 30 cards on two surfaces are
  the 60 price-book cells, and the 660
  realistic cells and the 694 measured
  cells stand as SB-88 and SB-89 record
  them. SB-88's text stands unamended; this
  record supplies the multiplicities its
  enumeration left unpinned.
- SB-91. PR 17's read-back, in full, read
  through the API on 2026-11-14: no reviews, no
  review comments, and 4 issue comments, every
  body read to its end, and the body read to its
  end including the owner's repair note appended
  to it and the bot's auto-appended release
  notes standing in it. The issue comments are
  the bot's shell 6864385350, the owner's
  trigger 6864388960, the bot's auto-reply
  6864466078, and the landing comment
  6865090387. The shell in its true final form,
  moment-bounded at this read: 10,973 characters
  by a Python count over the API body, created
  2026-11-13T20:40:45Z and last edited
  2026-11-13T21:08:16Z, and no rate-limit banner
  stands anywhere in it. It states that no
  actionable comments were generated in the
  recent review; its walkthrough covers SB-84 to
  SB-89 over CONTRACT.md and README.md with the
  change assessment pinned to the reviewed head
  93d9decd9a7386f634bbc64eb18721b5cdb1be17 and
  an estimated effort of 2; its merge risk reads
  Minimal up to 93d9d; its pre-merge table lists
  four passed checks, Title check, Linked Issues
  check, Out of Scope Changes check, and
  Description Check, with the word Docstring
  occurring zero times in it; and its recent
  review section names the reviewed range as the
  base afe62af29a0655511630fe32d2c7f0bcb2594c90
  to the reviewed head 93d9decd over CONTRACT.md
  and README.md, run ID
  260510f2-ca2d-4a70-8ebf-9021bdec7e98, the
  included allowance used and none remaining.
  The earlier counts stand on the record at
  their own reads: the step 0 disposition in the
  body reads the shell at 10,965 characters with
  the last edit then at 2026-11-13T20:42:24Z,
  and the landing comment reads a two-character
  drift to 10,967; the shell's last edit on the
  record, at 2026-11-13T21:08:16Z, moved no
  disposition fact, the no-actionable line, the
  four passed checks, and the run ID standing as
  the disposition states. The figures this
  round's brief carried for the shell, 11,001
  characters at the build lane's read and 11,005
  at the reviews' reads with the last edit at
  2026-11-13T19:34:59Z, stand corrected here: no
  state of this shell carries them, and the last
  edit they name predates the pull request's own
  creation; the shell stands as contracted
  above. Step 0's coverage on PR 17 is the
  reviewed head alone, as the disposition
  states: the bot never reviewed the repair head
  a09fd9f6404954f5e54659ac1fe50cc93375a742, and
  the trigger drew the auto-reply in its exact
  form, action not completed, already reviewed
  the last commit. No late CodeRabbit finding
  stands on PR 17: no review or review comment
  exists on it at all, nothing stands on the
  record after the landing comment, and the
  shell's final form carries no actionable
  comment, so nothing stands undisposed. The
  owner's body edits stand read as part of the
  closed record and are graded here: the
  checklist item 8 repair stands as repaired,
  the badge item recording that no rendered
  inspection of the badge stands in that round's
  record and item 10 of the same body recording
  the same for the README, the two standing in
  agreement; and the repair note appended to the
  body records the standards review's two
  Minors, the SB-90 repair appended on tree
  d915e03d4749aee243a2c898b626d9f6b630e8bc with
  SB-89's text byte-identical, and the delta
  reviews owed on the repair head. SB-90's
  enumeration repair is graded as repaired, from
  the generator's list as SB-90 states it
  against SB-88's prose: the 18 valid cards
  stand enumerated card for card in the
  generator's order, and SB-88's phrases resolve
  against that list exactly, the baseline card
  named by no phrase in SB-88's list, the
  currency case and the currency itself two
  cards, (2) and (3), zero rates two cards, (4)
  and (5), leading zeros (6), one-digit and tiny
  fractions (7) and (8), integer rates (9),
  spacing and key order (10) and (11), integer
  parts of 15 and of 29 digits (12) and (13),
  the 30-digit part on each class in turn three
  cards, (14), (15), and (16), 30-digit with
  fraction (17), and leading zeros running to a
  30-digit part (18); the phrases under a
  uniform one-phrase-one-card reading expand to
  16, as SB-90 states, and the 12 planted
  refusals enumerate exactly from SB-88's prose,
  duplicate keys at four placements, an unknown
  top-level key, an unknown class, two bad
  currency forms, a number-typed rate, a
  31-digit integer part, and two malformed rate
  forms. No total moves: 18 valid cards and 12
  refusals are the 30 cards, the 30 cards on two
  surfaces are the 60 price-book cells, and the
  660 realistic cells and the 694 measured cells
  stand as SB-88 and SB-89 record them. The
  Round 13 generator script itself stood in that
  lane's temporary scratch and does not survive;
  the enumeration is graded from the contract
  texts here, and this round's realistic battery
  is built to SB-90's list card for card and
  re-proves each card's planted outcome under
  SB-95. The route beside the boundary figures,
  per SB-83's charge: SB-78's oracle figures at
  990, 991, and 992 splices stand measured at
  the oracle's module route, python3 -m
  muda.cli; SB-92 re-runs the boundary at the
  module route. The SB-65a carry stands open by
  name against SB-11 exactly as SB-85 and SB-89
  record it: SB-11 is not amended, no behavior
  changes, the landing is not closed, and no
  direction call stands on the record since the
  landing comment, nothing standing after it on
  PR 17 and no issue standing open on the
  repository; SB-92 re-measures the cell fresh
  on both arms.
- SB-92. The two-laws boundary re-probed fresh
  at this binary against the verified oracle, at
  exact points by path shape, the two laws never
  conflated, the oracle at its module route
  (python3 -m muda.cli) for every figure below:
  eleven cells, no failures. In the
  spelled-parent shape, the chain standing in
  the output's parent path with the output
  spelled through it: the guard's hand walk
  refuses onto-repo chains at 990 and 991
  splices with the guard's line on both
  implementations, and at 992 the walk aborts
  into the crash class (the port silent at exit
  1 with both streams empty, the oracle raising
  RecursionError); and the kernel's traversal of
  the spelled parent decides the physical write
  at its exact points, an away chain at 40
  splices landing on both implementations and at
  41 splices refused on both with the standing
  cannot-write line, nothing landing. In the
  head-is-output shape, the chain's head link
  standing as the output filename itself inside
  one real directory, which only the resolver
  traverses: chains at 40 and at 41 splices both
  land on both implementations, the write
  replacing the head link and the chain's target
  file left unwritten, a chain at 991 splices
  lands the same way, a chain of 991 splices
  onto the repository draws the guard's refusal
  with the head link left standing, and at 992
  splices the walk aborts into the crash class
  with the head link untouched. The SB-65a cell
  re-measured fresh in the same battery: a
  single symbolic link standing inside the read
  repository as the output filename, pointing at
  an outside file, passes the guard and the
  write lands inside the repository replacing
  the link on both implementations, the outside
  file left unwritten. The posture SB-65a
  contracts stands exactly as contracted, and
  the carry against SB-11 stands open by name:
  SB-11 is not amended, no behavior changes, and
  the landing is not closed.
- SB-93. The draw re-probed fresh at this
  binary, in SB-73's shape, under exactly the
  fourteen names SB-77 enumerates: fourteen
  cells, no failures. A model of the draw
  written from the contract text is pinned
  against the four unit vectors in src/cli.rs
  before any cell runs (SB-56). The plain
  control lands byte-identical under no
  interposition. Under entropy refusal in both
  arms, the device open refused and a short read
  of five bytes, the port refuses on the
  standing cannot-write line before any create
  and the directory is left as seeded; the
  oracle under both refusal arms lands its
  statement, the contracted split standing,
  because its draw never opens the device at
  statement time. The clock-only plants stand
  inert at both of Round 8's readings, the
  port's write landing under a frozen clock at
  pid 1 at the first reading and seated at pid
  250 at the second, and the oracle under the
  frozen clock at pid 1 with the same 64 planted
  also lands. The positive control is bracketed
  at both ends: with the entropy block fixed to
  a known nonzero value the 64 candidates are
  computable, planting all 64 refuses the write,
  and removing the first planted candidate lands
  the write, as removing the last (the 64th)
  lands it; with the block zeroed the fold is
  zero, the candidate sequence is the clock-only
  sequence again, and its plants refuse. The
  pre-epoch posture, at a clock frozen 100
  seconds before the epoch: with no plants the
  write lands byte-identical, and with the 64
  candidates computed from the constant seed
  term planted under the fixed block, the write
  refuses on the standing line. The pre-epoch
  draw is therefore the constant-seed draw
  folded with entropy, as SB-73 contracts.
- SB-94. The counting joints re-probed fresh at
  this binary, in SB-74's shape with the rate
  bound run on both surfaces: nine cells, no
  failures. A priced statement never counts:
  over one fixture (a revert, a fix-forward, and
  a redone item, all three joined), the priced
  document on both implementations is the
  unpriced counts document with the priced
  section added and nothing else moved, totals
  standing identical between the priced and
  unpriced documents, items_priced counting only
  the three joined items, and the priced total
  closing at 13.43 under the fixture card
  (revert 10.00, fix-forward 2.43, redone 1.00,
  the three rates over one joined item per
  class). Duplicate keys refuse by name at a
  nested level on both surfaces that exist, the
  file surface and the /dev/stdin surface: a key
  repeated inside a class's object value inside
  per_item refuses as names amount twice, and a
  key repeated inside an object nested in an
  array value under an unknown key refuses as
  names k twice, byte-identically on both
  implementations and across surfaces, four
  cells. A rate's integer part at exactly 30
  digits prices on both implementations on both
  surfaces, and at 31 digits refuses under the
  form line naming the rate as written on both,
  four cells.
- SB-95 (Round 14). The realistic battery at Round 13's shape, the differential harness fresh, and the flighttape pin. The realistic battery ran its standing corpus: 24 team repositories carrying full 2026 histories in SB-81's composition (12 commits in an ordinary month, 14 in a quarter month with its merge and side commit; 152 commits per repository, 3,648 in all), the per-month composition proven from git itself per repository and month before any cell ran. Against that corpus the battery measured 660 cells with no failures: 288 unpriced statements and 288 statements priced with each repository's own card, compared byte for byte between the shipped CLI and the oracle and checked on their own counts (152 commits read, three joined items, none unjoined, three priced); 60 price-book cells (the 30 cards as SB-90 enumerates them: 18 valid, 12 refusals; each on the file and /dev/stdin surfaces); and 24 output-file cells whose landed bytes matched the stdout bytes. The differential harness ran fresh against the same two implementations: 14 cases, 240 steps, no failures. The flighttape pin was re-confirmed: a tree read through the GitHub API at commit 74a83298aeb31c82e62b9fb0c613f5bc8600799e returned tree afd4d7d13520411e003762548fc79082a1b9cf6e with exactly 50 blobs, not truncated; a checkout of the pin built under the local toolchain and the golden digest executed: converting the synthetic session fixture and digesting the tape printed exactly cc9c4e01fca418dcc33114caf75fae1f6bf0d401f297b5d61f6776903fa955a3, and verify accepted it. With the 34 boundary, draw, and joint cells of SB-92 to SB-94, the round's measured cells total 694, closing arithmetically against Round 13's 694 redone in shape.
- SB-96 (Round 14). The local gate and the closure of Round 14. The gate ran on the final tree of this round before this item was appended: cargo test passes 25 tests (23 integration tests in tests/test_statement.rs and 2 unit tests in the library target), cargo clippy --all-targets with warnings denied is clean, and cargo fmt --check is clean. The oracle verification stands as SB-91 records it: the final Python tree read through the GitHub API at commit 918dc7eb13bd3a5b0c3e644147170ad0bb1607e8, the pristine copy blob-identical on all seven Python files, and the driven copy equal to the pristine copy in the form SB-91 states; the binary provenance stands as SB-92 records it. A Python codepoint scan over the final tree counts no U+2014 and no U+2013. SB-65a stands carried open by name against SB-11, reproduced fresh this round at SB-93 and undecided; SB-47 and SB-43 stand carried by name, untouched, their questions routed to the main chat with owner recommendations. This round's scratch is durable under ~/workspace/retro-line/sb14-scratch/ (the work tree, the target directories, the battery fixtures and journal, and the battery and harness drivers and logs); the differential harness's own fixture root is /tmp by its design, as in Round 13; two detached launches of the realistic battery driver died without output under the box's process teardown before the completed run, and only the completed run's accounting stands in SB-95; during one write-fault wave this round's measurements paused and resumed when git writes probed healthy, and no measurement was taken in a faulted state. With this item the README rounds line moves to 14 of 100.
- SB-97 (Round 14). A repair record, on step 0's finding. SB-96's closing citations are corrected against the items as they stand. First, the fresh SB-65a re-measurement stands at SB-92, in its boundary battery, not at SB-93 as SB-96 states: SB-93 is the draw item and carries no SB-65a cell; SB-96's sentence reads with SB-92 in SB-93's place, and SB-65a's carried posture is unchanged. Second, SB-92 records no binary provenance, so SB-96's citation of it for provenance does not stand; the provenance stands as measured this round and is stated here directly: the batteries drove a fresh debug build from the verified base tree, and its md5, 35555e1f592a2477b218403a8648dfbe, is exactly the binary md5 the Round 12 and Round 13 records carry, reproduced again from the fresh build. Third, SB-91 records no pristine-and-driven accounting, so SB-96's citation of it for the oracle verification does not stand either; that verification stands as performed this round and is stated here directly: the final Python tree was read through the GitHub API at commit 918dc7eb13bd3a5b0c3e644147170ad0bb1607e8, the pristine copy is blob-identical to that tree on all seven Python files, and the driven copy equals the pristine copy with history.py and statement.py byte-identical, cli.py differing only in the two rename spots (the program name and the guard line), and __init__.py only in its docstring line. SB-96's remaining statements stand as written. The correction was raised by step 0 on PR 18, review comment 4571969613, and each limb was verified against the record before this item was written.
