pub mod error;
pub mod expr;
pub(super) mod macros;

pub use error::*;
pub use expr::*;

use crate::lexer::{Keyword, Token, TokenType};

pub struct Parser<'a> {
    tokens: &'a [Token],
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> ParseResult<Expr> {
        self.expression()
    }
}

impl<'a> Parser<'a> {
    fn expression(&mut self) -> ParseResult<Expr> {
        self.primary()
    }

    fn primary(&mut self) -> ParseResult<Expr> {
        match &self.peek().token_type {
            TokenType::Keyword(keyword) => match keyword {
                Keyword::False => Ok(Expr::Literal(Literal::new(LiteralType::Bool {
                    value: false,
                }))),
                Keyword::True => Ok(Expr::Literal(Literal::new(LiteralType::Bool {
                    value: true,
                }))),
                Keyword::Nil => Ok(Expr::Literal(Literal::new(LiteralType::Nil))),
                _ => todo!(),
            },
            _ => todo!(),
        }
    }
}

impl<'a> Parser<'a> {
    fn advance(&mut self) -> &'a Token {
        if !self.is_at_end() {
            self.current += 1
        }
        self.previous()
    }

    fn previous(&self) -> &'a Token {
        &self.tokens[self.current - 1]
    }

    fn consume(&mut self, token_type: &'a TokenType, message: &str) -> ParseResult<()> {
        if self.check(token_type) {
            self.advance();
            Ok(())
        } else {
            Err(ParseError::new(self.peek().clone(), message))
        }
    }

    fn matched(&mut self, token_types: &[TokenType]) -> bool {
        for tt in token_types {
            if self.check(tt) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().token_type == *token_type
        }
    }

    fn peek(&self) -> &'a Token {
        &self.tokens[self.current]
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }
}
