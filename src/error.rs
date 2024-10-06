use std::fmt::Display;

use crate::{lexer::LexerError, parser::ParseError};

#[derive(Debug)]
pub enum LoxError {
    LexerError(LexerError),
    ParseError(ParseError),
}

impl From<LexerError> for LoxError {
    fn from(value: LexerError) -> Self {
        Self::LexerError(value)
    }
}

impl From<ParseError> for LoxError {
    fn from(value: ParseError) -> Self {
        Self::ParseError(value)
    }
}

impl Display for LoxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "[line {}] Error: {}",
            match self {
                Self::LexerError(err) => err.line_number,
                Self::ParseError(err) => err.token.line_number,
            },
            match self {
                Self::LexerError(err) => err.error_type.to_string(),
                Self::ParseError(err) => err.message.to_string(),
            }
        )
    }
}
