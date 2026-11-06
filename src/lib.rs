//! muda: the second bill, the itemized
//! statement of what a repository's history cost in
//! rework. It reads one git repository, finds the
//! commits whose work was reverted, fixed forward,
//! or redone inside a month, and states them by
//! cause (SB-1 to SB-16).
//!
//! Round 3 is the Rust port. It consumes no sibling
//! crate and takes no dependency: the git read is a
//! subprocess and the rates card is read by a strict
//! scanner in this crate (SB-18, SB-19, SB-20).

pub mod card;
pub mod cli;
pub mod emit;
pub mod helptext;
pub mod history;
pub mod pyrepr;
pub mod repr_table;
pub mod statement;
pub mod util;
pub mod value;

/// The command line entry, in the cli module.
pub fn run_cli() -> i32 {
    // A generous stack on a spawned thread, the
    // family precedent: parsed values nest as deep
    // as their files, and cloning or dropping a deep
    // value recurses with its depth.
    let child = std::thread::Builder::new()
        .stack_size(512 * 1024 * 1024)
        .spawn(crate::cli::run_cli)
        .expect("the CLI thread starts");
    child.join().unwrap_or(1)
}
