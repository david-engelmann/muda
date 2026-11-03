//! The git read (SB-5, SB-5a, SB-5b, SB-19). One
//! repository, read and never written, through the
//! git subprocess with the caller's git environment
//! scrubbed, so replace objects, grafts, and
//! redirects from the environment never reach the
//! read. Every refusal names the repository.

use std::io::Read;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use crate::util::{py_is_space_char, strerror_text, Msg};

/// One commit of the read: the fields the statement
/// classifies over, ordered as the read produced
/// them until read_history sorts by author time.
pub struct Commit {
    pub sha: String,
    pub parents: Vec<String>,
    pub author_micros: i128,
    pub subject: String,
    pub body: String,
    pub paths: Vec<String>,
}

/// A named refusal (the Python's StatementError),
/// or the crash class, where the Python dies
/// uncaught (an author time its reader cannot
/// parse).
#[derive(Debug)]
pub enum Fail {
    Named(Msg),
    Crash,
}

const SCRUBBED: [&str; 8] = [
    "GIT_DIR",
    "GIT_WORK_TREE",
    "GIT_INDEX_FILE",
    "GIT_OBJECT_DIRECTORY",
    "GIT_ALTERNATE_OBJECT_DIRECTORIES",
    "GIT_NAMESPACE",
    "GIT_PREFIX",
    "GIT_CEILING",
];

fn repo_msg(repo: &[u16]) -> Msg {
    let mut message = Msg::str("cannot read the repository at ");
    // The Python interpolates str(repo): the
    // normalized path text.
    message.push_units(repo);
    message
}

fn cannot_read(repo: &[u16], detail: &Msg) -> Fail {
    let mut message = repo_msg(repo);
    message.push_str(": ");
    message.push_msg(detail);
    Fail::Named(message)
}

/// Python's str.strip() over a decoded string.
fn py_strip(text: &str) -> &str {
    text.trim_matches(py_is_space_char)
}

/// Python's str.splitlines(): lines split on the
/// wide boundary set, a CR LF pair counting once.
pub fn py_splitlines(text: &str) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current = String::new();
    let mut chars = text.chars().peekable();
    while let Some(ch) = chars.next() {
        match ch {
            '\n' => {
                lines.push(std::mem::take(&mut current));
            }
            '\r' => {
                if chars.peek() == Some(&'\n') {
                    chars.next();
                }
                lines.push(std::mem::take(&mut current));
            }
            '\u{0B}' | '\u{0C}' | '\u{1C}' | '\u{1D}' | '\u{1E}' | '\u{85}' | '\u{2028}'
            | '\u{2029}' => {
                lines.push(std::mem::take(&mut current));
            }
            _ => current.push(ch),
        }
    }
    if !current.is_empty() {
        lines.push(current);
    }
    lines
}

/// The interpreter's day count for a civil date
/// (Howard Hinnant's algorithm), proleptic
/// Gregorian.
pub fn days_from_civil(year: i64, month: u32, day: u32) -> i64 {
    let y = if month <= 2 { year - 1 } else { year };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let mp = ((month + 9) % 12) as i64;
    let doy = (153 * mp + 2) / 5 + day as i64 - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146097 + doe - 719468
}

/// One git call. `repo` is the normalized path in
/// units (for the phrases) and the OS path it came
/// from is the same text encoded for the OS.
fn git(repo: &[u16], args: &[&str], check: bool) -> Result<String, Fail> {
    let repo_os = crate::util::os_path_from_units(repo);
    let mut command = Command::new("git");
    command
        .arg("--no-replace-objects")
        .arg("--no-optional-locks")
        .arg("-C")
        .arg(&repo_os)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    for name in SCRUBBED {
        command.env_remove(name);
    }
    command.env("GIT_GRAFT_FILE", "/dev/null");
    let mut child = match command.spawn() {
        Ok(child) => child,
        Err(err) => {
            let mut detail = Msg::new();
            detail.push_str(&format!(
                "[Errno {}] {}: 'git'",
                err.raw_os_error().unwrap_or(0),
                strerror_text(&err)
            ));
            // The Python's OSError string carries no
            // errno prefix text beyond this shape when
            // the errno is unknown to it; the table in
            // strerror_text covers the spawn failures.
            return Err(cannot_read(repo, &detail));
        }
    };
    let mut out_pipe = child.stdout.take();
    let mut err_pipe = child.stderr.take();
    let out_thread = std::thread::spawn(move || {
        let mut buf = Vec::new();
        if let Some(mut pipe) = out_pipe.take() {
            let _ = pipe.read_to_end(&mut buf);
        }
        buf
    });
    let err_thread = std::thread::spawn(move || {
        let mut buf = Vec::new();
        if let Some(mut pipe) = err_pipe.take() {
            let _ = pipe.read_to_end(&mut buf);
        }
        buf
    });
    let deadline = Instant::now() + Duration::from_secs(60);
    let status = loop {
        match child.try_wait() {
            Ok(Some(status)) => break Some(status),
            Ok(None) => {
                if Instant::now() >= deadline {
                    let _ = child.kill();
                    let _ = child.wait();
                    break None;
                }
                std::thread::sleep(Duration::from_millis(5));
            }
            Err(_) => break None,
        }
    };
    let stdout = out_thread.join().unwrap_or_default();
    let stderr = err_thread.join().unwrap_or_default();
    let status = match status {
        Some(status) => status,
        None => {
            // The timeout shape: the Python's
            // TimeoutExpired names the command list.
            let mut list = String::from("Command '['git'");
            for extra in ["--no-replace-objects", "--no-optional-locks", "-C"] {
                list.push_str(", '");
                list.push_str(extra);
                list.push('\'');
            }
            list.push_str(", '");
            list.push_str(&String::from_utf16_lossy(repo));
            list.push('\'');
            for arg in args {
                list.push_str(", '");
                list.push_str(arg);
                list.push('\'');
            }
            list.push_str("]' timed out after 60 seconds");
            return Err(cannot_read(repo, &Msg::str(&list)));
        }
    };
    if !status.success() {
        if !check {
            return Ok(String::new());
        }
        let detail_text = String::from_utf8_lossy(&stderr).into_owned();
        let detail = py_strip(&detail_text);
        let detail = if detail.is_empty() {
            Msg::str("git failed")
        } else {
            Msg::str(detail)
        };
        return Err(cannot_read(repo, &detail));
    }
    match String::from_utf8(stdout) {
        Ok(text) => Ok(text),
        Err(_) => {
            let mut message = repo_msg(repo);
            message.push_str(": a commit message is not valid UTF-8");
            Err(Fail::Named(message))
        }
    }
}

/// git's %aI instant: an ISO date, a T, an ISO time,
/// and an offset (or Z), to microseconds since the
/// epoch in UTC. Anything else is the crash class,
/// where the Python's fromisoformat raises uncaught.
fn parse_author(text: &str) -> Option<i128> {
    let bytes = text.as_bytes();
    if bytes.len() < 20 {
        return None;
    }
    let digits = |from: usize, count: usize| -> Option<i64> {
        let mut value: i64 = 0;
        for at in from..from + count {
            let byte = *bytes.get(at)?;
            if !byte.is_ascii_digit() {
                return None;
            }
            value = value * 10 + (byte - b'0') as i64;
        }
        Some(value)
    };
    if bytes.get(4) != Some(&b'-') || bytes.get(7) != Some(&b'-') {
        return None;
    }
    let year = digits(0, 4)?;
    let month = digits(5, 2)? as u32;
    let day = digits(8, 2)? as u32;
    if bytes.get(10) != Some(&b'T') || bytes.get(13) != Some(&b':') || bytes.get(16) != Some(&b':')
    {
        return None;
    }
    let hour = digits(11, 2)?;
    let minute = digits(14, 2)?;
    let second = digits(17, 2)?;
    if !(1..=12).contains(&month)
        || !(1..=31).contains(&day)
        || hour > 23
        || minute > 59
        || second > 59
    {
        return None;
    }
    let rest = &text[19..];
    let offset_seconds: i64 = if rest == "Z" {
        0
    } else {
        let rb = rest.as_bytes();
        if rb.len() != 6 || rb.get(3) != Some(&b':') {
            return None;
        }
        let sign = match rb.first() {
            Some(b'+') => 1,
            Some(b'-') => -1,
            _ => return None,
        };
        let oh = {
            let mut value: i64 = 0;
            for at in 1..3 {
                let byte = *rb.get(at)?;
                if !byte.is_ascii_digit() {
                    return None;
                }
                value = value * 10 + (byte - b'0') as i64;
            }
            value
        };
        let om = {
            let mut value: i64 = 0;
            for at in 4..6 {
                let byte = *rb.get(at)?;
                if !byte.is_ascii_digit() {
                    return None;
                }
                value = value * 10 + (byte - b'0') as i64;
            }
            value
        };
        sign * (oh * 3600 + om * 60)
    };
    let days = days_from_civil(year, month, day);
    let local = days * 86_400 + hour * 3600 + minute * 60 + second;
    Some((local - offset_seconds) as i128 * 1_000_000)
}

/// Every commit reachable from HEAD, ordered by
/// author time (ties by read position, the sort is
/// stable), with the stated field set.
pub fn read_history(repo: &[u16]) -> Result<Vec<Commit>, Fail> {
    git(repo, &["rev-parse", "--git-dir"], true)?;
    let head = git(repo, &["rev-parse", "--verify", "HEAD"], false)?;
    if py_strip(&head).is_empty() {
        let mut message = Msg::str("no commits in the repository at ");
        message.push_units(repo);
        return Err(Fail::Named(message));
    }
    let shallow = git(repo, &["rev-parse", "--is-shallow-repository"], true)?;
    if py_strip(&shallow) == "true" {
        let mut message = Msg::str("the repository at ");
        message.push_units(repo);
        message.push_str(" is shallow; a statement needs the whole history");
        return Err(Fail::Named(message));
    }
    let format = "%H%x1f%P%x1f%aI%x1f%s%x1f%b%x1e";
    let raw = git(
        repo,
        &["log", "--reverse", &format!("--format={format}")],
        true,
    )?;
    let paths_raw = git(
        repo,
        &["log", "--reverse", "--name-status", "--format=%x1e%H"],
        true,
    )?;
    let mut paths_by_sha: std::collections::HashMap<String, Vec<String>> =
        std::collections::HashMap::new();
    for record in paths_raw.split('\u{1e}') {
        let lines: Vec<String> = py_splitlines(record)
            .into_iter()
            .filter(|line| !line.is_empty())
            .collect();
        if lines.is_empty() {
            continue;
        }
        let mut paths: Vec<String> = Vec::new();
        for entry in &lines[1..] {
            let fields: Vec<&str> = entry.split('\t').collect();
            for field in &fields[1..] {
                paths.push(field.to_string());
            }
        }
        paths.sort();
        paths.dedup();
        paths_by_sha.insert(lines[0].clone(), paths);
    }
    let mut commits = Vec::new();
    for record in raw.split('\u{1e}') {
        let record = record.trim_matches('\n');
        if record.is_empty() {
            continue;
        }
        let parts: Vec<&str> = record.splitn(5, '\u{1f}').collect();
        if parts.len() != 5 {
            let mut message = repo_msg(repo);
            message.push_str(": a commit record could not be parsed");
            return Err(Fail::Named(message));
        }
        let (sha, parents, author, subject, body) =
            (parts[0], parts[1], parts[2], parts[3], parts[4]);
        let author_micros = match parse_author(author) {
            Some(micros) => micros,
            None => return Err(Fail::Crash),
        };
        commits.push(Commit {
            sha: sha.to_string(),
            parents: if parents.is_empty() {
                Vec::new()
            } else {
                parents.split_whitespace().map(str::to_string).collect()
            },
            author_micros,
            subject: subject.to_string(),
            body: body.to_string(),
            paths: paths_by_sha.get(sha).cloned().unwrap_or_default(),
        });
    }
    if commits.is_empty() {
        let mut message = Msg::str("no commits in the repository at ");
        message.push_units(repo);
        return Err(Fail::Named(message));
    }
    commits.sort_by_key(|commit| commit.author_micros);
    Ok(commits)
}

/// The top of the work tree, for the output guard.
pub fn worktree_root(repo: &[u16]) -> Result<String, Fail> {
    Ok(py_strip(&git(repo, &["rev-parse", "--show-toplevel"], true)?).to_string())
}

/// The repository's git directory, absolute, for
/// the output guard.
pub fn git_dir(repo: &[u16]) -> Result<String, Fail> {
    Ok(py_strip(&git(repo, &["rev-parse", "--absolute-git-dir"], true)?).to_string())
}
