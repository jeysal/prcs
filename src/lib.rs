extern crate clap;
extern crate nom;

mod cli;
mod options;
mod status_code;
mod status_constraints;

use options::{rerun, stall};
use status_code::get_status_code;
use status_constraints::StatusCode;
use std::process;

pub use cli::Opts;

pub fn run(opts: &Opts) -> StatusCode {
    stall::handle(&opts, || {
        rerun::handle(&opts, || {
            get_status_code(
                &process::Command::new(&opts.command)
                    .args(&opts.args)
                    .status()
                    .unwrap_or_else(|err| {
                        eprintln!("Failed to run '{}'", opts.command);
                        eprintln!("{}", err);
                        process::exit(1)
                    }),
            )
        })
    })
}
