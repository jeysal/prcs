use std::fmt::Formatter;
use std::{fmt::Display, os::raw::c_int};

mod parse;

#[derive(Debug, PartialEq)]
pub struct SignalMapping(c_int, Option<c_int>);

impl Display for SignalMapping {
    fn fmt(&self, _: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let SignalMapping(source, target) = self;
        let target_str = match target {
            Some(signal) => signal.to_string(),
            None => String::new(),
        };
        format_args!("{}:{}", source, target_str);
        Ok(())
    }
}

pub use parse::parse_signal_mapping;
