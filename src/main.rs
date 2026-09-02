//! Command-line interface for `upath`.
//!
//! Takes a single path and prints an available (non-colliding) path for it.

use std::env;
use std::io::Write;
use std::path::PathBuf;
use std::process::ExitCode;

use upath::upath;

/// Usage string shown when invoked incorrectly.
const USAGE: &str = "usage: upath <path>";

fn main() -> ExitCode {
    let mut args = env::args_os().skip(1);

    let Some(arg) = args.next() else {
        eprintln!("{USAGE}");
        return ExitCode::FAILURE;
    };

    // Reject any further arguments so a single path per run is enforced.
    if args.next().is_some() {
        eprintln!("upath: expected exactly one path\n{USAGE}");
        return ExitCode::FAILURE;
    }

    let path = PathBuf::from(arg);
    match upath(&path) {
        Ok(available) => {
            let mut stdout = std::io::stdout().lock();
            match writeln!(stdout, "{}", available.display()) {
                Ok(()) => ExitCode::SUCCESS,
                Err(e) => {
                    eprintln!("upath: {e}");
                    ExitCode::FAILURE
                }
            }
        }
        Err(e) => {
            eprintln!("upath: {}: {e}", path.display());
            ExitCode::FAILURE
        }
    }
}
