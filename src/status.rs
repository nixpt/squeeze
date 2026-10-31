//! cargo-style status lines, on stderr so stdout stays the program's.

use std::io::IsTerminal;

fn color() -> bool {
    std::env::var_os("NO_COLOR").is_none() && std::io::stderr().is_terminal()
}

/// `   Compiling hello v0.1.0 (/path)` — the verb right-aligned to 12.
pub fn status(verb: &str, msg: &str) {
    if color() {
        eprintln!("\x1b[1;32m{verb:>12}\x1b[0m {msg}");
    } else {
        eprintln!("{verb:>12} {msg}");
    }
}

pub fn warning(msg: &str) {
    if color() {
        eprintln!("\x1b[1;33mwarning\x1b[0m: {msg}");
    } else {
        eprintln!("warning: {msg}");
    }
}

pub fn error(msg: &str) {
    if color() {
        eprintln!("\x1b[1;31merror\x1b[0m: {msg}");
    } else {
        eprintln!("error: {msg}");
    }
}
