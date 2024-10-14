use std::fmt::Display;

#[derive(PartialEq, Clone)]
pub enum Value {
    Boolean(bool),
    Number(f64),
    String(String),
    Nil,
}

#[derive(PartialEq, PartialOrd)]
pub struct NumberPair(pub f64, pub f64);

impl TryFrom<(&Value, &Value)> for NumberPair {
    type Error = ();

    fn try_from(value: (&Value, &Value)) -> Result<Self, Self::Error> {
        match value {
            (Value::Number(left), Value::Number(right)) => Ok(NumberPair(*left, *right)),
            _ => Err(()),
        }
    }
}

#[derive(PartialEq)]
pub struct StringPair(pub String, pub String);

impl TryFrom<(&Value, &Value)> for StringPair {
    type Error = ();

    fn try_from(value: (&Value, &Value)) -> Result<Self, Self::Error> {
        match value {
            (Value::String(left), Value::String(right)) => {
                Ok(StringPair(left.to_string(), right.to_string()))
            }
            _ => Err(()),
        }
    }
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
