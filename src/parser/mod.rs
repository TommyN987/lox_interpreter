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
        self.equality()
    }

    fn equality(&mut self) -> ParseResult<Expr> {
        let mut expr = self.comparison()?;

        while self.matched(&[TokenType::EqualEqual, TokenType::BangEqual]) {
            let operator = match self.previous().token_type {
                TokenType::EqualEqual => Ok(BinaryOp::Equal),
                TokenType::BangEqual => Ok(BinaryOp::NotEqual),
                _ => Err(ParseError::new(
                    self.previous().clone(),
                    "You're checking for equality, not whatever this was.",
                )),
            };
            let right = self.comparison()?;
            expr = Expr::Binary(Binary::new(Box::new(expr), operator?, Box::new(right)))
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> ParseResult<Expr> {
        let mut expr = self.term()?;

        while self.matched(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = match self.previous().token_type {
                TokenType::Greater => Ok(BinaryOp::Greater),
                TokenType::GreaterEqual => Ok(BinaryOp::GreaterEqual),
                TokenType::Less => Ok(BinaryOp::Less),
                TokenType::LessEqual => Ok(BinaryOp::LessEqual),
                _ => Err(ParseError::new(
                    self.previous().clone(),
                    "You're trying to compare, not whatever this is.",
                )),
            };

            let right = self.term()?;
            expr = Expr::Binary(Binary::new(Box::new(expr), operator?, Box::new(right)))
        }

        Ok(expr)
    }

    fn term(&mut self) -> ParseResult<Expr> {
        let mut expr = self.factor()?;

        while self.matched(&[TokenType::Minus, TokenType::Plus]) {
            let operator = match self.previous().token_type {
                TokenType::Plus => Ok(BinaryOp::Plus),
                TokenType::Minus => Ok(BinaryOp::Minus),
                _ => Err(ParseError::new(
                    self.previous().clone(),
                    "This really should be plus or minus",
                )),
            };
            let right = self.factor()?;
            expr = Expr::Binary(Binary::new(Box::new(expr), operator?, Box::new(right)))
        }

        Ok(expr)
    }

    fn factor(&mut self) -> ParseResult<Expr> {
        let mut expr = self.unary()?;

        while self.matched(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary()?;
            let operator: Result<BinaryOp, ParseError> = match operator.token_type {
                TokenType::Slash => Ok(BinaryOp::Div),
                TokenType::Star => Ok(BinaryOp::Mul),
                _ => Err(ParseError::new(
                    self.previous().clone(),
                    "If you're here, you messed up",
                )),
            };
            expr = Expr::Binary(Binary::new(Box::new(expr), operator?, Box::new(right)))
        }
        Ok(expr)
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
        self.skip_whitespace();
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
        self.skip_whitespace();
        for tt in token_types {
            if self.check(tt) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&mut self, token_type: &TokenType) -> bool {
        self.skip_whitespace();
        if self.is_at_end() {
            false
        } else {
            self.peek().token_type == *token_type
        }
    }

    fn peek(&self) -> &'a Token {
        &self.tokens[self.current]
    }

    fn skip_whitespace(&mut self) {
        while self.matching_whitespace() && !self.is_at_end() {
            self.current += 1;
        }
    }

    fn matching_whitespace(&self) -> bool {
        matches!(
            self.peek().token_type,
            TokenType::Whitespace | TokenType::Tab | TokenType::NewLine | TokenType::Comment
        )
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }
}
