"""SB-5, SB-5a. One git repository, read and never written."""
from __future__ import annotations

import subprocess
from datetime import datetime, timezone
from pathlib import Path


class StatementError(ValueError):
    """A named refusal (family form, SB-11)."""


def _git(repo: Path, *args: str) -> str:
    try:
        result = subprocess.run(
            ["git", "--no-optional-locks", "-C", str(repo), *args],
            capture_output=True, text=True, timeout=60)
    except (OSError, subprocess.SubprocessError) as exc:
        raise StatementError(
            f"cannot read the repository at {repo}: {exc}") from exc
    if result.returncode != 0:
        raise StatementError(
            f"cannot read the repository at {repo}: "
            f"{result.stderr.strip() or 'git failed'}")
    return result.stdout


def read_history(repo: Path) -> list[dict]:
    """Every commit reachable from HEAD, oldest first,
    with the stated field set. The read writes nothing."""
    _git(repo, "rev-parse", "--git-dir")
    head = _git(repo, "rev-parse", "--verify", "HEAD")
    if not head.strip():
        raise StatementError(f"no commits in the repository at {repo}")
    fmt = "%H%x1f%P%x1f%aI%x1f%s%x1f%b%x1e"
    raw = _git(repo, "log", "--reverse", f"--format={fmt}")
    paths_raw = _git(repo, "log", "--reverse", "--name-only",
                     "--format=%x1e%H")
    paths_by_sha: dict[str, list[str]] = {}
    for record in paths_raw.split("\x1e"):
        lines = [line for line in record.splitlines() if line]
        if lines:
            paths_by_sha[lines[0]] = sorted(set(lines[1:]))
    commits = []
    for record in raw.split("\x1e"):
        record = record.strip("\n")
        if not record:
            continue
        sha, parents, author, subject, body = record.split("\x1f", 4)
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
        raise StatementError(f"no commits in the repository at {repo}")
    return commits


def worktree_root(repo: Path) -> Path:
    return Path(_git(repo, "rev-parse", "--show-toplevel").strip())
