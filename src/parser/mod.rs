pub mod error;
pub mod expr;
pub(super) mod macros;
pub mod stmt;

pub use error::*;
pub use expr::*;
use stmt::{Block, Expression, Print, Stmt, Var};

use crate::lexer::{Keyword, Token, TokenType};

pub struct Parser<'a> {
    tokens: &'a [Token],
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> ParseResult<Vec<Stmt>> {
        let mut stmts = vec![];
        let mut had_error = None;

        loop {
            self.skip_whitespace();
            if self.is_at_end() {
                break;
            }
            match self.declaration() {
                Ok(stmt) => stmts.push(stmt),
                Err(err) => {
                    had_error = Some(err);
                    self.synchronize();
                }
            }
        }

        match had_error {
            Some(err) => Err(err),
            None => Ok(stmts),
        }
    }

    pub fn parse_expr(&mut self) -> ParseResult<Expr> {
        self.expression()
    }
}

impl<'a> Parser<'a> {
    fn declaration(&mut self) -> ParseResult<Stmt> {
        if self.matched(&[TokenType::Keyword(Keyword::Var)]) {
            let next = self.advance();
            return self.var_declaration(next);
        }
        self.statement()
    }

    fn var_declaration(&mut self, name_token: &Token) -> ParseResult<Stmt> {
        let name = if let TokenType::Ident(ref name_str) = name_token.token_type {
            name_str.clone()
        } else {
            return Err(ParseError::new(
                name_token.clone(),
                "Expected an identifier.",
            ));
        };

        let mut initializer = None;

        if self.matched(&[TokenType::Equal]) {
            initializer = Some(self.expression()?);
        }

        self.consume(
            &TokenType::Semicolon,
            "Expect ';' after variable declaration.",
        )?;

        Ok(Stmt::Var(Var::new(name, initializer)))
    }

    fn statement(&mut self) -> ParseResult<Stmt> {
        self.skip_whitespace();

        if self.matched(&[TokenType::Keyword(Keyword::Print)]) {
            return self.print_statement();
        }

        if self.matched(&[TokenType::LeftBrace]) {
            return Ok(Stmt::Block(Block::new(self.block()?)));
        }

        self.expression_statement()
    }

    fn block(&mut self) -> ParseResult<Vec<Stmt>> {
        let mut statements = vec![];

        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        self.consume(&TokenType::RightBrace, "Expect '}' after block.")?;
        Ok(statements)
    }

    fn expression_statement(&mut self) -> ParseResult<Stmt> {
        let expr = self.expression()?;
        self.consume(&TokenType::Semicolon, "Expect ';' afrer expression.")?;
        Ok(Stmt::Expression(Expression::new(expr)))
    }

    fn print_statement(&mut self) -> ParseResult<Stmt> {
        let value = self.expression()?;
        if self.is_at_end() {
            return Err(ParseError::new(
                self.peek().clone(),
                "Unexpected end of input after print statement.",
            ));
        }
        self.consume(&TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::Print(Print::new(value)))
    }

    fn expression(&mut self) -> ParseResult<Expr> {
        self.assignment()
    }

    fn assignment(&mut self) -> ParseResult<Expr> {
        let expr = self.equality()?;

        if self.matched(&[TokenType::Equal]) {
            let equals = self.previous();
            let value = self.assignment()?;

            match expr {
                Expr::Variable(var) => {
                    let name = var.name;
                    Ok(Expr::Assign(Assign::new(name, Box::new(value))))
                }
                _ => Err(ParseError::new(
                    equals.clone(),
                    "Invalid assignment target.",
                )),
            }
        } else {
            Ok(expr)
        }
    }

    fn equality(&mut self) -> ParseResult<Expr> {
        let mut expr = self.comparison()?;

        while self.matched(&[TokenType::EqualEqual, TokenType::BangEqual]) {
            let token = self.previous();
            let operator = match token.token_type {
                TokenType::EqualEqual => Ok(BinaryOp::Equal),
                TokenType::BangEqual => Ok(BinaryOp::NotEqual),
                _ => Err(ParseError::new(
                    token.clone(),
                    "You're checking for equality, not whatever this was.",
                )),
            };
            let right = self.comparison()?;
            expr = Expr::Binary(Binary::new(
                Box::new(expr),
                operator?,
                Box::new(right),
                token.line_number,
            ))
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
            let token = self.previous();
            let operator = match token.token_type {
                TokenType::Greater => Ok(BinaryOp::Greater),
                TokenType::GreaterEqual => Ok(BinaryOp::GreaterEqual),
                TokenType::Less => Ok(BinaryOp::Less),
                TokenType::LessEqual => Ok(BinaryOp::LessEqual),
                _ => Err(ParseError::new(
                    token.clone(),
                    "You're trying to compare, not whatever this is.",
                )),
            };

            let right = self.term()?;
            expr = Expr::Binary(Binary::new(
                Box::new(expr),
                operator?,
                Box::new(right),
                token.line_number,
            ))
        }

        Ok(expr)
    }

    fn term(&mut self) -> ParseResult<Expr> {
        let mut expr = self.factor()?;

        while self.matched(&[TokenType::Minus, TokenType::Plus]) {
            let token = self.previous();
            let operator = match token.token_type {
                TokenType::Plus => Ok(BinaryOp::Plus),
                TokenType::Minus => Ok(BinaryOp::Minus),
                _ => Err(ParseError::new(
                    token.clone(),
                    "This really should be plus or minus",
                )),
            };
            let right = self.factor()?;
            expr = Expr::Binary(Binary::new(
                Box::new(expr),
                operator?,
                Box::new(right),
                token.line_number,
            ))
        }

        Ok(expr)
    }

    fn factor(&mut self) -> ParseResult<Expr> {
        let mut expr = self.unary()?;

        while self.matched(&[TokenType::Slash, TokenType::Star]) {
            let token = self.previous();
            let right = self.unary()?;
            let operator: Result<BinaryOp, ParseError> = match token.token_type {
                TokenType::Slash => Ok(BinaryOp::Div),
                TokenType::Star => Ok(BinaryOp::Mul),
                _ => Err(ParseError::new(
                    token.clone(),
                    "If you're here, you messed up",
                )),
            };
            expr = Expr::Binary(Binary::new(
                Box::new(expr),
                operator?,
                Box::new(right),
                token.line_number,
            ))
        }
        Ok(expr)
    }

    fn unary(&mut self) -> ParseResult<Expr> {
        if self.matched(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary()?;
            return match operator.token_type {
                TokenType::Bang => Ok(Expr::Unary(Unary::new(
                    UnaryOp::Bang,
                    Box::new(right),
                    operator.line_number,
                ))),
                TokenType::Minus => Ok(Expr::Unary(Unary::new(
                    UnaryOp::Minus,
                    Box::new(right),
                    operator.line_number,
                ))),
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
            let token = self.previous();
            return Ok(Expr::Literal(Literal::new(
                LiteralType::Bool { value: false },
                token.line_number,
            )));
        }

        if self.matched(&[TokenType::Keyword(Keyword::True)]) {
            return Ok(Expr::Literal(Literal::new(
                LiteralType::Bool { value: true },
                token.line_number,
            )));
        }

        if self.matched(&[TokenType::Keyword(Keyword::Nil)]) {
            return Ok(Expr::Literal(Literal::new(
                LiteralType::Nil,
                token.line_number,
            )));
        }

        if let TokenType::Number { lexeme: _, literal } = &token.token_type {
            self.advance();
            return Ok(Expr::Literal(Literal::new(
                LiteralType::Number { value: *literal },
                token.line_number,
            )));
        }

        if let TokenType::String(value) = &token.token_type {
            self.advance();
            return Ok(Expr::Literal(Literal::new(
                LiteralType::String {
                    value: value.clone(),
                },
                token.line_number,
            )));
        }

        if self.matched(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(&TokenType::RightParen, "Expect ')' after expression.")?; // Match and consume ')'
            return Ok(Expr::Grouping(Grouping::new(
                Box::new(expr),
                token.line_number,
            )));
        }

        if let TokenType::Ident(_) = &token.token_type {
            self.advance();
            return Ok(Expr::Variable(Variable::new(token.clone())));
        }

        Err(ParseError::new(
            self.peek().clone(),
            "You should not be here",
        ))
    }
}

impl<'a> Parser<'a> {
    fn synchronize(&mut self) {
        if self.is_at_end() {
            return;
        }

        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }

            match self.peek().token_type {
                TokenType::Keyword(Keyword::Class)
                | TokenType::Keyword(Keyword::Fun)
                | TokenType::Keyword(Keyword::Var)
                | TokenType::Keyword(Keyword::For)
                | TokenType::Keyword(Keyword::If)
                | TokenType::Keyword(Keyword::While)
                | TokenType::Keyword(Keyword::Print)
                | TokenType::Keyword(Keyword::Return) => return,
                _ => {}
            }

            self.advance();
        }
    }

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
        self.skip_whitespace();
        if self.is_at_end() {
            return Err(ParseError::new(
                self.peek().clone(),
                "Unexpected end of input.",
            ));
        }
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
        if self.current >= self.tokens.len() {
            return &self.tokens[self.tokens.len() - 1];
        }
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
