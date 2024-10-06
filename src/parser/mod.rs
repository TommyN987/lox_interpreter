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
        self.unary()
    }

    fn unary(&mut self) -> ParseResult<Expr> {
        if self.matched(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary()?;
            return match operator.token_type {
                TokenType::Bang => Ok(Expr::Unary(Unary::new(UnaryOp::Bang, Box::new(right)))),
                TokenType::Minus => Ok(Expr::Unary(Unary::new(UnaryOp::Minus, Box::new(right)))),
                _ => Err(ParseError::new(
                    self.previous().clone(),
                    "You should not be here",
                )),
            };
        }

        self.primary()
    }

    fn primary(&mut self) -> ParseResult<Expr> {
        let token = self.peek();
        if self.matched(&[TokenType::Keyword(Keyword::False)]) {
            return Ok(Expr::Literal(Literal::new(LiteralType::Bool {
                value: false,
            })));
        }

        if self.matched(&[TokenType::Keyword(Keyword::True)]) {
            return Ok(Expr::Literal(Literal::new(LiteralType::Bool {
                value: true,
            })));
        }

        if self.matched(&[TokenType::Keyword(Keyword::Nil)]) {
            return Ok(Expr::Literal(Literal::new(LiteralType::Nil)));
        }

        if let TokenType::Number { lexeme: _, literal } = &token.token_type {
            self.advance();
            return Ok(Expr::Literal(Literal::new(LiteralType::Number {
                value: *literal,
            })));
        }

        if let TokenType::String(value) = &token.token_type {
            self.advance();
            return Ok(Expr::Literal(Literal::new(LiteralType::String {
                value: value.clone(),
            })));
        }

        if self.matched(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(&TokenType::RightParen, "Expect ')' after expression.")?; // Match and consume ')'
            return Ok(Expr::Grouping(Grouping::new(Box::new(expr))));
        }

        Err(ParseError::new(
            self.peek().clone(),
            "You should not be here",
        ))
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
