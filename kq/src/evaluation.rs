use crate::Operator;
use kdl::KdlValue;

pub(crate) fn evaluate(lhs: &KdlValue, operator: &Operator, rhs: &KdlValue) -> bool {
    match operator {
        Operator::Contains => contains(lhs, rhs),
        Operator::EndsWith => ends_with(lhs, rhs),
        Operator::Equal => equal(lhs, rhs),
        Operator::GreaterThan => greater_than(lhs, rhs),
        Operator::GreaterThanOrEqualTo => greater_than_or_equal_to(lhs, rhs),
        Operator::LessThan => less_than(lhs, rhs),
        Operator::LessThanOrEqualTo => less_than_or_equal_to(lhs, rhs),
        Operator::NotEqual => not_equal(lhs, rhs),
        Operator::StartsWith => starts_with(lhs, rhs),
    }
}

fn contains(lhs: &KdlValue, rhs: &KdlValue) -> bool {
    match lhs {
        KdlValue::String(lhs) => match rhs {
            KdlValue::String(rhs) => lhs.contains(rhs),
            KdlValue::Int(_) | KdlValue::Float(_) | KdlValue::Boolean(_) | KdlValue::Null => false,
        },
        KdlValue::Int(_) | KdlValue::Boolean(_) | KdlValue::Null | KdlValue::Float(_) => false,
    }
}

fn ends_with(lhs: &KdlValue, rhs: &KdlValue) -> bool {
    match lhs {
        KdlValue::Int(_) | KdlValue::Float(_) | KdlValue::Boolean(_) | KdlValue::Null => false,
        KdlValue::String(lhs) => match rhs {
            KdlValue::String(rhs) => lhs.ends_with(rhs),
            KdlValue::Int(_) | KdlValue::Float(_) | KdlValue::Boolean(_) | KdlValue::Null => false,
        },
    }
}

fn equal(lhs: &KdlValue, rhs: &KdlValue) -> bool {
    match lhs {
        KdlValue::Int(lhs) => match rhs {
            KdlValue::Int(rhs) => lhs == rhs,
            KdlValue::Float(_) | KdlValue::String(_) | KdlValue::Boolean(_) | KdlValue::Null => {
                false
            }
        },
        KdlValue::Float(lhs) => match rhs {
            #[allow(clippy::float_cmp)]
            KdlValue::Float(rhs) => lhs == rhs,
            KdlValue::Int(_) | KdlValue::String(_) | KdlValue::Boolean(_) | KdlValue::Null => false,
        },
        KdlValue::String(lhs) => match rhs {
            KdlValue::String(rhs) => lhs == rhs,
            KdlValue::Int(_) | KdlValue::Float(_) | KdlValue::Boolean(_) | KdlValue::Null => false,
        },
        KdlValue::Boolean(lhs) => match rhs {
            KdlValue::Boolean(rhs) => lhs == rhs,
            KdlValue::Int(_) | KdlValue::Float(_) | KdlValue::String(_) | KdlValue::Null => false,
        },
        KdlValue::Null => match rhs {
            KdlValue::Null => true,
            KdlValue::Int(_) | KdlValue::Float(_) | KdlValue::String(_) | KdlValue::Boolean(_) => {
                false
            }
        },
    }
}

fn greater_than(lhs: &KdlValue, rhs: &KdlValue) -> bool {
    match lhs {
        KdlValue::Int(lhs) => match rhs {
            KdlValue::Int(rhs) => lhs > rhs,
            KdlValue::Float(_) | KdlValue::String(_) | KdlValue::Boolean(_) | KdlValue::Null => {
                false
            }
        },
        KdlValue::Float(lhs) => match rhs {
            KdlValue::Float(rhs) => lhs > rhs,
            KdlValue::Int(_) | KdlValue::String(_) | KdlValue::Boolean(_) | KdlValue::Null => false,
        },
        KdlValue::String(_) | KdlValue::Boolean(_) | KdlValue::Null => false,
    }
}

fn greater_than_or_equal_to(lhs: &KdlValue, rhs: &KdlValue) -> bool {
    match lhs {
        KdlValue::Int(lhs) => match rhs {
            KdlValue::Int(rhs) => lhs >= rhs,
            KdlValue::Float(_) | KdlValue::String(_) | KdlValue::Boolean(_) | KdlValue::Null => {
                false
            }
        },
        KdlValue::Float(lhs) => match rhs {
            KdlValue::Float(rhs) => lhs >= rhs,
            KdlValue::Int(_) | KdlValue::String(_) | KdlValue::Boolean(_) | KdlValue::Null => false,
        },
        KdlValue::String(_) | KdlValue::Boolean(_) | KdlValue::Null => false,
    }
}

fn less_than(lhs: &KdlValue, rhs: &KdlValue) -> bool {
    match lhs {
        KdlValue::Int(lhs) => match rhs {
            KdlValue::Int(rhs) => lhs < rhs,
            KdlValue::Float(_) | KdlValue::String(_) | KdlValue::Boolean(_) | KdlValue::Null => {
                false
            }
        },
        KdlValue::Float(lhs) => match rhs {
            KdlValue::Float(rhs) => lhs < rhs,
            KdlValue::Int(_) | KdlValue::String(_) | KdlValue::Boolean(_) | KdlValue::Null => false,
        },
        KdlValue::String(_) | KdlValue::Boolean(_) | KdlValue::Null => false,
    }
}

fn less_than_or_equal_to(lhs: &KdlValue, rhs: &KdlValue) -> bool {
    match lhs {
        KdlValue::Int(lhs) => match rhs {
            KdlValue::Int(rhs) => lhs <= rhs,
            KdlValue::Float(_) | KdlValue::String(_) | KdlValue::Boolean(_) | KdlValue::Null => {
                false
            }
        },
        KdlValue::Float(lhs) => match rhs {
            KdlValue::Float(rhs) => lhs <= rhs,
            KdlValue::Int(_) | KdlValue::String(_) | KdlValue::Boolean(_) | KdlValue::Null => false,
        },
        KdlValue::String(_) | KdlValue::Boolean(_) | KdlValue::Null => false,
    }
}

fn not_equal(lhs: &KdlValue, rhs: &KdlValue) -> bool {
    match lhs {
        KdlValue::Int(lhs) => match rhs {
            KdlValue::Int(rhs) => lhs != rhs,
            KdlValue::Float(_) | KdlValue::String(_) | KdlValue::Boolean(_) | KdlValue::Null => {
                false
            }
        },
        KdlValue::Float(lhs) => match rhs {
            #[allow(clippy::float_cmp)]
            KdlValue::Float(rhs) => lhs != rhs,
            KdlValue::Int(_) | KdlValue::String(_) | KdlValue::Boolean(_) | KdlValue::Null => false,
        },
        KdlValue::String(lhs) => match rhs {
            KdlValue::String(rhs) => lhs != rhs,
            KdlValue::Int(_) | KdlValue::Float(_) | KdlValue::Boolean(_) | KdlValue::Null => false,
        },
        KdlValue::Boolean(lhs) => match rhs {
            KdlValue::Boolean(rhs) => lhs != rhs,
            KdlValue::Int(_) | KdlValue::Float(_) | KdlValue::String(_) | KdlValue::Null => false,
        },
        KdlValue::Null => false,
    }
}

fn starts_with(lhs: &KdlValue, rhs: &KdlValue) -> bool {
    match lhs {
        KdlValue::String(lhs) => match rhs {
            KdlValue::String(rhs) => lhs.starts_with(rhs),
            KdlValue::Int(_) | KdlValue::Float(_) | KdlValue::Boolean(_) | KdlValue::Null => false,
        },
        KdlValue::Int(_) | KdlValue::Float(_) | KdlValue::Boolean(_) | KdlValue::Null => false,
    }
}
