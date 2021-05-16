use crate::status_constraints::MatchStatusCode;
use crate::{cli::Opts, status_constraints::StatusCode};

pub fn handle<F>(opts: &Opts, delegate: F) -> StatusCode
where
    F: Fn() -> StatusCode,
{
    loop {
        let child_status = delegate();
        if !opts.rerun.iter().any(|rerun| rerun.matches(child_status)) {
            return child_status;
        }
    }
}
