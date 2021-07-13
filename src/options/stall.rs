use crate::status_constraints::MatchStatusCode;
use crate::{cli::Opts, status_constraints::StatusCode};

pub fn handle<F>(opts: &Opts, delegate: F) -> StatusCode
where
    F: Fn() -> StatusCode,
{
    let child_status = delegate();
    if opts.stall.iter().any(|stall| stall.matches(child_status)) {
        loop {
            std::thread::park()
        }
    }
    child_status
}
