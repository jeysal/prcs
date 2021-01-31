use clap::Clap;
use prcs::{run, Opts};
use std::process;

fn main() {
    let opts: Opts = Opts::parse();

    let status_code = run(&opts);

    process::exit(status_code);
}
