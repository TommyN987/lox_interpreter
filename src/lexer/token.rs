use std::fmt::Display;

use super::Keyword;

#[derive(Debug, Clone)]
pub struct Token {
    pub line_number: usize,
    pub token_type: TokenType,
}

impl Token {
    pub fn new(token_type: TokenType, line_number: usize) -> Self {
        Self {
            line_number,
            token_type,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.token_type,
            self.token_type.lexeme(),
            self.token_type.literal()
        )
    }
}

#[derive(Debug, Clone)]
pub enum TokenType {
    // Single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Ident(String),
    String(String),
    Number { lexeme: String, literal: f64 },

    // Keyword
    Keyword(Keyword),

    // Whitespace
    Whitespace,
    Tab,
    NewLine,
    Comment,

    Eof,
}

impl TokenType {
    pub fn lexeme(&self) -> String {
        match self {
            Self::LeftParen => "(".to_string(),
            Self::RightParen => ")".to_string(),
            Self::LeftBrace => "{".to_string(),
            Self::RightBrace => "}".to_string(),
            Self::Comma => ",".to_string(),
            Self::Dot => ".".to_string(),
            Self::Minus => "-".to_string(),
            Self::Plus => "+".to_string(),
            Self::Semicolon => ";".to_string(),
            Self::Slash => "/".to_string(),
            Self::Star => "*".to_string(),
            Self::Bang => "!".to_string(),
            Self::BangEqual => "!=".to_string(),
            Self::Equal => "=".to_string(),
            Self::EqualEqual => "==".to_string(),
            Self::Greater => ">".to_string(),
            Self::GreaterEqual => ">=".to_string(),
            Self::Less => "<".to_string(),
            Self::LessEqual => "<=".to_string(),
            Self::Ident(ident) => ident.to_string(),
            Self::String(string) => format!("\"{}\"", string),
            Self::Number { lexeme, literal: _ } => lexeme.to_string(),
            Self::Keyword(keyword) => keyword.to_string(),
            Self::Whitespace | Self::Tab | Self::NewLine | Self::Comment => "".to_string(),
            Self::Eof => "".to_string(),
        }
    }

    pub fn literal(&self) -> String {
        match self {
            Self::String(string) => string.to_string(),
            Self::Number { lexeme: _, literal } => {
                let mut stringified = literal.to_string();
                if stringified.contains('.') {
                    stringified
                } else {
                    stringified.push_str(".0");
                    stringified
                }
            }
            _ => "null".to_string(),
        }
    }
}

impl PartialEq for TokenType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            // Compare simple tokens directly
            (TokenType::LeftParen, TokenType::LeftParen)
            | (TokenType::RightParen, TokenType::RightParen)
            | (TokenType::LeftBrace, TokenType::LeftBrace)
            | (TokenType::RightBrace, TokenType::RightBrace)
            | (TokenType::Comma, TokenType::Comma)
            | (TokenType::Dot, TokenType::Dot)
            | (TokenType::Minus, TokenType::Minus)
            | (TokenType::Plus, TokenType::Plus)
            | (TokenType::Semicolon, TokenType::Semicolon)
            | (TokenType::Slash, TokenType::Slash)
            | (TokenType::Star, TokenType::Star)
            | (TokenType::Bang, TokenType::Bang)
            | (TokenType::BangEqual, TokenType::BangEqual)
            | (TokenType::Equal, TokenType::Equal)
            | (TokenType::EqualEqual, TokenType::EqualEqual)
            | (TokenType::Greater, TokenType::Greater)
            | (TokenType::GreaterEqual, TokenType::GreaterEqual)
            | (TokenType::Less, TokenType::Less)
            | (TokenType::LessEqual, TokenType::LessEqual)
            | (TokenType::Whitespace, TokenType::Whitespace)
            | (TokenType::Tab, TokenType::Tab)
            | (TokenType::NewLine, TokenType::NewLine)
            | (TokenType::Comment, TokenType::Comment)
            | (TokenType::Eof, TokenType::Eof) => true,

            (TokenType::Ident(_), TokenType::Ident(_)) => true,
            (TokenType::String(_), TokenType::String(_)) => true,
            (TokenType::Number { .. }, TokenType::Number { .. }) => true,

            (TokenType::Keyword(a), TokenType::Keyword(b)) => a == b,

            _ => false,
        }
    }
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::LeftParen => "LEFT_PAREN",
                Self::RightParen => "RIGHT_PAREN",
                Self::LeftBrace => "LEFT_BRACE",
                Self::RightBrace => "RIGHT_BRACE",
                Self::Comma => "COMMA",
                Self::Dot => "DOT",
                Self::Minus => "MINUS",
                Self::Plus => "PLUS",
                Self::Semicolon => "SEMICOLON",
                Self::Slash => "SLASH",
                Self::Star => "STAR",
                Self::Bang => "BANG",
                Self::BangEqual => "BANG_EQUAL",
                Self::Equal => "EQUAL",
                Self::EqualEqual => "EQUAL_EQUAL",
                Self::Greater => "GREATER",
                Self::GreaterEqual => "GREATER_EQUAL",
                Self::Less => "LESS",
                Self::LessEqual => "LESS_EQUAL",
                Self::Ident(_) => "IDENTIFIER",
                Self::String(_) => "STRING",
                Self::Number {
                    lexeme: _,
                    literal: _,
                } => "NUMBER",
                Self::Keyword(keyword) => {
                    let stringified = keyword.to_string().to_uppercase();
                    Box::leak(stringified.into_boxed_str())
                }
                Self::Whitespace | Self::Tab | Self::NewLine | Self::Comment => "",
                Self::Eof => "EOF",
            }
        )
    }
}
