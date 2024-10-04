use std::fmt::Display;

use crate::lexer::Token;

#[derive(Debug)]
pub struct ParseError {
    pub token: Token,
    pub message: String,
}

pub type ParseResult<T> = Result<T, ParseError>;

impl ParseError {
    pub fn new(token: Token, message: &str) -> Self {
        Self {
            token,
            message: message.to_string(),
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
