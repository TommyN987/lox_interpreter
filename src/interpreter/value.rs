use std::fmt::Display;

pub enum Value {
    Boolean(bool),
    Number(f64),
    String(String),
    Nil,
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Nil => "nil".to_string(),
                Self::Boolean(value) => value.to_string(),
                Self::String(value) => value.to_string(),
                Self::Number(value) => value.to_string(),
            }
        )
    }
}
