"""SB-6 to SB-10. The itemized statement for one month."""
from __future__ import annotations

import re
from datetime import datetime, timezone

from .history import StatementError

REVERT_TRAILER = re.compile(
    r"^This reverts commit ([0-9a-f]{7,40})\.?\s*$")
FIXES_TRAILER = re.compile(r"^Fixes: ([0-9a-f]{7,40})\s*$")
REVERT_SUBJECT = re.compile(r'^Revert "(.*)"$')

CLASSES = ("revert", "fix-forward", "redone")


def _month_window(month: str):
    try:
        year, mon = (int(part) for part in month.split("-"))
        start = datetime(year, mon, 1, tzinfo=timezone.utc)
        end = (datetime(year + 1, 1, 1, tzinfo=timezone.utc)
               if mon == 12 else
               datetime(year, mon + 1, 1, tzinfo=timezone.utc))
    except (ValueError, AttributeError):
        raise StatementError(
            f"month must be stated as YYYY-MM, {month}") from None
    return start, end


def _resolve_prefix(commits, prefix: str, own_sha: str):
    matches = [c for c in commits
               if c["sha"].startswith(prefix)
               and c["sha"] != own_sha]
    if len(matches) == 1:
        return matches[0], None
    if not matches:
        return None, f"the trailer names no commit in the read, {prefix}"
    return None, f"the trailer names several commits in the read, {prefix}"


def _classify(commit, all_commits, earlier_all, earlier_window):
    """One commit's item, or None. Classes are checked
    in the SB-6a order; the first match is the class."""
    body_lines = commit["body"].splitlines()
    for line in body_lines:
        match = REVERT_TRAILER.match(line.strip())
        if match:
            cause, reason = _resolve_prefix(all_commits, match.group(1), commit["sha"])
            return {"class": "revert", "join": "revert trailer",
                    "cause": cause["sha"] if cause else None,
                    "reason": reason}
    subject_match = REVERT_SUBJECT.match(commit["subject"])
    if subject_match:
        quoted = subject_match.group(1)
        candidates = [c for c in earlier_all
                      if c["subject"] == quoted]
        if len(candidates) == 1:
            return {"class": "revert", "join": "revert subject",
                    "cause": candidates[0]["sha"], "reason": None}
        reason = ("no earlier commit carries the quoted subject"
                  if not candidates else
                  "several earlier commits carry the quoted subject")
        return {"class": "revert", "join": "revert subject",
                "cause": None, "reason": reason}
    for line in body_lines:
        match = FIXES_TRAILER.match(line.strip())
        if match:
            cause, reason = _resolve_prefix(all_commits, match.group(1), commit["sha"])
            return {"class": "fix-forward", "join": "fixes trailer",
                    "cause": cause["sha"] if cause else None,
                    "reason": reason}
    matches = [c for c in earlier_window
               if c["subject"] == commit["subject"]
               and set(c["paths"]) & set(commit["paths"])]
    if matches:
        join = ("redone task" if len(matches) == 1 else
                "redone task, earliest of several matches")
        return {"class": "redone", "join": join,
                "cause": matches[0]["sha"], "reason": None}
    return None


def build_statement(commits: list[dict], month: str) -> dict:
    start, end = _month_window(month)
    merges = [c for c in commits if len(c["parents"]) > 1]
    in_window = [c for c in commits
                 if start <= c["author_time"] < end]
    position = {c["sha"]: i for i, c in enumerate(commits)}
    items = []
    for commit in in_window:
        if len(commit["parents"]) > 1:
            continue
        earlier_all = commits[:position[commit["sha"]]]
        earlier_window = [c for c in in_window
                          if position[c["sha"]] < position[commit["sha"]]]
        found = _classify(commit, commits, earlier_all,
                          earlier_window)
        if found is not None:
            items.append({"sha": commit["sha"], **found})
    causes: dict[str, list[dict]] = {}
    unjoined = []
    for item in items:
        entry = {"sha": item["sha"], "class": item["class"],
                 "join": item["join"]}
        if item["cause"] is None:
            unjoined.append({**entry, "reason": item["reason"]})
        else:
            causes.setdefault(item["cause"], []).append(entry)
    per_class = {name: 0 for name in CLASSES}
    for item in items:
        per_class[item["class"]] += 1
    return {
        "month": month,
        "window": {"from": start.date().isoformat(),
                   "to": end.date().isoformat()},
        "commits_read": len(commits),
        "merges_read": len(merges),
        "causes": [{"cause": cause, "items": causes[cause]}
                   for cause in sorted(causes,
                                       key=lambda s: position[s])],
        "unjoined": unjoined,
        "totals": {
            "items": len(items),
            "per_class": per_class,
            "per_cause": {cause: len(causes[cause])
                          for cause in sorted(
                              causes, key=lambda s: position[s])},
            "unjoined": len(unjoined)},
    }
