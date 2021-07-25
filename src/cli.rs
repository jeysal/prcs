use super::signal_mapping::{parse_signal_mapping, SignalMapping};
use super::status_constraints::{parse_status_constraints, StatusConstraint};
use clap::Clap;

#[derive(Clap, Debug)]
#[clap(
    about = concat!(env!("CARGO_PKG_DESCRIPTION"), "\nUse long form --help for more details"),
    long_about = concat!(env!("CARGO_PKG_DESCRIPTION"), "\nUse short form -h for a less detailed overview"),
    version,
)]
pub struct Opts {
    #[clap(
        long, short,
        require_equals = true, min_values = 0,
        default_missing_value = "!0", parse(try_from_str = parse_status_constraints),
        value_name = "status",
        about = "After the process exited with a matching status code, rerun it",
        long_about = "\
            After the process exited with a matching status code (by default anything but 0 matches), rerun it.\n\
            Status can also be one or more ranges given as '[!][{>|<}][=]<status>...', which must all match the exit status of the process.\n\
            Option can be repeated, in which case one value matching suffices to cause a rerun.\n\n\
            Examples\n\
            Retry mycmd until it succeeds (exits with 0):\n\
            prcs -r mycmd\n\
            Rerun mycmd while it is successful or terminated by a signal (status > 128) other than program interrupt (SIGINT, status 130):\n\
            prcs -r=0 -r='>128!130' mycmd\n\n\
            If --stall is also specified, only stall when not rerunning anymore.\n\
            Rerun mycmd until it succeeds, then do nothing until interrupted:\n\
            prcs -s -r mycmd\n\
        "
    )]
    pub rerun: Option<Vec<Vec<StatusConstraint>>>,
    #[cfg(target_family = "unix")]
    #[clap(
        long,
        parse(try_from_str = parse_signal_mapping),
        value_name = "signal",
        about = "Forward received signals to the process",
        long_about = "\
            After prcs received a matching catchable signal (by default any catchable signal matches), forward it to the process.\n\
            Signal can be used to match the specified, or all but the specified catchable signals,\n\
            and optionally remap them to a different signal to send to the process,\n\
            specified as '{[=]<caught signal>...|!<caught signal>...}[:<remapped signal>]'.\n\
            Option can be repeated, in which case the first matching value is used to remap.\n\n\
            Examples\n\
            Forward all catchable signals to mycmd:\n\
            prcs --signal mycmd\n\
            When SIGUSR1 or SIGUSR2 is caught, send SIGTERM to mycmd; for any other catchable signal except SIGINT and SIGTERM, send SIGKILL to mycmd:\n\
            prcs --signal='=SIGUSR1=SIGUSR2:SIGTERM' --signl='!SIGINT!SIGTERM:SIGKILL' mycmd\n\
        "
    )]
    pub signal: Option<Vec<SignalMapping>>,
    #[clap(
        long, short,
        require_equals = true, min_values = 0,
        default_missing_value = "0", parse(try_from_str = parse_status_constraints),
        value_name = "status",
        about = "After the process exited with a matching status code, do nothing until interrupted",
        long_about = "\
            After the process exited with a matching status code (by default only 0 matches), do nothing until interrupted.\n\
            Status can also be one or more ranges given as '[!][{>|<}][=]<status>...', which must all match the exit status of the process.\n\
            Option can be repeated, in which case one value matching suffices to cause stalling.\n\n\
            Examples\n\
            Do nothing until interrupted (assuming true exits with 0):\n\
            prcs -s true\n\
            If mycmd exits with status over 42, but not 100, or with status 21, do nothing until interrupted:\n\
            prcs -s='>42!100' -s=21 mycmd\n\
        "
    )]
    pub stall: Option<Vec<Vec<StatusConstraint>>>,

    pub command: String,
    pub args: Vec<String>,
}
