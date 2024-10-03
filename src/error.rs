use std::fmt::Display;

use crate::lexer::LexerError;

pub enum LoxError {
    LexerError(LexerError),
    SyntaxError(String, usize),
}

impl From<LexerError> for LoxError {
    fn from(value: LexerError) -> Self {
        Self::LexerError(value)
    }
}

impl Display for LoxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "[line {}] Error: {}",
            match self {
                Self::LexerError(err) => err.line_number,
                Self::SyntaxError(_, line_number) => *line_number,
            },
            match self {
                Self::LexerError(err) => err.error_type.to_string(),
                Self::SyntaxError(message, _) => message.to_string(),
            }
        )
    }
}
