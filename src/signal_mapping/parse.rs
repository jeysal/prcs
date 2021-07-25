use crate::signal_mapping::SignalMapping;
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
pub enum ParseSignalMappingError<I> {
    ParseInt(I, ParseIntError),
    UnexpectedToken { allowed_tokens: String, input: I },
}

pub fn parse_signal_mapping(input: &str) -> Result<SignalMapping, ParseSignalMappingError<&str>> {
    // TODO
    Err(ParseSignalMappingError::UnexpectedToken {
        allowed_tokens: String::new(),
        input,
    })
}
