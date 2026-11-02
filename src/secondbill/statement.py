"""SB-6 to SB-16. The itemized statement for one month,
and its pricing."""
from __future__ import annotations

import json
import re
from datetime import datetime, timezone

from .history import StatementError

REVERT_TRAILER = re.compile(
    r"^This reverts commit ([0-9a-f]{7,40})\.?\s*$")
FIXES_TRAILER = re.compile(r"^Fixes: ([0-9a-f]{7,40})\s*$")
REVERT_SUBJECT = re.compile(r'^Revert "(.*)"$')

CLASSES = ("revert", "fix-forward", "redone")


MONTH_FORM = re.compile(r"^\d{4}-(0[1-9]|1[0-2])$")
RATE_FORM = re.compile(r"^(\d+)(?:\.(\d{1,2}))?$")
CURRENCY_FORM = re.compile(r"^[A-Za-z]{3}$")


def _rate_cents(value, name: str) -> int:
    """One rate string to minor units, exactly (SB-14)."""
    if not isinstance(value, str):
        raise StatementError(
            f"the rate for {name} is not a decimal string, "
            f"{value!r}")
    match = RATE_FORM.match(value)
    if not match:
        raise StatementError(
            f"the rate for {name} is not a non-negative "
            f"decimal with at most two fraction digits, "
            f"{value}")
    cents = int(match.group(1)) * 100
    if match.group(2):
        cents += int(match.group(2).ljust(2, "0"))
    return cents


def _amount(cents: int) -> str:
    return f"{cents // 100}.{cents % 100:02d}"


def load_rates(data: bytes) -> dict:
    """A rates card, validated whole (SB-13). Returns
    {currency, cents per class}."""
    try:
        text = data.decode("utf-8-sig")
    except UnicodeDecodeError:
        raise StatementError(
            "the rates file is not utf-8") from None
    try:
        card = json.loads(text)
    except (ValueError, RecursionError):
        raise StatementError(
            "the rates file is not JSON") from None
    if not isinstance(card, dict):
        raise StatementError("the rates file is not an object")
    currency = card.get("currency")
    if not isinstance(currency, str) \
            or not CURRENCY_FORM.match(currency):
        raise StatementError(
            f"the rates file names no currency of three "
            f"letters, {currency!r}")
    per_item = card.get("per_item")
    if not isinstance(per_item, dict):
        raise StatementError(
            "the rates file carries no per_item rates")
    for name in per_item:
        if name not in CLASSES:
            raise StatementError(
                f"the rates file names an unknown class, "
                f"{name}")
    cents = {}
    for name in CLASSES:
        if name not in per_item:
            raise StatementError(
                f"the rates file prices no {name}")
        cents[name] = _rate_cents(per_item[name], name)
    return {"currency": currency, "cents": cents}


def price_statement(document: dict, rates: dict) -> dict:
    """The counts statement plus its priced section
    (SB-14 to SB-16). Only joined items are priced:
    the causes carry exactly those."""
    cents = rates["cents"]
    per_class_counts = {name: 0 for name in CLASSES}
    per_cause = []
    total_cents = 0
    for cause in document["causes"]:
        amount = 0
        for item in cause["items"]:
            per_class_counts[item["class"]] += 1
            amount += cents[item["class"]]
        per_cause.append({"cause": cause["cause"],
                          "amount": _amount(amount)})
        total_cents += amount
    priced = {
        "currency": rates["currency"],
        "items_priced": sum(per_class_counts.values()),
        "per_class": {
            name: {"items": per_class_counts[name],
                   "rate": _amount(cents[name]),
                   "amount": _amount(
                       per_class_counts[name] * cents[name])}
            for name in CLASSES},
        "per_cause": per_cause,
        "total": _amount(total_cents),
    }
    return {**document, "priced": priced}


def _month_window(month: str):
    if not isinstance(month, str) or not MONTH_FORM.match(month):
        raise StatementError(
            f"month must be stated as YYYY-MM, {month}")
    year, mon = (int(part) for part in month.split("-"))
    start = datetime(year, mon, 1, tzinfo=timezone.utc)
    end = (datetime(year + 1, 1, 1, tzinfo=timezone.utc)
           if mon == 12 else
           datetime(year, mon + 1, 1, tzinfo=timezone.utc))
    return start, end


def _resolve_prefix(commits, prefix: str, own_sha: str):
    matches = [c for c in commits
               if c["sha"].startswith(prefix)
               and c["sha"] != own_sha]
    if len(matches) == 1:
        return matches[0], None
    if not matches:
        if own_sha.startswith(prefix):
            return None, (f"the trailer names the item itself, "
                          f"{prefix}")
        return None, f"the trailer names no commit in the read, {prefix}"
    return None, f"the trailer names several commits in the read, {prefix}"


def _trailer_item(commit, all_commits, lines, pattern,
                  kind, join):
    for line in lines:
        match = pattern.match(line)
        if match:
            cause, reason = _resolve_prefix(
                all_commits, match.group(1), commit["sha"])
            return {"class": kind, "join": join,
                    "cause": cause["sha"] if cause else None,
                    "reason": reason}
    return None


def _classify(commit, all_commits, earlier_all, earlier_window):
    """One commit's item, or None. Classes are checked
    in the SB-6a order; the first match is the class."""
    body_lines = commit["body"].splitlines()
    found = _trailer_item(commit, all_commits, body_lines,
                          REVERT_TRAILER, "revert",
                          "revert trailer")
    if found is not None:
        return found
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
    found = _trailer_item(commit, all_commits, body_lines,
                          FIXES_TRAILER, "fix-forward",
                          "fixes trailer")
    if found is not None:
        return found
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
