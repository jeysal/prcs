use crate::status_constraints::StatusCode;
use std::process::ExitStatus;

#[cfg(target_family = "unix")]
pub fn get_status_code(child_status: &ExitStatus) -> StatusCode {
    use std::os::unix::prelude::ExitStatusExt;
    child_status
        .code()
        .or_else(|| ExitStatusExt::signal(child_status).map(|signal| 128 + signal))
        .unwrap_or_else(|| {
            println!("Failed to interpret child status code or signal, exiting with 128");
            128
        })
}
#[cfg(not(target_family = "unix"))]
pub fn get_status_code(child_status: &ExitStatus) -> StatusCode {
    child_status.code().unwrap_or_else(|| {
        println!("Failed to interpret child status code, exiting with 1");
        1
    })
}
