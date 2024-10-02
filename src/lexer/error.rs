use std::fmt::Display;

#[derive(Debug)]
pub enum LexerError {
    UnknownChar(char),
    UnterminatedString,
}

impl Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownChar(c) => write!(f, "Unexpected character: {}", c),
            Self::UnterminatedString => write!(f, "Unterminated string."),
        }
    }
}
