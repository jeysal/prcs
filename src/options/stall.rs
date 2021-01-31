use std::process::ExitStatus;

use crate::cli::Opts;
use crate::status_constraints::MatchStatusCode;

pub fn handle(opts: &Opts, child_status: &ExitStatus) {
    if match (&opts.stall, child_status.code()) {
        (Some(stall), Some(code)) => stall.matches(code),
        _ => false,
    } {
        std::thread::park()
    }
}
