use super::{ConstraintType, StatusConstraint};
use nom::combinator::eof;
use nom::multi::many_till;
use nom::{
    character::complete::{char, digit1, one_of, space0},
    combinator::{opt, recognize},
    error::{ErrorKind, ParseError},
    sequence::tuple,
    Parser,
};
use std::{i32, num::ParseIntError, str::FromStr};

// https://github.com/rust-lang/rust/issues/22639
fn i32_pos_overflow_error() -> ParseIntError {
    i32::from_str(&i64::max_value().to_string()).unwrap_err()
}
fn i32_neg_overflow_error() -> ParseIntError {
    i32::from_str(&i64::min_value().to_string()).unwrap_err()
}

#[derive(Debug, PartialEq)]
pub enum ParseStatusConstraintsError<I> {
    ParseInt(I, ParseIntError),
    UnexpectedToken { allowed_tokens: String, input: I },
    UnknownError(I, ErrorKind),
}
impl<I> ParseError<I> for ParseStatusConstraintsError<I> {
    fn from_error_kind(input: I, kind: nom::error::ErrorKind) -> Self {
        ParseStatusConstraintsError::UnknownError(input, kind)
    }

    fn append(_: I, _: nom::error::ErrorKind, other: Self) -> Self {
        other
    }
}
impl<I: std::fmt::Display> std::fmt::Display for ParseStatusConstraintsError<I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ParseStatusConstraintsError::ParseInt(input, parse_int_err)
                if *parse_int_err == i32_pos_overflow_error() =>
            {
                f.write_fmt(format_args!(
                    "Failed to parse '{}' as a status code. Integer too high.",
                    input
                ))
            }
            ParseStatusConstraintsError::ParseInt(input, parse_int_err)
                if *parse_int_err == i32_neg_overflow_error() =>
            {
                f.write_fmt(format_args!(
                    "Failed to parse '{}' as a status code. Integer too low.",
                    input
                ))
            }
            ParseStatusConstraintsError::ParseInt(input, parse_int_err) => {
                f.write_fmt(format_args!(
                    "Failed to parse '{}' as a status code. Unknown error: {}",
                    input, parse_int_err
                ))
            }
            ParseStatusConstraintsError::UnexpectedToken {
                allowed_tokens,
                input,
            } => f.write_fmt(format_args!(
                "Expected one of '{}', got: {}",
                allowed_tokens, input
            )),
            ParseStatusConstraintsError::UnknownError(input, _kind) => {
                f.write_fmt(format_args!("Unknown parse error at '{}'", input))
            }
        }
    }
}

fn parse_status_constraint(
    input: &str,
) -> Result<(&str, StatusConstraint), nom::Err<ParseStatusConstraintsError<&str>>> {
    let mut allowed_tokens = String::new();
    let (input, _) = space0(input)?;

    let (input, negated) = opt(char('!'))
        .map(|exclamation_mark| exclamation_mark.is_some())
        .parse(input)?;
    if negated {
        allowed_tokens.clear();
    } else {
        allowed_tokens.push('!')
    }

    let (input, _) = space0(input)?;

    let (input, operator) = opt(one_of("><")).parse(input)?;
    if operator.is_some() {
        allowed_tokens.clear();
    } else {
        allowed_tokens.push_str("><")
    }

    let (input, allow_equal) = opt(char('='))
        .map(|equal_sign| equal_sign.is_some())
        .parse(input)?;
    if allow_equal {
        allowed_tokens.clear();
    } else {
        allowed_tokens.push('=')
    }

    let (input, _) = space0(input)?;

    let (input, code) = recognize(tuple((opt(char('-')), digit1)))
        .parse(input)
        .map_err(move |_: nom::Err<()>| {
            let input_after_minus = input.strip_prefix('-');

            if input_after_minus.is_some() {
                allowed_tokens.clear();
            } else {
                allowed_tokens.push('-');
            }
            allowed_tokens += "0123456789";

            nom::Err::Error(ParseStatusConstraintsError::UnexpectedToken {
                allowed_tokens,
                input: input_after_minus.unwrap_or(input),
            })
        })?;

    let (input, _) = space0(input)?;

    Ok((
        input,
        StatusConstraint {
            typ: match (operator, allow_equal) {
                (Some('>'), true) => ConstraintType::Gte,
                (Some('>'), false) => ConstraintType::Gt,
                (Some('<'), true) => ConstraintType::Lte,
                (Some('<'), false) => ConstraintType::Lt,
                _ => ConstraintType::Eq,
            },
            code: i32::from_str(code)
                .map_err(|err| nom::Err::Error(ParseStatusConstraintsError::ParseInt(code, err)))?,
            negated,
        },
    ))
}
pub fn parse_status_constraints(
    input: &str,
) -> Result<Vec<StatusConstraint>, ParseStatusConstraintsError<&str>> {
    match many_till(parse_status_constraint, eof).map(|(constraints, _)| constraints).parse(input) {
        Ok(("", constraints)) => Ok(constraints),
        Ok((input, constraints)) => panic!(
            "Failed to parse status constraints '{}': Unconsumed input '{}' after parsing contraints {:?}",
            input, input, constraints
        ),
        Err(nom::Err::Failure(err)) | Err(nom::Err::Error(err)) => Err(err),
        Err(err) => panic!("Failed to parse status constraints '{}': Unknown error: {:?}", input, err),
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_status_constraints, ConstraintType, StatusConstraint};

    // success cases

    #[test]
    fn positive_number() {
        assert_eq!(
            parse_status_constraints("42"),
            Ok(vec![StatusConstraint {
                typ: ConstraintType::Eq,
                code: 42,
                negated: false
            }])
        )
    }
    #[test]
    fn negative_number() {
        assert_eq!(
            parse_status_constraints("-42"),
            Ok(vec![StatusConstraint {
                typ: ConstraintType::Eq,
                code: -42,
                negated: false
            }])
        )
    }

    #[test]
    fn negated() {
        assert_eq!(
            parse_status_constraints("!42"),
            Ok(vec![StatusConstraint {
                typ: ConstraintType::Eq,
                code: 42,
                negated: true
            }])
        )
    }

    #[test]
    fn explicit_equals() {
        assert_eq!(
            parse_status_constraints("=42"),
            Ok(vec![StatusConstraint {
                typ: ConstraintType::Eq,
                code: 42,
                negated: false
            }])
        )
    }
    #[test]
    fn greater_than() {
        assert_eq!(
            parse_status_constraints(">42"),
            Ok(vec![StatusConstraint {
                typ: ConstraintType::Gt,
                code: 42,
                negated: false
            }])
        )
    }
    #[test]
    fn less_than() {
        assert_eq!(
            parse_status_constraints("<42"),
            Ok(vec![StatusConstraint {
                typ: ConstraintType::Lt,
                code: 42,
                negated: false
            }])
        )
    }
    #[test]
    fn or_equal() {
        assert_eq!(
            parse_status_constraints(">=42"),
            Ok(vec![StatusConstraint {
                typ: ConstraintType::Gte,
                code: 42,
                negated: false
            }])
        )
    }

    #[test]
    fn multiple_constraints() {
        assert_eq!(
            parse_status_constraints("42 1337"),
            Ok(vec![
                StatusConstraint {
                    typ: ConstraintType::Eq,
                    code: 42,
                    negated: false
                },
                StatusConstraint {
                    typ: ConstraintType::Eq,
                    code: 1337,
                    negated: false
                }
            ])
        )
    }

    #[test]
    fn whitespace() {
        assert_eq!(
            parse_status_constraints(" ! >= -42 "),
            Ok(vec![StatusConstraint {
                typ: ConstraintType::Gte,
                code: -42,
                negated: true
            }])
        )
    }

    // error cases

    #[test]
    fn integer_out_of_bounds_error() {
        let input = i64::max_value().to_string();
        let err = parse_status_constraints(&input).unwrap_err();
        assert!(err.to_string().contains(&input));
        assert!(err.to_string().to_lowercase().contains("too high"));
    }

    #[test]
    fn invalid_token() {
        insta::assert_display_snapshot!(parse_status_constraints("x").unwrap_err(), @"Expected one of '!><=-0123456789', got: x");
    }
    #[test]
    fn invalid_token_after_negation() {
        insta::assert_display_snapshot!(parse_status_constraints("!x").unwrap_err(), @"Expected one of '><=-0123456789', got: x");
    }
    #[test]
    fn invalid_token_after_operator() {
        insta::assert_display_snapshot!(parse_status_constraints(">x").unwrap_err(), @"Expected one of '=-0123456789', got: x");
    }
    #[test]
    fn invalid_token_after_equal_sign() {
        insta::assert_display_snapshot!(parse_status_constraints("=x").unwrap_err(), @"Expected one of '-0123456789', got: x");
    }
    #[test]
    fn invalid_token_after_minus() {
        insta::assert_display_snapshot!(parse_status_constraints("-x").unwrap_err(), @"Expected one of '0123456789', got: x");
    }
    #[test]
    fn invalid_token_after_number() {
        insta::assert_display_snapshot!(parse_status_constraints("4x").unwrap_err(), @"Expected one of '!><=-0123456789', got: x");
    }
}
