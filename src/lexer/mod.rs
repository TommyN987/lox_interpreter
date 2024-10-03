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
    line_number: usize,
    eof: bool,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source: source.chars().peekable(),
            line_number: 1,
            eof: false,
        }
    }

    fn handle_string_literal(&mut self) -> Result<Token, LexerError> {
        let mut content = String::new();
        for c in self.source.by_ref() {
            if c == '"' {
                return Ok(Token::new(TokenType::String(content), self.line_number));
            } else if c == '\n' {
                return Err(LexerError::new(
                    LexerErrorType::UnterminatedString,
                    self.line_number,
                ));
            } else {
                content.push(c);
            }
        }
        Err(LexerError::new(
            LexerErrorType::UnterminatedString,
            self.line_number,
        ))
    }

    fn handle_number_literals(&mut self, c: char) -> Token {
        let mut value = String::from(c);
        let mut is_float = false;
        while let Some(char) = self.source.peek() {
            if *char == '.' {
                if is_float {
                    break;
                }
                is_float = true;
                value.push(*char);
                self.source.next();
            } else if char.is_numeric() {
                value.push(*char);
                self.source.next();
            } else {
                break;
            }
        }

        Token::new(
            TokenType::Number(value.parse::<f64>().unwrap()),
            self.line_number,
        )
    }

    fn handle_ident_and_reserved(&mut self, c: char) -> Token {
        let mut value = String::from(c);
        while let Some(char) = self.source.peek() {
            if *char == ' ' || (!char.is_alphanumeric() && *char != '_') {
                break;
            } else {
                value.push(*char);
                self.source.next();
            }
        }
        if let Some((_, token)) = KEYWORDS.get_key_value(&*value) {
            Token::new(TokenType::Keyword(token.to_owned()), self.line_number)
        } else {
            Token::new(TokenType::Ident(value), self.line_number)
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token, LexerError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.eof {
            None
        } else {
            let token = match self.source.next() {
                Some('{') => Ok(Token::new(TokenType::LeftBrace, self.line_number)),
                Some('}') => Ok(Token::new(TokenType::RightBrace, self.line_number)),
                Some('(') => Ok(Token::new(TokenType::LeftParen, self.line_number)),
                Some(')') => Ok(Token::new(TokenType::RightParen, self.line_number)),
                Some(';') => Ok(Token::new(TokenType::Semicolon, self.line_number)),
                Some(',') => Ok(Token::new(TokenType::Comma, self.line_number)),
                Some('.') => Ok(Token::new(TokenType::Dot, self.line_number)),
                Some('*') => Ok(Token::new(TokenType::Star, self.line_number)),
                Some('+') => Ok(Token::new(TokenType::Plus, self.line_number)),
                Some('-') => Ok(Token::new(TokenType::Minus, self.line_number)),
                Some(' ') => Ok(Token::new(TokenType::Whitespace, self.line_number)),
                Some('\t') => Ok(Token::new(TokenType::Tab, self.line_number)),
                Some('\n') => {
                    self.line_number += 1;
                    Ok(Token::new(TokenType::NewLine, self.line_number - 1))
                }
                Some('/') => {
                    if self.source.peek() == Some(&'/') {
                        while let Some(&next_char) = self.source.peek() {
                            if next_char == '\n' {
                                break;
                            }
                            self.source.next();
                        }
                        Ok(Token::new(TokenType::Comment, self.line_number))
                    } else {
                        Ok(Token::new(TokenType::Slash, self.line_number))
                    }
                }
                Some('=') => {
                    if self.source.peek() == Some(&'=') {
                        self.source.next();
                        Ok(Token::new(TokenType::EqualEqual, self.line_number))
                    } else {
                        Ok(Token::new(TokenType::Equal, self.line_number))
                    }
                }
                Some('!') => {
                    if self.source.peek() == Some(&'=') {
                        self.source.next();
                        Ok(Token::new(TokenType::BangEqual, self.line_number))
                    } else {
                        Ok(Token::new(TokenType::Bang, self.line_number))
                    }
                }
                Some('<') => {
                    if self.source.peek() == Some(&'=') {
                        self.source.next();
                        Ok(Token::new(TokenType::LessEqual, self.line_number))
                    } else {
                        Ok(Token::new(TokenType::Less, self.line_number))
                    }
                }
                Some('>') => {
                    if self.source.peek() == Some(&'=') {
                        self.source.next();
                        Ok(Token::new(TokenType::GreaterEqual, self.line_number))
                    } else {
                        Ok(Token::new(TokenType::Greater, self.line_number))
                    }
                }
                Some('"') => self.handle_string_literal(),
                Some(c) => {
                    if c.is_numeric() {
                        Ok(self.handle_number_literals(c))
                    } else if c.is_alphabetic() || c == '_' {
                        Ok(self.handle_ident_and_reserved(c))
                    } else {
                        Err(LexerError::new(
                            LexerErrorType::UnknownChar(c),
                            self.line_number,
                        ))
                    }
                }
                None => {
                    self.eof = true;
                    Ok(Token::new(TokenType::Eof, self.line_number))
                }
            };
            Some(token)
        }
    }
}
