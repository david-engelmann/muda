//! SB-5 to SB-16, ported file for file from the
//! Python suite: the git side statement, priced.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::OnceLock;

use secondbill::emit::{dumps_document, Mode};
use secondbill::history::{read_history, Commit, Fail};
use secondbill::statement::{build_statement, load_rates_bytes, price_statement};
use secondbill::value::{Text, Value};

fn git(repo: &Path, args: &[&str], date: Option<&str>, author: Option<&str>) -> Output {
    let mut command = Command::new("git");
    command
        .arg("-C")
        .arg(repo)
        .arg("-c")
        .arg("user.name=Synthetic")
        .arg("-c")
        .arg("user.email=synthetic@example.invalid")
        .arg("-c")
        .arg("commit.gpgsign=false")
        .args(args)
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env_remove("GIT_DIR")
        .env_remove("GIT_WORK_TREE");
    if let Some(date) = date {
        command.env("GIT_AUTHOR_DATE", author.unwrap_or(date));
        command.env("GIT_COMMITTER_DATE", date);
    }
    command.output().expect("git runs")
}

fn git_ok(repo: &Path, args: &[&str], date: Option<&str>, author: Option<&str>) -> Output {
    let out = git(repo, args, date, author);
    assert!(
        out.status.success(),
        "git {args:?}: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    out
}

fn commit(
    repo: &Path,
    subject: &str,
    date: &str,
    files: &[(&str, &str)],
    body: Option<&str>,
    author: Option<&str>,
) -> String {
    for (name, content) in files {
        let target = repo.join(name);
        std::fs::create_dir_all(target.parent().unwrap()).unwrap();
        std::fs::write(&target, content).unwrap();
    }
    git_ok(repo, &["add", "-A"], None, None);
    let mut args = vec!["commit", "-q", "-m", subject];
    if let Some(body) = body {
        args.push("-m");
        args.push(body);
    }
    git_ok(repo, &args, Some(date), author);
    let out = git_ok(repo, &["rev-parse", "HEAD"], None, None);
    String::from_utf8(out.stdout).unwrap().trim().to_string()
}

fn temp_dir(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "secondbill-test-{}-{}-{}",
        tag,
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

struct Fixture {
    repo: PathBuf,
    shas: HashMap<&'static str, String>,
}

fn fixture() -> &'static Fixture {
    static FIXTURE: OnceLock<Fixture> = OnceLock::new();
    FIXTURE.get_or_init(|| {
        let repo = temp_dir("repo");
        git_ok(&repo, &["init", "-q", "-b", "main"], None, None);
        let mut shas: HashMap<&'static str, String> = HashMap::new();
        shas.insert(
            "c1",
            commit(
                &repo,
                "Add the parser",
                "2026-01-05T09:00:00Z",
                &[("src/parser.py", "v1\n")],
                None,
                None,
            ),
        );
        git_ok(&repo, &["checkout", "-q", "-b", "side"], None, None);
        shas.insert(
            "cB",
            commit(
                &repo,
                "Add branch work",
                "2026-01-06T09:00:00Z",
                &[("src/branch.py", "b\n")],
                None,
                None,
            ),
        );
        git_ok(&repo, &["checkout", "-q", "main"], None, None);
        shas.insert(
            "cM",
            commit(
                &repo,
                "Main work",
                "2026-01-06T10:00:00Z",
                &[("src/main.py", "m\n")],
                None,
                None,
            ),
        );
        let merge = Command::new("git")
            .arg("-C")
            .arg(&repo)
            .arg("-c")
            .arg("user.name=Synthetic")
            .arg("-c")
            .arg("user.email=synthetic@example.invalid")
            .args(["merge", "-q", "--no-ff", "-m", "Merge branch side", "side"])
            .env("GIT_CONFIG_NOSYSTEM", "1")
            .env("GIT_AUTHOR_DATE", "2026-01-07T09:00:00Z")
            .env("GIT_COMMITTER_DATE", "2026-01-07T09:00:00Z")
            .output()
            .unwrap();
        assert!(
            merge.status.success(),
            "{}",
            String::from_utf8_lossy(&merge.stderr)
        );
        shas.insert("merge", {
            let out = git_ok(&repo, &["rev-parse", "HEAD"], None, None);
            String::from_utf8(out.stdout).unwrap().trim().to_string()
        });
        shas.insert(
            "c2",
            commit(
                &repo,
                "Add the parser",
                "2026-01-12T09:00:00Z",
                &[("src/parser.py", "v2\n"), ("src/extra.py", "e\n")],
                None,
                None,
            ),
        );
        let fixes_c1 = format!("Fixes: {}", &shas["c1"][..12]);
        shas.insert(
            "c3",
            commit(
                &repo,
                "Repair the parser edge",
                "2026-01-13T09:00:00Z",
                &[("src/parser.py", "v3\n")],
                Some(&fixes_c1),
                None,
            ),
        );
        let revert_c2 = format!("This reverts commit {}.", shas["c2"]);
        shas.insert(
            "c4",
            commit(
                &repo,
                "Revert \"Add the parser\"",
                "2026-01-14T09:00:00Z",
                &[("src/parser.py", "v1\n")],
                Some(&revert_c2),
                None,
            ),
        );
        shas.insert(
            "c5",
            commit(
                &repo,
                "Revert \"Add the parser\"",
                "2026-01-15T09:00:00Z",
                &[("src/parser.py", "v2\n")],
                None,
                None,
            ),
        );
        shas.insert(
            "c6",
            commit(
                &repo,
                "Fix the tokenizer",
                "2026-01-16T09:00:00Z",
                &[("src/tok.py", "t\n")],
                Some("Fixes: deadbeef"),
                None,
            ),
        );
        shas.insert(
            "c7",
            commit(
                &repo,
                "Tidy the docs",
                "2026-01-20T09:00:00Z",
                &[("README.md", "docs\n")],
                None,
                None,
            ),
        );
        shas.insert(
            "c9",
            commit(
                &repo,
                "Add the parser",
                "2026-01-22T09:00:00Z",
                &[("src/other.py", "o\n")],
                None,
                None,
            ),
        );
        shas.insert(
            "c10",
            commit(
                &repo,
                "Add the parser",
                "2026-01-25T09:00:00Z",
                &[("src/parser.py", "v4\n")],
                None,
                None,
            ),
        );
        shas.insert(
            "cY",
            commit(
                &repo,
                "Repeat me",
                "2026-01-26T09:00:00Z",
                &[("src/repeat.py", "y\n")],
                None,
                Some("2026-01-20T09:00:00Z"),
            ),
        );
        shas.insert(
            "cX",
            commit(
                &repo,
                "Repeat me",
                "2026-01-25T09:00:00Z",
                &[("src/repeat.py", "x\n")],
                None,
                None,
            ),
        );
        let fixes_c1_again = format!("Fixes: {}", &shas["c1"][..12]);
        shas.insert(
            "c12",
            commit(
                &repo,
                "Add the parser",
                "2026-01-27T09:00:00Z",
                &[("src/parser.py", "v6\n")],
                Some(&fixes_c1_again),
                None,
            ),
        );
        let revert_c3 = format!("This reverts commit {}.", shas["c3"]);
        shas.insert(
            "c11",
            commit(
                &repo,
                "Revert \"Repair the parser edge\"",
                "2026-02-02T09:00:00Z",
                &[("src/parser.py", "v1\n")],
                Some(&revert_c3),
                None,
            ),
        );
        shas.insert(
            "c8",
            commit(
                &repo,
                "Add the parser",
                "2026-02-03T09:00:00Z",
                &[("src/parser.py", "v5\n")],
                None,
                None,
            ),
        );
        Fixture { repo, shas }
    })
}

fn units(text: &str) -> Vec<u16> {
    text.encode_utf16().collect()
}

fn repo_units(f: &Fixture) -> Vec<u16> {
    units(&f.repo.to_string_lossy())
}

fn named(failure: Fail) -> String {
    match failure {
        Fail::Named(message) => message.render_stderr(),
        Fail::Crash => panic!("expected a named refusal, got the crash class"),
    }
}

fn j<'a>(value: &'a Value, key: &str) -> &'a Value {
    value.get(key).unwrap_or_else(|| panic!("no key {key}"))
}

fn s(value: &Value) -> String {
    match value {
        Value::Str(text) => text.as_str().expect("clean text"),
        other => panic!("not a string: {other:?}"),
    }
}

fn n(value: &Value) -> i64 {
    match value {
        Value::Int(digits) => digits.parse().unwrap(),
        other => panic!("not an int: {other:?}"),
    }
}

fn arr(value: &Value) -> &Vec<Value> {
    value.as_array().expect("an array")
}

/// The January statement over the shared fixture,
/// byte for byte as the Python renders it. The pin
/// is a literal, not the crate's own renderer over
/// the crate's own builder, so a rendering drift
/// fails here and not only in the differential
/// harness.
const JANUARY_DOCUMENT: &str = r#"{
  "month": "2026-01",
  "window": {
    "from": "2026-01-01",
    "to": "2026-02-01"
  },
  "commits_read": 17,
  "merges_read": 1,
  "causes": [
    {
      "cause": "6799ef527c8055da078336e8d5807ac40e195305",
      "items": [
        {
          "sha": "6fa465cedc20c0899567389cba4faa0e8a38620a",
          "class": "redone",
          "join": "redone task"
        },
        {
          "sha": "f3052c290163c8a0c775e9cdd60f06c2f7c9058e",
          "class": "fix-forward",
          "join": "fixes trailer"
        },
        {
          "sha": "48e25e85e584eae359acf10749a660d7cd5effed",
          "class": "redone",
          "join": "redone task, earliest of several matches"
        },
        {
          "sha": "e8305a126e8f837c1fada9592221a1c3f984a1e0",
          "class": "fix-forward",
          "join": "fixes trailer"
        }
      ]
    },
    {
      "cause": "6fa465cedc20c0899567389cba4faa0e8a38620a",
      "items": [
        {
          "sha": "29fc726aae854a645a13fe7997f32a6ed8a8f77a",
          "class": "revert",
          "join": "revert trailer"
        }
      ]
    },
    {
      "cause": "cdc8d81cd01903ccab3e2130e556cd2b32e8e9c9",
      "items": [
        {
          "sha": "aa77aca182be334a1c52af507c043398c28c7471",
          "class": "redone",
          "join": "redone task"
        }
      ]
    }
  ],
  "unjoined": [
    {
      "sha": "f1038687b9210184ece4c7b8d981955142a72bcd",
      "class": "revert",
      "join": "revert subject",
      "reason": "several earlier commits carry the quoted subject"
    },
    {
      "sha": "bbbb25c6624495bfb8198e29f27ecbf4ddd35ee6",
      "class": "fix-forward",
      "join": "fixes trailer",
      "reason": "the trailer names no commit in the read, deadbeef"
    }
  ],
  "totals": {
    "items": 8,
    "per_class": {
      "revert": 2,
      "fix-forward": 3,
      "redone": 3
    },
    "per_cause": {
      "6799ef527c8055da078336e8d5807ac40e195305": 4,
      "6fa465cedc20c0899567389cba4faa0e8a38620a": 1,
      "cdc8d81cd01903ccab3e2130e556cd2b32e8e9c9": 1
    },
    "unjoined": 2
  }
}
"#;

#[test]
fn the_january_statement() {
    let f = fixture();
    let commits = read_history(&repo_units(f)).unwrap();
    let document = build_statement(&commits, &units("2026-01")).unwrap();
    assert_eq!(n(j(&document, "commits_read")), 17);
    assert_eq!(n(j(&document, "merges_read")), 1);
    assert_eq!(s(j(j(&document, "window"), "from")), "2026-01-01");
    assert_eq!(s(j(j(&document, "window"), "to")), "2026-02-01");
    let mut by_cause: HashMap<String, Vec<(String, String, String)>> = HashMap::new();
    for cause in arr(j(&document, "causes")) {
        let items = arr(j(cause, "items"))
            .iter()
            .map(|item| (s(j(item, "sha")), s(j(item, "class")), s(j(item, "join"))))
            .collect();
        by_cause.insert(s(j(cause, "cause")), items);
    }
    assert_eq!(by_cause.len(), 3);
    let sh = |key: &str| f.shas[key].clone();
    assert_eq!(
        by_cause[&sh("c1")],
        vec![
            (sh("c2"), "redone".into(), "redone task".into()),
            (sh("c3"), "fix-forward".into(), "fixes trailer".into()),
            (
                sh("c10"),
                "redone".into(),
                "redone task, earliest of several matches".into()
            ),
            (sh("c12"), "fix-forward".into(), "fixes trailer".into()),
        ]
    );
    assert_eq!(
        by_cause[&sh("c2")],
        vec![(sh("c4"), "revert".into(), "revert trailer".into())]
    );
    assert_eq!(
        by_cause[&sh("cY")],
        vec![(sh("cX"), "redone".into(), "redone task".into())]
    );
    let unjoined: HashMap<(String, String), String> = arr(j(&document, "unjoined"))
        .iter()
        .map(|u| ((s(j(u, "sha")), s(j(u, "class"))), s(j(u, "reason"))))
        .collect();
    assert_eq!(unjoined.len(), 2);
    assert_eq!(
        unjoined[&(sh("c5"), "revert".into())],
        "several earlier commits carry the quoted subject"
    );
    assert_eq!(
        unjoined[&(sh("c6"), "fix-forward".into())],
        "the trailer names no commit in the read, deadbeef"
    );
    let totals = j(&document, "totals");
    assert_eq!(totals.as_object().unwrap().len(), 4);
    assert_eq!(j(totals, "per_class").as_object().unwrap().len(), 3);
    assert_eq!(j(totals, "per_cause").as_object().unwrap().len(), 3);
    assert_eq!(n(j(totals, "items")), 8);
    assert_eq!(n(j(j(totals, "per_class"), "revert")), 2);
    assert_eq!(n(j(j(totals, "per_class"), "fix-forward")), 3);
    assert_eq!(n(j(j(totals, "per_class"), "redone")), 3);
    assert_eq!(n(j(j(totals, "per_cause"), &sh("c1"))), 4);
    assert_eq!(n(j(j(totals, "per_cause"), &sh("c2"))), 1);
    assert_eq!(n(j(j(totals, "per_cause"), &sh("cY"))), 1);
    assert_eq!(n(j(totals, "unjoined")), 2);
    let mut item_shas: Vec<String> = Vec::new();
    for cause in arr(j(&document, "causes")) {
        for item in arr(j(cause, "items")) {
            item_shas.push(s(j(item, "sha")));
        }
    }
    for u in arr(j(&document, "unjoined")) {
        item_shas.push(s(j(u, "sha")));
    }
    assert!(!item_shas.contains(&sh("merge")));
}

#[test]
fn the_february_statement_bills_an_out_of_month_cause() {
    let f = fixture();
    let commits = read_history(&repo_units(f)).unwrap();
    let document = build_statement(&commits, &units("2026-02")).unwrap();
    assert_eq!(n(j(j(&document, "totals"), "items")), 1);
    let causes = arr(j(&document, "causes"));
    assert_eq!(causes.len(), 1);
    assert_eq!(s(j(&causes[0], "cause")), f.shas["c3"]);
    let items = arr(j(&causes[0], "items"));
    assert_eq!(s(j(&items[0], "sha")), f.shas["c11"]);
    assert_eq!(s(j(&items[0], "class")), "revert");
    assert_eq!(s(j(&items[0], "join")), "revert trailer");
    assert!(arr(j(&document, "unjoined")).is_empty());
}

#[test]
fn the_statement_is_deterministic() {
    let f = fixture();
    let commits = read_history(&repo_units(f)).unwrap();
    let first = dumps_document(
        &build_statement(&commits, &units("2026-01")).unwrap(),
        Mode::Stdout,
    )
    .unwrap();
    let commits = read_history(&repo_units(f)).unwrap();
    let second = dumps_document(
        &build_statement(&commits, &units("2026-01")).unwrap(),
        Mode::Stdout,
    )
    .unwrap();
    assert_eq!(first, second);
}

#[test]
fn unreadable_and_empty_repositories_are_refused() {
    let dir = temp_dir("plain");
    let failure = read_history(&units(&dir.to_string_lossy())).err().unwrap();
    assert!(named(failure).contains("cannot read"));
    let empty = dir.join("empty");
    std::fs::create_dir_all(&empty).unwrap();
    git_ok(&empty, &["init", "-q", "-b", "main"], None, None);
    let failure = read_history(&units(&empty.to_string_lossy()))
        .err()
        .unwrap();
    let text = named(failure);
    assert!(text.contains("cannot read") || text.contains("no commits"));
}

#[test]
fn the_field_set_is_complete() {
    let f = fixture();
    let commits = read_history(&repo_units(f)).unwrap();
    let by_sha: HashMap<&str, &Commit> = commits.iter().map(|c| (c.sha.as_str(), c)).collect();
    let merge = by_sha[f.shas["merge"].as_str()];
    assert_eq!(merge.parents.len(), 2);
    let c3 = by_sha[f.shas["c3"].as_str()];
    assert_eq!(c3.subject, "Repair the parser edge");
    assert!(c3.body.contains(&format!("Fixes: {}", &f.shas["c1"][..12])));
    assert_eq!(c3.paths, vec!["src/parser.py".to_string()]);
    // 2026-01-13T09:00:00+00:00 in epoch
    // microseconds, as a literal: the Python pins
    // the instant itself, not a figure recomputed
    // by the code under test.
    assert_eq!(c3.author_micros, 1_768_294_800_000_000);
}

fn run_bin(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_secondbill"))
        .args(args)
        .output()
        .expect("the binary runs")
}

#[test]
fn the_cli_and_its_guards() {
    let f = fixture();
    let dir = temp_dir("cli");
    let out = dir.join("statement.json");
    let repo_text = f.repo.to_string_lossy().into_owned();
    let run = run_bin(&[
        "statement",
        &repo_text,
        "--month",
        "2026-01",
        "-o",
        &out.to_string_lossy(),
    ]);
    assert_eq!(
        run.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&run.stderr)
    );
    assert_eq!(std::fs::read(&out).unwrap(), JANUARY_DOCUMENT.as_bytes());
    let inside = f.repo.join("statement.json");
    let run = run_bin(&[
        "statement",
        &repo_text,
        "--month",
        "2026-01",
        "-o",
        &inside.to_string_lossy(),
    ]);
    assert_eq!(run.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&run.stderr).contains("writes nothing inside"));
    assert!(!inside.exists());
    let run = run_bin(&["statement", &repo_text, "--month", "2026-13"]);
    assert_eq!(run.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&run.stderr).contains("YYYY-MM"));
}

#[test]
fn a_shallow_repository_is_refused() {
    let f = fixture();
    let dir = temp_dir("shallow");
    let clone = dir.join("shallow");
    let out = Command::new("git")
        .args(["clone", "-q", "--depth", "1"])
        .arg(format!("file://{}", f.repo.display()))
        .arg(&clone)
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let failure = read_history(&units(&clone.to_string_lossy()))
        .err()
        .unwrap();
    assert!(named(failure).contains("shallow"));
}

#[test]
fn the_environment_never_redirects_the_read() {
    // The surface that must scrub is the binary's
    // own subprocess environment, so the case runs
    // against the compiled binary with the variables
    // set on that command alone. The Python set them
    // through monkeypatch, which restores after the
    // test; mutating this test process's environment
    // would race every other test's git calls.
    let f = fixture();
    let repo_text = f.repo.to_string_lossy().into_owned();
    let run = Command::new(env!("CARGO_BIN_EXE_secondbill"))
        .args(["statement", &repo_text, "--month", "2026-01"])
        .env("GIT_DIR", "/nonexistent/elsewhere")
        .env("GIT_WORK_TREE", "/nonexistent")
        .output()
        .expect("the binary runs");
    assert_eq!(
        run.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&run.stderr)
    );
    let text = String::from_utf8(run.stdout).unwrap();
    assert!(text.contains("\"commits_read\": 17"), "{text}");
}

fn synthetic_commit(
    sha: &str,
    parents: &[&str],
    micros: i128,
    subject: &str,
    body: &str,
    paths: &[&str],
) -> Commit {
    Commit {
        sha: sha.to_string(),
        parents: parents.iter().map(|p| p.to_string()).collect(),
        author_micros: micros,
        subject: subject.to_string(),
        body: body.to_string(),
        paths: paths.iter().map(|p| p.to_string()).collect(),
    }
}

fn jan(day: u32) -> i128 {
    secondbill::history::days_from_civil(2026, 1, day) as i128 * 86_400_000_000
}

#[test]
fn a_merge_is_never_an_item_even_with_a_trailer() {
    let base = synthetic_commit(&"a".repeat(40), &["p"], jan(5), "Base", "", &["a.py"]);
    let merge = synthetic_commit(
        &"m".repeat(40),
        &["p", "q"],
        jan(6),
        "Merge",
        &format!("Fixes: {}", "a".repeat(12)),
        &[],
    );
    let document = build_statement(&[base, merge], &units("2026-01")).unwrap();
    assert_eq!(n(j(j(&document, "totals"), "items")), 0);
    let base = synthetic_commit(&"a".repeat(40), &["p"], jan(5), "Base", "", &["a.py"]);
    let plain = synthetic_commit(
        &"b".repeat(40),
        &["p"],
        jan(6),
        "Merge",
        &format!("Fixes: {}", "a".repeat(12)),
        &[],
    );
    let document = build_statement(&[base, plain], &units("2026-01")).unwrap();
    assert_eq!(n(j(j(&document, "totals"), "items")), 1);
}

#[test]
fn a_trailer_naming_the_item_itself_is_unjoined() {
    let commit = synthetic_commit(
        &"c".repeat(40),
        &[],
        jan(5),
        "Self fix",
        &format!("Fixes: {}", "c".repeat(12)),
        &["a.py"],
    );
    let document = build_statement(&[commit], &units("2026-01")).unwrap();
    let unjoined = arr(j(&document, "unjoined"));
    assert!(s(j(&unjoined[0], "reason")).starts_with("the trailer names the item itself"));
}

#[test]
fn an_indented_trailer_is_text_not_a_trailer() {
    let dir = temp_dir("quoted");
    let repo = dir.join("quoted");
    std::fs::create_dir_all(&repo).unwrap();
    git_ok(&repo, &["init", "-q", "-b", "main"], None, None);
    let first = commit(
        &repo,
        "Base work",
        "2026-01-05T09:00:00Z",
        &[("a.py", "1\n")],
        None,
        None,
    );
    let body = format!("Example:\n\n    This reverts commit {first}.");
    commit(
        &repo,
        "Document the revert",
        "2026-01-06T09:00:00Z",
        &[("b.py", "2\n")],
        Some(&body),
        None,
    );
    let commits = read_history(&units(&repo.to_string_lossy())).unwrap();
    let document = build_statement(&commits, &units("2026-01")).unwrap();
    assert_eq!(n(j(j(&document, "totals"), "items")), 0);
}

const CARD: &str = "{\"currency\": \"USD\", \"per_item\": {\"revert\": \"10.00\", \"fix-forward\": \"0.10\", \"redone\": \"3.33\"}}";

fn priced_fixture() -> (Value, Value, HashMap<&'static str, String>) {
    let f = fixture();
    let commits = read_history(&repo_units(f)).unwrap();
    let document = build_statement(&commits, &units("2026-01")).unwrap();
    let rates = load_rates_bytes(CARD.as_bytes()).unwrap();
    (
        price_statement(document.clone(), &rates),
        document,
        f.shas.clone(),
    )
}

#[test]
fn the_priced_statement_closes() {
    let (priced, plain, shas) = priced_fixture();
    let section = j(&priced, "priced");
    assert_eq!(s(j(section, "currency")), "USD");
    assert_eq!(n(j(section, "items_priced")), 6);
    assert_eq!(n(j(j(&priced, "totals"), "items")), 8);
    let per_class = j(section, "per_class");
    for (name, items, rate, amount) in [
        ("revert", 1, "10.00", "10.00"),
        ("fix-forward", 2, "0.10", "0.20"),
        ("redone", 3, "3.33", "9.99"),
    ] {
        let entry = j(per_class, name);
        assert_eq!(n(j(entry, "items")), items);
        assert_eq!(s(j(entry, "rate")), rate);
        assert_eq!(s(j(entry, "amount")), amount);
    }
    let per_cause = arr(j(section, "per_cause"));
    let expected = [
        (shas["c1"].as_str(), "6.86"),
        (shas["c2"].as_str(), "10.00"),
        (shas["cY"].as_str(), "3.33"),
    ];
    assert_eq!(per_cause.len(), 3);
    for (entry, (cause, amount)) in per_cause.iter().zip(expected) {
        assert_eq!(s(j(entry, "cause")), cause);
        assert_eq!(s(j(entry, "amount")), amount);
    }
    assert_eq!(s(j(section, "total")), "20.19");
    // The closure the Python states: each family of
    // amounts sums to the total.
    let cents = |text: &str| -> i64 {
        let (whole, frac) = text.split_once('.').unwrap();
        whole.parse::<i64>().unwrap() * 100 + frac.parse::<i64>().unwrap()
    };
    let class_sum: i64 = ["revert", "fix-forward", "redone"]
        .iter()
        .map(|name| cents(&s(j(j(per_class, name), "amount"))))
        .sum();
    assert_eq!(class_sum, cents(&s(j(section, "total"))));
    let cause_sum: i64 = per_cause
        .iter()
        .map(|entry| cents(&s(j(entry, "amount"))))
        .sum();
    assert_eq!(cause_sum, cents(&s(j(section, "total"))));
    // The counts statement is untouched beside the
    // priced section.
    if let (Value::Object(plain_entries), Value::Object(priced_entries)) = (&plain, &priced) {
        for (key, value) in plain_entries {
            let found = priced_entries
                .iter()
                .find(|(name, _)| name == key)
                .map(|(_, value)| value);
            assert_eq!(found, Some(value), "key {:?}", key.as_str());
        }
    } else {
        panic!("objects");
    }
}

#[test]
fn no_rates_no_priced_section() {
    let f = fixture();
    let commits = read_history(&repo_units(f)).unwrap();
    let document = build_statement(&commits, &units("2026-01")).unwrap();
    assert!(document.get("priced").is_none());
}

#[test]
fn equivalent_cards_price_identically() {
    let f = fixture();
    let commits = read_history(&repo_units(f)).unwrap();
    let document = build_statement(&commits, &units("2026-01")).unwrap();
    let other = "{\"currency\": \"USD\", \"per_item\": {\"revert\": \"10\", \"fix-forward\": \"0.1\", \"redone\": \"3.33\"}}";
    let first = price_statement(
        document.clone(),
        &load_rates_bytes(CARD.as_bytes()).unwrap(),
    );
    let second = price_statement(document, &load_rates_bytes(other.as_bytes()).unwrap());
    assert_eq!(j(&first, "priced"), j(&second, "priced"));
}

fn refuses(data: &[u8], fragment: &str) {
    let failure = load_rates_bytes(data).err().unwrap();
    let text = named(failure);
    assert!(text.contains(fragment), "want {fragment:?} in {text:?}");
}

#[test]
fn bad_cards_refuse_by_name() {
    let card = |per_item: &str, currency: &str| -> Vec<u8> {
        format!("{{\"currency\": \"{currency}\", \"per_item\": {per_item}}}").into_bytes()
    };
    refuses(
        &card("{\"revert\": \"1.00\", \"fix-forward\": \"1.00\"}", "USD"),
        "prices no redone",
    );
    refuses(
        &card("{\"revert\": \"1.00\", \"fix-forward\": \"0.10\", \"redone\": \"3.33\", \"merge\": \"1.00\"}", "USD"),
        "unknown class",
    );
    refuses(
        &card(
            "{\"revert\": \"1.005\", \"fix-forward\": \"0.10\", \"redone\": \"3.33\"}",
            "USD",
        ),
        "at most two",
    );
    refuses(
        &card(
            "{\"revert\": 10.0, \"fix-forward\": \"0.10\", \"redone\": \"3.33\"}",
            "USD",
        ),
        "not a decimal",
    );
    refuses(
        &card(
            "{\"revert\": \"-1.00\", \"fix-forward\": \"0.10\", \"redone\": \"3.33\"}",
            "USD",
        ),
        "at most two",
    );
    refuses(
        &card(
            "{\"revert\": \"1.00\", \"fix-forward\": \"0.10\", \"redone\": \"3.33\"}",
            "US",
        ),
        "currency",
    );
    refuses(b"rates go here", "not JSON");
}

#[test]
fn the_rates_command_and_missing_file() {
    let f = fixture();
    let dir = temp_dir("ratescmd");
    let rates = dir.join("rates.json");
    std::fs::write(&rates, CARD).unwrap();
    let out = dir.join("statement.json");
    let repo_text = f.repo.to_string_lossy().into_owned();
    let run = run_bin(&[
        "statement",
        &repo_text,
        "--month",
        "2026-01",
        "--rates",
        &rates.to_string_lossy(),
        "-o",
        &out.to_string_lossy(),
    ]);
    assert_eq!(
        run.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&run.stderr)
    );
    let text = std::fs::read_to_string(&out).unwrap();
    assert!(text.contains("\"total\": \"20.19\""));
    let missing = dir.join("missing.json");
    let run = run_bin(&[
        "statement",
        &repo_text,
        "--month",
        "2026-01",
        "--rates",
        &missing.to_string_lossy(),
    ]);
    assert_eq!(run.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&run.stderr).contains("cannot read the rates file"));
}

#[test]
fn duplicate_keys_refuse_by_name() {
    refuses(
        b"{\"currency\": \"USD\", \"currency\": \"EUR\", \"per_item\": {\"revert\": \"1.00\", \"fix-forward\": \"0.10\", \"redone\": \"3.33\"}}",
        "names currency twice",
    );
    refuses(
        b"{\"currency\": \"USD\", \"per_item\": {\"revert\": \"1.00\", \"revert\": \"999.00\", \"fix-forward\": \"0.10\", \"redone\": \"3.33\"}}",
        "names revert twice",
    );
    refuses(
        b"{\"currency\": \"USD\", \"per_item\": {\"rev\\u0065rt\": \"1.00\", \"revert\": \"9.00\", \"fix-forward\": \"0.10\", \"redone\": \"3.33\"}}",
        "names revert twice",
    );
    refuses(
        b"{\"currency\": \"USD\", \"per_item\": {\"revert\": \"1.00\", \"fix-forward\": \"0.10\", \"redone\": \"3.33\"}, \"per_item\": {\"revert\": \"7.00\", \"fix-forward\": \"0.10\", \"redone\": \"3.33\"}}",
        "names per_item twice",
    );
}

#[test]
fn card_level_refusals_are_all_named() {
    refuses(b"\xff\xfe not utf-8", "not utf-8");
    refuses(b"[1, 2, 3]", "not an object");
    refuses(b"{\"currency\": \"USD\"}", "no per_item");
    refuses(
        b"{\"currency\": \"USD\", \"discount\": \"0.50\", \"per_item\": {\"revert\": \"1.00\", \"fix-forward\": \"0.10\", \"redone\": \"3.33\"}}",
        "unknown key, discount",
    );
    refuses(
        "{\"currency\": \"USD\\n\", \"per_item\": {\"revert\": \"1.00\", \"fix-forward\": \"0.10\", \"redone\": \"3.33\"}}".as_bytes(),
        "no currency",
    );
    refuses(
        b"{\"currency\": \"USD\", \"per_item\": {\"revert\": \"1.00\\n\", \"fix-forward\": \"0.10\", \"redone\": \"3.33\"}}",
        "rate for revert",
    );
    refuses(
        "{\"currency\": \"USD\", \"per_item\": {\"revert\": \"\\u0661\\u0662.\\u0663\\u0664\", \"fix-forward\": \"0.10\", \"redone\": \"3.33\"}}".as_bytes(),
        "rate for revert",
    );
}

#[test]
fn oversized_rates_refuse_by_name() {
    let with_revert = |rate: &str| -> Vec<u8> {
        format!(
            "{{\"currency\": \"USD\", \"per_item\": {{\"revert\": \"{rate}\", \"fix-forward\": \"0.10\", \"redone\": \"3.33\"}}}}"
        )
        .into_bytes()
    };
    refuses(&with_revert(&"9".repeat(5000)), "rate for revert");
    refuses(
        &with_revert(&format!("{}1.00", "0".repeat(5000))),
        "rate for revert",
    );
    let rates = load_rates_bytes(&with_revert(&format!("{}.99", "9".repeat(30)))).unwrap();
    // int("9" * 30) * 100 + 99: thirty nines of
    // units times a hundred, plus 99, is thirty-two
    // nines of minor units.
    assert_eq!(rates.cents[0], "9".repeat(32));
}

#[test]
fn the_priced_key_sets_are_exact() {
    let (priced, plain, _) = priced_fixture();
    let keyset = |value: &Value| -> Vec<String> {
        let mut keys: Vec<String> = value
            .as_object()
            .unwrap()
            .iter()
            .map(|(key, _)| key.as_str().unwrap())
            .collect();
        keys.sort();
        keys
    };
    let mut want = keyset(&plain);
    want.push("priced".to_string());
    want.sort();
    assert_eq!(keyset(&priced), want);
    let section = j(&priced, "priced");
    assert_eq!(
        keyset(section),
        vec![
            "currency",
            "items_priced",
            "per_cause",
            "per_class",
            "total"
        ]
    );
    if let Value::Object(per_class) = j(section, "per_class") {
        for (_, entry) in per_class {
            assert_eq!(keyset(entry), vec!["amount", "items", "rate"]);
        }
    }
    for entry in arr(j(section, "per_cause")) {
        assert_eq!(keyset(entry), vec!["amount", "cause"]);
    }
}

#[test]
fn pricing_is_deterministic_and_exact_at_scale() {
    let (priced, plain, _) = priced_fixture();
    let rates = load_rates_bytes(CARD.as_bytes()).unwrap();
    assert_eq!(price_statement(plain.clone(), &rates), priced);
    let big = "{\"currency\": \"USD\", \"per_item\": {\"revert\": \"9007199254740993.00\", \"fix-forward\": \"0.10\", \"redone\": \"3.33\"}}";
    let document = Value::Object(vec![
        (
            Text::from_str("causes"),
            Value::Array(vec![Value::Object(vec![
                (
                    Text::from_str("cause"),
                    Value::Str(Text::from_str(&"c".repeat(40))),
                ),
                (
                    Text::from_str("items"),
                    Value::Array(vec![
                        Value::Object(vec![
                            (
                                Text::from_str("sha"),
                                Value::Str(Text::from_str(&"a".repeat(40))),
                            ),
                            (
                                Text::from_str("class"),
                                Value::Str(Text::from_str("revert")),
                            ),
                        ]),
                        Value::Object(vec![
                            (
                                Text::from_str("sha"),
                                Value::Str(Text::from_str(&"b".repeat(40))),
                            ),
                            (
                                Text::from_str("class"),
                                Value::Str(Text::from_str("revert")),
                            ),
                        ]),
                        Value::Object(vec![
                            (
                                Text::from_str("sha"),
                                Value::Str(Text::from_str(&"d".repeat(40))),
                            ),
                            (
                                Text::from_str("class"),
                                Value::Str(Text::from_str("revert")),
                            ),
                        ]),
                    ]),
                ),
            ])]),
        ),
        (Text::from_str("unjoined"), Value::Array(vec![])),
        (
            Text::from_str("totals"),
            Value::Object(vec![(Text::from_str("items"), Value::Int("3".to_string()))]),
        ),
    ]);
    let section = price_statement(document, &load_rates_bytes(big.as_bytes()).unwrap());
    let priced = j(&section, "priced");
    assert_eq!(
        s(j(j(j(priced, "per_class"), "revert"), "amount")),
        "27021597764222979.00"
    );
    assert_eq!(s(j(priced, "total")), "27021597764222979.00");
}

#[test]
fn no_rates_output_is_the_round_1_document() {
    let f = fixture();
    let dir = temp_dir("plainout");
    let out = dir.join("plain.json");
    let repo_text = f.repo.to_string_lossy().into_owned();
    let run = run_bin(&[
        "statement",
        &repo_text,
        "--month",
        "2026-01",
        "-o",
        &out.to_string_lossy(),
    ]);
    assert_eq!(run.status.code(), Some(0));
    assert_eq!(std::fs::read(&out).unwrap(), JANUARY_DOCUMENT.as_bytes());
}

#[test]
fn a_malformed_card_through_the_command() {
    let f = fixture();
    let dir = temp_dir("badcard");
    let rates = dir.join("rates.json");
    std::fs::write(
        &rates,
        b"{\"currency\": \"USD\", \"currency\": \"EUR\", \"per_item\": {\"revert\": \"1.00\", \"fix-forward\": \"0.10\", \"redone\": \"3.33\"}}",
    )
    .unwrap();
    let repo_text = f.repo.to_string_lossy().into_owned();
    let run = run_bin(&[
        "statement",
        &repo_text,
        "--month",
        "2026-01",
        "--rates",
        &rates.to_string_lossy(),
    ]);
    assert_eq!(run.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&run.stderr).contains("names currency twice"));
}
