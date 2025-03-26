use std::fmt;

#[derive(Debug, Clone)]
pub enum Value {
    Int(i64),
    None,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int(i) => write!(f, "{i}"),
            Value::None => write!(f, "()"),
        }
    }
}
