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
- SB-98. PR 18's read-back, in full, read
  through the API on 2026-11-14: one review, one
  review comment, and 4 issue comments, every
  body read to its end, and the body read to its
  end including the owner's repair note standing
  in its Known deviations and the bot's
  auto-appended release notes standing in it.
  The issue comments are the owner's trigger
  6873125953, the bot's shell 6873127077, the
  bot's auto-reply 6873180081, and the landing
  comment 6873415738. The review is 5902302495,
  standing COMMENTED with one actionable comment
  posted, and its one review comment is
  4571969613 on CONTRACT.md, which carries no
  reply. The shell in its true final form,
  moment-bounded at this read: 8,665 characters
  by a Python count over the API body, created
  2026-11-14T06:28:39Z and last edited
  2026-11-14T06:50:44Z, and no rate-limit banner
  stands anywhere in it. Its walkthrough covers
  SB-91 to SB-96 over CONTRACT.md and README.md
  with the change assessment pinned to the
  closure head
  fa4f85b5446f7425e421076e53de10a558616174 and
  an estimated effort of 1; its merge risk reads
  Minimal up to fa4f8 and states that one
  cross-reference in the closure record should
  be corrected; its pre-merge table lists four
  passed checks, Title check, Linked Issues
  check, Out of Scope Changes check, and
  Description Check, the Title check passed
  carrying prose that the title's range names
  SB-97, which the body's objectives do not
  describe; the word Docstring occurs zero times
  in it; and it carries no Run ID and no
  recent-review status line of its own, the Run
  ID 7b3fae88-8fb2-407e-9093-c75a249d3077
  standing in the review body. The earlier
  counts stand on the record at their own reads:
  the step 0 disposition in the body reads the
  shell at 8,659 characters with the last edit
  then at 2026-11-14T06:30:59Z, and the landing
  comment reads 8,663 characters with the last
  edit at 2026-11-14T06:39:01Z; the shell's last
  edit on the record, at 2026-11-14T06:50:44Z,
  precedes the landing comment and moved no
  disposition fact, the walkthrough, the merge
  risk, and the four passed checks standing as
  the disposition states. Step 0's coverage on
  PR 18 is the closure head fa4f85b544 alone:
  the bot never reviewed the repair head
  1147089a94bea24087d16d1429c435a4f715b8ea, and
  the re-trigger drew the auto-reply 6873180081
  in its exact form, action not completed,
  already reviewed the last commit, which
  carries no finding and no correction of any
  closed disposition. Step 0's one actionable
  finding, review comment 4571969613, stood
  VALID on all three of its limbs when checked
  against the record, and it stands repaired,
  not dismissed: SB-96 cited SB-93 for the fresh
  SB-65a re-measurement where the re-measurement
  stands at SB-92, SB-93 carrying the draw and
  no SB-65a cell; SB-96 cited SB-92 for the
  binary provenance, and SB-92 records the
  boundary battery and the oracle's module route
  and no provenance; and SB-96 cited SB-91 for a
  pristine-and-driven oracle accounting, and
  SB-91 records none. Repair record SB-97 is
  graded as repaired, each limb verified against
  the items as they stand in this tree: SB-97
  stands appended after SB-96 with SB-96
  unrewritten, it corrects the three citations
  one by one, and it states the provenance and
  the oracle verification directly, the fresh
  build's md5 35555e1f592a2477b218403a8648dfbe
  reproduced again this round from a fresh build
  of the verified base tree, and the oracle read
  through the GitHub API at commit
  918dc7eb13bd3a5b0c3e644147170ad0bb1607e8 with
  the pristine copy blob-identical on all seven
  Python files and the driven copy equal to it
  in the rename form, verified firsthand again
  this round, the accounting standing at SB-103.
  The owner's repair of the body's Known
  deviations line is graded as repaired from the
  body as it stands: the line now cites SB-92
  for the SB-65a re-measurement and carries the
  owner's repair note on its face, recording
  that the line first cited SB-93, the same
  mis-citation SB-97 corrects in SB-96; the edit
  changed no tree, the merge standing at tree
  269e0277044cb4fd82949191110507a7509d69e3. No
  late CodeRabbit finding stands on PR 18:
  nothing stands on the record after the landing
  comment, no review or review comment stands
  beyond the one review and its one comment, and
  no issue stands open on the repository, so
  nothing stands undisposed. The carries stand
  open by name exactly as the charges direct:
  SB-65a against SB-11, SB-11 not amended and no
  behavior changed, re-measured fresh on both
  arms this round at SB-99; and SB-47 and SB-43,
  the description surfaces, untouched, their
  questions routed to the main chat with owner
  recommendations.
- SB-99. The two-laws boundary re-probed fresh
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
- SB-100. The draw re-probed fresh at this
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
- SB-101. The counting joints re-probed fresh at
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
- SB-102. The realistic battery at Round 14's
  shape, the differential harness fresh, and the
  flighttape pin. The realistic battery ran its
  standing corpus: 24 team repositories carrying
  full 2026 histories in SB-81's composition (12
  commits in an ordinary month, 14 in a quarter
  month with its merge and side commit; 152
  commits per repository, 3,648 in all), the
  per-month composition proven from git itself
  per repository and month before any cell ran.
  Against that corpus the battery measured 660
  cells with no failures: 288 unpriced
  statements and 288 statements priced with each
  repository's own card, compared byte for byte
  between the shipped CLI and the oracle and
  checked on their own counts (152 commits read,
  three joined items, none unjoined, three
  priced); 60 price-book cells (the 30 cards as
  SB-90 enumerates them: 18 valid, 12 refusals;
  each on the file and /dev/stdin surfaces); and
  24 output-file cells whose landed bytes
  matched the stdout bytes. The differential
  harness ran fresh against the same two
  implementations: 14 cases, 240 steps, no
  failures. The flighttape pin was re-confirmed:
  a tree read through the GitHub API at commit
  74a83298aeb31c82e62b9fb0c613f5bc8600799e
  returned tree
  afd4d7d13520411e003762548fc79082a1b9cf6e with
  exactly 50 blobs, not truncated; a checkout of
  the pin built under the local toolchain and
  the golden digest executed: converting the
  synthetic session fixture and digesting the
  tape printed exactly
  cc9c4e01fca418dcc33114caf75fae1f6bf0d401f297b5d61f6776903fa955a3,
  and verify accepted it. With the 34 boundary,
  draw, and joint cells of SB-99 to SB-101, the
  round's measured cells total 694, closing
  arithmetically against Round 14's 694 redone
  in shape.
- SB-103. The local gate and the closure of
  Round 15. The gate ran on the final tree of
  this round before this item was appended:
  cargo test passes 25 tests (23 integration
  tests in tests/test_statement.rs and 2 unit
  tests in the library target), cargo clippy
  --all-targets with warnings denied is clean,
  and cargo fmt --check is clean. The oracle
  verification stands as performed this round
  and is stated here directly: the final Python
  tree was read through the GitHub API at commit
  918dc7eb13bd3a5b0c3e644147170ad0bb1607e8, tree
  f331cd155f507c7013ccd6a3f5157670d80201f5, 13
  blobs, not truncated; the pristine copy is
  blob-identical to that tree on all seven
  Python files, and the driven copy equals the
  pristine copy with history.py and statement.py
  byte-identical, cli.py differing only in the
  two rename spots (the program name and the
  guard line), and __init__.py only in its
  docstring line. The binary provenance stands
  as measured this round and is stated here
  directly: the batteries drove a fresh debug
  build from the verified base tree, and its
  md5, 35555e1f592a2477b218403a8648dfbe, is
  exactly the binary md5 the Round 12, Round 13,
  and Round 14 records carry, reproduced again
  from the fresh build. A Python codepoint scan
  over the final tree counts no U+2014 and no
  U+2013. SB-65a stands carried open by name
  against SB-11, reproduced fresh this round at
  SB-99 and undecided; SB-47 and SB-43 stand
  carried by name, untouched, their questions
  routed to the main chat with owner
  recommendations. This round's scratch is
  durable under
  ~/workspace/retro-line/sb15-scratch/ (the work
  tree built from the base tree read through the
  API and verified blob for blob, the target
  directories, the battery fixtures and journal,
  and the battery and harness drivers and logs);
  the differential harness's own fixture root is
  a temporary directory under
  ~/workspace/retro-line by its design; the
  realistic battery completed on its first
  detached launch this round, and only the
  completed run's accounting stands in SB-102.
  With this item the README rounds line moves to
  15 of 100.
- SB-104. PR 19's read-back, in full, read
  through the API on 2026-11-14: no review, no
  review comment, and 4 issue comments, every
  body read to its end, and the body read to its
  end including the bot's auto-appended release
  notes standing in it. The issue comments are
  the bot's shell 6877688854, the owner's
  trigger 6877692407, the bot's auto-reply
  6877752204, and the landing comment
  6878121599. The identifiers the owner's brief
  cited for this census (trigger 6877004730,
  shell 6877005812, auto-reply 6877053508,
  review 5904213373) stand nowhere on PR 19 at
  this read; the census as it stands is stated
  here from the live record, and the landing
  comment's own citations agree with it. The
  shell in its true final form, moment-bounded
  at this read: 12,377 characters by a Python
  count over the API body, created
  2026-11-14T13:06:29Z and last edited
  2026-11-14T13:21:32Z, and no rate-limit banner
  stands anywhere in it. Its walkthrough covers
  SB-98 to SB-103 over CONTRACT.md and README.md
  with the change assessment pinned to the
  closure head
  0e5e6e08f06b5f31e94b318e8d2acce7ff1a2eba and
  an estimated effort of 2; its merge risk reads
  Minimal up to 0e5e6 and states the pull
  request only updates project documentation;
  its pre-merge table lists three passed checks,
  Linked Issues check, Out of Scope Changes
  check, and Description Check, and the Title
  check inconclusive; its own recent-review line
  stands that no actionable comments were
  generated in the recent review; the word
  Docstring occurs zero times in it; and it
  carries its Run ID
  fd65445d-970e-43be-b838-5f52d320d491 in its
  recent review info, with the included
  allowance used and none remaining. The earlier
  counts stand on the record at their own reads:
  the step 0 disposition in the body reads the
  shell at 12,461 characters with the last edit
  then at 2026-11-14T13:10:03Z; the shell's last
  edit on the record, at 2026-11-14T13:21:32Z,
  shortened it by 84 characters to the form
  contracted here and precedes the landing
  comment, and the landing comment's account of
  the reviews' reads agrees with the form
  contracted here, so no disposition fact moved.
  Step 0's coverage on PR 19 is the closure head
  0e5e6e08f0 alone, the branch carrying no other
  head; the bot's review opened on the pull
  request's creation, the shell standing 18
  seconds before the trigger on the record, and
  the trigger drew the auto-reply 6877752204 in
  its exact form, action not completed, already
  reviewed the last commit, which carries no
  finding and no correction of any closed
  disposition. Step 0 raised no actionable
  finding on PR 19, and one reservation stands
  dismissed with counted grounds, graded here
  limb by limb: the Title check stood
  inconclusive, calling the title a partial
  match because it names the item range and the
  round without a descriptive phrase; the title
  (SB-98 to SB-103: Round 15) carries this
  repository's standing form, identical in shape
  to PR 18's title (SB-91 to SB-97: Round 14)
  read through the API at this read; the range
  it names is the range the body discharges; and
  the check's own full details concede the title
  is not off-topic and refers to the real
  changeset, so the reservation is a
  descriptiveness preference against the
  standing form, not a defect in the record, and
  the dismissal stands sound. The walkthrough's
  own labels stand counted at their full scope:
  the lagging Round 14 label stands in three
  places in the shell, its prose summary (the
  pull request adds Round 14 measurement records
  and a Round 15 closure record), its changes
  table heading (Round 14 findings and
  measurements), and its merge risk prose (the
  Round 14 and 15 measurement records); the
  body's disposition addressed the walkthrough's
  compression of SB-98 as a review finding and
  repair as the bot's summary wording, SB-98
  being the read-back item and stating the
  finding and the repair as PR 18's, and the
  fuller count of the label moves no
  disposition, the shell's own change assessment
  pinning the correct base and head and its
  Title check naming Round 15, bot summary
  wording only with no check or disposition
  turning on it. PR 19's closure states its
  provenance and its oracle accounting directly
  at SB-103, the SB-97 lesson applied at the
  source, and no repair record was needed in
  Round 15. No late CodeRabbit finding stands on
  PR 19: nothing stands on the record after the
  landing comment, no review or review comment
  stands at all, and no issue stands open on the
  repository, so nothing stands undisposed. The
  carries stand open by name exactly as the
  charges direct: SB-65a against SB-11, SB-11
  not amended and no behavior changed,
  re-measured fresh on both arms this round at
  SB-105; and SB-47 and SB-43, the description
  surfaces, untouched, their questions routed to
  the main chat with owner recommendations.
- SB-105. The two-laws boundary re-probed fresh
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
- SB-106. The draw re-probed fresh at this
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
- SB-107. The counting joints re-probed fresh at
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
- SB-108. The realistic battery at Round 15's
  shape, the differential harness fresh, and the
  flighttape pin. The realistic battery ran its
  standing corpus: 24 team repositories carrying
  full 2026 histories in SB-81's composition (12
  commits in an ordinary month, 14 in a quarter
  month with its merge and side commit; 152
  commits per repository, 3,648 in all), the
  per-month composition proven from git itself
  per repository and month before any cell ran.
  Against that corpus the battery measured 660
  cells with no failures: 288 unpriced
  statements and 288 statements priced with each
  repository's own card, compared byte for byte
  between the shipped CLI and the oracle and
  checked on their own counts (152 commits read,
  three joined items, none unjoined, three
  priced); 60 price-book cells (the 30 cards as
  SB-90 enumerates them: 18 valid, 12 refusals;
  each on the file and /dev/stdin surfaces); and
  24 output-file cells whose landed bytes
  matched the stdout bytes. The differential
  harness ran fresh against the same two
  implementations: 14 cases, 240 steps, no
  failures. The flighttape pin was re-confirmed:
  a tree read through the GitHub API at commit
  74a83298aeb31c82e62b9fb0c613f5bc8600799e
  returned tree
  afd4d7d13520411e003762548fc79082a1b9cf6e with
  exactly 50 blobs, not truncated; a checkout of
  the pin built under the local toolchain and
  the golden digest executed: converting the
  synthetic session fixture and digesting the
  tape printed exactly
  cc9c4e01fca418dcc33114caf75fae1f6bf0d401f297b5d61f6776903fa955a3,
  and verify accepted it. With the 34 boundary,
  draw, and joint cells of SB-105 to SB-107, the
  round's measured cells total 694, closing
  arithmetically against Round 15's 694 redone
  in shape.
- SB-109. The local gate and the closure of
  Round 16. The gate ran on the final tree of
  this round before this item was appended:
  cargo test passes 25 tests (23 integration
  tests in tests/test_statement.rs and 2 unit
  tests in the library target), cargo clippy
  --all-targets with warnings denied is clean,
  and cargo fmt --check is clean. The oracle
  verification stands as performed this round
  and is stated here directly: the final Python
  tree was read through the GitHub API at commit
  918dc7eb13bd3a5b0c3e644147170ad0bb1607e8, tree
  f331cd155f507c7013ccd6a3f5157670d80201f5, 13
  blobs, not truncated; the pristine copy is
  blob-identical to that tree on all seven
  Python files, and the driven copy equals the
  pristine copy with history.py and statement.py
  byte-identical, cli.py differing only in the
  two rename spots (the program name and the
  guard line), and __init__.py only in its
  docstring line. The binary provenance stands
  as measured this round and is stated here
  directly: the batteries drove a fresh debug
  build from the verified base tree, and its
  md5, 35555e1f592a2477b218403a8648dfbe, is
  exactly the binary md5 the Round 12, Round 13,
  Round 14, and Round 15 records carry,
  reproduced again from the fresh build. A
  Python codepoint scan over the final tree
  counts no U+2014 and no U+2013. SB-65a stands
  carried open by name against SB-11, reproduced
  fresh this round at SB-105 and undecided;
  SB-47 and SB-43 stand carried by name,
  untouched, their questions routed to the main
  chat with owner recommendations. This round's
  scratch is durable under
  ~/workspace/retro-line/sb16-scratch/ (the work
  tree built from the base tree read through the
  API and verified blob for blob, the target
  directories, the battery fixtures and journal,
  and the battery and harness drivers and logs);
  the differential harness's own fixture root is
  a temporary directory under
  ~/workspace/retro-line by its design; the
  realistic battery completed on its first
  detached launch this round, and only the
  completed run's accounting stands in SB-108.
  With this item the README rounds line moves to
  16 of 100.
- SB-110. PR 20's read-back, in full, read
  through the API on 2026-11-15: no review, no
  review comment, and 4 issue comments, every
  body read to its end, and the body read to its
  end including the bot's auto-appended release
  notes standing in it. The issue comments are
  the owner's trigger 6885245435 (20 characters,
  created 2026-11-14T22:33:14Z and never
  edited), the bot's shell 6885246512, the bot's
  auto-reply 6885282425 (573 characters, created
  2026-11-14T22:36:22Z, in its exact form,
  action not completed, already reviewed the
  last commit, which carries no finding and no
  correction of any closed disposition), and the
  landing comment 6885576627 (2,550 characters,
  created 2026-11-14T23:01:22Z). The identifiers
  the owner's brief cited for this census stand
  verified against the live record exactly as
  cited, the brief's census having been sourced
  from the landing comment itself; no correction
  is owed this round. The shell in its true
  final form, moment-bounded at this read:
  11,294 characters by a Python count over the
  API body, created 2026-11-14T22:33:19Z and
  last edited 2026-11-14T22:38:55Z, unchanged
  since the Round 16 reviews' reads, and no
  rate-limit banner stands anywhere in it. Its
  walkthrough covers SB-104 to SB-109 over
  CONTRACT.md and README.md with the change
  assessment pinned to the closure head
  faa8d745d2061e6635f14f6c5e465855cf991fe4 and
  an estimated effort of 2; its merge risk reads
  Minimal up to faa8d and states the pull
  request adds Round 16 measurement and closure
  records to the contract and does not change
  runtime behavior; its pre-merge table lists
  four passed checks, Title check, Linked Issues
  check, Out of Scope Changes check, and
  Description Check; its own recent-review line
  stands that no actionable comments were
  generated in the recent review; the word
  Docstring occurs zero times in it; and it
  carries its Run ID
  9d6b9103-8139-4190-a506-fc4120d2447e in its
  recent review info, the review standing over
  the files changed between
  3b6ba4c7e410447b9fc45b947b491be9e2a829b1 and
  faa8d745d2061e6635f14f6c5e465855cf991fe4 with
  CONTRACT.md and README.md the two files
  selected, and the included allowance used with
  none remaining. The earlier count stands on
  the record at its own read: the step 0
  disposition in the body reads the shell at
  11,292 characters with the last edit then at
  2026-11-14T22:35:26Z; the shell's last edit on
  the record, at 2026-11-14T22:38:55Z,
  lengthened it by 2 characters (two added blank
  lines and the Description Check row
  re-rendered at a shifted position, as the
  landing comment states) to the form contracted
  here and precedes the landing comment, and the
  landing comment's account of the reviews'
  reads agrees with the form contracted here, so
  no disposition fact moved. Step 0's coverage
  on PR 20 is the closure head faa8d745 alone,
  the branch carrying no other head; the bot's
  review opened on the pull request's creation,
  the shell standing 5 seconds after the trigger
  on the record, and the trigger drew the
  auto-reply 6885282425. Step 0 raised no
  actionable finding on PR 20, and one
  reservation stands dismissed with counted
  grounds, graded here limb by limb: the Title
  check passed, carrying prose that the title is
  only partially descriptive because it does not
  say what the round did; the title (SB-104 to
  SB-109: Round 16) carries this repository's
  standing form, identical in shape to PR 19's
  title (SB-98 to SB-103: Round 15) and PR 18's
  title (SB-91 to SB-97: Round 14) read through
  the API at this read; the range it names is
  the range the body discharges; and the check's
  own full details concede the title is specific
  to the round and related to the main change,
  so the reservation is a descriptiveness
  preference against the standing form, not a
  defect in the record, and the dismissal stands
  sound. The bot's release-notes appendix at the
  body's foot stands censused in the whole-body
  read: inside its own markers its first bullet
  claims the round added "Round 15 and Round 16
  review, measurement, verification, and closure
  records", where the branch adds Round 16's
  records only (SB-104 to SB-109, the first of
  them the read-back of PR 19), and its second
  bullet, the README updated to reflect 16
  completed perfection rounds, is correct; the
  appendix is the bot's own text inside its own
  markers, its one error is the bot's, and no
  authored sentence adopts it, so it stands
  dispositioned at that scope and nothing turns
  on it. The shell's walkthrough carries no such
  overstatement at its full scope: the words
  Round 15 occur exactly once in it, naming the
  read-back's subject in its summary sentence,
  and its changes table names the PR 19
  read-back and Round 16's targeted measurements
  and battery and closure without a lagging
  label. No late CodeRabbit finding stands on PR
  20: nothing stands on the record after the
  landing comment, no review or review comment
  stands at all, and no issue stands open on the
  repository, so nothing stands undisposed. The
  carries stand open by name exactly as the
  charges direct: SB-65a against SB-11, SB-11
  not amended and no behavior changed,
  re-measured fresh on both arms this round at
  SB-111; and SB-47 and SB-43, the description
  surfaces, untouched, their questions routed to
  the main chat with owner recommendations.
- SB-111. The two-laws boundary re-probed fresh
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
- SB-112. The draw re-probed fresh at this
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
- SB-113. The counting joints re-probed fresh at
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
- SB-114. The realistic battery at Round 16's
  shape, the differential harness fresh, and the
  flighttape pin. The realistic battery ran its
  standing corpus: 24 team repositories carrying
  full 2026 histories in SB-81's composition (12
  commits in an ordinary month, 14 in a quarter
  month with its merge and side commit; 152
  commits per repository, 3,648 in all), the
  per-month composition proven from git itself
  per repository and month before any cell ran.
  Against that corpus the battery measured 660
  cells with no failures: 288 unpriced
  statements and 288 statements priced with each
  repository's own card, compared byte for byte
  between the shipped CLI and the oracle and
  checked on their own counts (152 commits read,
  three joined items, none unjoined, three
  priced); 60 price-book cells (the 30 cards as
  SB-90 enumerates them: 18 valid, 12 refusals;
  each on the file and /dev/stdin surfaces); and
  24 output-file cells whose landed bytes
  matched the stdout bytes. The differential
  harness ran fresh against the same two
  implementations: 14 cases, 240 steps, no
  failures. The flighttape pin was re-confirmed:
  a tree read through the GitHub API at commit
  74a83298aeb31c82e62b9fb0c613f5bc8600799e
  returned tree
  afd4d7d13520411e003762548fc79082a1b9cf6e with
  exactly 50 blobs, not truncated; a checkout of
  the pin built under the local toolchain and
  the golden digest executed: converting the
  synthetic session fixture and digesting the
  tape printed exactly
  cc9c4e01fca418dcc33114caf75fae1f6bf0d401f297b5d61f6776903fa955a3,
  and verify accepted it. With the 34 boundary,
  draw, and joint cells of SB-111 to SB-113, the
  round's measured cells total 694, closing
  arithmetically against Round 16's 694 redone
  in shape.
- SB-115. The local gate and the closure of
  Round 17. The gate ran on the final tree of
  this round before this item was appended:
  cargo test passes 25 tests (23 integration
  tests in tests/test_statement.rs and 2 unit
  tests in the library target), cargo clippy
  --all-targets with warnings denied is clean,
  and cargo fmt --check is clean. The oracle
  verification stands as performed this round
  and is stated here directly: the final Python
  tree was read through the GitHub API at commit
  918dc7eb13bd3a5b0c3e644147170ad0bb1607e8, tree
  f331cd155f507c7013ccd6a3f5157670d80201f5, 13
  blobs, not truncated; the pristine copy is
  blob-identical to that tree on all seven
  Python files, and the driven copy equals the
  pristine copy with history.py and statement.py
  byte-identical, cli.py differing only in the
  two rename spots (the program name and the
  guard line), and __init__.py only in its
  docstring line. The binary provenance stands
  as measured this round and is stated here
  directly: the batteries drove a fresh debug
  build from the verified base tree, and its
  md5, 35555e1f592a2477b218403a8648dfbe, is
  exactly the binary md5 the Round 12, Round 13,
  Round 14, Round 15, and Round 16 records
  carry, reproduced again from the fresh build.
  A Python codepoint scan over the final tree
  counts no U+2014 and no U+2013. SB-65a stands
  carried open by name against SB-11, reproduced
  fresh this round at SB-111 and undecided, and
  SB-11 itself stands carried open by name on
  the same terms; SB-47 and SB-43 stand carried
  by name, untouched, their questions routed to
  the main chat with owner recommendations. This
  round's scratch is durable under
  ~/workspace/retro-line/sb17-scratch/ (the work
  tree built from the base tree read through the
  API and verified blob for blob, the target
  directories, the battery fixtures and journal,
  and the battery and harness drivers and logs);
  the differential harness's own fixture root is
  a temporary directory under
  ~/workspace/retro-line by its design; the
  realistic battery completed on its first
  detached launch this round, and only the
  completed run's accounting stands in SB-114.
  With this item the README rounds line moves to
  17 of 100.
- SB-116. PR 21's read-back, in full, read
  through the API on 2026-11-15: no review, no
  review comment, and 2 issue comments, every
  body read to its end, and the body read to its
  end including the bot's auto-appended release
  notes standing in it. The issue comments are
  the bot's shell 6889418947 and the landing
  comment 6890202916 (2,277 characters, created
  2026-11-15T06:12:00Z and never edited), the
  landing comment standing last on the record.
  No trigger comment stands on the pull request:
  the bot's review opened on the pull request's
  creation on its own run, the shell created
  2026-11-15T04:54:08Z, twenty-two seconds after
  the pull request itself was created at
  2026-11-15T04:53:46Z, and no auto-reply
  stands. The identifiers the owner's brief
  cited for this census stand verified against
  the live record exactly as cited, the brief's
  census having been sourced from the landing
  comment itself; no correction is owed this
  round. The shell in its true final form,
  moment-bounded at this read: 11,463 characters
  by a Python count over the API body, created
  2026-11-15T04:54:08Z and last edited
  2026-11-15T04:58:06Z, unchanged since the
  Round 17 reviews' reads, and no rate-limit
  banner stands anywhere in it. Its walkthrough
  covers SB-110 to SB-115 over CONTRACT.md and
  README.md with the change assessment pinned to
  the closure head
  88148bdc28b37f20cdf4dc8652cf21833ab51811 and
  an estimated effort of 2; its merge risk reads
  Minimal up to 88148 and states the pull
  request adds Round 17 measurement and closure
  records to the contract and does not change
  runtime behavior; its pre-merge table lists
  four passed checks, Linked Issues check, Out
  of Scope Changes check, Description Check, and
  Title check, the Title check standing Passed
  in this final form; its own recent-review line
  stands that no actionable comments were
  generated in the recent review; the word
  Docstring occurs zero times in it; and it
  carries its Run ID
  2bf350a9-d311-4f4a-9ce1-22e9cee337cd in its
  recent review info, the review standing over
  the files changed between
  0cac99b4ad13369574c1f411ecdcb247ccd50601 and
  88148bdc28b37f20cdf4dc8652cf21833ab51811 with
  CONTRACT.md and README.md the two files
  selected, and the included allowance used with
  none remaining. The earlier form stands on the
  record at its own read: the step 0 disposition
  in the body reads the shell at 12,983
  characters with the last edit then at
  2026-11-15T04:55:55Z, three checks passed and
  the Title check inconclusive; the shell's last
  edit, at 2026-11-15T04:58:06Z, re-rendered it
  downward by 1,520 characters to the form
  contracted here, flipping the Title check to
  Passed with the milder partially related prose
  and restoring the four-passed table, and that
  edit precedes the landing comment, so no
  disposition fact moved: the reservation the
  body dismisses stands withdrawn by the bot
  itself in the final form. Step 0's coverage on
  PR 21 is the closure head 88148bdc alone, the
  branch carrying no other head. Step 0 raised
  no actionable finding on PR 21, and the body's
  one dismissal is re-graded here limb by limb
  against both of the shell's forms and stands
  sound against each: against the bounded form,
  the Title check stood inconclusive, calling
  the title a naming of the work items and the
  round that does not describe the main change
  and proposing a rewritten title in its place;
  the title (SB-110 to SB-115: Round 17) carries
  this repository's standing form, identical in
  shape to PR 20's title (SB-104 to SB-109:
  Round 16), PR 19's title (SB-98 to SB-103:
  Round 15), and PR 18's title (SB-91 to SB-97:
  Round 14) read through the API at this read;
  the range it names is the range the body
  discharges; and the check's own full details
  concede the title refers to the real scope of
  the pull request, so the reservation is a
  descriptiveness preference against the
  standing form, not a defect in the record.
  Against the final form, the check stands
  Passed and its prose is the milder partially
  related reservation, that the title names the
  commit range and the round but relies on
  internal identifiers and omits the main
  change; the dismissal's counted grounds hold
  against the milder prose on the same terms,
  and the reservation the body answered stands
  withdrawn by the bot itself, so no disposition
  moves. The bot's release-notes appendix at the
  body's foot stands censused in the whole-body
  read: inside its own markers its first bullet
  claims the round added "Round 16 review
  records, including comment handling, review
  and title-check outcomes, and the status of
  carried issues", compressing SB-110, which is
  Round 17's own read-back of PR 20's closed
  record, as Round 16 review records, where the
  branch adds Round 17's records only (SB-110 to
  SB-115, the first of them the read-back of PR
  20); its second bullet, the Round 17 records
  covering boundary, draw, pricing and counting,
  and battery measurements with verification
  results and round status, is correct, and its
  third bullet, the README updated to reflect 17
  perfection rounds closed, is correct; the
  appendix is the bot's own text inside its own
  markers, its one compression is the bot's, and
  no authored sentence adopts it, so it stands
  dispositioned at that scope and nothing turns
  on it. The shell's walkthrough carries no such
  compression at its full scope: the words Round
  16 occur exactly once in it, naming the
  read-back's subject in its summary sentence,
  and its changes table names the PR 20
  read-back and Round 17's boundary and draw
  measurements, counting and pricing and battery
  results, and closure without a lagging label.
  No late CodeRabbit finding stands on PR 21:
  nothing stands on the record after the landing
  comment, no review or review comment stands at
  all, and no issue stands open on the
  repository, so nothing stands undisposed. The
  carries stand open by name exactly as the
  charges direct: SB-65a against SB-11, SB-11
  not amended and no behavior changed,
  re-measured fresh on both arms this round at
  SB-117; and SB-47 and SB-43, the description
  surfaces, untouched, their questions routed to
  the main chat with owner recommendations.
- SB-117. The two-laws boundary re-probed fresh
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
- SB-118. The draw re-probed fresh at this
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
- SB-119. The counting joints re-probed fresh at
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
- SB-120. The realistic battery at Round 16's
  shape, the differential harness fresh, and the
  flighttape pin. The realistic battery ran its
  standing corpus: 24 team repositories carrying
  full 2026 histories in SB-81's composition (12
  commits in an ordinary month, 14 in a quarter
  month with its merge and side commit; 152
  commits per repository, 3,648 in all), the
  per-month composition proven from git itself
  per repository and month before any cell ran.
  Against that corpus the battery measured 660
  cells with no failures: 288 unpriced
  statements and 288 statements priced with each
  repository's own card, compared byte for byte
  between the shipped CLI and the oracle and
  checked on their own counts (152 commits read,
  three joined items, none unjoined, three
  priced); 60 price-book cells (the 30 cards as
  SB-90 enumerates them: 18 valid, 12 refusals;
  each on the file and /dev/stdin surfaces); and
  24 output-file cells whose landed bytes
  matched the stdout bytes. The accounting
  stands on the third detached launch: the box
  write fault killed the first launch in the
  corpus phase, on a fixture write in team-02's
  build that failed after the battery's own
  retries, and the second in team-15's build, on
  a corpus commit that did not land; both are
  disclosed here and counted nowhere, and the
  completed run resumed from the corpus
  checkpoints the earlier launches had banked,
  each repository's composition re-proven from
  git before any cell ran. The differential
  harness ran fresh against the same two
  implementations: 14 cases, 240 steps, no
  failures; its first launch died on a fixture
  write under the same fault and is counted
  nowhere, the completed run standing as the
  second. The flighttape pin was re-confirmed: a
  tree read through the GitHub API at commit
  74a83298aeb31c82e62b9fb0c613f5bc8600799e
  returned tree
  afd4d7d13520411e003762548fc79082a1b9cf6e with
  exactly 50 blobs, not truncated; a checkout of
  the pin built under the local toolchain and
  the golden digest executed: converting the
  synthetic session fixture and digesting the
  tape printed exactly
  cc9c4e01fca418dcc33114caf75fae1f6bf0d401f297b5d61f6776903fa955a3,
  and verify accepted it. With the 34 boundary,
  draw, and joint cells of SB-117 to SB-119, the
  round's measured cells total 694, closing
  arithmetically against Round 16's 694 redone
  in shape.
- SB-121. The local gate and the closure of
  Round 18. The gate ran on the final tree of
  this round before this item was appended:
  cargo test passes 25 tests (23 integration
  tests in tests/test_statement.rs and 2 unit
  tests in the library target), cargo clippy
  --all-targets with warnings denied is clean,
  and cargo fmt --check is clean. The oracle
  verification stands as performed this round
  and is stated here directly: the final Python
  tree was read through the GitHub API at commit
  918dc7eb13bd3a5b0c3e644147170ad0bb1607e8, tree
  f331cd155f507c7013ccd6a3f5157670d80201f5, 13
  blobs, not truncated; the pristine copy is
  blob-identical to that tree on all seven
  Python files, and the driven copy equals the
  pristine copy with history.py and statement.py
  byte-identical, cli.py differing only in the
  two rename spots (the program name and the
  guard line), and __init__.py only in its
  docstring line. The binary provenance stands
  as measured this round and is stated here
  directly: the batteries drove a fresh debug
  build from the verified base tree, and its
  md5, 35555e1f592a2477b218403a8648dfbe, is
  exactly the binary md5 the Round 12, Round 13,
  Round 14, Round 15, Round 16, and Round 17
  records carry, reproduced again from the fresh
  build. A Python codepoint scan over the final
  tree counts no U+2014 and no U+2013. SB-65a
  stands carried open by name against SB-11,
  reproduced fresh this round at SB-117 and
  undecided, and SB-11 itself stands carried
  open by name on the same terms; SB-47 and
  SB-43 stand carried by name, untouched, their
  questions routed to the main chat with owner
  recommendations. This round's scratch is
  durable under
  ~/workspace/retro-line/sb18-scratch/ (the work
  tree built from the base tree read through the
  API and verified blob for blob, the target
  directories, the battery fixtures and journal,
  and the battery and harness drivers and logs);
  the differential harness's own fixture root is
  a temporary directory under
  ~/workspace/retro-line by its design; the
  realistic battery completed on its third
  detached launch this round under the box write
  fault, the two killed launches disclosed at
  SB-120 and counted nowhere, and only the
  completed run's accounting stands in SB-120.
  With this item the README rounds line moves to
  18 of 100.
- SB-122. PR 22's read-back, in full, read
  through the API on 2026-11-15: no review, no
  review comment, and 2 issue comments, every
  body read to its end, and the body read to its
  end including the bot's auto-appended release
  notes standing in it. The issue comments are
  the bot's shell 6895612298 and the landing
  comment 6897405779 (2,441 characters, created
  2026-11-15T16:10:46Z and never edited), the
  landing comment standing last on the record.
  No trigger comment stands on the pull request:
  the bot's review opened on the pull request's
  creation on its own run, and no auto-reply
  stands. The identifiers the owner's brief
  cited for this census stand verified against
  the live record exactly as cited, the brief's
  census having been sourced from the landing
  comment itself; no correction is owed this
  round. The shell in its true final form,
  moment-bounded at this read: 11,172 characters
  by a Python count over the API body, created
  2026-11-15T14:06:37Z and last edited
  2026-11-15T14:21:22Z, and no rate-limit banner
  stands anywhere in it. The form the pull
  request's own body read stands on that record
  at 11,170 characters with its last edit then
  at 2026-11-15T14:08:43Z; the shell's last
  edit, at 2026-11-15T14:21:22Z, re-rendered it
  upward by 2 characters to the form contracted
  here, and that edit precedes the landing
  comment, so no disposition fact moved. Its
  walkthrough covers SB-116 to SB-121 over
  CONTRACT.md and README.md with the change
  assessment pinned to the closure head
  30f3e648a7331efb977a2ba8bedc6e78432ac28a and
  an estimated effort of 2; its merge risk reads
  Minimal up to 30f3e and states the pull
  request changes documentation only, has no
  runtime impact, and is ready to merge; its
  pre-merge table lists four passed checks,
  Description Check, Title check, Linked Issues
  check, and Out of Scope Changes check, the
  Title check standing Passed in this final
  form; its own recent-review line stands that
  no actionable comments were generated in the
  recent review; the word Docstring occurs zero
  times in it; and it carries its Run ID
  eb460e63-e78c-45ec-af92-f4c179498637 in its
  recent review info, the review standing over
  the files changed between
  6af9c13d6c4ae33983973c5ee0c1343848ff52f5 and
  30f3e648a7331efb977a2ba8bedc6e78432ac28a with
  CONTRACT.md and README.md the two files
  selected, and the included allowance used with
  none remaining. Step 0's coverage on PR 22 is
  that whole range, the branch carrying no head
  the review does not name. Step 0 raised no
  actionable finding on PR 22, and the body's
  one dismissal is re-graded here limb by limb
  against the shell's final form and stands
  sound: the Title check stands Passed, and its
  prose calls the title partially related,
  naming the round and the SB range without
  saying what the change does; the title (SB-116
  to SB-121: Round 18) stands at 26 characters
  and carries this repository's standing form,
  identical in shape to PR 21's title (SB-110 to
  SB-115: Round 17), PR 20's (SB-104 to SB-109:
  Round 16), PR 19's (SB-98 to SB-103: Round
  15), and PR 18's (SB-91 to SB-97: Round 14)
  read through the API at this read; the range
  it names is the range the body discharges; and
  the check's own full details concede the title
  refers to a real part of the change, is not
  off-topic, and is not vague enough to be
  inconclusive, so the reservation is a
  descriptiveness preference against the
  standing form, not a defect in the record, and
  the check's own verdict stands Passed. The
  bot's release-notes appendix at PR 22's body's
  foot stands censused in the whole-body read
  and stands dispositioned in that body itself
  at its true scope: its first bullet, the
  status section updated to reflect 18 completed
  perfection rounds, is correct; its second, the
  Round 18 records covering output-path
  boundaries, entropy draws, pricing and
  counting behavior, and realistic comparisons,
  with completed runs reporting no failures and
  runs affected by write faults retried, is
  correct at its scope; its third, the open case
  where writing through an output symlink inside
  the repository replaces the symlink, is
  SB-65a, carried open by name against SB-11;
  the appendix is the bot's own text inside its
  own markers, and no authored sentence adopts
  it beyond that census. The shell's walkthrough
  carries no lagging label at its full scope: it
  names the PR 21 read-back, Round 18's
  boundary, draw, pricing and counting
  measurements, and the realistic battery and
  closure under their own round. One process
  fact stands disclosed on PR 22's body's face
  and is named here at its true scope: the Round
  18 build lane banked all six items and the
  closure on the branch and ended before the
  body was finished, and the body was completed
  afterward by a second lane from the banked
  record, every figure in it re-read from the
  contract at the head, the run logs, or the
  live record at the completion, and graded
  accurate by all three of the round's review
  axes. One charge from the Round 18 landing
  stands named here with its posture: at that
  round's gate the owner's realistic
  re-execution ran all 660 cells and 656 passed
  outright, and the final four output-file
  cells, teams 20 to 23, stand untested under
  the box write fault wave then live, both
  implementations refusing identically at the
  content-write step and a raw write probe
  failing across four filesystems while file
  creation succeeded; they are counted neither
  for nor against, their re-execution in a clean
  window is charged to this round, this round's
  build re-runs the realistic battery whole
  including those four cells at SB-126, and
  their discharge by re-execution at the gate
  belongs to this round's red team. No late
  CodeRabbit finding stands on PR 22: nothing
  stands on the record after the landing
  comment, no review or review comment stands at
  all, and no issue stands open on the
  repository, so nothing stands undisposed. The
  carries stand open by name exactly as the
  charges direct: SB-65a against SB-11, SB-11
  not amended and no behavior changed,
  re-measured fresh on both arms this round at
  SB-123; and SB-47 and SB-43, the description
  surfaces, untouched, their questions routed to
  the main chat with owner recommendations.
- SB-123. The two-laws boundary re-probed fresh
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
- SB-124. The draw re-probed fresh at this
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
- SB-125. The counting joints re-probed fresh at
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
- SB-126. The realistic battery, the
  differential harness, the flighttape pin, and
  the closure of Round 19. The realistic battery
  ran its standing corpus: 24 team repositories
  carrying full 2026 histories in SB-81's
  composition (12 commits in an ordinary month,
  14 in a quarter month with its merge and side
  commit; 152 commits per repository, 3,648 in
  all), the per-month composition proven from
  git itself per repository and month before any
  cell ran. Against that corpus the battery
  measured 660 cells with no failures: 288
  unpriced statements and 288 statements priced
  with each repository's own card, compared byte
  for byte between the shipped CLI and the
  oracle and checked on their own counts (152
  commits read, three joined items, none
  unjoined, three priced); 60 price-book cells
  (the 30 cards as SB-90 enumerates them: 18
  valid, 12 refusals; each on the file and
  /dev/stdin surfaces); and 24 output-file cells
  whose landed bytes matched the stdout bytes.
  The 24 output-file cells include the four
  cells of teams 20 to 23 that the Round 18 gate
  left untested under that round's write fault
  wave; in this round's completed pass all four
  stand executed inside the whole at their
  contracted postures, their landed bytes
  matching the stdout bytes, counted in the 660
  like every other cell. The accounting stands
  on the second detached launch: the box write
  fault killed the first launch in the corpus
  phase, in team-12's build, on a corpus commit
  that did not land, and that launch is
  disclosed here and counted nowhere; the
  completed run resumed from the corpus
  checkpoints the first launch had banked, each
  repository's composition re-proven from git
  before any cell ran. The differential harness
  ran fresh against the same two implementations
  on its first launch: 14 cases, 240 steps, no
  failures. The flighttape pin was re-confirmed:
  a tree read through the GitHub API at commit
  74a83298aeb31c82e62b9fb0c613f5bc8600799e
  returned tree
  afd4d7d13520411e003762548fc79082a1b9cf6e with
  exactly 50 blobs, not truncated; the local
  extraction of that tree failed on its writes
  three times under the same wave and those
  attempts are counted nowhere, the standing
  checkout instead verified blob for blob
  against the live tree, all 50 blobs matching;
  a checkout of the pin built under the local
  toolchain on the second attempt, the first
  build's writes refused under the wave and
  counted nowhere, and its suite passed whole at
  241 tests; and the golden digest executed at
  the pin's binary: converting the synthetic
  session fixture printed the 1,663-byte tape,
  digesting the tape printed exactly
  cc9c4e01fca418dcc33114caf75fae1f6bf0d401f297b5d61f6776903fa955a3,
  and verify accepted it. One tooling incident
  stands in the same accounting: a scratch
  driver file for the pin fetch was zeroed by a
  failed write under the wave and was rewritten
  and verified by read-back; it is a scratch
  record only and no repository file was touched
  by it. With the 34 boundary, draw, and joint
  cells of SB-123 to SB-125, the round's
  measured cells total 694, the differential
  harness's 14 cases and 240 steps stated beside
  the total. The local gate ran on the final
  tree of this round before this item was
  appended: cargo test passes 25 tests (23
  integration tests in tests/test_statement.rs
  and 2 unit tests in the library target), cargo
  clippy --all-targets with warnings denied is
  clean, and cargo fmt --check is clean. The
  oracle verification stands as performed this
  round and is stated here directly: the final
  Python tree was read through the GitHub API at
  commit
  918dc7eb13bd3a5b0c3e644147170ad0bb1607e8, tree
  f331cd155f507c7013ccd6a3f5157670d80201f5, 13
  blobs, not truncated; the pristine copy is
  blob-identical to that tree on all seven
  Python files, and the driven copy equals the
  pristine copy with history.py and statement.py
  byte-identical, cli.py differing only in the
  two rename spots (the program name and the
  guard line), and __init__.py only in its
  docstring line. The binary provenance stands
  as measured this round and is stated here
  directly: the batteries drove a fresh debug
  build from the verified base tree, and its
  md5, 35555e1f592a2477b218403a8648dfbe, is
  exactly the binary md5 the Round 12, Round 13,
  Round 14, Round 15, Round 16, Round 17, and
  Round 18 records carry, reproduced again from
  the fresh build. A Python codepoint scan over
  the final tree counts no U+2014 and no U+2013.
  SB-65a stands carried open by name against
  SB-11, reproduced fresh this round at SB-123
  and undecided, and SB-11 itself stands carried
  open by name on the same terms; SB-47 and
  SB-43 stand carried by name, untouched, their
  questions routed to the main chat with owner
  recommendations. This round's scratch is
  durable under
  ~/workspace/retro-line/sb19-scratch/ (the work
  tree built from the base tree read through the
  API and verified blob for blob, the target
  directories, the battery fixtures and journal,
  and the battery and harness drivers and logs);
  the differential harness's own fixture root is
  a temporary directory under
  ~/workspace/retro-line by its design. With
  this item the README rounds line moves to 19
  of 100.
- SB-127. PR 23's read-back, in full, read
  through the API on 2026-11-16: one review, one
  review comment, and 2 issue comments, every
  body read to its end, and the body read to its
  end including the bot's auto-appended release
  notes standing in it. The issue comments are
  the bot's shell 6906938573 and the landing
  comment 6907951655 (2,784 characters, created
  2026-11-16T05:12:25Z and never edited), the
  landing comment standing last on the record.
  No trigger comment stands on the pull request:
  the bot's review opened on the pull request's
  creation on its own run, and no auto-reply
  stands. The identifiers the owner's brief
  cited for this census stand verified against
  the live record exactly as cited, the brief's
  census having been sourced from the landing
  comment itself, and the base contract's tail,
  read through the API at this round's start,
  runs SB-1 to SB-126, so this round's items are
  SB-127 onward as the brief expected; no
  correction is owed this round. The shell in
  its true final form, moment-bounded at this
  read: 8,385 characters by a Python count over
  the API body, created 2026-11-16T04:03:54Z and
  last edited 2026-11-16T04:14:28Z, and no
  rate-limit banner stands anywhere in it. Its
  earlier forms stand on PR 23's own record:
  5,196 characters in progress at its creation;
  8,381 characters with its last edit then at
  2026-11-16T04:06:06Z at the body's first read;
  and 8,383 characters as the body's stated
  closing form, re-rendered at
  2026-11-16T04:11:27Z. The shell's last edit,
  at 2026-11-16T04:14:28Z, re-rendered it upward
  by 2 characters to the form contracted here,
  the landing comment recording that same form
  at the gate's reads, and no disposition fact
  moved in either re-render. Its walkthrough
  covers the Round 19 records over CONTRACT.md
  and README.md, naming the PR 22 read-back, the
  boundary, entropy, pricing and counting
  measurements, and the battery and closure
  results, with the change assessment pinned to
  the closure head
  1cb9d494c598f1545bddaeded9cf91a3648363cb and
  an estimated effort of 2; its merge risk reads
  Low up to 1cb9d and carries the reservation
  that the closure record does not show the
  red-team re-execution of the four output-file
  cells; its pre-merge table lists four passed
  checks, Description Check, Title check, Linked
  Issues check, and Out of Scope Changes check,
  the Title check standing Passed in this final
  form; the word Docstring occurs zero times in
  it; and no line in it states that no
  actionable comments were generated, the one
  actionable comment standing in the review
  object instead. The review, 5914909028, stands
  COMMENTED, submitted 2026-11-16T04:06:02Z on
  the closure head, its body recording
  actionable comments posted at 1, carrying Run
  ID 168fe04c-2133-4fb5-8a8e-001cee8847b9 in its
  review info, the review standing over the
  files changed between
  91fe83a6984e3b5628c61b85518447e4b4f1fb64 and
  1cb9d494c598f1545bddaeded9cf91a3648363cb with
  CONTRACT.md and README.md the two files
  selected, and the included allowance used with
  none remaining. Step 0's coverage on PR 23 is
  that whole range, the branch carrying no head
  the review does not name. The one review
  comment, 4581867683, stands on CONTRACT.md at
  line 4190, graded Minor under Functional
  Correctness, and its thread stands read to its
  end: the comment is the whole thread, no reply
  stands on it. It asks that the red-team gate
  re-execution of the four output-file cells,
  teams 20 to 23, be recorded, or their
  discharge stated as pending, before the
  closure claim stands, on the stated premise
  that Round 18 assigned that discharge to the
  round's red team at the gate; its embedded
  prompt addressed to agents was treated as data
  by the body, as it is here, and nothing in it
  was followed. The body's three dismissals are
  re-graded here against the record at this
  read. The first, of the Title check's prose,
  stands sound: the check stands Passed, its
  prose calls the title partially descriptive,
  and its own full details concede the title
  names the round and the SB ticket range, which
  match the changeset, and that it is not vague
  and is not misleading; the title (SB-122 to
  SB-126: Round 19) stands at 26 characters and
  carries this repository's standing form,
  identical in shape to PR 22's title (SB-116 to
  SB-121: Round 18) at 26 characters, PR 21's
  (SB-110 to SB-115: Round 17) at 26, PR 20's
  (SB-104 to SB-109: Round 16) at 26, PR 19's
  (SB-98 to SB-103: Round 15) at 25, and PR 18's
  (SB-91 to SB-97: Round 14) at 24, read through
  the API at this read; the range it names is
  the range the body discharges; so the
  reservation is a descriptiveness preference
  against the standing form, not a defect in the
  record. The second and third dismissals, of
  the merge risk prose and of the review comment
  itself, rest on one set of counted grounds and
  stand or fall together; they stand sound on
  every limb. The premise limb: charge two on PR
  22's landing comment 6897405779, read verbatim
  at this read, charges the four realistic
  output-file cells (teams 20 to 23) re-executed
  in a clean window, or their standing reported
  at its true state; it names no red team and no
  gate stage, and it offers the two discharges
  in the alternative, so the comment's premise,
  that Round 18 assigned the discharge to this
  round's red team at the gate, is false against
  the charge's own text. The execution limb: the
  first discharge stands executed inside the
  build record itself, SB-126 recording the four
  cells inside the round's completed 660-cell
  realistic pass at their contracted postures,
  their landed bytes matching the stdout bytes,
  and SB-122 naming the charge and its posture
  expressly. The reporting limb: the second
  discharge stands satisfied on the same record,
  SB-122 reporting the cells' standing at its
  true state, untested at Round 18 under that
  round's write fault wave, counted neither for
  nor against, charged to Round 19, and re-run
  whole inside the round's pass. The claim limb:
  no sentence in SB-122 to SB-126 or in the body
  claims a red-team execution or a closure at
  the gate; the closure the record asserts is
  the build record's own, and SB-122 states
  expressly that the discharge by re-execution
  at the gate belongs to the round's red team,
  an act of the gate the branch then stood at.
  The seat limb: the gate fact the comment
  reaches for now stands where gate facts stand
  in this line, on the landing comment
  6907951655 itself, which records that Round
  19's red team re-executed the realistic pass
  whole on its own verified builds, 660 cells at
  0 failures, teams 20 to 23 each contributing
  their full block with the output-file cell's
  landed bytes compared across implementations
  and against each implementation's own stdout,
  and that the charge stands discharged with no
  charged surface open. The dismissal therefore
  stands sound, and the finding's requested
  record exists on the record at its proper
  seat. The bot's release-notes appendix at PR
  23's body's foot stands censused in the
  whole-body read: 395 characters from its
  opening marker, two bullets, each correct at
  its scope, the first recording Round 19 test
  results including successful resumed testing
  after an interrupted run, which is the
  realistic battery's completed second launch,
  and the second recording the status updated to
  show 19 of 100 perfection rounds complete; the
  appendix is the bot's own text inside its own
  markers, and no authored sentence adopts it
  beyond that census. The round's gate posture
  stands as the landing comment records it and
  as this read verifies: the gate closed fully
  clean, spec fidelity PASS, standards APPROVE,
  red team APPROVE, no findings at any grade on
  any axis, and PR 23 merged at
  84c06f2e7134a6d1f799051cb9cf2b13162dbee6, the
  merge tree equal to the reviewed closure tree
  9c44474e1e1ec805a72e37b15324d6f123f96c14
  exactly, hosted CI green on the closure head
  (push run 43801732948 and pull request run
  43801762885, re-read at completed success at
  this read), and the README on main reading 19
  of 100. One process fact from Round 19's
  record stands named here at its true scope, as
  the landing comment discloses it: the Round 19
  build lane banked the round whole, closure,
  pull request, step 0, and body complete, and
  ended without a final answer, the owner
  proceeding on firsthand verification of the
  banked record. No late CodeRabbit material
  stands on PR 23: nothing stands on the record
  after the landing comment, the shell's last
  edit preceding it, so charge five on the
  landing comment stands discharged with none
  standing, and nothing stands undisposed. The
  carries stand open by name exactly as the
  charges direct: SB-65a against SB-11, SB-11
  not amended and no behavior changed,
  re-measured fresh on both arms this round at
  SB-128; and SB-47 and SB-43, the description
  surfaces, untouched, their questions routed to
  the main chat with owner recommendations.
- SB-128. The two-laws boundary re-probed fresh
  at this binary against the verified oracle, at
  exact points by path shape, the two laws never
  conflated, the oracle at its module route
  (python3 -m muda.cli) for every figure below:
  eleven cells, no failures, on the battery's
  first launch this round. In the spelled-parent
  shape, the chain standing in the output's
  parent path with the output spelled through
  it: the guard's hand walk refuses onto-repo
  chains at 990 and 991 splices with the guard's
  line on both implementations, and at 992 the
  walk aborts into the crash class (the port
  silent at exit 1 with both streams empty, the
  oracle raising RecursionError); and the
  kernel's traversal of the spelled parent
  decides the physical write at its exact
  points, an away chain at 40 splices landing on
  both implementations and at 41 splices refused
  on both with the standing cannot-write line,
  nothing landing. In the head-is-output shape,
  the chain's head link standing as the output
  filename itself inside one real directory,
  which only the resolver traverses: chains at
  40 and at 41 splices both land on both
  implementations, the write replacing the head
  link and the chain's target file left
  unwritten, a chain at 991 splices lands the
  same way, a chain of 991 splices onto the
  repository draws the guard's refusal with the
  head link left standing, and at 992 splices
  the walk aborts into the crash class with the
  head link untouched. The SB-65a cell
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
- SB-129. The draw re-probed fresh at this
  binary, in SB-73's shape, under exactly the
  fourteen names SB-77 enumerates: fourteen
  cells, no failures, on the battery's first
  launch this round. A model of the draw written
  from the contract text is pinned against the
  four unit vectors in src/cli.rs before any
  cell runs (SB-56). The plain control lands
  byte-identical under no interposition. Under
  entropy refusal in both arms, the device open
  refused and a short read of five bytes, the
  port refuses on the standing cannot-write line
  before any create and the directory is left as
  seeded; the oracle under both refusal arms
  lands its statement, the contracted split
  standing, because its draw never opens the
  device at statement time. The clock-only
  plants stand inert at both of Round 8's
  readings, the port's write landing under a
  frozen clock at pid 1 at the first reading and
  seated at pid 250 at the second, and the
  oracle under the frozen clock at pid 1 with
  the same 64 planted also lands. The positive
  control is bracketed at both ends: with the
  entropy block fixed to a known nonzero value
  the 64 candidates are computable, planting all
  64 refuses the write, and removing the first
  planted candidate lands the write, as removing
  the last (the 64th) lands it; with the block
  zeroed the fold is zero, the candidate
  sequence is the clock-only sequence again, and
  its plants refuse. The pre-epoch posture, at a
  clock frozen 100 seconds before the epoch:
  with no plants the write lands byte-identical,
  and with the 64 candidates computed from the
  constant seed term planted under the fixed
  block, the write refuses on the standing line.
  The pre-epoch draw is therefore the
  constant-seed draw folded with entropy, as
  SB-73 contracts.
- SB-130. The counting joints re-probed fresh at
  this binary, in SB-74's shape with the rate
  bound run on both surfaces: nine cells, no
  failures, on the battery's first launch this
  round. A priced statement never counts: over
  one fixture (a revert, a fix-forward, and a
  redone item, all three joined), the priced
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
- SB-131. The realistic battery, the
  differential harness, the flighttape pin, and
  the closure of Round 20. The realistic battery
  ran its standing corpus: 24 team repositories
  carrying full 2026 histories in SB-81's
  composition (12 commits in an ordinary month,
  14 in a quarter month with its merge and side
  commit; 152 commits per repository, 3,648 in
  all), the per-month composition proven from
  git itself per repository and month before any
  cell ran. Against that corpus the battery
  measured 660 cells with no failures: 288
  unpriced statements and 288 statements priced
  with each repository's own card, compared byte
  for byte between the shipped CLI and the
  oracle and checked on their own counts (152
  commits read, three joined items, none
  unjoined, three priced); 60 price-book cells
  (the 30 cards as SB-90 enumerates them: 18
  valid, 12 refusals; each on the file and
  /dev/stdin surfaces); and 24 output-file cells
  whose landed bytes were compared across
  implementations and against each
  implementation's own stdout bytes for the same
  month and matched on both comparisons. The 24
  output-file cells include the four cells of
  teams 20 to 23 whose Round 18 untestedness was
  discharged at the Round 19 gate; in this
  round's completed pass all four stand executed
  inside the whole at their contracted postures,
  counted in the 660 like every other cell, so
  that discharge stands re-verified inside this
  pass and not assumed. The accounting stands on
  the second detached launch: the first launch
  died silently in the corpus phase, in
  team-02's build at 74 of that repository's 152
  commits, its process group not surviving its
  launch session, and that launch is disclosed
  here and counted nowhere; the completed run
  resumed from the corpus checkpoints the first
  launch had banked, each repository's
  composition re-proven from git before any cell
  ran. The differential harness ran fresh
  against the same two implementations on its
  first launch: 14 cases, 240 steps, no
  failures. The flighttape pin was re-confirmed:
  a tree read through the GitHub API at commit
  74a83298aeb31c82e62b9fb0c613f5bc8600799e
  returned tree
  afd4d7d13520411e003762548fc79082a1b9cf6e with
  exactly 50 blobs, not truncated, the local
  extraction verified blob for blob against that
  tree, all 50 blobs matching; a checkout of the
  pin built under the local toolchain and its
  suite passed whole at 241 tests; and the
  golden digest executed at the pin's binary:
  converting the synthetic session fixture
  printed the 1,663-byte tape, digesting the
  tape printed exactly
  cc9c4e01fca418dcc33114caf75fae1f6bf0d401f297b5d61f6776903fa955a3,
  and verify accepted it. The pin's first build
  attempt failed while compiling its
  dependencies, the compiler's temporary writes
  refused with no space left on device because
  the shared temporary filesystem stood full,
  and that attempt is counted nowhere; the build
  was re-run whole with its temporary directory
  seated on this round's scratch and passed as
  recorded. With the 34 boundary, draw, and
  joint cells of SB-128 to SB-130, the round's
  measured cells total 694, the differential
  harness's 14 cases and 240 steps stated beside
  the total. The local gate ran on the final
  tree of this round before this item was
  appended: cargo test passes 25 tests (23
  integration tests in tests/test_statement.rs
  and 2 unit tests in the library target), cargo
  clippy --all-targets with warnings denied is
  clean, and cargo fmt --check is clean, and the
  34 battery cells were re-run against the
  closure tree's build at no failures. That gate
  stands on its second running: the first
  running reported 17 of the 23 integration
  tests failed, every failure the same
  environmental shape, the tests' own git init
  refused with a copy-fd write error, operation
  not permitted, while copying the git
  templates, under the box write fault's wave, a
  raw content-write probe at the same moment
  failing with the same refusal in this round's
  scratch, in the shared temporary filesystem,
  and in shared memory alike, while the base
  gate on the same source had passed whole; that
  running is a fault reading, discarded and
  counted nowhere, and the gate was re-run whole
  once a probe showed content writes landing
  again. The same gate figures stand on the base
  tree, run before any item was appended. The
  oracle verification stands as performed this
  round and is stated here directly: the final
  Python tree was read through the GitHub API at
  commit
  918dc7eb13bd3a5b0c3e644147170ad0bb1607e8, tree
  f331cd155f507c7013ccd6a3f5157670d80201f5, 13
  blobs, not truncated; the pristine copy is
  blob-identical to that tree on all seven
  Python files, and the driven copy equals the
  pristine copy with history.py and statement.py
  byte-identical, cli.py differing only in the
  two rename spots (the program name and the
  guard line), and __init__.py only in its
  docstring line. The binary provenance stands
  as measured this round and is stated here
  directly: the batteries drove a fresh debug
  build from the verified base tree, and its
  md5, 35555e1f592a2477b218403a8648dfbe, is
  exactly the binary md5 the Round 12, Round 13,
  Round 14, Round 15, Round 16, Round 17, Round
  18, and Round 19 records carry, reproduced
  again from the fresh build and again from the
  closure tree's build. A Python codepoint scan
  over the final tree counts no U+2014 and no
  U+2013. One further attempt stands in the
  round's accounting: this round's first scratch
  progress write was refused under the
  file-count quota at the round's start, the
  download caches already standing clear and
  this line's spent target directories already
  standing pruned, so no pruning of this lane's
  was needed or done, and the write landed whole
  on a spaced retry; it is a scratch record
  only, counted nowhere. SB-65a stands carried
  open by name against SB-11, reproduced fresh
  this round at SB-128 and undecided, and SB-11
  itself stands carried open by name on the same
  terms; SB-47 and SB-43 stand carried by name,
  untouched, their questions routed to the main
  chat with owner recommendations. This round's
  scratch is durable under
  ~/workspace/retro-line/sb20-scratch/ (the work
  tree built from the base tree read through the
  API and verified blob for blob, the target
  directories, the battery fixtures and journal,
  and the battery and harness drivers and logs);
  the differential harness's own fixture root is
  a temporary directory under
  ~/workspace/retro-line by its design. With
  this item the README rounds line moves to 20
  of 100.
- SB-132. PR 24's read-back, in full, read
  through the API on 2026-11-16: no reviews, no
  review comments, and 2 issue comments, every
  body read to its end, and the body read to its
  end including the bot's auto-appended release
  notes standing in it. The issue comments are
  the bot's shell 6918621695 and the landing
  comment 6920367320 (2,455 characters, created
  2026-11-16T15:12:27Z and never edited), the
  landing comment standing last on the record.
  No trigger comment stands on the pull request:
  the bot's review opened on the pull request's
  creation on its own run, and no auto-reply
  stands. The identifiers the owner's brief
  cited for this census stand verified against
  the live record exactly as cited, the brief's
  census having been sourced from the landing
  comment itself, and the base contract's tail,
  read through the API at this round's start,
  runs SB-1 to SB-131, each item standing
  exactly once and in order (SB-95, SB-96, and
  SB-97 carrying the parenthetical Round 14
  form), so this round's items are SB-132 onward
  as the brief expected; no correction is owed
  this round. The shell in its true final form,
  moment-bounded at this read: 10,930 characters
  by a Python count over the API body, created
  2026-11-16T14:03:56Z and last edited
  2026-11-16T14:11:15Z, and no rate-limit banner
  stands anywhere in it. Its earlier form stands
  on PR 24's own record: the body's expressly
  moment-bounded step 0 account read it at
  10,928 characters, and its last edit
  re-rendered it upward by 2 characters to the
  form contracted here, the landing comment
  recording that same closing form, and no
  disposition fact moved in the re-render,
  verified by reading the closing form whole.
  Its walkthrough covers the Round 20 records
  over CONTRACT.md and README.md, naming the PR
  23 read-back, the boundary re-probe with the
  unresolved output-symlink case, the entropy,
  counting, and pricing re-probes, and the
  battery and closure results, with the change
  assessment pinned to the closure head
  f59e2981d4acda68d064c6e87d1d361b056f786c and
  an estimated effort of 2; its merge risk reads
  Minimal up to f59e2 and carries no
  reservation, its text recording that the pull
  request only adds round records and updates
  the README round count and changes no runtime
  behavior; its pre-merge table lists four
  passed checks, Title check, Linked Issues
  check, Out of Scope Changes check, and
  Description Check, the Title check standing
  Passed in this final form; the line stating
  that no actionable comments were generated in
  the recent review stands in it; the word
  Docstring occurs zero times in it; and its
  recent review info records Run ID
  d7cc4886-678a-427c-8d8e-6b3ae5b2f3bd, the
  review standing over the files changed between
  84c06f2e7134a6d1f799051cb9cf2b13162dbee6 and
  f59e2981d4acda68d064c6e87d1d361b056f786c with
  CONTRACT.md and README.md the two files
  selected, and the included allowance used with
  none remaining. Step 0's coverage on PR 24 is
  that whole range, the branch carrying no head
  the review does not name. PR 24's five commits
  stand verified live in order at this read:
  ecac5e34b1d67f057babc72ed194d091383d49aa
  (SB-127, tree
  3abbf38a12a30dcbbe2f858cef7592d9de6a9fd7),
  6960c1a770ec80255609cf0a9e3e68a62fcaea9b
  (SB-128, tree
  7f5c6c51490c20477b15aaf3e046034b5de48bfb),
  f3761ca33faf6544f42561122c38fac248769af5
  (SB-129, tree
  25c60c6a45d820dfdbc4119cdce7aae6fb2141b4),
  44626ea566e02653ffc9567d0cca37c00ddad0b2
  (SB-130, tree
  b96b956a816c8a46cdf6767ef0b812bb959ce7c1), and
  f59e2981d4acda68d064c6e87d1d361b056f786c
  (SB-131, tree
  e1f22a60f5a400a999eb7a4cd74924fbd422483d). The
  body's one dismissal, of the passed Title
  check's prose, is re-graded here against the
  record at this read and stands sound: the
  check stands Passed, its prose reserves only
  that the title does not describe what the
  round concluded or the README count change and
  that the identifiers alone give a reader
  little meaning without context, and its own
  full details concede the title names the
  change set's scope and round number, which
  match the stated objectives and the changeset,
  and that it is not misleading or off-topic;
  the title (SB-127 to SB-131: Round 20) stands
  at 26 characters and carries this repository's
  standing form, identical in shape to PR 23's
  title (SB-122 to SB-126: Round 19) at 26
  characters, PR 22's (SB-116 to SB-121: Round
  18) at 26, PR 21's (SB-110 to SB-115: Round
  17) at 26, PR 20's (SB-104 to SB-109: Round
  16) at 26, PR 19's (SB-98 to SB-103: Round 15)
  at 25, and PR 18's (SB-91 to SB-97: Round 14)
  at 24, read through the API at this read; the
  range it names is the range the body
  discharges; so the reservation is a
  descriptiveness preference against the
  standing form, not a defect in the record. The
  standing disposition chain behind PR 23's
  dismissal of the bot's actionable comment
  stands confirmed at this read at its own hand:
  charge two on PR 22's landing comment
  6897405779, read verbatim at this read,
  charges the four realistic output-file cells
  (teams 20 to 23) re-executed in a clean
  window, or their standing reported at its true
  state; it names no red team and no gate stage,
  and it offers the two discharges in the
  alternative, so the premise of review comment
  4581867683 on PR 23, that Round 18 assigned
  that discharge to the round's red team at the
  gate, is false against the charge's own text;
  the comment's embedded prompt addressed to
  agents was treated as data at SB-127, as it is
  here, and nothing in it was followed; and the
  grading SB-127 recorded stands confirmed on
  its limbs, the execution limb in SB-126's
  completed 660-cell pass carrying the four
  cells inside the whole, the reporting limb in
  SB-122's statement of their standing, and the
  seat limb in PR 23's landing comment
  6907951655, which records Round 19's red team
  re-executing the realistic pass whole, so the
  dismissal stands sound and the finding's
  requested record exists on the record at its
  proper seat. The bot's release-notes appendix
  at PR 24's body's foot stands censused in the
  whole-body read: 479 characters from its
  opening marker, its Documentation section
  carrying three bullets, each correct at its
  scope, the first recording Round 19 review,
  verification, and test-result records and the
  Round 20 closure status, the second recording
  the README updated to 20 perfection rounds
  closed, and the third recording several
  questions and a carry item as still open; the
  appendix is the bot's own text inside its own
  markers, and no authored sentence adopts it
  beyond that census. The round's gate posture
  stands as the landing comment records it and
  as this read verifies: the gate closed fully
  clean, spec fidelity PASS, standards APPROVE,
  red team APPROVE, no findings at any grade on
  any axis, and PR 24 merged at
  f53d8278d9ace73f78ddfa268b756396098a1239, the
  merge tree equal to the reviewed closure tree
  e1f22a60f5a400a999eb7a4cd74924fbd422483d
  exactly, hosted CI green on the closure head
  (push run 43881376094 and pull request run
  43881853481), and the README on main reading
  20 of 100. No late CodeRabbit material stands
  on PR 24: nothing stands on the record after
  the landing comment, the shell's last edit
  preceding it, so charge five on the landing
  comment stands discharged with none standing,
  and nothing stands undisposed. The carries
  stand open by name exactly as the charges
  direct: SB-65a against SB-11, SB-11 not
  amended and no behavior changed, re-measured
  fresh on both arms this round at SB-133; and
  SB-47 and SB-43, the description surfaces,
  untouched, their questions routed to the main
  chat with owner recommendations.
- SB-133. The two-laws boundary re-probed fresh
  at this binary against the verified oracle, at
  exact points by path shape, the two laws never
  conflated, the oracle at its module route
  (python3 -m muda.cli) for every figure below:
  eleven cells, no failures, on the battery's
  first launch this round. In the spelled-parent
  shape, the chain standing in the output's
  parent path with the output spelled through
  it: the guard's hand walk refuses onto-repo
  chains at 990 and 991 splices with the guard's
  line on both implementations, and at 992 the
  walk aborts into the crash class (the port
  silent at exit 1 with both streams empty, the
  oracle raising RecursionError); and the
  kernel's traversal of the spelled parent
  decides the physical write at its exact
  points, an away chain at 40 splices landing on
  both implementations and at 41 splices refused
  on both with the standing cannot-write line,
  nothing landing. In the head-is-output shape,
  the chain's head link standing as the output
  filename itself inside one real directory,
  which only the resolver traverses: chains at
  40 and at 41 splices both land on both
  implementations, the write replacing the head
  link and the chain's target file left
  unwritten, a chain at 991 splices lands the
  same way, a chain of 991 splices onto the
  repository draws the guard's refusal with the
  head link left standing, and at 992 splices
  the walk aborts into the crash class with the
  head link untouched. The SB-65a cell
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
- SB-134. The draw re-probed fresh at this
  binary, in SB-73's shape, under exactly the
  fourteen names SB-77 enumerates: fourteen
  cells, no failures, on the battery's first
  launch this round. A model of the draw written
  from the contract text is pinned against the
  four unit vectors in src/cli.rs before any
  cell runs (SB-56). The plain control lands
  byte-identical under no interposition. Under
  entropy refusal in both arms, the device open
  refused and a short read of five bytes, the
  port refuses on the standing cannot-write line
  before any create and the directory is left as
  seeded; the oracle under both refusal arms
  lands its statement, the contracted split
  standing, because its draw never opens the
  device at statement time. The clock-only
  plants stand inert at both of Round 8's
  readings, the port's write landing under a
  frozen clock at pid 1 at the first reading and
  seated at pid 250 at the second, and the
  oracle under the frozen clock at pid 1 with
  the same 64 planted also lands. The positive
  control is bracketed at both ends: with the
  entropy block fixed to a known nonzero value
  the 64 candidates are computable, planting all
  64 refuses the write, and removing the first
  planted candidate lands the write, as removing
  the last (the 64th) lands it; with the block
  zeroed the fold is zero, the candidate
  sequence is the clock-only sequence again, and
  its plants refuse. The pre-epoch posture, at a
  clock frozen 100 seconds before the epoch:
  with no plants the write lands byte-identical,
  and with the 64 candidates computed from the
  constant seed term planted under the fixed
  block, the write refuses on the standing line.
  The pre-epoch draw is therefore the
  constant-seed draw folded with entropy, as
  SB-73 contracts.
- SB-135. The counting joints re-probed fresh at
  this binary, in SB-74's shape with the rate
  bound run on both surfaces: nine cells, no
  failures, on the battery's first launch this
  round. A priced statement never counts: over
  one fixture (a revert, a fix-forward, and a
  redone item, all three joined), the priced
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
SB-136. Round 21 gate record and closure. The
  realistic battery's standing measurement is
  its thirteenth attempt's completed pass, run
  detached under the build slot on 2026-11-17
  with the slot held from about 05:43Z: the
  corpus stood proven from git at 24
  repositories and 3,648 commits in the SB-81
  composition, every fixture was written, and
  all 660 cells ran (288 unpriced statement
  cells, 288 priced statement cells, 60
  price-book cells, 24 output-file cells), with
  649 cells passing and the pass exiting 1. The
  60 price-book cells and all 576 statement
  cells passed, and the output-file cells of
  teams 00 to 12 passed; the output-file cells
  of teams 13 to 23, eleven cells, each failed
  on both implementations' writes with the
  identical line, error, cannot write the
  statement, the box write fault hardening
  mid-pass and refusing the temporary writes in
  the output directories on the port and the
  oracle alike. The output-file cells of teams
  20 to 23 stand executed inside the whole, and
  their statements passed as statement cells in
  the same pass, but their landed-byte
  comparisons stand unlanded this round on the
  refused writes, so the Round 18 discharge does
  not stand re-verified inside this pass and is
  stated exactly so. The differential battery
  stands beside the totals at 14 cases and 240
  steps with no failures, on its fourth launch,
  the first three launches dead at the fixture
  build's first git init inside the fault wave.
  The flighttape pin stands reconfirmed through
  the API at commit
  74a83298aeb31c82e62b9fb0c613f5bc8600799e with
  tree afd4d7d13520411e003762548fc79082a1b9cf6e
  and exactly 50 blobs, and its counted build
  ran on the pin build's sixth launch: its 241
  tests pass with no failures, and the golden
  tape stands executed at 1,663 bytes with
  digest
  cc9c4e01fca418dcc33114caf75fae1f6bf0d401f297b5d61f6776903fa955a3,
  the digest printed and matched and verify
  accepting at exit 0. The round's cells trace
  as 34 battery cells (boundary 11, draw 14,
  joints 9) plus the realistic pass's 660 cells,
  694 cells measured in the round, of which 683
  stand at no failure (the 34 battery cells and
  649 realistic cells) and 11 stand failed on
  the refused writes as recorded above, the
  differential's 14 cases and 240 steps standing
  beside the total. The closure gate ran whole
  on the gate tree on its third launch: cargo
  test passes 25 tests (2 unit, 0 bin, 23
  integration, 0 doc) with no failures, clippy
  with warnings denied stands clean, fmt stands
  clean, the build stands, the codepoint scan
  over the gate tree reads em dash 0 and en dash
  0, and the three batteries re-ran on the
  closure build at boundary 11 cells, draw 14
  cells, and joints 9 cells with no failures.
  The base gate stands at the same figures on
  the base tree: 25 tests (2 unit, 0 bin, 23
  integration, 0 doc) with no failures, clippy
  and fmt clean, the three batteries at 11, 14,
  and 9 cells with no failures, and the
  codepoint scan at em dash 0 and en dash 0. The
  oracle stands as verified directly above: the
  pristine commit
  918dc7eb13bd3a5b0c3e644147170ad0bb1607e8 at 13
  blobs, the driven copy differing only in the
  two rename spots and the docstring line.
  Provenance stands as stated directly: the md5
  figure 35555e1f592a2477b218403a8648dfbe is
  carried by the Round 12 through Round 20
  records, and the fresh build of this tree and
  the closure build of the gate tree reproduce
  it. The attempt history stands whole and
  counted nowhere except where a completed run's
  figures are counted above. The realistic
  battery's attempts 1 to 6 each died at the
  price-book currency fixture write inside the
  battery's own retries as the write fault
  hardened (attempt 1 built the corpus whole and
  banked its checkpoints; the later attempts
  resumed from them with the composition
  re-proven each time); attempt 7 and its
  relaunch loop were killed queued by a box
  restart on the evening of 2026-11-16; attempt
  8 waited some forty minutes behind a sibling
  lane's self-deadlocked nested flock, then ran
  and died at the first repo card write; attempt
  9 queued over ninety minutes behind
  fault-riding holds, one sibling probe loop
  among them holding ninety-seven minutes at
  zero CPU, then ran and died at the first repo
  card write; attempt 10 queued over seventy
  minutes behind a sibling hold of one hour
  forty-five minutes, then ran and died at the
  first repo card write; attempts 11 and 12 were
  killed queued by the box restarts of about
  03:28Z and about 05:30Z without holding the
  slot or running a cell; attempt 13 completed
  as recorded above; attempt 14 queued
  fifty-eight minutes behind a sibling lane's
  chain hold without holding the slot or running
  a cell and was stood down by the lane at the
  closure on the owner's order to land the round
  without another wait cycle, its launch wrapper
  and flock killed and the stand-down recorded
  with its state evidence in attempts21.md. The
  pin build's first five launches are counted
  nowhere (two setsid launches and one tracked
  launch died silently in their flock waits, and
  two more were killed queued by the two
  small-hours restarts) with the counted build
  on the sixth launch, and the closure gate's
  first two launches are counted nowhere (the
  first SIGTERM-reaped in its tracked wait as
  the 03:28Z restart hit, the second killed
  queued by the 05:30Z restart) with the counted
  gate on the third launch. The gate tree's own
  assembly stands disclosed: the blob-for-blob
  fetch route's writes were refused whole in a
  hard phase, per-file copier launches landed
  nothing in sustained hard phases, one
  wrapper's log redirections were refused at
  redirection time so its cycles never ran, a
  contract build script ground some eighteen
  minutes at zero bytes and was killed (its
  wrapper's removal of the stale tree is why the
  tree was rebuilt whole), a reflink clone was
  refused, and an append loop's stage writes
  were refused and it was killed before
  transcription; the tree stands assembled by
  file-tool transcription from the verified base
  extraction with the contract's base content
  placed by hardlink, every one of its 21 files
  verified by recomputed git blob id. SB-11 and
  SB-65a stand carried open by name, untouched,
  and SB-47 and SB-43 stand carried open by
  name, untouched, neither description surface
  changed. The round's scratch stands durable at
  ~/workspace/retro-line/sb21-scratch/, and the
  differential battery's fixture root is a
  temporary directory under
  ~/workspace/retro-line by its design. The
  README rounds line moves to 21 of 100 in this
  closure commit.
- SB-137. PR 25's read-back, in full, read
  through the API on 2026-11-17: 1 review, 1
  review comment, and 2 issue comments, every
  body read to its end, and the body read to its
  end including the bot's auto-appended release
  notes standing in it and the owner's gate
  patch standing in its step 0 account. The
  issue comments are the bot's shell 6941587062
  and the landing comment 6944298472 (2,486
  characters, created 2026-11-17T09:38:04Z and
  never edited), the landing comment standing
  last on the record. No trigger comment stands
  on the pull request: the bot's review opened
  on the pull request's creation on its own run,
  and no auto-reply stands. The identifiers the
  owner's brief cited for this census stand
  verified against the live record exactly as
  cited, the brief's census having been sourced
  from the landing comment itself, and the base
  contract's tail, read through the API at this
  round's start, runs SB-1 to SB-136, each item
  standing exactly once and in order, so this
  round's items are SB-137 onward as the brief
  expected; no correction is owed this round.
  The shell in its true final form,
  moment-bounded at this read: 9,738 characters
  by a Python count over the API body, created
  2026-11-17T07:23:34Z and last edited
  2026-11-17T09:35:11Z, standing unchanged since
  the landing comment recorded the same form,
  and no rate-limit banner stands anywhere in
  it. Its earlier forms stand on PR 25's own
  record: the body's expressly moment-bounded
  step 0 account read its completed form at
  9,735 characters, last edited
  2026-11-17T07:27:26Z, and read again unchanged
  after a further wait; the owner's gate patch
  then recorded its final form at the gate read,
  9,736 characters, last edited
  2026-11-17T07:40:10Z, verified element by
  element with no disposition-bearing element
  moved; and its last edit re-rendered it at the
  merge moment to the form contracted here. Its
  walkthrough covers the Round 21 records over
  CONTRACT.md and README.md, naming the PR 24
  read-back, the focused re-probes, and the
  battery and closure record with 649 of 660
  cells passing and 11 unverified output-file
  comparisons, with the change assessment pinned
  to the closure head
  88d91cd004c0fea744ef8057b91f88ea36605d15 and
  an estimated effort of 2; its merge risk reads
  Low up to 88d91 and carries the one prose
  reservation the body dismisses second below,
  its text recording that no code behavior
  changes; its pre-merge table lists four passed
  checks, Title check, Linked Issues check, Out
  of Scope Changes check, and Description Check,
  the Title check standing Passed in this final
  form with the concession the body dismisses
  third below; the word Docstring occurs zero
  times in it; and its finishing-touch and
  autopilot checkboxes stand unacted-on. The
  formal review 5934530166 (COMMENTED, submitted
  2026-11-17T07:27:22Z) records Run ID
  7b0c8b7d-9873-4dc1-a036-bae6b6c8efc6, the
  review standing over the files changed between
  f53d8278d9ace73f78ddfa268b756396098a1239 and
  88d91cd004c0fea744ef8057b91f88ea36605d15 with
  CONTRACT.md and README.md the two files
  selected, the included allowance used with
  none remaining, and exactly one actionable
  comment posted, the review comment 4596554734
  at CONTRACT.md line 5335; the shell's
  operational warning records its LanguageTool
  checks on CONTRACT.md as incomplete at the
  tool's per-file request limit, with no
  findings surfaced from them. Step 0's coverage
  on PR 25 is that whole range, the branch
  carrying no head the review does not name. PR
  25's five commits stand verified live in order
  at this read:
  07e1599d5595a828ea92895d7ee4235c91380dad
  (SB-132, tree
  0f625a8a16e2f3aeffb5bec1cbf3110451725184),
  f12e1269d224fab3c3537d9dc1b524f3deb2cb18
  (SB-133, tree
  9ba86c73426b5356594e27a0f20e53752eddde6b),
  5272dc0a1e99a5857f6c1a2a30adffafe7cc1d33
  (SB-134, tree
  543ab2a8e30ee025e84a921851e55fb49fd2f707),
  8c18f9c0667290142711a9d40bd9866d7eb974be
  (SB-135, tree
  bcbc6579a4fcbe62ab5e43e0f104147a657f313b), and
  88d91cd004c0fea744ef8057b91f88ea36605d15
  (SB-136, tree
  08728b3b8eb7608bb3c43a4cb18c891d912e9696). The
  body's three dismissals are re-graded here
  against the record at this read. The first, of
  the review comment 4596554734, stands sound on
  every limb: the comment cites CONTRACT.md line
  5051 as the Round 20 closure recording the
  red-team, verdict, and approval stages, and
  line 5051, read in the head contract at this
  read, stands inside SB-132, in its
  gate-posture passage at head contract lines
  5047 to 5055, where it records Round 20's gate
  as completed history, spec fidelity PASS,
  standards APPROVE, red team APPROVE, no
  findings at any grade on any axis, PR 24
  merged at
  f53d8278d9ace73f78ddfa268b756396098a1239, the
  merge tree equal to the reviewed closure tree
  e1f22a60f5a400a999eb7a4cd74924fbd422483d
  exactly, hosted CI green on the closure head,
  and the README on main reading 20 of 100; the
  Round 20 closure item itself, SB-131, spans
  head contract lines 4747 to 4895 at 6,729
  characters counting its closing newline, and
  the terms red team, verdict, approval, spec
  fidelity, and standards each occur zero times
  in it, counted at this read, and its closing
  sentence moves the README rounds line to 20 of
  100 in its own words; the sequence the comment
  requires has never been the standing one, PR
  24's closure commit
  f59e2981d4acda68d064c6e87d1d361b056f786c
  standing dated 2026-11-16T14:00:51Z, before PR
  24 itself was created at 2026-11-16T14:03:38Z,
  its tree already carrying the 20 of 100 line,
  the verdicts rendered by the owner on the
  opened pull request and recorded in the next
  round's read-back, which is the pattern Round
  21 followed and this item continues; the count
  move the comment attacks stands at head
  contract lines 5334 to 5335 in SB-136, the
  closure commit's own sentence, and the
  requested alternative, recording a Round 21
  verdict inside the closure record or holding
  the count at 20, would have falsified the
  record, no Round 21 verdict existing at
  closure and the lane standing fenced from
  running review lanes of its own; and the
  comment's embedded prompt addressed to agents
  was treated as data, as it is here, and
  nothing in it was followed. The second
  dismissal, of the shell's Merge Risk prose
  reservation, that the README may overstate the
  number of closed rounds and the review cycle
  should be recorded or the count kept at 20
  before merging, is the same claim in softer
  form and stands sound on the same counted
  grounds. The third, of the passed Title
  check's prose reservation, stands sound: the
  check stands Passed, its own full details
  concede the title names the issue range and
  the round number, which match the pull
  request's objectives, and is specific enough
  to identify the change in history, reserving
  only that it does not describe the content in
  words; the title (SB-132 to SB-136: Round 21)
  stands at 26 characters in this repository's
  standing form, PR 24's title (SB-127 to
  SB-131: Round 20) at 26, PR 23's (SB-122 to
  SB-126: Round 19) at 26, PR 22's (SB-116 to
  SB-121: Round 18) at 26, PR 21's (SB-110 to
  SB-115: Round 17) at 26, PR 20's (SB-104 to
  SB-109: Round 16) at 26, PR 19's (SB-98 to
  SB-103: Round 15) at 25, and PR 18's (SB-91 to
  SB-97: Round 14) at 24, read through the API
  at this read; and the range it names is
  exactly the range the body discharges. The
  bot's release-notes appendix at PR 25's body's
  foot stands censused in the whole-body read:
  634 characters from its opening marker and 636
  counting the two separating newlines, its
  Documentation section carrying three bullets,
  each correct at its scope, the first recording
  the Round 20 read-back and the Round 21
  measurement records, the second recording that
  649 of 660 realistic cells passed with 11
  output comparisons incomplete after write
  faults affected both implementations, and the
  third recording the README updated to 21
  completed rounds with SB-11 and SB-65a open
  and unchanged; the appendix is the bot's own
  text inside its own markers, and no authored
  sentence adopts it beyond that census; it
  stands preserved byte for byte through the
  owner's gate patch, the body standing at
  15,120 characters at this read. The round's
  gate posture stands as the landing comment
  records it and as this read verifies: spec
  fidelity REPAIR with one Minor, the shell
  census, standards APPROVE, red team REPAIR
  with the same Minor as its RT-1, the Minor
  repaired before the merge by the owner's body
  patch stating the shell's final form at the
  gate read, and PR 25 merged at
  91eb0cfcdc75c8865b882b534c72579f075755b6, the
  merge tree equal to the reviewed closure tree
  08728b3b8eb7608bb3c43a4cb18c891d912e9696
  exactly, hosted CI green on the closure head
  (push run 44037475622 and pull request run
  44038317716), and the README on main reading
  21 of 100. No late CodeRabbit material stands
  on PR 25: nothing stands on the record after
  the landing comment, the shell's last edit
  preceding it, so charge five on the landing
  comment stands discharged with none standing,
  and nothing stands undisposed. The carries
  stand open by name exactly as the charges
  direct: SB-65a against SB-11, SB-11 not
  amended and no behavior changed, re-measured
  fresh on both arms this round at SB-138; and
  SB-47 and SB-43, the description surfaces,
  untouched, their questions routed to the main
  chat with owner recommendations.
- SB-138. The two-laws boundary re-probed fresh
  at this binary against the verified oracle, at
  exact points by path shape, the two laws never
  conflated, the oracle at its module route
  (python3 -m muda.cli) for every figure below:
  eleven cells, no failures, on the battery's
  first launch this round. In the spelled-parent
  shape, the chain standing in the output's
  parent path with the output spelled through
  it: the guard's hand walk refuses onto-repo
  chains at 990 and 991 splices with the guard's
  line on both implementations, and at 992 the
  walk aborts into the crash class (the port
  silent at exit 1 with both streams empty, the
  oracle raising RecursionError); and the
  kernel's traversal of the spelled parent
  decides the physical write at its exact
  points, an away chain at 40 splices landing on
  both implementations and at 41 splices refused
  on both with the standing cannot-write line,
  nothing landing. In the head-is-output shape,
  the chain's head link standing as the output
  filename itself inside one real directory,
  which only the resolver traverses: chains at
  40 and at 41 splices both land on both
  implementations, the write replacing the head
  link and the chain's target file left
  unwritten, a chain at 991 splices lands the
  same way, a chain of 991 splices onto the
  repository draws the guard's refusal with the
  head link left standing, and at 992 splices
  the walk aborts into the crash class with the
  head link untouched. The SB-65a cell
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
- SB-139. The draw re-probed fresh at this
  binary, in SB-73's shape, under exactly the
  fourteen names SB-77 enumerates: fourteen
  cells, no failures, on the battery's first
  launch this round. A model of the draw written
  from the contract text is pinned against the
  four unit vectors in src/cli.rs before any
  cell runs (SB-56). The plain control lands
  byte-identical under no interposition. Under
  entropy refusal in both arms, the device open
  refused and a short read of five bytes, the
  port refuses on the standing cannot-write line
  before any create and the directory is left as
  seeded; the oracle under both refusal arms
  lands its statement, the contracted split
  standing, because its draw never opens the
  device at statement time. The clock-only
  plants stand inert at both of Round 8's
  readings, the port's write landing under a
  frozen clock at pid 1 at the first reading and
  seated at pid 250 at the second, and the
  oracle under the frozen clock at pid 1 with
  the same 64 planted also lands. The positive
  control is bracketed at both ends: with the
  entropy block fixed to a known nonzero value
  the 64 candidates are computable, planting all
  64 refuses the write, and removing the first
  planted candidate lands the write, as removing
  the last (the 64th) lands it; with the block
  zeroed the fold is zero, the candidate
  sequence is the clock-only sequence again, and
  its plants refuse. The pre-epoch posture, at a
  clock frozen 100 seconds before the epoch:
  with no plants the write lands byte-identical,
  and with the 64 candidates computed from the
  constant seed term planted under the fixed
  block, the write refuses on the standing line.
  The pre-epoch draw is therefore the
  constant-seed draw folded with entropy, as
  SB-73 contracts.
- SB-140. The counting joints re-probed fresh at
  this binary, in SB-74's shape with the rate
  bound run on both surfaces: nine cells, no
  failures, on the battery's first launch this
  round. A priced statement never counts: over
  one fixture (a revert, a fix-forward, and a
  redone item, all three joined), the priced
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
- SB-141. The Round 21 red team's owed
  executions, discharged by execution this
  round. The red team's own runner, rt_gate.py,
  died in Round 21's gate window having supplied
  tree verification only: both of its trees
  repaired and verified ALL 21 EXACT (its base
  tree at the Round 20 closure tree
  e1f22a60f5a400a999eb7a4cd74924fbd422483d, its
  closure tree at
  08728b3b8eb7608bb3c43a4cb18c891d912e9696),
  then every build step on its base tree timed
  out in the build-slot queue, both binaries
  stood missing, and the runner died on a
  refused content write while writing its
  results, so the executions it owed stood owed
  at this round's start. This round's rt chain
  executed the whole owed set on that same base
  tree, re-verified ALL 21 EXACT at this lane's
  hand before the run, through driver_rt22.py
  against the red team's own target directory:
  the chain acquired the build slot at
  2026-11-17T15:23:16Z and completed at
  15:29:36Z with every pass done on its first
  launch: cargo test exit 0 with 25 tests
  passing (2 unit, 23 integration) and no
  failures, clippy exit 0 with warnings denied,
  fmt exit 0, the debug build exit 0 at md5
  35555e1f592a2477b218403a8648dfbe, the release
  build exit 0 at md5
  e3c72e1813c603539597268c7b169a4a, the boundary
  battery at 11 cells with no failures, the draw
  battery at 14 cells with no failures, the
  joints battery at 9 cells with no failures,
  and the differential at 14 cases and 240 steps
  with no failures. The debug build's md5 is the
  standing provenance figure, standing
  re-measured here on the red team's builds as
  on this round's own. These figures are the
  Round 21 base tree's own accounting, executed
  to discharge the debt the runner left; they
  enter no cell total of this round, whose own
  batteries stand at SB-138 to SB-140 and whose
  differential and realistic stand at the
  closure.
- SB-142. Round 22 gate record and closure. The
  realistic battery's standing measurement is
  its counted pass under the completion lane's
  final relaunch, run detached under the build
  slot on 2026-11-18 and completed at 00:25:11Z
  on the chain's fifth launch: the corpus stood
  proven from git at 24 repositories and 3,648
  commits in the SB-81 composition, teams 00 to
  19 standing as checkpoint hits from the
  chain's earlier launches and teams 20 to 23
  built fresh inside the counted launch, and all
  660 cells ran (288 unpriced statement cells,
  288 priced statement cells, 60 price-book
  cells, 24 output-file cells) with no failures
  and the pass exiting 0. The output-file cells
  of teams 20 to 23 stand executed and passing
  inside this whole, so the landed-byte
  comparisons Round 21's thirteenth attempt left
  unlanded stand re-verified here, and the Round
  18 discharge stands re-verified inside this
  pass, stated exactly so; this discharges
  charge two of the Round 21 landing comment.
  The differential battery stands beside the
  totals at 14 cases and 240 steps with no
  failures, on the batteries chain's first
  launch. The flighttape pin stands reconfirmed
  through the API at commit
  74a83298aeb31c82e62b9fb0c613f5bc8600799e with
  tree afd4d7d13520411e003762548fc79082a1b9cf6e
  and exactly 50 blobs, and its counted run
  completed whole on 2026-11-17 at 17:58:48Z on
  the verified pin tree where it stands in the
  Round 21 scratch (the copy into this round's
  scratch could not be written by any exec route
  in the fault's phases and stands disclosed as
  unwritten in the attempt record; the tree run
  on is the pin tree byte for byte, verified ALL
  50 EXACT at the lane's hand this round): its
  241 tests pass with no failures, and the
  golden tape stands executed at 1,663 bytes
  with digest
  cc9c4e01fca418dcc33114caf75fae1f6bf0d401f297b5d61f6776903fa955a3,
  the digest printed and matched and verify
  accepting at exit 0. The round's cells trace
  as 34 battery cells (boundary 11, draw 14,
  joints 9) plus the realistic pass's 660 cells,
  694 cells measured in the round, all at no
  failure, the differential's 14 cases and 240
  steps standing beside the total. The closure
  gate ran whole on the closure tree (this
  round's final tree before this item, differing
  from the verified base tree only in
  CONTRACT.md and README.md): cargo test passes
  25 tests (2 unit, 0 bin, 23 integration, 0
  doc) with no failures, clippy with warnings
  denied stands clean, fmt stands clean, the
  build stands at md5
  35555e1f592a2477b218403a8648dfbe, the
  codepoint scan over the closure tree reads em
  dash 0 and en dash 0, and the three batteries
  re-ran on the closure build at boundary 11
  cells, draw 14 cells, and joints 9 cells with
  no failures, the differential on the closure
  build at 14 cases and 240 steps with no
  failures. The base gate stands at the same
  figures on the base tree: 25 tests (2 unit, 0
  bin, 23 integration, 0 doc) with no failures,
  clippy and fmt clean, the three batteries at
  11, 14, and 9 cells with no failures, and the
  codepoint scan at em dash 0 and en dash 0. The
  oracle stands verified through the API at the
  pristine commit
  918dc7eb13bd3a5b0c3e644147170ad0bb1607e8 (tree
  f331cd155f507c7013ccd6a3f5157670d80201f5, 13
  blobs): history.py and statement.py
  blob-identical to the driven copy, and
  __init__.py and cli.py differing in exactly
  the three contracted lines, the __init__
  docstring line, cli.py's prog name, and
  cli.py's guard line, each secondbill to muda.
  Provenance stands as stated directly: the md5
  figure 35555e1f592a2477b218403a8648dfbe is
  carried by the Round 12 through Round 21
  records, and the fresh builds of the base tree
  and the closure tree reproduce it, as does the
  Round 21 base tree's debug build at SB-141.
  The attempt history stands whole in
  attempts22.md in the round's scratch and
  counted nowhere except where a completed run's
  figures are counted above: the first build
  lane errored on a transport failure at
  2026-11-17T11:38:11Z with the base gate and
  all four measurement chains killed queued by
  the restart at about the same moment; the
  completion lane's relaunches died queued in
  the restarts of about 13:37Z and about 15:19Z,
  its 15:20Z seating ran the base gate against a
  gate tree the fault had silently zeroed (every
  figure of that run discarded as
  fault-produced) and completed the charge three
  chain whole at SB-141, and its reassembled
  tree's gate and pin chain completed whole on
  2026-11-17 at 17:57Z and 17:58Z; the batteries
  and realistic chains died queued in the
  restarts of about 19:50Z, 20:44Z, 22:01Z, and
  23:50Z and inside the file-count quota's hard
  window, the realistic chain's launches failing
  deterministically at one team's month 9 merge
  or at refused writes with teams 00 to 05 and
  later 00 to 19 checkpointed and never redone,
  until the batteries chain completed whole on
  its first launch at 23:59:11Z and the
  realistic chain completed whole on its fifth
  launch as recorded above; the completion lane
  itself errored on a transport failure at
  2026-11-17T22:03:18Z, and a third seating
  adopted the banked state, cleared the quota by
  deleting two spent target directories (about
  59,000 files), and brought the round to this
  gate. The closure gate's own first launch
  queued from 00:28Z on 2026-11-18 behind the
  fleet's holds and its cargo stages failed in
  the fault's window at 01:48Z with every figure
  discarded as fault-produced; a second launch
  at 01:54Z landed no figure and stands gone
  from the process table; and the gate then ran
  whole at the completion seating's relaunch as
  recorded above, each earlier launch counted
  nowhere. The Round 21 red team's runner stands
  dead and not left armed, its fate and its
  landed tree verifications recorded at SB-141,
  and no runner of this round stands armed
  undisclosed. SB-11 and SB-65a stand carried
  open by name, untouched, and SB-47 and SB-43
  stand carried open by name, untouched, neither
  description surface changed. The round's
  scratch stands durable at
  ~/workspace/retro-line/sb22-scratch/, and the
  differential battery's fixture root is a
  temporary directory under
  ~/workspace/retro-line by its design. The
  README rounds line moves to 22 of 100 in this
  closure commit.
- SB-143. Repair record, appended after this
  round's reviews and standing outside the
  round's item range: the spec review's Minor on
  SB-141 and the standards review's two Minors
  on SB-142. Three clauses stand qualified here,
  the originals unrewritten. First, SB-141's
  discharge of the Round 21 red team's owed
  executions stood incomplete by one figure at
  the gate: the release binary md5 on the Round
  21 closure tree 08728b3b, on which the round
  had executed cargo test, clippy, fmt, and the
  debug build only. That figure stands executed
  in this repair lane: the tree was assembled
  through the API blob by blob (21 blobs, 0
  mismatches) at the lane's scratch and built in
  the release profile through the build slot,
  the build running whole on its first launch
  with no failed attempt standing to count, and
  the release binary stands at md5
  e3c72e1813c603539597268c7b169a4a at build-path
  scope, agreeing with the release figure SB-141
  recorded on the Round 21 base tree e1f22a60,
  the two trees' code standing blob-identical at
  this lane's comparison of their trees, and
  SB-141's discharge is complete only with this
  record. Second, SB-142's attempt summary,
  restated in the body's Known deviations
  paragraph, states that the batteries and
  realistic chains died queued in the restarts
  of about 19:50Z, 20:44Z, 22:01Z, and 23:50Z on
  2026-11-17; the posture word overdistributes
  over two of the four restarts. At 19:50Z and
  23:50Z both chains died queued, and the
  realistic chain died queued at all four; at
  20:44Z the batteries chain died after it had
  acquired the slot, its three battery logs
  created at zero bytes and no pass completing a
  launch; at 22:01Z the batteries chain was
  killed inside its slot hold, its log echoes
  refused whole and no pass completing a launch.
  The two chains died on the slot queue or in
  the hold before any pass completed a launch,
  the exact postures standing in attempts22.md,
  and the summary's conclusion, every death
  counted nowhere with no figure standing in
  any, stands unaffected. Third, SB-142 states
  the closure gate ran whole on the closure
  tree, this round's final tree before this
  item, differing from the verified base tree
  only in CONTRACT.md and README.md; the
  difference set overstates by one file. The
  tree the gate ran on, the SB-141 head's tree
  2dfc112e, differs from the base tree in
  CONTRACT.md only, its README blob standing
  identical to the base's, the rounds line
  reading 21 of 100 in both, until the closure
  commit moved it. The clause's load-bearing
  content, the gate tree's code standing
  identical to the closure tree's code, holds
  exactly. SB-141 and SB-142 stand unamended;
  this record qualifies the three clauses and
  supplies the one figure.
