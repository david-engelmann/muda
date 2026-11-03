//! The statement (SB-6 to SB-16): one month of a
//! repository's history classified into the work
//! that had to be done again, and its pricing in
//! exact integer money (SB-21). Counts only, never
//! currency, until a rates card prices them.

use crate::history::{days_from_civil, py_splitlines, Commit, Fail};
use crate::pyrepr::repr_value;
use crate::util::{py_is_space_char, Msg};
use crate::value::{Text, Value};

pub const CLASSES: [&str; 3] = ["revert", "fix-forward", "redone"];

fn text(value: &str) -> Text {
    Text::from_str(value)
}

fn object(entries: Vec<(&str, Value)>) -> Value {
    Value::Object(
        entries
            .into_iter()
            .map(|(key, value)| (text(key), value))
            .collect(),
    )
}

fn string(value: &str) -> Value {
    Value::Str(text(value))
}

fn int(value: usize) -> Value {
    Value::Int(value.to_string())
}

fn text_of(value: &Value) -> Option<&Text> {
    match value {
        Value::Str(inner) => Some(inner),
        _ => None,
    }
}

// ---------- money: digit-string integer arithmetic ----------

fn money_normalize(digits: &str) -> String {
    let stripped = digits.trim_start_matches('0');
    if stripped.is_empty() {
        "0".to_string()
    } else {
        stripped.to_string()
    }
}

fn money_add(a: &str, b: &str) -> String {
    let (a, b) = (a.as_bytes(), b.as_bytes());
    let mut out = Vec::new();
    let mut carry = 0u8;
    let (mut i, mut j) = (a.len(), b.len());
    while i > 0 || j > 0 || carry > 0 {
        let mut sum = carry;
        if i > 0 {
            i -= 1;
            sum += a[i] - b'0';
        }
        if j > 0 {
            j -= 1;
            sum += b[j] - b'0';
        }
        out.push(b'0' + sum % 10);
        carry = sum / 10;
    }
    out.reverse();
    money_normalize(std::str::from_utf8(&out).unwrap_or("0"))
}

fn money_mul_small(a: &str, n: u64) -> String {
    if n == 0 || a == "0" {
        return "0".to_string();
    }
    let mut out = Vec::new();
    let mut carry: u64 = 0;
    for byte in a.bytes().rev() {
        let total = (byte - b'0') as u64 * n + carry;
        out.push(b'0' + (total % 10) as u8);
        carry = total / 10;
    }
    while carry > 0 {
        out.push(b'0' + (carry % 10) as u8);
        carry /= 10;
    }
    out.reverse();
    money_normalize(std::str::from_utf8(&out).unwrap_or("0"))
}

/// Minor units rendered as the Python's amount:
/// the integer part, a point, two fraction digits.
fn amount(cents: &str) -> String {
    let cents = money_normalize(cents);
    let padded = format!("{:0>3}", cents);
    let split = padded.len() - 2;
    format!("{}.{}", &padded[..split], &padded[split..])
}

// ---------- the rates card (SB-13, SB-13a) ----------

pub struct Rates {
    pub currency: Text,
    pub cents: [String; 3],
}

/// The rate form over a text's units: an integer
/// part of 1 to 30 ASCII digits as written, and an
/// optional fraction of one or two digits, to the
/// very end. Returns the exact minor units.
fn rate_cents(value: &Text) -> Option<String> {
    let units = value.units();
    let mut at = 0;
    let mut int_digits = String::new();
    while at < units.len() && units[at] != 0x2E {
        let unit = units[at];
        if !(0x30..=0x39).contains(&unit) {
            return None;
        }
        int_digits.push(unit as u8 as char);
        at += 1;
    }
    if int_digits.is_empty() || int_digits.len() > 30 {
        return None;
    }
    let mut cents = int_digits;
    if at < units.len() {
        at += 1; // the point
        let mut frac = String::new();
        while at < units.len() {
            let unit = units[at];
            if !(0x30..=0x39).contains(&unit) {
                return None;
            }
            frac.push(unit as u8 as char);
            at += 1;
        }
        if frac.is_empty() || frac.len() > 2 {
            return None;
        }
        while frac.len() < 2 {
            frac.push('0');
        }
        cents.push_str(&frac);
    } else {
        cents.push_str("00");
    }
    Some(money_normalize(&cents))
}

fn currency_ok(value: &Text) -> bool {
    let units = value.units();
    units.len() == 3
        && units
            .iter()
            .all(|&unit| (0x41..=0x5A).contains(&unit) || (0x61..=0x7A).contains(&unit))
}

/// The whole card read, the Python's load_rates
/// over bytes: the utf-8-sig decode, the strict
/// scan with its named refusals, then validation.
pub fn load_rates_bytes(data: &[u8]) -> Result<Rates, Fail> {
    let decoded = match String::from_utf8(data.to_vec()) {
        Ok(text) => text,
        Err(_) => {
            return Err(Fail::Named(Msg::str("the rates file is not utf-8")));
        }
    };
    let decoded = decoded.strip_prefix('\u{FEFF}').unwrap_or(&decoded);
    let units: Vec<u16> = decoded.encode_utf16().collect();
    let card = match crate::card::parse_card(&units) {
        Ok(card) => card,
        Err(crate::card::CardError::Duplicate(key)) => {
            let mut message = Msg::str("the rates file names ");
            message.push_text(&key);
            message.push_str(" twice");
            return Err(Fail::Named(message));
        }
        Err(crate::card::CardError::NotJson) => {
            return Err(Fail::Named(Msg::str("the rates file is not JSON")));
        }
    };
    load_rates(&card)
}

/// The card's own validation over the scanned
/// value, in the Python's order (SB-20). The
/// scanner has already refused duplicates.
pub fn load_rates(card: &Value) -> Result<Rates, Fail> {
    let entries = match card.as_object() {
        Some(entries) => entries,
        None => return Err(Fail::Named(Msg::str("the rates file is not an object"))),
    };
    for (key, _) in entries {
        let known = key
            .as_str()
            .map(|name| name == "currency" || name == "per_item")
            .unwrap_or(false);
        if !known {
            let mut message = Msg::str("the rates file carries an unknown key, ");
            message.push_text(key);
            return Err(Fail::Named(message));
        }
    }
    let currency_value = card.get("currency").cloned().unwrap_or(Value::Null);
    let currency = match &currency_value {
        Value::Str(inner) if currency_ok(inner) => inner.clone(),
        _ => {
            let mut message = Msg::str("the rates file names no currency of three letters, ");
            message.push_str(&repr_value(&currency_value));
            return Err(Fail::Named(message));
        }
    };
    let per_item = match card.get("per_item") {
        Some(value @ Value::Object(_)) => value,
        _ => {
            return Err(Fail::Named(Msg::str(
                "the rates file carries no per_item rates",
            )))
        }
    };
    for (name, _) in per_item.as_object().unwrap() {
        let known = name
            .as_str()
            .map(|inner| CLASSES.iter().any(|&class| class == inner))
            .unwrap_or(false);
        if !known {
            let mut message = Msg::str("the rates file names an unknown class, ");
            message.push_text(name);
            return Err(Fail::Named(message));
        }
    }
    let mut cents: [String; 3] = [String::new(), String::new(), String::new()];
    for (index, name) in CLASSES.iter().enumerate() {
        let rate = match per_item.get(name) {
            Some(rate) => rate,
            None => {
                return Err(Fail::Named(Msg::str(&format!(
                    "the rates file prices no {name}"
                ))))
            }
        };
        match rate {
            Value::Str(inner) => match rate_cents(inner) {
                Some(value) => cents[index] = value,
                None => {
                    let mut message = Msg::str(&format!(
                        "the rate for {name} is not a non-negative decimal with at most two fraction digits, "
                    ));
                    message.push_text(inner);
                    return Err(Fail::Named(message));
                }
            },
            _ => {
                let mut message =
                    Msg::str(&format!("the rate for {name} is not a decimal string, "));
                message.push_str(&repr_value(rate));
                return Err(Fail::Named(message));
            }
        }
    }
    Ok(Rates { currency, cents })
}

// ---------- pricing (SB-14 to SB-16) ----------

pub fn price_statement(document: Value, rates: &Rates) -> Value {
    let mut per_class_counts = [0u64; 3];
    let mut per_cause: Vec<Value> = Vec::new();
    let mut total = "0".to_string();
    if let Some(causes) = document.get("causes").and_then(Value::as_array).cloned() {
        for cause in &causes {
            let mut cause_amount = "0".to_string();
            if let Some(items) = cause.get("items").and_then(Value::as_array) {
                for item in items {
                    if let Some(class) = item.get("class").and_then(text_of) {
                        if let Some(index) = CLASSES
                            .iter()
                            .position(|&name| class.units() == text(name).units())
                        {
                            per_class_counts[index] += 1;
                            cause_amount = money_add(&cause_amount, &rates.cents[index]);
                        }
                    }
                }
            }
            total = money_add(&total, &cause_amount);
            per_cause.push(object(vec![
                ("cause", cause.get("cause").cloned().unwrap_or(Value::Null)),
                ("amount", string(&amount(&cause_amount))),
            ]));
        }
    }
    let per_class: Vec<(Text, Value)> = CLASSES
        .iter()
        .enumerate()
        .map(|(index, name)| {
            (
                text(name),
                object(vec![
                    ("items", int(per_class_counts[index] as usize)),
                    ("rate", string(&amount(&rates.cents[index]))),
                    (
                        "amount",
                        string(&amount(&money_mul_small(
                            &rates.cents[index],
                            per_class_counts[index],
                        ))),
                    ),
                ]),
            )
        })
        .collect();
    let priced = object(vec![
        ("currency", Value::Str(rates.currency.clone())),
        (
            "items_priced",
            int(per_class_counts.iter().sum::<u64>() as usize),
        ),
        ("per_class", Value::Object(per_class)),
        ("per_cause", Value::Array(per_cause)),
        ("total", string(&amount(&total))),
    ]);
    let mut document = document;
    if let Value::Object(entries) = &mut document {
        entries.push((text("priced"), priced));
    }
    document
}

// ---------- the statement itself (SB-6 to SB-12) ----------

struct Item {
    sha: String,
    class: &'static str,
    join: &'static str,
    cause: Option<String>,
    reason: Option<String>,
}

fn trailer_prefix(line: &str, revert: bool) -> Option<String> {
    let stripped: &str = {
        let trimmed = line.trim_end_matches(py_is_space_char);
        trimmed
    };
    let rest = if revert {
        stripped.strip_prefix("This reverts commit ")?
    } else {
        stripped.strip_prefix("Fixes: ")?
    };
    let hex_len = rest
        .chars()
        .take_while(|ch| ch.is_ascii_hexdigit() && !ch.is_ascii_uppercase())
        .count();
    if !(7..=40).contains(&hex_len) {
        return None;
    }
    let tail = &rest[hex_len..];
    if tail.is_empty() || (revert && tail == ".") {
        Some(rest[..hex_len].to_string())
    } else {
        None
    }
}

fn resolve_prefix(
    commits: &[Commit],
    prefix: &str,
    own_sha: &str,
) -> (Option<String>, Option<String>) {
    let matches: Vec<&Commit> = commits
        .iter()
        .filter(|commit| commit.sha.starts_with(prefix) && commit.sha != own_sha)
        .collect();
    if matches.len() == 1 {
        return (Some(matches[0].sha.clone()), None);
    }
    if matches.is_empty() {
        if own_sha.starts_with(prefix) {
            return (
                None,
                Some(format!("the trailer names the item itself, {prefix}")),
            );
        }
        return (
            None,
            Some(format!("the trailer names no commit in the read, {prefix}")),
        );
    }
    (
        None,
        Some(format!(
            "the trailer names several commits in the read, {prefix}"
        )),
    )
}

fn trailer_item(
    commit: &Commit,
    commits: &[Commit],
    lines: &[String],
    revert: bool,
    class: &'static str,
    join: &'static str,
) -> Option<Item> {
    for line in lines {
        if let Some(prefix) = trailer_prefix(line, revert) {
            let (cause, reason) = resolve_prefix(commits, &prefix, &commit.sha);
            return Some(Item {
                sha: commit.sha.clone(),
                class,
                join,
                cause,
                reason,
            });
        }
    }
    None
}

fn classify(
    commit: &Commit,
    commits: &[Commit],
    earlier_all: &[Commit],
    earlier_window: &[&Commit],
) -> Option<Item> {
    let lines = py_splitlines(&commit.body);
    if let Some(found) = trailer_item(commit, commits, &lines, true, "revert", "revert trailer") {
        return Some(found);
    }
    let subject = &commit.subject;
    if subject.len() > 8 && subject.starts_with("Revert \"") && subject.ends_with('"') {
        let quoted = &subject[8..subject.len() - 1];
        let candidates: Vec<&Commit> = earlier_all
            .iter()
            .filter(|candidate| candidate.subject == quoted)
            .collect();
        if candidates.len() == 1 {
            return Some(Item {
                sha: commit.sha.clone(),
                class: "revert",
                join: "revert subject",
                cause: Some(candidates[0].sha.clone()),
                reason: None,
            });
        }
        let reason = if candidates.is_empty() {
            "no earlier commit carries the quoted subject"
        } else {
            "several earlier commits carry the quoted subject"
        };
        return Some(Item {
            sha: commit.sha.clone(),
            class: "revert",
            join: "revert subject",
            cause: None,
            reason: Some(reason.to_string()),
        });
    }
    if let Some(found) = trailer_item(
        commit,
        commits,
        &lines,
        false,
        "fix-forward",
        "fixes trailer",
    ) {
        return Some(found);
    }
    let matches: Vec<&Commit> = earlier_window
        .iter()
        .copied()
        .filter(|candidate| {
            candidate.subject == commit.subject
                && candidate
                    .paths
                    .iter()
                    .any(|path| commit.paths.contains(path))
        })
        .collect();
    if !matches.is_empty() {
        let join = if matches.len() == 1 {
            "redone task"
        } else {
            "redone task, earliest of several matches"
        };
        return Some(Item {
            sha: commit.sha.clone(),
            class: "redone",
            join,
            cause: Some(matches[0].sha.clone()),
            reason: None,
        });
    }
    None
}

/// The month form over units: exactly YYYY-MM,
/// ASCII digits, the month 01 to 12, to the very
/// end (SB-13a).
fn month_form(units: &[u16]) -> Option<(i64, u32)> {
    if units.len() != 7 || units[4] != 0x2D {
        return None;
    }
    let digit = |at: usize| -> Option<u32> {
        let unit = units[at];
        if (0x30..=0x39).contains(&unit) {
            Some((unit - 0x30) as u32)
        } else {
            None
        }
    };
    let year = (digit(0)? * 1000 + digit(1)? * 100 + digit(2)? * 10 + digit(3)?) as i64;
    let month = digit(5)? * 10 + digit(6)?;
    if !(1..=12).contains(&month) {
        return None;
    }
    Some((year, month))
}

pub fn build_statement(commits: &[Commit], month: &[u16]) -> Result<Value, Fail> {
    let (year, mon) = match month_form(month) {
        Some(parsed) => parsed,
        None => {
            let mut message = Msg::str("month must be stated as YYYY-MM, ");
            message.push_units(month);
            return Err(Fail::Named(message));
        }
    };
    // The window's construction is the crash class
    // at the edges the form admits (year 0, and
    // December of 9999 needing a year 10000 end).
    if year == 0 || (year == 9999 && mon == 12) {
        return Err(Fail::Crash);
    }
    let start_days = days_from_civil(year, mon, 1);
    let (end_year, end_mon) = if mon == 12 {
        (year + 1, 1)
    } else {
        (year, mon + 1)
    };
    let end_days = days_from_civil(end_year, end_mon, 1);
    let start = start_days as i128 * 86_400_000_000;
    let end = end_days as i128 * 86_400_000_000;
    let date_text = |y: i64, m: u32| format!("{:04}-{:02}-01", y, m);
    let merges = commits.iter().filter(|c| c.parents.len() > 1).count();
    let in_window: Vec<&Commit> = commits
        .iter()
        .filter(|c| start <= c.author_micros && c.author_micros < end)
        .collect();
    let position: std::collections::HashMap<&str, usize> = commits
        .iter()
        .enumerate()
        .map(|(index, commit)| (commit.sha.as_str(), index))
        .collect();
    let mut items: Vec<Item> = Vec::new();
    for commit in &in_window {
        if commit.parents.len() > 1 {
            continue;
        }
        let at = position[commit.sha.as_str()];
        let earlier_all = &commits[..at];
        let earlier_window: Vec<&Commit> = in_window
            .iter()
            .copied()
            .filter(|candidate| position[candidate.sha.as_str()] < at)
            .collect();
        if let Some(found) = classify(commit, commits, earlier_all, &earlier_window) {
            items.push(found);
        }
    }
    // Causes keyed by the causing commit, in that
    // commit's read position order.
    let mut cause_order: Vec<&str> = Vec::new();
    for item in &items {
        if let Some(cause) = &item.cause {
            if !cause_order.contains(&cause.as_str()) {
                cause_order.push(cause.as_str());
            }
        }
    }
    cause_order.sort_by_key(|cause| position[*cause]);
    let mut causes_json = Vec::new();
    let mut per_cause_counts: Vec<(Text, Value)> = Vec::new();
    for cause in &cause_order {
        let entries: Vec<Value> = items
            .iter()
            .filter(|item| item.cause.as_deref() == Some(cause))
            .map(|item| {
                object(vec![
                    ("sha", string(&item.sha)),
                    ("class", string(item.class)),
                    ("join", string(item.join)),
                ])
            })
            .collect();
        let count = entries.len();
        causes_json.push(object(vec![
            ("cause", string(cause)),
            ("items", Value::Array(entries)),
        ]));
        per_cause_counts.push((text(cause), int(count)));
    }
    let unjoined: Vec<Value> = items
        .iter()
        .filter(|item| item.cause.is_none())
        .map(|item| {
            object(vec![
                ("sha", string(&item.sha)),
                ("class", string(item.class)),
                ("join", string(item.join)),
                ("reason", string(item.reason.as_deref().unwrap_or(""))),
            ])
        })
        .collect();
    let per_class: Vec<(Text, Value)> = CLASSES
        .iter()
        .map(|&name| {
            (
                text(name),
                int(items.iter().filter(|item| item.class == name).count()),
            )
        })
        .collect();
    Ok(object(vec![
        ("month", Value::Str(Text::from_units(month.to_vec()))),
        (
            "window",
            object(vec![
                ("from", string(&date_text(year, mon))),
                ("to", string(&date_text(end_year, end_mon))),
            ]),
        ),
        ("commits_read", int(commits.len())),
        ("merges_read", int(merges)),
        ("causes", Value::Array(causes_json)),
        ("unjoined", Value::Array(unjoined)),
        (
            "totals",
            object(vec![
                ("items", int(items.len())),
                ("per_class", Value::Object(per_class)),
                ("per_cause", Value::Object(per_cause_counts)),
                (
                    "unjoined",
                    int(items.iter().filter(|i| i.cause.is_none()).count()),
                ),
            ]),
        ),
    ]))
}
