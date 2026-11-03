//! The command line (SB-11, SB-11a): statement,
//! and the argument parser that matches argparse's
//! bytes, including which error fires first. The
//! parser acts token by token, as argparse does: a
//! token's options are matched in full (a missing
//! value is that token's terminal error), then the
//! token's actions execute, help included. Extras
//! from the subcommand are reported by the top
//! parser, with the top parser's usage, after the
//! run's own checks.

use std::io::Write;
use std::path::{Path, PathBuf};

use crate::emit::{dumps_document, Mode};
use crate::helptext::{STATEMENT_HELP, STATEMENT_USAGE, TOP_HELP, TOP_USAGE};
use crate::history::{read_history, Fail};
use crate::statement::{build_statement, price_statement};
use crate::util::{closed_at_launch, fsdecode_units, os_path_from_units, Msg};
use crate::value::Value;

struct ClosedFlags {
    stdout: bool,
    stderr: bool,
}

pub fn run_cli() -> i32 {
    let closed = ClosedFlags {
        stdout: closed_at_launch(1),
        stderr: closed_at_launch(2),
    };
    let os_args: Vec<Vec<u8>> = std::env::args_os()
        .map(|arg| {
            #[cfg(unix)]
            {
                use std::os::unix::ffi::OsStrExt;
                arg.as_os_str().as_bytes().to_vec()
            }
            #[cfg(not(unix))]
            {
                arg.to_string_lossy().into_owned().into_bytes()
            }
        })
        .collect();
    let argv: Vec<Vec<u16>> = os_args.iter().map(|arg| fsdecode_units(arg)).collect();
    run_with_args(&argv[1.min(argv.len())..], &closed)
}

fn parse_error(failure: &ParseFailure, closed: &ClosedFlags) -> i32 {
    if closed.stderr {
        // argparse prints its usage through the
        // surviving stream, but its exit message is
        // written to the missing stderr and lost:
        // stdout carries the usage alone.
        let mut text = String::new();
        text.push_str(failure.usage);
        text.push('\n');
        let _ = std::io::stdout().write_all(text.as_bytes());
        let _ = std::io::stdout().flush();
        return 2;
    }
    let mut text = String::new();
    text.push_str(failure.usage);
    text.push('\n');
    text.push_str(failure.prog);
    text.push_str(": error: ");
    text.push_str(&failure.message.render_stderr());
    text.push('\n');
    let _ = std::io::stderr().write_all(text.as_bytes());
    let _ = std::io::stderr().flush();
    2
}

fn help_text(text: &str, closed: &ClosedFlags) -> i32 {
    print_help(text, closed)
}

/// argparse's explicit help argument error (-h=x):
/// the usage and message go to stderr, exit 2.
fn help_explicit(
    prog: &'static str,
    usage: &'static str,
    arg: &[u16],
    closed: &ClosedFlags,
) -> i32 {
    let mut message = Msg::str("argument -h/--help: ignored explicit argument ");
    message.push_str(&crate::pyrepr::repr_text(&crate::value::Text::from_units(
        arg.to_vec(),
    )));
    parse_error(
        &ParseFailure {
            prog,
            usage,
            message,
        },
        closed,
    )
}

fn print_help(text: &str, closed: &ClosedFlags) -> i32 {
    if closed.stdout {
        let _ = std::io::stderr().write_all(text.as_bytes());
        let _ = std::io::stderr().flush();
        return 0;
    }
    let mut out = std::io::stdout();
    match out.write_all(text.as_bytes()).and_then(|_| out.flush()) {
        Ok(()) => 0,
        Err(_) => {
            let _ = std::io::stderr().write_all(text.as_bytes());
            let _ = std::io::stderr().flush();
            0
        }
    }
}

struct ParseFailure {
    prog: &'static str,
    usage: &'static str,
    message: Msg,
}

fn invalid_choice_top(value: &[u16], closed: &ClosedFlags) -> i32 {
    let mut message = Msg::str("argument command: invalid choice: ");
    message.push_str(&crate::pyrepr::repr_text(&crate::value::Text::from_units(
        value.to_vec(),
    )));
    message.push_str(" (choose from 'statement')");
    parse_error(
        &ParseFailure {
            prog: "secondbill",
            usage: TOP_USAGE,
            message,
        },
        closed,
    )
}

fn required_error_top(closed: &ClosedFlags) -> i32 {
    parse_error(
        &ParseFailure {
            prog: "secondbill",
            usage: TOP_USAGE,
            message: Msg::str("the following arguments are required: command"),
        },
        closed,
    )
}

fn extras_error_top(extras: &[Vec<u16>], closed: &ClosedFlags) -> i32 {
    let mut message = Msg::str("unrecognized arguments: ");
    for (index, extra) in extras.iter().enumerate() {
        if index > 0 {
            message.push_str(" ");
        }
        message.push_units(extra);
    }
    parse_error(
        &ParseFailure {
            prog: "secondbill",
            usage: TOP_USAGE,
            message,
        },
        closed,
    )
}

#[derive(Clone, Copy, PartialEq)]
enum LongMatch {
    Help,
    Month,
    Rates,
    Output,
    Unknown,
    Ambiguous,
}

fn match_long(token: &[u16]) -> LongMatch {
    // token starts with "--"; the name is the part
    // before any '='. The option names are encoded
    // once, here, and compared as slices.
    let name: Vec<u16> = token[2..]
        .iter()
        .take_while(|&&unit| unit != 0x3D)
        .copied()
        .collect();
    let options: [(Vec<u16>, LongMatch); 4] = [
        ("help".encode_utf16().collect(), LongMatch::Help),
        ("month".encode_utf16().collect(), LongMatch::Month),
        ("rates".encode_utf16().collect(), LongMatch::Rates),
        ("output".encode_utf16().collect(), LongMatch::Output),
    ];
    if name.is_empty() {
        // "--=..." prefix-matches every long option.
        return LongMatch::Ambiguous;
    }
    if let Some((_, found)) = options.iter().find(|(option, _)| *option == name) {
        return *found;
    }
    let matches: Vec<LongMatch> = options
        .iter()
        .filter(|(option, _)| option.len() > name.len() && option[..name.len()] == name[..])
        .map(|(_, found)| *found)
        .collect();
    match matches.len() {
        1 => matches[0],
        0 => LongMatch::Unknown,
        _ => LongMatch::Ambiguous,
    }
}

fn match_long_top(token: &[u16]) -> LongMatch {
    let name: Vec<u16> = token[2..]
        .iter()
        .take_while(|&&unit| unit != 0x3D)
        .copied()
        .collect();
    let help: Vec<u16> = "help".encode_utf16().collect();
    if name.is_empty() || (help.len() >= name.len() && help.iter().take(name.len()).eq(name.iter()))
    {
        LongMatch::Help
    } else {
        LongMatch::Unknown
    }
}

fn is_option_like(token: &[u16]) -> bool {
    if token.len() < 2 || token[0] != 0x2D {
        return false;
    }
    if token.len() == 2 && token[1] == 0x2D {
        // "--" is not option-like in itself; that it
        // can never serve as an option's value is
        // next_value's rule, and the parse loops meet
        // it as the separator.
        return false;
    }
    // argparse's negative number rule: a token that
    // is entirely a negative number is a value when
    // the parser defines no numeric options.
    !is_negative_number(token)
}

fn is_negative_number(token: &[u16]) -> bool {
    let scalars = crate::util::scalars_of(token);
    if scalars.len() < 2 || scalars[0] != 0x2D {
        return false;
    }
    let rest = &scalars[1..];
    let mut at = 0;
    let mut int_digits = 0;
    while at < rest.len() && crate::util::decimal_digit(rest[at]).is_some() {
        at += 1;
        int_digits += 1;
    }
    if at == rest.len() {
        return int_digits > 0;
    }
    if rest[at] == 0x2E {
        at += 1;
        let mut frac_digits = 0;
        while at < rest.len() && crate::util::decimal_digit(rest[at]).is_some() {
            at += 1;
            frac_digits += 1;
        }
        return at == rest.len() && frac_digits > 0;
    }
    false
}

fn token_glued_value(token: &[u16]) -> Option<Vec<u16>> {
    token
        .iter()
        .position(|&unit| unit == 0x3D)
        .map(|at| token[at + 1..].to_vec())
}

fn next_value(tokens: &[Vec<u16>], at: usize) -> Option<Vec<u16>> {
    let candidate = tokens.get(at + 1)?;
    // "--" is never an option's value: argparse
    // meets it as the separator and reports the
    // option's value missing.
    if is_option_like(candidate) || candidate == &vec![0x2D, 0x2D] {
        None
    } else {
        Some(candidate.clone())
    }
}

struct StatementArgs {
    repo: Vec<u16>,
    month: Vec<u16>,
    rates: Option<Vec<u16>>,
    output: Option<Vec<u16>>,
}

enum SubOutcome {
    Done(i32),
    Parsed(Box<StatementArgs>, Vec<Vec<u16>>),
}

fn sub_error(message: Msg, closed: &ClosedFlags) -> i32 {
    parse_error(
        &ParseFailure {
            prog: "secondbill statement",
            usage: STATEMENT_USAGE,
            message,
        },
        closed,
    )
}

fn ambiguous_sub(token: &[u16], closed: &ClosedFlags) -> i32 {
    let mut message = Msg::str("ambiguous option: ");
    message.push_units(token);
    message.push_str(" could match --help, --month, --rates, --output");
    sub_error(message, closed)
}

fn missing_value_sub(name: &str, closed: &ClosedFlags) -> i32 {
    sub_error(
        Msg::str(&format!("argument {name}: expected one argument")),
        closed,
    )
}

/// A help token's signed argument, at the top level
/// or in the subcommand.
fn help_token_outcome(
    token: &[u16],
    prog: &'static str,
    usage: &'static str,
    closed: &ClosedFlags,
) -> Option<i32> {
    if let Some(glued) = token_glued_value(token) {
        return Some(help_explicit(prog, usage, &glued, closed));
    }
    None
}

fn parse_statement(tokens: &[Vec<u16>], closed: &ClosedFlags) -> SubOutcome {
    let mut repo: Option<Vec<u16>> = None;
    let mut month: Option<Vec<u16>> = None;
    let mut rates: Option<Vec<u16>> = None;
    let mut output: Option<Vec<u16>> = None;
    let mut extras: Vec<Vec<u16>> = Vec::new();
    let mut positional_only = false;
    let mut at = 0;
    while at < tokens.len() {
        let token = &tokens[at];
        if !positional_only && token == &vec![0x2D, 0x2D] {
            if repo.is_none() || month.is_none() {
                positional_only = true;
                at += 1;
            } else {
                // With every required argument already
                // satisfied, argparse does not take
                // the token as a separator: it and
                // everything after it stand as extras,
                // verbatim, reported by the top parser.
                extras.extend(tokens[at..].iter().cloned());
                break;
            }
        } else if !positional_only && token.len() > 2 && token[0] == 0x2D && token[1] == 0x2D {
            // A long option: matched once, in full,
            // then acted on.
            let matched = match_long(token);
            match matched {
                LongMatch::Ambiguous => return SubOutcome::Done(ambiguous_sub(token, closed)),
                LongMatch::Unknown => {
                    extras.push(token.clone());
                    at += 1;
                }
                LongMatch::Help => {
                    if let Some(done) =
                        help_token_outcome(token, "secondbill statement", STATEMENT_USAGE, closed)
                    {
                        return SubOutcome::Done(done);
                    }
                    return SubOutcome::Done(help_text(STATEMENT_HELP, closed));
                }
                LongMatch::Month | LongMatch::Rates | LongMatch::Output => {
                    let (slot, name) = match matched {
                        LongMatch::Month => (&mut month, "--month"),
                        LongMatch::Rates => (&mut rates, "--rates"),
                        _ => (&mut output, "-o/--output"),
                    };
                    if let Some(glued) = token_glued_value(token) {
                        *slot = Some(glued);
                        at += 1;
                    } else if let Some(value) = next_value(tokens, at) {
                        *slot = Some(value);
                        at += 2;
                    } else {
                        return SubOutcome::Done(missing_value_sub(name, closed));
                    }
                }
            }
        } else if !positional_only
            && token.len() >= 2
            && token[0] == 0x2D
            && !is_negative_number(token)
        {
            // A short cluster, walked character by
            // character as argparse matches it. 'h'
            // records help, and an '=' glued to it is
            // the ignored-explicit-argument error at
            // this parser. 'o' ends the token, taking
            // the rest as its value (one leading '='
            // stripped) or the next token as its
            // value. An unknown character ends the
            // walk: the token is an extra only when
            // nothing was matched before it, and help
            // already recorded still fires. A missing
            // value fires ahead of help; a fully
            // matched token acts help first.
            let mut help = false;
            let mut extra = false;
            let mut output_value: Option<Option<Vec<u16>>> = None;
            let mut terminal: Option<i32> = None;
            let mut index = 1;
            while index < token.len() {
                match token[index] {
                    0x68 => {
                        if token.get(index + 1) == Some(&0x3D) {
                            terminal = Some(help_explicit(
                                "secondbill statement",
                                STATEMENT_USAGE,
                                &token[index + 2..],
                                closed,
                            ));
                            break;
                        }
                        help = true;
                        index += 1;
                    }
                    0x6F => {
                        let rest: Vec<u16> = token[index + 1..].to_vec();
                        if rest.is_empty() {
                            output_value = Some(None);
                        } else if rest[0] == 0x3D {
                            output_value = Some(Some(rest[1..].to_vec()));
                        } else {
                            output_value = Some(Some(rest));
                        }
                        index = token.len();
                    }
                    _ => {
                        if !help && output_value.is_none() {
                            extra = true;
                        }
                        break;
                    }
                }
            }
            if let Some(code) = terminal {
                return SubOutcome::Done(code);
            }
            if extra {
                extras.push(token.clone());
                at += 1;
                continue;
            }
            match output_value {
                Some(Some(value)) => {
                    if help {
                        return SubOutcome::Done(help_text(STATEMENT_HELP, closed));
                    }
                    output = Some(value);
                    at += 1;
                }
                Some(None) => match next_value(tokens, at) {
                    Some(value) => {
                        if help {
                            return SubOutcome::Done(help_text(STATEMENT_HELP, closed));
                        }
                        output = Some(value);
                        at += 2;
                    }
                    None => {
                        return SubOutcome::Done(missing_value_sub("-o/--output", closed));
                    }
                },
                None => {
                    if help {
                        return SubOutcome::Done(help_text(STATEMENT_HELP, closed));
                    }
                    at += 1;
                }
            }
        } else {
            if repo.is_none() {
                repo = Some(token.clone());
            } else {
                extras.push(token.clone());
            }
            at += 1;
        }
    }
    let mut missing: Vec<&str> = Vec::new();
    if repo.is_none() {
        missing.push("repo");
    }
    if month.is_none() {
        missing.push("--month");
    }
    if !missing.is_empty() {
        return SubOutcome::Done(sub_error(
            Msg::str(&format!(
                "the following arguments are required: {}",
                missing.join(", ")
            )),
            closed,
        ));
    }
    SubOutcome::Parsed(
        Box::new(StatementArgs {
            repo: repo.unwrap(),
            month: month.unwrap(),
            rates,
            output,
        }),
        extras,
    )
}

fn run_with_args(args: &[Vec<u16>], closed: &ClosedFlags) -> i32 {
    let mut extras: Vec<Vec<u16>> = Vec::new();
    let mut at = 0;
    while at < args.len() {
        let token = &args[at];
        if token == &vec![0x2D, 0x2D] {
            // A leading "--" at the top level is the
            // command value, not a separator, when
            // anything follows it; trailing, it is
            // consumed and the command stays missing.
            if at + 1 < args.len() {
                return invalid_choice_top(token, closed);
            }
            at += 1;
            continue;
        }
        let help_shaped = token.len() > 2
            && token[0] == 0x2D
            && token[1] == 0x2D
            && match_long_top(token) == LongMatch::Help;
        let short_help = token.len() >= 2
            && token[0] == 0x2D
            && token[1] != 0x2D
            && !is_negative_number(token)
            // The cluster's first char decides: this
            // parser's only short option is -h, so an
            // 'h' first acts and anything else makes
            // the token an extra.
            && token[1] == 0x68;
        if help_shaped {
            if let Some(glued) = token_glued_value(token) {
                return help_explicit("secondbill", TOP_USAGE, &glued, closed);
            }
            return help_text(TOP_HELP, closed);
        }
        if short_help {
            if token.get(2) == Some(&0x3D) {
                return help_explicit("secondbill", TOP_USAGE, &token[3..], closed);
            }
            return help_text(TOP_HELP, closed);
        }
        if token.len() >= 2 && token[0] == 0x2D {
            extras.push(token.clone());
            at += 1;
            continue;
        }
        // The first value token decides.
        if token == &vec![0x73, 0x74, 0x61, 0x74, 0x65, 0x6D, 0x65, 0x6E, 0x74] {
            match parse_statement(&args[at + 1..], closed) {
                SubOutcome::Done(code) => return code,
                SubOutcome::Parsed(parsed, sub_extras) => {
                    extras.extend(sub_extras);
                    if !extras.is_empty() {
                        return extras_error_top(&extras, closed);
                    }
                    return statement(&parsed, closed);
                }
            }
        }
        return invalid_choice_top(token, closed);
    }
    required_error_top(closed)
}

// ---------- running the statement ----------

fn error_line(message: &Msg, closed: &ClosedFlags) -> i32 {
    let mut text = String::from("error, ");
    text.push_str(&message.render_stderr());
    text.push('\n');
    if closed.stderr {
        // The Python's stderr is gone; its writers
        // migrate (argparse) or vanish (prints raise
        // and are swallowed by the interpreter's
        // shutdown in the shapes the differential
        // matrix pinned; the guarded emit prints
        // migrate with the surviving stream).
        let _ = std::io::stdout().write_all(text.as_bytes());
        let _ = std::io::stdout().flush();
    } else {
        let _ = std::io::stderr().write_all(text.as_bytes());
        let _ = std::io::stderr().flush();
    }
    1
}

fn fail_out(failure: Fail, closed: &ClosedFlags) -> i32 {
    match failure {
        Fail::Named(message) => error_line(&message, closed),
        Fail::Crash => 1,
    }
}

/// The path forms this module keeps, and why the
/// output guard may compare across them. Three
/// normalizers serve three different Python shapes:
/// normalize_pathlib is pathlib's string form, what
/// the repository messages print and what git is
/// handed; resolve_path is pathlib's resolve, the
/// form the guard compares; normalize_lexical is
/// resolve_path's own finisher for the part of a
/// path that does not exist. The guard never
/// compares across the forms: the target and both
/// roots pass through resolve_path, so the compared
/// values share one rule by construction, and
/// path_components only splits a resolved value for
/// the prefix test.
///
/// pathlib's string form over units: redundant
/// separators collapse (a leading pair survives),
/// dot components drop, the empty path is ".".
fn normalize_pathlib(units: &[u16]) -> Vec<u16> {
    if units.is_empty() {
        return vec![0x2E];
    }
    let leading = units.iter().take_while(|&&unit| unit == 0x2F).count();
    let rooted = leading > 0;
    let parts: Vec<&[u16]> = units
        .split(|&unit| unit == 0x2F)
        .filter(|part| !(part.is_empty() || (part.len() == 1 && part[0] == 0x2E)))
        .collect();
    let mut out: Vec<u16> = Vec::new();
    if leading == 2 {
        out.extend_from_slice(&[0x2F, 0x2F]);
    } else if rooted {
        out.push(0x2F);
    }
    for (index, part) in parts.iter().enumerate() {
        if index > 0 {
            out.push(0x2F);
        }
        out.extend_from_slice(part);
    }
    if out.is_empty() {
        out.push(0x2E);
    }
    out
}

/// A path's components in normalized unit form,
/// for the output guard's comparisons.
fn path_components(units: &[u16]) -> Vec<Vec<u16>> {
    let normalized = normalize_pathlib(units);
    normalized
        .split(|&unit| unit == 0x2F)
        .filter(|part| !part.is_empty())
        .map(|part| part.to_vec())
        .collect()
}

fn os_bytes(value: &std::ffi::OsStr) -> Vec<u8> {
    #[cfg(unix)]
    {
        use std::os::unix::ffi::OsStrExt;
        value.as_bytes().to_vec()
    }
    #[cfg(not(unix))]
    {
        value.to_string_lossy().into_owned().into_bytes()
    }
}

/// pathlib's resolve, in unit form: absolute
/// against the working directory, symlinks resolved
/// where the path exists, the rest normalized
/// lexically.
fn resolve_path(units: &[u16]) -> Vec<u16> {
    let path = os_path_from_units(units);
    let absolute = if path.is_absolute() {
        path
    } else {
        std::env::current_dir().unwrap_or_default().join(&path)
    };
    let resolved = match std::fs::canonicalize(&absolute) {
        Ok(found) => found,
        Err(_) => {
            // Peel one component at a time, any kind:
            // a ".." or "." component peels like a
            // name, so a tail carrying one still gets
            // its existing prefix resolved, and the
            // lexical finish then pops the resolved
            // prefix exactly as realpath does.
            use std::path::Component;
            let mut prefix = absolute.clone();
            let mut rest: Vec<std::ffi::OsString> = Vec::new();
            loop {
                match std::fs::canonicalize(&prefix) {
                    Ok(found) => {
                        let mut out = found;
                        for part in rest.iter().rev() {
                            out.push(part);
                        }
                        break normalize_lexical(&out);
                    }
                    Err(_) => {
                        let last = prefix.components().next_back();
                        let peeled = match last {
                            Some(Component::Normal(name)) => Some(name.to_os_string()),
                            Some(Component::ParentDir) => Some(std::ffi::OsString::from("..")),
                            Some(Component::CurDir) => Some(std::ffi::OsString::from(".")),
                            _ => None,
                        };
                        match (peeled, prefix.parent()) {
                            (Some(name), Some(parent)) => {
                                rest.push(name);
                                prefix = parent.to_path_buf();
                            }
                            _ => break normalize_lexical(&absolute),
                        }
                    }
                }
            }
        }
    };
    fsdecode_units(&os_bytes(resolved.as_os_str()))
}

fn normalize_lexical(path: &Path) -> PathBuf {
    use std::path::Component;
    let mut out = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                out.pop();
            }
            other => out.push(other.as_os_str()),
        }
    }
    if out.as_os_str().is_empty() {
        out.push("/");
    }
    out
}

/// The atomic write, tempfile.mkstemp's shape with
/// no suffix: a sibling temp named with the file's
/// name and a dot as prefix, mode 0600, replaced on
/// success, unlinked on failure.
fn write_atomically(path: &Path, bytes: &[u8]) -> std::io::Result<()> {
    use std::os::unix::fs::OpenOptionsExt;
    let parent = path.parent().unwrap_or(Path::new(""));
    let parent = if parent.as_os_str().is_empty() {
        Path::new(".")
    } else {
        parent
    };
    let name = path
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .into_owned();
    let mut seed = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.subsec_nanos() as u64 ^ (std::process::id() as u64) << 16)
        .unwrap_or(0x9E3779B9);
    let mut candidate = PathBuf::new();
    let mut handle = None;
    for _ in 0..64 {
        seed = seed
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        let mut random = String::new();
        let mut value = seed >> 11;
        for _ in 0..8 {
            let pick = (value % 38) as u8;
            random.push(match pick {
                0..=25 => (b'a' + pick) as char,
                26..=35 => (b'0' + pick - 26) as char,
                36 => '_',
                _ => 'X',
            });
            value = value / 38 + 7;
        }
        candidate = parent.join(format!("{name}.{random}"));
        match std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o600)
            .open(&candidate)
        {
            Ok(file) => {
                handle = Some(file);
                break;
            }
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(error) => return Err(error),
        }
    }
    let mut handle = match handle {
        Some(handle) => handle,
        None => {
            return Err(std::io::Error::from(std::io::ErrorKind::AlreadyExists));
        }
    };
    let result = handle.write_all(bytes).and_then(|_| {
        drop(handle);
        std::fs::rename(&candidate, path)
    });
    if result.is_err() {
        let _ = std::fs::remove_file(&candidate);
    }
    result
}

fn statement(args: &StatementArgs, closed: &ClosedFlags) -> i32 {
    let repo = normalize_pathlib(&args.repo);
    if let Some(output) = &args.output {
        // The guard: nothing is written inside the
        // repository being read. The worktree root's
        // failure is swallowed, the git directory's
        // propagates (the Python's shape).
        let mut roots: Vec<Vec<Vec<u16>>> = Vec::new();
        if let Ok(root) = crate::history::worktree_root(&repo) {
            roots.push(path_components(&text_units(&root)));
        }
        let git_dir = match crate::history::git_dir(&repo) {
            Ok(found) => found,
            Err(failure) => return fail_out(failure, closed),
        };
        roots.push(path_components(&text_units(&git_dir)));
        let target = path_components(&resolve_path(output));
        let inside = roots.iter().any(|root| {
            target == *root || (target.len() > root.len() && target[..root.len()] == root[..])
        });
        if inside {
            return error_line(
                &Msg::str("secondbill writes nothing inside the repository it reads"),
                closed,
            );
        }
    }
    let commits = match read_history(&repo) {
        Ok(commits) => commits,
        Err(failure) => return fail_out(failure, closed),
    };
    let mut document = match build_statement(&commits, &args.month) {
        Ok(document) => document,
        Err(failure) => return fail_out(failure, closed),
    };
    if let Some(rates_path) = &args.rates {
        let data = match std::fs::read(os_path_from_units(&normalize_pathlib(rates_path))) {
            Ok(data) => data,
            Err(_) => {
                let mut message = Msg::str("cannot read the rates file, ");
                message.push_units(rates_path);
                return error_line(&message, closed);
            }
        };
        let rates = match crate::statement::load_rates_bytes(&data) {
            Ok(rates) => rates,
            Err(failure) => return fail_out(failure, closed),
        };
        document = price_statement(document, &rates);
    }
    emit(&document, args.output.as_deref(), closed)
}

fn text_units(text: &str) -> Vec<u16> {
    text.encode_utf16().collect()
}

fn emit(document: &Value, out: Option<&[u16]>, closed: &ClosedFlags) -> i32 {
    match out {
        None => {
            if closed.stdout {
                // sys.stdout is None: the Python's
                // write raises uncaught (crash class)
                // after the run's own effects.
                return 1;
            }
            let bytes = dumps_document(document, Mode::Stdout).unwrap_or_default();
            let mut stdout = std::io::stdout();
            match stdout.write_all(&bytes).and_then(|_| stdout.flush()) {
                Ok(()) => 0,
                Err(_) => error_line(&Msg::str("cannot write to stdout"), closed),
            }
        }
        Some(out) => {
            let path = os_path_from_units(&normalize_pathlib(out));
            // The Python dumps the text, creates the
            // temp, and fails the encode at the write,
            // unlinking the temp; the port's strict
            // render folds dump and encode together, so
            // its failure leaves no temp where the
            // Python leaves none either, and the
            // refusal is the same line.
            let bytes = dumps_document(document, Mode::Strict);
            let result = match &bytes {
                Ok(bytes) => write_atomically(&path, bytes),
                Err(_) => Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "unencodable",
                )),
            };
            match result {
                Ok(()) => 0,
                Err(_) => {
                    let mut message = Msg::str("cannot write the statement, ");
                    message.push_units(out);
                    error_line(&message, closed)
                }
            }
        }
    }
}
