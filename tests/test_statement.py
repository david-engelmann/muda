"""SB-5 to SB-12. The git side statement."""
import json
import os
import subprocess
from pathlib import Path

import pytest

from secondbill.cli import main
from secondbill.history import StatementError, read_history
from secondbill.statement import build_statement

ENV = {**os.environ,
       "GIT_CONFIG_NOSYSTEM": "1",
       "HOME": os.environ["HOME"]}


def git(repo, *args, date=None, check=True):
    env = dict(ENV)
    if date:
        env["GIT_AUTHOR_DATE"] = date
        env["GIT_COMMITTER_DATE"] = date
    result = subprocess.run(
        ["git", "-C", str(repo),
         "-c", "user.name=Synthetic",
         "-c", "user.email=synthetic@example.invalid",
         "-c", "commit.gpgsign=false", *args],
        capture_output=True, text=True, env=env)
    if check:
        assert result.returncode == 0, result.stderr
    return result


def commit(repo, subject, date, files, body=None, branch=None):
    for name, content in files.items():
        target = repo / name
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_text(content)
    git(repo, "add", "-A")
    args = ["commit", "-q", "-m", subject]
    if body:
        args += ["-m", body]
    git(repo, *args, date=date)
    return git(repo, "rev-parse", "HEAD").stdout.strip()


@pytest.fixture(scope="module")
def history(tmp_path_factory):
    repo = tmp_path_factory.mktemp("repo")
    git(repo, "init", "-q", "-b", "main")
    shas = {}
    shas["c1"] = commit(repo, "Add the parser", "2026-01-05T09:00:00Z",
                        {"src/parser.py": "v1\n"})
    git(repo, "checkout", "-q", "-b", "side")
    shas["cB"] = commit(repo, "Add branch work", "2026-01-06T09:00:00Z",
                        {"src/branch.py": "b\n"})
    git(repo, "checkout", "-q", "main")
    shas["cM"] = commit(repo, "Main work", "2026-01-06T10:00:00Z",
                        {"src/main.py": "m\n"})
    env = dict(ENV, GIT_AUTHOR_DATE="2026-01-07T09:00:00Z",
               GIT_COMMITTER_DATE="2026-01-07T09:00:00Z")
    subprocess.run(
        ["git", "-C", str(repo), "-c", "user.name=Synthetic",
         "-c", "user.email=synthetic@example.invalid",
         "merge", "-q", "--no-ff", "-m", "Merge branch side", "side"],
        capture_output=True, text=True, env=env, check=True)
    shas["merge"] = git(repo, "rev-parse", "HEAD").stdout.strip()
    shas["c2"] = commit(repo, "Add the parser", "2026-01-12T09:00:00Z",
                        {"src/parser.py": "v2\n", "src/extra.py": "e\n"})
    shas["c3"] = commit(repo, "Repair the parser edge",
                        "2026-01-13T09:00:00Z", {"src/parser.py": "v3\n"},
                        body=f"Fixes: {shas['c1'][:12]}")
    shas["c4"] = commit(repo, 'Revert "Add the parser"',
                        "2026-01-14T09:00:00Z", {"src/parser.py": "v1\n"},
                        body=f"This reverts commit {shas['c2']}.")
    shas["c5"] = commit(repo, 'Revert "Add the parser"',
                        "2026-01-15T09:00:00Z", {"src/parser.py": "v2\n"})
    shas["c6"] = commit(repo, "Fix the tokenizer",
                        "2026-01-16T09:00:00Z", {"src/tok.py": "t\n"},
                        body="Fixes: deadbeef")
    shas["c7"] = commit(repo, "Tidy the docs", "2026-01-20T09:00:00Z",
                        {"README.md": "docs\n"})
    shas["c9"] = commit(repo, "Add the parser", "2026-01-22T09:00:00Z",
                        {"src/other.py": "o\n"})
    shas["c10"] = commit(repo, "Add the parser", "2026-01-25T09:00:00Z",
                         {"src/parser.py": "v4\n"})
    shas["c8"] = commit(repo, "Add the parser", "2026-02-03T09:00:00Z",
                        {"src/parser.py": "v5\n"})
    return repo, shas


def test_the_january_statement(history):
    repo, s = history
    document = build_statement(read_history(repo), "2026-01")
    assert document["commits_read"] == 13
    assert document["merges_read"] == 1
    assert document["window"] == {"from": "2026-01-01",
                                   "to": "2026-02-01"}
    by_cause = {c["cause"]: [(i["sha"], i["class"], i["join"])
                             for i in c["items"]]
                for c in document["causes"]}
    assert by_cause == {
        s["c1"]: [(s["c2"], "redone", "redone task"),
                  (s["c3"], "fix-forward", "fixes trailer"),
                  (s["c10"], "redone",
                   "redone task, earliest of several matches")],
        s["c2"]: [(s["c4"], "revert", "revert trailer")]}
    unjoined = {(u["sha"], u["class"]): u["reason"]
                for u in document["unjoined"]}
    assert unjoined == {
        (s["c5"], "revert"):
            "several earlier commits carry the quoted subject",
        (s["c6"], "fix-forward"):
            "the trailer names no commit in the read, deadbeef"}
    assert document["totals"] == {
        "items": 6,
        "per_class": {"revert": 2, "fix-forward": 2, "redone": 2},
        "per_cause": {s["c1"]: 3, s["c2"]: 1},
        "unjoined": 2}
    assert document["totals"]["per_cause"] and s["merge"] not in str(
        document["causes"])


def test_the_february_statement_has_no_items(history):
    repo, s = history
    document = build_statement(read_history(repo), "2026-02")
    assert document["totals"]["items"] == 0
    assert document["causes"] == [] and document["unjoined"] == []


def test_the_statement_is_deterministic(history):
    repo, _ = history
    commits = read_history(repo)
    first = json.dumps(build_statement(commits, "2026-01"),
                       indent=2, ensure_ascii=False)
    second = json.dumps(build_statement(read_history(repo), "2026-01"),
                        indent=2, ensure_ascii=False)
    assert first == second


def test_unreadable_and_empty_repositories_are_refused(tmp_path):
    with pytest.raises(StatementError, match="cannot read"):
        read_history(tmp_path)
    empty = tmp_path / "empty"
    empty.mkdir()
    git(empty, "init", "-q", "-b", "main")
    with pytest.raises(StatementError, match="cannot read|no commits"):
        read_history(empty)


def test_the_field_set_is_complete(history):
    repo, s = history
    commits = {c["sha"]: c for c in read_history(repo)}
    merge = commits[s["merge"]]
    assert len(merge["parents"]) == 2
    c3 = commits[s["c3"]]
    assert c3["subject"] == "Repair the parser edge"
    assert f"Fixes: {s['c1'][:12]}" in c3["body"]
    assert c3["paths"] == ["src/parser.py"]
    assert c3["author_time"].isoformat() == "2026-01-13T09:00:00+00:00"


def test_the_cli_and_its_guards(history, tmp_path, capsys):
    repo, s = history
    out = tmp_path / "statement.json"
    assert main(["statement", str(repo), "--month", "2026-01",
                 "-o", str(out)]) == 0
    document = json.loads(out.read_text())
    assert document["totals"]["items"] == 6
    inside = repo / "statement.json"
    assert main(["statement", str(repo), "--month", "2026-01",
                 "-o", str(inside)]) == 1
    assert "writes nothing inside" in capsys.readouterr().err
    assert not inside.exists()
    assert main(["statement", str(repo), "--month", "2026-13"]) == 1
    assert "YYYY-MM" in capsys.readouterr().err
