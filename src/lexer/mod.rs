pub mod error;
pub mod keywords;
pub mod token;

use std::{iter::Peekable, str::Chars};

pub use error::*;
pub use keywords::*;
pub use token::*;

#[derive(Debug)]
pub struct Lexer<'a> {
    source: Peekable<Chars<'a>>,
    eof: bool,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source: source.chars().peekable(),
            eof: false,
        }
    }
}
