use std::{collections::HashMap, fmt::Display, sync::LazyLock};

pub(super) static KEYWORDS: LazyLock<HashMap<&'static str, Keyword>> = LazyLock::new(|| {
    HashMap::from([
        ("and", Keyword::And),
        ("class", Keyword::Class),
        ("else", Keyword::Else),
        ("false", Keyword::False),
        ("for", Keyword::For),
        ("fun", Keyword::Fun),
        ("if", Keyword::If),
        ("nil", Keyword::Nil),
        ("or", Keyword::Or),
        ("print", Keyword::Print),
        ("return", Keyword::Return),
        ("super", Keyword::Super),
        ("this", Keyword::This),
        ("true", Keyword::True),
        ("var", Keyword::Var),
        ("while", Keyword::While),
    ])
});

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
}

impl Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::And => "and",
                Self::Class => "class",
                Self::Else => "else",
                Self::False => "false",
                Self::For => "for",
                Self::Fun => "fun",
                Self::If => "if",
                Self::Nil => "nil",
                Self::Or => "or",
                Self::Print => "print",
                Self::Return => "return",
                Self::Super => "super",
                Self::This => "this",
                Self::True => "true",
                Self::Var => "var",
                Self::While => "while",
            }
        )
    }
}
