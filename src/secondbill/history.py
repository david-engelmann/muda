"""SB-5, SB-5a, SB-5b. One git repository, read and
never written.

The read states its depth honestly: every path a
commit touched, both sides of a rename, merges with
no paths (a merge's own diff is not read in Round 1).
The caller's git environment never redirects the
read, replace objects and grafts are off, a shallow
repository is refused, and message bytes that are
not UTF-8 are a named refusal (SB-6b).
"""
from __future__ import annotations

import os
import subprocess
from datetime import datetime, timezone
from pathlib import Path


class StatementError(ValueError):
    """A named refusal (family form, SB-11)."""


_SCRUBBED = ("GIT_DIR", "GIT_WORK_TREE", "GIT_INDEX_FILE",
             "GIT_OBJECT_DIRECTORY",
             "GIT_ALTERNATE_OBJECT_DIRECTORIES",
             "GIT_NAMESPACE", "GIT_PREFIX", "GIT_CEILING")


def _clean_env() -> dict:
    env = {key: value for key, value in os.environ.items()
           if key not in _SCRUBBED}
    env["GIT_GRAFT_FILE"] = os.devnull
    return env


def _git(repo: Path, *args: str, check: bool = True) -> str:
    try:
        result = subprocess.run(
            ["git", "--no-replace-objects", "--no-optional-locks",
             "-C", str(repo), *args],
            capture_output=True, timeout=60, env=_clean_env())
    except (OSError, subprocess.SubprocessError) as exc:
        raise StatementError(
            f"cannot read the repository at {repo}: {exc}") from exc
    if check and result.returncode != 0:
        detail = result.stderr.decode("utf-8", "replace").strip()
        raise StatementError(
            f"cannot read the repository at {repo}: "
            f"{detail or 'git failed'}")
    if result.returncode != 0:
        return ""
    try:
        return result.stdout.decode("utf-8")
    except UnicodeDecodeError:
        raise StatementError(
            f"cannot read the repository at {repo}: a commit "
            f"message is not valid UTF-8") from None


def read_history(repo: Path) -> list[dict]:
    """Every commit reachable from HEAD, ordered by
    author time (ties by read position, SB-5b), with
    the stated field set. The read writes nothing."""
    _git(repo, "rev-parse", "--git-dir")
    head = _git(repo, "rev-parse", "--verify", "HEAD", check=False)
    if not head.strip():
        raise StatementError(
            f"no commits in the repository at {repo}")
    shallow = _git(repo, "rev-parse", "--is-shallow-repository")
    if shallow.strip() == "true":
        raise StatementError(
            f"the repository at {repo} is shallow; a statement "
            f"needs the whole history")
    fmt = "%H%x1f%P%x1f%aI%x1f%s%x1f%b%x1e"
    raw = _git(repo, "log", "--reverse", f"--format={fmt}")
    paths_raw = _git(repo, "log", "--reverse", "--name-status",
                     "--format=%x1e%H")
    paths_by_sha: dict[str, list[str]] = {}
    for record in paths_raw.split("\x1e"):
        lines = [line for line in record.splitlines() if line]
        if not lines:
            continue
        paths = []
        for entry in lines[1:]:
            paths.extend(entry.split("\t")[1:])
        paths_by_sha[lines[0]] = sorted(set(paths))
    commits = []
    for record in raw.split("\x1e"):
        record = record.strip("\n")
        if not record:
            continue
        parts = record.split("\x1f", 4)
        if len(parts) != 5:
            raise StatementError(
                f"cannot read the repository at {repo}: a commit "
                f"record could not be parsed")
        sha, parents, author, subject, body = parts
        moment = datetime.fromisoformat(author)
        if moment.tzinfo is not None:
            moment = moment.astimezone(timezone.utc)
        commits.append({
            "sha": sha,
            "parents": parents.split() if parents else [],
            "author_time": moment,
            "subject": subject,
            "body": body,
            "paths": paths_by_sha.get(sha, []),
        })
    if not commits:
        raise StatementError(
            f"no commits in the repository at {repo}")
    commits.sort(key=lambda c: c["author_time"])
    return commits


def worktree_root(repo: Path) -> Path:
    return Path(_git(repo, "rev-parse", "--show-toplevel").strip())


def git_dir(repo: Path) -> Path:
    return Path(_git(repo, "rev-parse",
                     "--absolute-git-dir").strip())
