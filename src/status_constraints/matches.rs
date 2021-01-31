use super::StatusConstraint;
use super::{ConstraintType, StatusCode};

pub trait MatchStatusCode {
    fn matches(&self, code: StatusCode) -> bool;
}

impl MatchStatusCode for StatusConstraint {
    fn matches(&self, code: StatusCode) -> bool {
        (match self.typ {
            ConstraintType::Lte => code <= self.code,
            ConstraintType::Lt => code < self.code,
            ConstraintType::Eq => code == self.code,
            ConstraintType::Gt => code > self.code,
            ConstraintType::Gte => code >= self.code,
        }) ^ self.negated
    }
}
impl MatchStatusCode for Vec<StatusConstraint> {
    fn matches(&self, code: StatusCode) -> bool {
        self.iter().all(|constraint| constraint.matches(code))
    }
}
impl MatchStatusCode for Vec<Vec<StatusConstraint>> {
    fn matches(&self, code: StatusCode) -> bool {
        self.iter().any(|constraints| constraints.matches(code))
    }
}

#[cfg(test)]
mod tests {
    use super::{ConstraintType, MatchStatusCode, StatusConstraint};

    #[test]
    fn lt_match() {
        assert!(StatusConstraint {
            typ: ConstraintType::Lt,
            code: 42,
            negated: false
        }
        .matches(41))
    }
    #[test]
    fn lt_mismatch() {
        assert!(!StatusConstraint {
            typ: ConstraintType::Lt,
            code: 42,
            negated: false
        }
        .matches(42))
    }

    #[test]
    fn lte_match() {
        assert!(StatusConstraint {
            typ: ConstraintType::Lte,
            code: 42,
            negated: false
        }
        .matches(42))
    }
    #[test]
    fn lte_mismatch() {
        assert!(!StatusConstraint {
            typ: ConstraintType::Lte,
            code: 42,
            negated: false
        }
        .matches(43))
    }

    #[test]
    fn eq_match() {
        assert!(StatusConstraint {
            typ: ConstraintType::Eq,
            code: 42,
            negated: false
        }
        .matches(42))
    }
    #[test]
    fn eq_mismatch() {
        assert!(!StatusConstraint {
            typ: ConstraintType::Eq,
            code: 42,
            negated: false
        }
        .matches(1337))
    }

    #[test]
    fn gt_match() {
        assert!(StatusConstraint {
            typ: ConstraintType::Gt,
            code: 42,
            negated: false
        }
        .matches(43))
    }
    #[test]
    fn gt_mismatch() {
        assert!(!StatusConstraint {
            typ: ConstraintType::Gt,
            code: 42,
            negated: false
        }
        .matches(42))
    }

    #[test]
    fn gte_match() {
        assert!(StatusConstraint {
            typ: ConstraintType::Gte,
            code: 42,
            negated: false
        }
        .matches(42))
    }
    #[test]
    fn gte_mismatch() {
        assert!(!StatusConstraint {
            typ: ConstraintType::Gte,
            code: 42,
            negated: false
        }
        .matches(41))
    }

    #[test]
    fn negation() {
        assert!(StatusConstraint {
            typ: ConstraintType::Eq,
            code: 42,
            negated: true
        }
        .matches(1337))
    }

    #[test]
    fn multiple_constraints() {
        assert!(!vec![
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
        ]
        .matches(42))
    }

    #[test]
    fn constraint_options() {
        assert!(vec![
            vec![StatusConstraint {
                typ: ConstraintType::Eq,
                code: 42,
                negated: false
            }],
            vec![StatusConstraint {
                typ: ConstraintType::Eq,
                code: 1337,
                negated: false
            }]
        ]
        .matches(42))
    }
}
