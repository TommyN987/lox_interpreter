use std::fmt::Display;

#[derive(Debug)]
pub struct RuntimeError {
    pub line_number: usize,
    pub message: String,
}

pub type RuntimeResult<T> = Result<T, RuntimeError>;

impl RuntimeError {
    pub fn new(line_number: usize, message: &str) -> Self {
        Self {
            line_number,
            message: message.to_string(),
        }
    }
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
