"""SB-5 to SB-12. The git side statement."""
import json
import os
import subprocess
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


def commit(repo, subject, date, files, body=None, author=None):
    for name, content in files.items():
        target = repo / name
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_text(content)
    git(repo, "add", "-A")
    args = ["commit", "-q", "-m", subject]
    if body:
        args += ["-m", body]
    if author is None:
        git(repo, *args, date=date)
    else:
        env = dict(ENV, GIT_AUTHOR_DATE=author,
                   GIT_COMMITTER_DATE=date)
        result = subprocess.run(
            ["git", "-C", str(repo), "-c", "user.name=Synthetic",
             "-c", "user.email=synthetic@example.invalid",
             "-c", "commit.gpgsign=false", *args],
            capture_output=True, text=True, env=env)
        assert result.returncode == 0, result.stderr
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
    shas["cY"] = commit(repo, "Repeat me", "2026-01-26T09:00:00Z",
                        {"src/repeat.py": "y\n"},
                        author="2026-01-20T09:00:00Z")
    shas["cX"] = commit(repo, "Repeat me", "2026-01-25T09:00:00Z",
                        {"src/repeat.py": "x\n"},
                        author="2026-01-25T09:00:00Z")
    shas["c12"] = commit(repo, "Add the parser", "2026-01-27T09:00:00Z",
                         {"src/parser.py": "v6\n"},
                         body=f"Fixes: {shas['c1'][:12]}")
    shas["c11"] = commit(repo, 'Revert "Repair the parser edge"',
                         "2026-02-02T09:00:00Z",
                         {"src/parser.py": "v1\n"},
                         body=f"This reverts commit {shas['c3']}.")
    shas["c8"] = commit(repo, "Add the parser", "2026-02-03T09:00:00Z",
                        {"src/parser.py": "v5\n"})
    return repo, shas


def test_the_january_statement(history):
    repo, s = history
    document = build_statement(read_history(repo), "2026-01")
    assert document["commits_read"] == 17
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
                   "redone task, earliest of several matches"),
                  (s["c12"], "fix-forward", "fixes trailer")],
        s["c2"]: [(s["c4"], "revert", "revert trailer")],
        s["cY"]: [(s["cX"], "redone", "redone task")]}
    unjoined = {(u["sha"], u["class"]): u["reason"]
                for u in document["unjoined"]}
    assert unjoined == {
        (s["c5"], "revert"):
            "several earlier commits carry the quoted subject",
        (s["c6"], "fix-forward"):
            "the trailer names no commit in the read, deadbeef"}
    assert document["totals"] == {
        "items": 8,
        "per_class": {"revert": 2, "fix-forward": 3, "redone": 3},
        "per_cause": {s["c1"]: 4, s["c2"]: 1, s["cY"]: 1},
        "unjoined": 2}
    item_shas = [i["sha"] for c in document["causes"]
                 for i in c["items"]]
    item_shas += [u["sha"] for u in document["unjoined"]]
    assert s["merge"] not in item_shas


def test_the_february_statement_bills_an_out_of_month_cause(history):
    repo, s = history
    document = build_statement(read_history(repo), "2026-02")
    assert document["totals"]["items"] == 1
    assert document["causes"] == [{
        "cause": s["c3"],
        "items": [{"sha": s["c11"], "class": "revert",
                   "join": "revert trailer"}]}]
    assert document["unjoined"] == []


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
    assert document["totals"]["items"] == 8
    inside = repo / "statement.json"
    assert main(["statement", str(repo), "--month", "2026-01",
                 "-o", str(inside)]) == 1
    assert "writes nothing inside" in capsys.readouterr().err
    assert not inside.exists()
    assert main(["statement", str(repo), "--month", "2026-13"]) == 1
    assert "YYYY-MM" in capsys.readouterr().err


def test_a_shallow_repository_is_refused(history, tmp_path):
    repo, _ = history
    clone = tmp_path / "shallow"
    result = subprocess.run(
        ["git", "clone", "-q", "--depth", "1",
         f"file://{repo}", str(clone)],
        capture_output=True, text=True, env=ENV)
    assert result.returncode == 0, result.stderr
    with pytest.raises(StatementError, match="shallow"):
        read_history(clone)


def test_the_environment_never_redirects_the_read(history,
                                                  monkeypatch):
    repo, _ = history
    monkeypatch.setenv("GIT_DIR", "/nonexistent/elsewhere")
    monkeypatch.setenv("GIT_WORK_TREE", "/nonexistent")
    commits = read_history(repo)
    assert len(commits) == 17


def test_a_merge_is_never_an_item_even_with_a_trailer():
    from datetime import datetime, timezone
    base = {"parents": ["p"], "author_time":
            datetime(2026, 1, 5, tzinfo=timezone.utc),
            "subject": "Base", "body": "", "paths": ["a.py"]}
    merge = {"sha": "m" * 40, "parents": ["p", "q"],
             "author_time": datetime(2026, 1, 6,
                                     tzinfo=timezone.utc),
             "subject": "Merge", "paths": [],
             "body": f"Fixes: {'a' * 12}"}
    base = {**base, "sha": "a" * 40}
    plain = {**merge, "sha": "b" * 40, "parents": ["p"]}
    document = build_statement([base, merge], "2026-01")
    assert document["totals"]["items"] == 0
    document = build_statement([base, plain], "2026-01")
    assert document["totals"]["items"] == 1


def test_a_trailer_naming_the_item_itself_is_unjoined():
    from datetime import datetime, timezone
    commit = {"sha": "c" * 40, "parents": [],
              "author_time": datetime(2026, 1, 5,
                                      tzinfo=timezone.utc),
              "subject": "Self fix", "paths": ["a.py"],
              "body": f"Fixes: {'c' * 12}"}
    document = build_statement([commit], "2026-01")
    assert document["unjoined"][0]["reason"].startswith(
        "the trailer names the item itself")


def test_an_indented_trailer_is_text_not_a_trailer(tmp_path):
    repo = tmp_path / "quoted"
    repo.mkdir()
    git(repo, "init", "-q", "-b", "main")
    first = commit(repo, "Base work", "2026-01-05T09:00:00Z",
                   {"a.py": "1\n"})
    commit(repo, "Document the revert", "2026-01-06T09:00:00Z",
           {"b.py": "2\n"},
           body=f"Example:\n\n    This reverts commit {first}.")
    document = build_statement(read_history(repo), "2026-01")
    assert document["totals"]["items"] == 0
