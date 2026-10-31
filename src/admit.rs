//! The build admission hook.
//!
//! The fleet runs builds through an admission scheduler (buildsched, "bob")
//! that decides when a build may start. squeeze does not reimplement any of
//! that: when `SQUEEZE_ADMIT` holds a command prefix (for example
//! `bob admit --`), squeeze re-runs itself under that prefix for the commands
//! that compile or run Crush code (`build`, `run`, `check`, `test`, bare
//! `squeeze`):
//!
//! ```text
//! SQUEEZE_ADMIT="bob admit --" squeeze build --release
//!   → bob admit -- /path/to/squeeze build --release     (SQUEEZE_ADMITTED=1)
//! ```
//!
//! The inner squeeze sees `SQUEEZE_ADMITTED=1` and does the work directly;
//! the outer one exits with the wrapper's exit status. The prefix is split on
//! whitespace (no shell quoting). Unset or empty: no wrapper. `new`, `add`,
//! `remove`, `clean`, `fmt` and `lint` are never wrapped.

use std::process::{Command, ExitCode};

pub const ENV_ADMIT: &str = "SQUEEZE_ADMIT";
const ENV_ADMITTED: &str = "SQUEEZE_ADMITTED";

/// Re-run this invocation under `$SQUEEZE_ADMIT` when it applies; the exit
/// code to end with, or `None` to carry on in this process.
pub fn maybe_reexec(is_build_work: bool) -> Option<ExitCode> {
    if !is_build_work || std::env::var_os(ENV_ADMITTED).is_some_and(|v| v == "1") {
        return None;
    }
    let prefix = std::env::var(ENV_ADMIT).ok()?;
    let words: Vec<&str> = prefix.split_whitespace().collect();
    let (program, args) = words.split_first()?;
    let exe = match std::env::current_exe() {
        Ok(exe) => exe,
        Err(e) => {
            crate::status::error(&format!(
                "{ENV_ADMIT}: cannot locate the squeeze executable: {e}"
            ));
            return Some(ExitCode::FAILURE);
        }
    };
    let result = Command::new(program)
        .args(args)
        .arg(exe)
        .args(std::env::args_os().skip(1))
        .env(ENV_ADMITTED, "1")
        .status();
    Some(match result {
        Ok(st) => ExitCode::from(st.code().map_or(1, |c| (c & 0xff) as u8)),
        Err(e) => {
            crate::status::error(&format!(
                "{ENV_ADMIT}={prefix:?}: cannot run `{program}`: {e}"
            ));
            ExitCode::FAILURE
        }
    })
}
