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
#[cfg(not(windows))]
use crate::util::fsdecode_units;
use crate::util::{closed_at_launch, os_path_from_units, Msg};
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
    #[cfg(windows)]
    let argv: Vec<Vec<u16>> = {
        use std::os::windows::ffi::OsStrExt;
        std::env::args_os()
            .map(|arg| arg.as_os_str().encode_wide().collect())
            .collect()
    };
    #[cfg(not(windows))]
    let argv: Vec<Vec<u16>> = {
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
        os_args.iter().map(|arg| fsdecode_units(arg)).collect()
    };
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
            prog: "muda",
            usage: TOP_USAGE,
            message,
        },
        closed,
    )
}

fn required_error_top(closed: &ClosedFlags) -> i32 {
    parse_error(
        &ParseFailure {
            prog: "muda",
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
            prog: "muda",
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
            prog: "muda statement",
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
                        help_token_outcome(token, "muda statement", STATEMENT_USAGE, closed)
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
                                "muda statement",
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
                return help_explicit("muda", TOP_USAGE, &glued, closed);
            }
            return help_text(TOP_HELP, closed);
        }
        if short_help {
            if token.get(2) == Some(&0x3D) {
                return help_explicit("muda", TOP_USAGE, &token[3..], closed);
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
/// output guard may compare across them. Two
/// normalizers serve two different Python shapes:
/// normalize_pathlib is pathlib's string form, what
/// the repository messages print and what git is
/// handed; resolve_path is pathlib's resolve, the
/// interpreter's own walk finished lexically,
/// failing where the interpreter's resolve raises.
/// The guard's comparison keeps the Python's own
/// asymmetry: the output target alone passes
/// through resolve_path, while both roots enter
/// as the unresolved strings git printed, and
/// path_components splits target and roots alike
/// for the prefix test.
///
/// pathlib's string form over units: redundant
/// separators collapse (a leading pair survives),
/// dot components drop, the empty path is ".".
#[cfg(not(windows))]
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

#[cfg(windows)]
fn normalize_pathlib(units: &[u16]) -> Vec<u16> {
    let is_sep = |unit: u16| unit == 0x2F || unit == 0x5C;
    if units.is_empty() {
        return vec![0x2E];
    }
    let has_drive = units.len() >= 2
        && units[1] == 0x3A
        && ((0x41..=0x5A).contains(&units[0]) || (0x61..=0x7A).contains(&units[0]));
    if has_drive {
        let rooted = units.get(2).is_some_and(|&u| is_sep(u));
        let rest = if rooted { &units[3..] } else { &units[2..] };
        let parts: Vec<&[u16]> = rest
            .split(|&u| is_sep(u))
            .filter(|part| !(part.is_empty() || (part.len() == 1 && part[0] == 0x2E)))
            .collect();
        let mut out = vec![units[0], 0x3A];
        if rooted {
            out.push(0x5C);
        }
        for (index, part) in parts.iter().enumerate() {
            if index > 0 {
                out.push(0x5C);
            }
            out.extend_from_slice(part);
        }
        return out;
    }
    let leading = units.iter().take_while(|&&u| is_sep(u)).count();
    let parts: Vec<&[u16]> = units[leading..]
        .split(|&u| is_sep(u))
        .filter(|part| !(part.is_empty() || (part.len() == 1 && part[0] == 0x2E)))
        .collect();
    let mut out: Vec<u16> = Vec::new();
    if leading >= 2 {
        out.extend_from_slice(&[0x5C, 0x5C]);
    } else if leading == 1 {
        out.push(0x5C);
    }
    for (index, part) in parts.iter().enumerate() {
        if index > 0 {
            out.push(0x5C);
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
#[cfg(not(windows))]
fn path_components(units: &[u16]) -> Vec<Vec<u16>> {
    let normalized = normalize_pathlib(units);
    normalized
        .split(|&unit| unit == 0x2F)
        .filter(|part| !part.is_empty())
        .map(|part| part.to_vec())
        .collect()
}

#[cfg(windows)]
fn path_components(units: &[u16]) -> Vec<Vec<u16>> {
    let normalized = normalize_pathlib(units);
    normalized
        .split(|&unit| unit == 0x2F || unit == 0x5C)
        .filter(|part| !part.is_empty())
        .map(|part| {
            part.iter()
                .map(|&u| {
                    if (0x41..=0x5A).contains(&u) {
                        u + 0x20
                    } else {
                        u
                    }
                })
                .collect()
        })
        .collect()
}

#[cfg(not(windows))]
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

#[cfg(not(windows))]
const MAX_WALK_DEPTH: usize = 991;

#[cfg(not(windows))]
fn join_bytes(first: &[u8], second: &[u8]) -> Vec<u8> {
    if second.first() == Some(&0x2F) {
        return second.to_vec();
    }
    let mut out = first.to_vec();
    if !out.is_empty() && !out.ends_with(b"/") {
        out.push(0x2F);
    }
    out.extend_from_slice(second);
    out
}

#[cfg(not(windows))]
fn split_bytes(path: &[u8]) -> (Vec<u8>, Vec<u8>) {
    let index = path
        .iter()
        .rposition(|byte| *byte == 0x2F)
        .map_or(0, |i| i + 1);
    let (mut head, tail) = (path[..index].to_vec(), path[index..].to_vec());
    if !head.is_empty() && !head.iter().all(|byte| *byte == 0x2F) {
        while head.ends_with(b"/") {
            head.pop();
        }
    }
    (head, tail)
}

#[cfg(not(windows))]
fn path_from_bytes(bytes: &[u8]) -> PathBuf {
    os_path_from_units(&fsdecode_units(bytes))
}

#[cfg(not(windows))]
fn joinrealpath(
    mut path: Vec<u8>,
    rest: Vec<u8>,
    seen: &mut std::collections::HashMap<Vec<u8>, Option<Vec<u8>>>,
    depth: usize,
) -> Option<(Vec<u8>, bool)> {
    if depth > MAX_WALK_DEPTH {
        return None;
    }
    let mut rest: &[u8] = &rest;
    if rest.first() == Some(&0x2F) {
        rest = &rest[1..];
        path = vec![0x2F];
    }
    while !rest.is_empty() {
        let (name, tail) = match rest.iter().position(|byte| *byte == 0x2F) {
            Some(index) => (&rest[..index], &rest[index + 1..]),
            None => (rest, &rest[rest.len()..]),
        };
        rest = tail;
        if name.is_empty() || name == b"." {
            continue;
        }
        if name == b".." {
            if !path.is_empty() {
                let (head, popped) = split_bytes(&path);
                path = head;
                if popped == b".." {
                    path = join_bytes(&join_bytes(&path, b".."), b"..");
                }
            } else {
                path = b"..".to_vec();
            }
            continue;
        }
        let newpath = join_bytes(&path, name);
        let is_link = match std::fs::symlink_metadata(path_from_bytes(&newpath)) {
            Ok(meta) => meta.file_type().is_symlink(),
            Err(_) => false,
        };
        if !is_link {
            path = newpath;
            continue;
        }
        if let Some(cached) = seen.get(&newpath) {
            match cached {
                Some(resolved) => {
                    path = resolved.clone();
                    continue;
                }
                None => return Some((join_bytes(&newpath, rest), false)),
            }
        }
        seen.insert(newpath.clone(), None);
        let target = match std::fs::read_link(path_from_bytes(&newpath)) {
            Ok(target) => os_bytes(target.as_os_str()),
            Err(_) => return None,
        };
        let (resolved, ok) = joinrealpath(path, target, seen, depth + 1)?;
        if !ok {
            return Some((join_bytes(&resolved, rest), false));
        }
        seen.insert(newpath, Some(resolved.clone()));
        path = resolved;
    }
    Some((path, true))
}

#[cfg(not(windows))]
fn resolve_path(units: &[u16]) -> Option<Vec<u16>> {
    let path = os_path_from_units(units);
    let absolute = if path.is_absolute() {
        path
    } else {
        std::env::current_dir().ok()?.join(&path)
    };
    let spelling = os_bytes(absolute.as_os_str());
    let mut seen = std::collections::HashMap::new();
    let (resolved, _ok) = joinrealpath(Vec::new(), spelling, &mut seen, 0)?;
    let walked = fsdecode_units(&resolved);
    let mut parts: Vec<&[u16]> = Vec::new();
    for part in walked.split(|&unit| unit == 0x2F) {
        if part.is_empty() || (part.len() == 1 && part[0] == 0x2E) {
            continue;
        }
        if part.len() == 2 && part[0] == 0x2E && part[1] == 0x2E {
            parts.pop();
            continue;
        }
        parts.push(part);
    }
    let mut finished: Vec<u16> = vec![0x2F];
    for (index, part) in parts.iter().enumerate() {
        if index > 0 {
            finished.push(0x2F);
        }
        finished.extend_from_slice(part);
    }
    if let Err(error) = std::fs::metadata(os_path_from_units(&finished)) {
        #[cfg(unix)]
        if error.raw_os_error() == Some(crate::util::ELOOP) {
            return None;
        }
    }
    Some(finished)
}

#[cfg(windows)]
fn resolve_path(units: &[u16]) -> Option<Vec<u16>> {
    use std::os::windows::ffi::{OsStrExt, OsStringExt};
    use std::path::Component;

    let raw = os_path_from_units(&normalize_pathlib(units));
    let absolute = if raw.is_absolute() {
        raw
    } else {
        std::env::current_dir().ok()?.join(raw)
    };
    if let Err(error) = std::fs::metadata(&absolute) {
        if error.raw_os_error() == Some(1921) {
            return None;
        }
    }
    let mut tail_parts: Vec<std::ffi::OsString> = Vec::new();
    let mut cursor = absolute.as_path();
    let resolved_base = loop {
        match std::fs::canonicalize(cursor) {
            Ok(canon) => break Some(canon),
            Err(err) => {
                if err.raw_os_error() == Some(1921) {
                    return None;
                }
                if let Some(name) = cursor.file_name() {
                    tail_parts.push(name.to_os_string());
                }
                match cursor.parent() {
                    Some(parent) if !parent.as_os_str().is_empty() && parent != cursor => {
                        cursor = parent;
                    }
                    _ => break None,
                }
            }
        }
    };
    let mut combined = match resolved_base {
        Some(canon) => {
            let wide: Vec<u16> = canon.as_os_str().encode_wide().collect();
            let stripped: Vec<u16> = if wide
                .starts_with(&[0x5C, 0x5C, 0x3F, 0x5C, 0x55, 0x4E, 0x43, 0x5C])
                || wide.starts_with(&[0x5C, 0x5C, 0x3F, 0x5C, 0x75, 0x6E, 0x63, 0x5C])
            {
                let mut unc = vec![0x5C, 0x5C];
                unc.extend_from_slice(&wide[8..]);
                unc
            } else if wide.starts_with(&[0x5C, 0x5C, 0x3F, 0x5C]) {
                wide[4..].to_vec()
            } else {
                wide
            };
            PathBuf::from(std::ffi::OsString::from_wide(&stripped))
        }
        None => absolute,
    };
    for part in tail_parts.into_iter().rev() {
        combined.push(part);
    }
    let mut normalized = PathBuf::new();
    for comp in combined.components() {
        match comp {
            Component::Prefix(p) => normalized.push(p.as_os_str()),
            Component::RootDir => normalized.push(comp.as_os_str()),
            Component::CurDir => {}
            Component::ParentDir => {
                let _ = normalized.pop();
            }
            Component::Normal(part) => normalized.push(part),
        }
    }
    Some(normalized.as_os_str().encode_wide().collect())
}

#[cfg(not(windows))]
fn normpath_bytes(path: &[u8]) -> Vec<u8> {
    if path.is_empty() {
        return vec![0x2E];
    }
    let leading = path.iter().take_while(|&&byte| byte == 0x2F).count();
    let initial = match leading {
        2 => 2,
        0 => 0,
        _ => 1,
    };
    let mut comps: Vec<&[u8]> = Vec::new();
    for comp in path.split(|&byte| byte == 0x2F) {
        if comp.is_empty() || comp == b"." {
            continue;
        }
        let append = comp != b".."
            || (initial == 0 && comps.is_empty())
            || comps.last().is_some_and(|last| *last == b"..");
        if append {
            comps.push(comp);
        } else if !comps.is_empty() {
            comps.pop();
        }
    }
    let mut out = Vec::new();
    out.resize(initial, 0x2F);
    for (index, comp) in comps.iter().enumerate() {
        if index > 0 {
            out.push(0x2F);
        }
        out.extend_from_slice(comp);
    }
    if out.is_empty() {
        out.push(0x2E);
    }
    out
}

#[cfg(not(windows))]
fn lexical_abspath(parent: &Path) -> std::io::Result<PathBuf> {
    let bytes = os_bytes(parent.as_os_str());
    let joined = if bytes.first() == Some(&0x2F) {
        bytes
    } else {
        let cwd = std::env::current_dir()?;
        join_bytes(&os_bytes(cwd.as_os_str()), &bytes)
    };
    Ok(path_from_bytes(&normpath_bytes(&joined)))
}

#[cfg(windows)]
fn lexical_abspath(parent: &Path) -> std::io::Result<PathBuf> {
    use std::path::Component;
    let joined = if parent.is_absolute() {
        parent.to_path_buf()
    } else {
        std::env::current_dir()?.join(parent)
    };
    let mut out = PathBuf::new();
    for comp in joined.components() {
        match comp {
            Component::Prefix(p) => out.push(p.as_os_str()),
            Component::RootDir => out.push(comp.as_os_str()),
            Component::CurDir => {}
            Component::ParentDir => {
                let _ = out.pop();
            }
            Component::Normal(part) => out.push(part),
        }
    }
    if out.as_os_str().is_empty() {
        out.push(".");
    }
    Ok(out)
}

fn os_entropy() -> Option<[u8; 16]> {
    #[cfg(unix)]
    {
        use std::io::Read;
        let mut bytes = [0u8; 16];
        let mut file = std::fs::File::open("/dev/urandom").ok()?;
        file.read_exact(&mut bytes).ok()?;
        Some(bytes)
    }
    #[cfg(windows)]
    {
        const BCRYPT_USE_SYSTEM_PREFERRED_RNG: u32 = 0x0000_0002;
        #[link(name = "bcrypt")]
        extern "system" {
            fn BCryptGenRandom(
                h_algorithm: *mut core::ffi::c_void,
                pb_buffer: *mut u8,
                cb_buffer: u32,
                dw_flags: u32,
            ) -> i32;
        }
        let mut bytes = [0u8; 16];
        let status = unsafe {
            BCryptGenRandom(
                core::ptr::null_mut(),
                bytes.as_mut_ptr(),
                bytes.len() as u32,
                BCRYPT_USE_SYSTEM_PREFERRED_RNG,
            )
        };
        if status == 0 {
            Some(bytes)
        } else {
            None
        }
    }
    #[cfg(not(any(unix, windows)))]
    {
        None
    }
}

/// One temp candidate's suffix (SB-56): eight
/// characters over the shipped 38-symbol alphabet,
/// drawn from the LCG state's high bits folded
/// with the candidate's entropy, the sixteen
/// bytes' two u64 lanes XORed into the value
/// before the first pick. The fold is what the
/// plant cells measure; its order is pinned by
/// the unit tests below.
fn temp_suffix(state: u64, entropy: &[u8; 16]) -> String {
    let fold = u64::from_le_bytes(entropy[..8].try_into().unwrap())
        ^ u64::from_le_bytes(entropy[8..].try_into().unwrap());
    let mut value = (state >> 11) ^ fold;
    let mut random = String::new();
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
    random
}

/// The atomic write, tempfile.mkstemp's shape with
/// no suffix: a sibling temp named with the file's
/// name and a dot as prefix, mode 0600, replaced on
/// success, unlinked on failure. The temp stands
/// under the lexically absolutized parent of the
/// path (SB-32), not under the parent as spelled:
/// where a link in the parent is popped by a
/// following dotdot, the interpreter's mkstemp
/// reaches the lexical directory or fails trying,
/// and the write's success boundary sits there,
/// while the rename destination stays the path
/// itself. Each candidate's suffix also draws
/// fresh operating system entropy (SB-56): the
/// shipped seed terms, the clock's sub-second
/// nanos and the pid, remain as mixer inputs, but
/// no candidate is a public function of them, and
/// an entropy failure refuses the write before
/// any create.
fn write_atomically(path: &Path, bytes: &[u8]) -> std::io::Result<()> {
    let parent = path.parent().unwrap_or(Path::new(""));
    let parent = if parent.as_os_str().is_empty() {
        Path::new(".")
    } else {
        parent
    };
    let parent = lexical_abspath(parent)?;
    // SB-37: the temp's name is assembled from the
    // path's raw bytes on Unix (or wide units on
    // Windows) so non-UTF-8 / unpaired units keep
    // their exact length.
    #[cfg(not(windows))]
    let name = os_bytes(path.file_name().unwrap_or_default());
    let mut seed = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.subsec_nanos() as u64 ^ (std::process::id() as u64) << 16)
        .unwrap_or(0x9E3779B9);
    let mut candidate = PathBuf::new();
    let mut handle = None;
    for _ in 0..64 {
        // SB-56: fresh entropy per candidate,
        // folded into the draw by temp_suffix. A
        // failed draw refuses the write outright
        // with the loop's own exhaustion error;
        // the computable terms alone never name a
        // candidate.
        let entropy = match os_entropy() {
            Some(bytes) => bytes,
            None => {
                return Err(std::io::Error::from(std::io::ErrorKind::AlreadyExists));
            }
        };
        seed = seed
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        let random = temp_suffix(seed, &entropy);
        #[cfg(not(windows))]
        {
            let mut tail = name.clone();
            tail.push(b'.');
            tail.extend_from_slice(random.as_bytes());
            candidate = path_from_bytes(&join_bytes(&os_bytes(parent.as_os_str()), &tail));
        }
        #[cfg(windows)]
        {
            use std::os::windows::ffi::{OsStrExt, OsStringExt};
            let mut wide: Vec<u16> = path.file_name().unwrap_or_default().encode_wide().collect();
            wide.push(b'.' as u16);
            wide.extend(random.encode_utf16());
            candidate = parent.join(std::ffi::OsString::from_wide(&wide));
        }
        let mut options = std::fs::OpenOptions::new();
        options.write(true).create_new(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o600);
        }
        match options.open(&candidate) {
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
        let root_components = |raw: &str| -> Vec<Vec<u16>> {
            let units = text_units(raw);
            #[cfg(windows)]
            {
                if let Some(resolved) = resolve_path(&units) {
                    return path_components(&resolved);
                }
            }
            path_components(&units)
        };
        if let Ok(root) = crate::history::worktree_root(&repo) {
            roots.push(root_components(&root));
        }
        let git_dir = match crate::history::git_dir(&repo) {
            Ok(found) => found,
            Err(failure) => return fail_out(failure, closed),
        };
        roots.push(root_components(&git_dir));
        // A resolution failure is the interpreter's
        // crash at resolve(): the guard never gets
        // to compare (SB-25).
        let target = match resolve_path(output) {
            Some(resolved) => path_components(&resolved),
            None => return fail_out(Fail::Crash, closed),
        };
        let inside = roots.iter().any(|root| {
            target == *root || (target.len() > root.len() && target[..root.len()] == root[..])
        });
        if inside {
            return error_line(
                &Msg::str("muda writes nothing inside the repository it reads"),
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

#[cfg(test)]
mod tests {
    use super::temp_suffix;

    /// SB-56: the entropy fold's order, pinned
    /// against vectors computed independently of
    /// this implementation. A change to the lane
    /// order, the shift, or the pick walk moves
    /// the vectors.
    #[test]
    fn temp_suffix_fold_vectors() {
        let ascending: [u8; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        assert_eq!(temp_suffix(0x0123_4567_89AB_CDEF, &ascending), "jb8d7h6m");
        assert_eq!(temp_suffix(0, &[0u8; 16]), "ahhhhhhh");
        assert_eq!(temp_suffix(u64::MAX, &[0xFFu8; 16]), "j2jXpv16");
        assert_eq!(temp_suffix(1442695040888963407, &[0x9Eu8; 16]), "sc8319bi");
    }

    /// SB-56: the suffix keeps the contracted
    /// shape, eight characters over the shipped
    /// alphabet, at any state and entropy.
    #[test]
    fn temp_suffix_shape() {
        for state in [0u64, 1, u64::MAX] {
            let suffix = temp_suffix(state, &[0xA5u8; 16]);
            assert_eq!(suffix.len(), 8);
            assert!(suffix
                .bytes()
                .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_' || b == b'X'));
        }
    }
}
