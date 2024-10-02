use std::fmt::Display;

use crate::lexer::LexerError;

pub struct LoxError {
    line_number: usize,
    error_type: ErrorType,
}

enum ErrorType {
    LexerError(LexerError),
    SyntaxError(String),
}

impl Display for LoxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "[line {}] Error: {}",
            self.line_number,
            match &self.error_type {
                ErrorType::LexerError(err) => err.to_string(),
                ErrorType::SyntaxError(message) => message.to_string(),
            }
        )
    }
}
