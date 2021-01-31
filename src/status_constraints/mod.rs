mod matches;
mod parse;

pub type StatusCode = i32;

#[derive(Debug, PartialEq)]
pub enum ConstraintType {
    Lt,
    Lte,
    Eq,
    Gte,
    Gt,
}
#[derive(Debug, PartialEq)]
pub struct StatusConstraint {
    typ: ConstraintType,
    code: StatusCode,
    negated: bool,
}

pub use matches::MatchStatusCode;
pub use parse::parse_status_constraints;
