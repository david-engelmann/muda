"""SB-11, SB-11a. A thin command line: statement.
Failures are named errors with exit 1."""
from __future__ import annotations

import argparse
import json
import os
import sys
import tempfile
from pathlib import Path

from .history import StatementError, read_history, worktree_root
from .statement import build_statement


def _write_atomically(path: Path, text: str) -> None:
    fd, tmp = tempfile.mkstemp(dir=str(path.parent),
                               prefix=path.name + ".")
    try:
        with os.fdopen(fd, "w", encoding="utf-8") as handle:
            handle.write(text)
        os.replace(tmp, path)
    except BaseException:
        try:
            os.unlink(tmp)
        except OSError:
            pass
        raise


def _emit_text(text: str) -> int:
    try:
        sys.stdout.write(text)
        sys.stdout.flush()
    except (OSError, UnicodeEncodeError):
        print("error, cannot write to stdout", file=sys.stderr)
        try:
            sys.stdout.close()
        except OSError:
            pass
        sys.stdout = open(os.devnull, "w", encoding="utf-8")
        return 1
    return 0


def _emit(document, out) -> int:
    text = json.dumps(document, indent=2, ensure_ascii=False) + "\n"
    if out is not None:
        try:
            _write_atomically(Path(out), text)
        except (OSError, UnicodeEncodeError):
            print(f"error, cannot write the statement, {out}",
                  file=sys.stderr)
            return 1
        return 0
    return _emit_text(text)


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(prog="secondbill")
    sub = parser.add_subparsers(dest="command", required=True)
    st = sub.add_parser("statement", help="the monthly statement")
    st.add_argument("repo")
    st.add_argument("--month", required=True)
    st.add_argument("-o", "--output")
    args = parser.parse_args(argv)
    repo = Path(args.repo)
    try:
        if args.output is not None:
            root = worktree_root(repo)
            target = Path(args.output).resolve()
            if target == root or root in target.parents:
                print("error, secondbill writes nothing inside "
                      "the repository it reads", file=sys.stderr)
                return 1
        commits = read_history(repo)
        document = build_statement(commits, args.month)
    except StatementError as exc:
        print(f"error, {exc}", file=sys.stderr)
        return 1
    return _emit(document, args.output)


if __name__ == "__main__":
    raise SystemExit(main())
