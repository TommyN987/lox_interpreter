use std::fmt::Display;

#[derive(Debug)]
pub struct LexerError {
    pub line_number: usize,
    pub error_type: LexerErrorType,
}

impl LexerError {
    pub fn new(error_type: LexerErrorType, line_number: usize) -> Self {
        Self {
            line_number,
            error_type,
        }
    }
}

#[derive(Debug)]
pub enum LexerErrorType {
    UnknownChar(char),
    UnterminatedString,
}

impl Display for LexerErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownChar(c) => write!(f, "Unexpected character: {}", c),
            Self::UnterminatedString => write!(f, "Unterminated string."),
        }
    }
}
