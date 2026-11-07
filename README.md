# muda

Meters the rework tax across a fleet. Every reverted diff, redone task, and fix-forward is charged to the instruction that caused it, and the charges come back as an itemized statement.

Muda is a command line tool that reads one git repository's whole history and writes an itemized statement for one stated calendar month as JSON. The read covers the commits reachable from the checked out HEAD, so work that lives only on unmerged branches is not read. Check out the branch whose history the statement should cover. It finds the month's rework, commits that revert an earlier commit, commits that fix one forward, and tasks done twice, and it bills every item to the commit that caused it, with the join stated on the item itself. An item whose cause cannot be named from the record is listed as unjoined with the reason, never silently billed and never dropped. Catchledger prices checks, muda prices work.

[![ci](https://github.com/david-engelmann/muda/actions/workflows/ci.yml/badge.svg)](https://github.com/david-engelmann/muda/actions/workflows/ci.yml)

```console
$ muda statement --month 2026-01 demo
{
  "month": "2026-01",
  "window": {
    "from": "2026-01-01",
    "to": "2026-02-01"
  },
  "commits_read": 2,
  "merges_read": 0,
  "causes": [
    {
      "cause": "7b5cc76ab7533dcbecb076996b9ebbbceddd4f19",
      "items": [
        {
          "sha": "b2d53b23e15ec4ea0b6545f4794ef38d55024b83",
          "class": "revert",
          "join": "revert subject"
        }
      ]
    }
  ],
  "unjoined": [],
  "totals": {
    "items": 1,
    "per_class": {
      "revert": 1,
      "fix-forward": 0,
      "redone": 0
    },
    "per_cause": {
      "7b5cc76ab7533dcbecb076996b9ebbbceddd4f19": 1
    },
    "unjoined": 0
  }
}
```

## Install

A Rust toolchain and Git are the prerequisites. The build needs a Unix environment, Linux or macOS, or WSL on Windows, because the file write behind the -o output uses Unix-only facilities and the crate does not compile on native Windows. Clone the repo and install the command from it.

```console
$ git clone https://github.com/david-engelmann/muda
$ cd muda
$ cargo install --path .
```

The installed command lands in Cargo's bin directory, $HOME/.cargo/bin by default, and the quickstart below assumes that directory is on the PATH. The crate is named muda and it takes no dependencies. The git read is a subprocess call to the git on the PATH.

## Quickstart

Build a small repository with one reverted commit in its January history, then take its January statement. The dates are pinned so the history is the same history every time it is built.

```console
$ git init -q demo
$ cd demo
$ git config user.name "J. Doe"
$ git config user.email "jdoe@example.com"
$ echo "button" > export.py
$ git add .
$ GIT_AUTHOR_DATE="2026-01-05T09:00:00+00:00" GIT_COMMITTER_DATE="2026-01-05T09:00:00+00:00" git commit -q -m "Add the export button"
$ echo "no button" > export.py
$ GIT_AUTHOR_DATE="2026-01-12T09:00:00+00:00" GIT_COMMITTER_DATE="2026-01-12T09:00:00+00:00" git commit -q -am 'Revert "Add the export button"'
$ cd ..
$ muda statement --month 2026-01 demo
{
  "month": "2026-01",
  "window": {
    "from": "2026-01-01",
    "to": "2026-02-01"
  },
  "commits_read": 2,
  "merges_read": 0,
  "causes": [
    {
      "cause": "7b5cc76ab7533dcbecb076996b9ebbbceddd4f19",
      "items": [
        {
          "sha": "b2d53b23e15ec4ea0b6545f4794ef38d55024b83",
          "class": "revert",
          "join": "revert subject"
        }
      ]
    }
  ],
  "unjoined": [],
  "totals": {
    "items": 1,
    "per_class": {
      "revert": 1,
      "fix-forward": 0,
      "redone": 0
    },
    "per_cause": {
      "7b5cc76ab7533dcbecb076996b9ebbbceddd4f19": 1
    },
    "unjoined": 0
  }
}
```

The statement reads the whole history and bills the one item in the window. The revert is charged to the commit it reverts, and the join is named on the item, revert subject here because the revert's subject quotes the original subject. The totals count items per class and per cause, in counts only.

## Usage

Traces end at the session while outcomes live in git, so no run can be priced by whether its work survived, and a fast run that generates tomorrow's revert looks identical to a fast run that shipped. Muda makes that join on the git side. Its one subcommand is statement. The month is named with --month and is required, in the form 2026-01. A commit is counted in the month its author timestamp falls in, read in UTC, so a commit near a month boundary can land in the neighbouring month against the local date its own timestamp carries. The document goes to stdout, or to a file with -o. A rates card passed with --rates adds a priced section to the same document.

A revert joins by its quoted subject or by the trailer git itself writes, a fix-forward joins by its Fixes trailer naming the commit it fixes, and a task done twice joins by subject and shared paths to the earlier attempt. The revert and fix-forward joins resolve against the whole read, across month boundaries. The redone join looks only at earlier commits inside the stated month, so a task whose first attempt landed in an earlier month is not reported as redone. Adding two parser commits to the demo, the second repeating the first inside the same month, adds a redone item to the January statement.

```console
$ cd demo
$ echo "parser v1" > parser.py
$ git add .
$ GIT_AUTHOR_DATE="2026-01-19T09:00:00+00:00" GIT_COMMITTER_DATE="2026-01-19T09:00:00+00:00" git commit -q -m "Write the parser"
$ echo "parser v2" > parser.py
$ GIT_AUTHOR_DATE="2026-01-26T09:00:00+00:00" GIT_COMMITTER_DATE="2026-01-26T09:00:00+00:00" git commit -q -am "Write the parser"
$ cd ..
$ muda statement --month 2026-01 demo
{
  "month": "2026-01",
  "window": {
    "from": "2026-01-01",
    "to": "2026-02-01"
  },
  "commits_read": 4,
  "merges_read": 0,
  "causes": [
    {
      "cause": "7b5cc76ab7533dcbecb076996b9ebbbceddd4f19",
      "items": [
        {
          "sha": "b2d53b23e15ec4ea0b6545f4794ef38d55024b83",
          "class": "revert",
          "join": "revert subject"
        }
      ]
    },
    {
      "cause": "67bf0a26e7049e5dcb0787874fb4921d50e3ea54",
      "items": [
        {
          "sha": "8c4330b91cb23a017b33f4a69279474a0482752c",
          "class": "redone",
          "join": "redone task"
        }
      ]
    }
  ],
  "unjoined": [],
  "totals": {
    "items": 2,
    "per_class": {
      "revert": 1,
      "fix-forward": 0,
      "redone": 1
    },
    "per_cause": {
      "7b5cc76ab7533dcbecb076996b9ebbbceddd4f19": 1,
      "67bf0a26e7049e5dcb0787874fb4921d50e3ea54": 1
    },
    "unjoined": 0
  }
}
```

With -o the document is written through a sibling temporary file and replaced into place. An output path inside the repository being read is refused, and nothing is written.

```console
$ muda statement --month 2026-01 -o demo/out.json demo
error, muda writes nothing inside the repository it reads
```

A rates card is a small JSON file naming a currency of three letters and a rate for each of the three classes. Pricing counts only the joined items, exact in minor units, and the priced section rides beside the counts in the same document. This run prices February, a month with no items in its window, so the priced form shows at zero.

```console
$ cat > rates.json <<'EOF'
{"currency": "USD", "per_item": {"revert": "10.00", "fix-forward": "5.00", "redone": "2.50"}}
EOF
$ muda statement --month 2026-02 --rates rates.json demo
{
  "month": "2026-02",
  "window": {
    "from": "2026-02-01",
    "to": "2026-03-01"
  },
  "commits_read": 4,
  "merges_read": 0,
  "causes": [],
  "unjoined": [],
  "totals": {
    "items": 0,
    "per_class": {
      "revert": 0,
      "fix-forward": 0,
      "redone": 0
    },
    "per_cause": {},
    "unjoined": 0
  },
  "priced": {
    "currency": "USD",
    "items_priced": 0,
    "per_class": {
      "revert": {
        "items": 0,
        "rate": "10.00",
        "amount": "0.00"
      },
      "fix-forward": {
        "items": 0,
        "rate": "5.00",
        "amount": "0.00"
      },
      "redone": {
        "items": 0,
        "rate": "2.50",
        "amount": "0.00"
      }
    },
    "per_cause": [],
    "total": "0.00"
  }
}
```

The same card over January prices that statement's two joined items, the revert at 10.00 and the redone task at 2.50, for a total of 12.50 in the priced section.

## Status

The statement command is complete at its contracted scope and runs as a Rust binary, with the Python of the first two rounds standing as its behavioral spec. The statement bills each item to the commit that caused it. Joining causes onward to the instruction, prompt, or agent behind the commit is not built yet. Closed perfection rounds, 7 of 100. A round counts only when a full cycle of work, audit, review, red team, and approval has closed.

## License

MIT. See the [LICENSE](LICENSE) file.
