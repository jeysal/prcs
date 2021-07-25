extern crate signal_hook;

use crate::{cli::Opts, status_constraints::StatusCode};

pub fn handle<F>(opts: &Opts, delegate: F) -> StatusCode
where
    F: Fn() -> StatusCode,
{
    // TODO check opts.signal
    delegate()
    // TODO sync with reruns, yay parallelism
}
