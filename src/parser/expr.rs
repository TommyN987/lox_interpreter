use std::fmt::Display;

use crate::define_ast;

define_ast!(Expr,
    Binary(binary) { left: Box<Expr>, operator: BinaryOp, right: Box<Expr>, line_number: usize },
    Grouping(grouping) { expression: Box<Expr>, line_number: usize  },
    Literal(literal) { literal_type: LiteralType, line_number: usize  },
    Unary(unary) { operator: UnaryOp, right: Box<Expr>, line_number: usize  },
);

#[derive(Debug, Clone)]
pub enum BinaryOp {
    Plus,
    Minus,
    Mul,
    Div,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Equal,
    NotEqual,
}

impl Display for BinaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Plus => "+",
                Self::Minus => "-",
                Self::Mul => "*",
                Self::Div => "/",
                Self::Greater => ">",
                Self::GreaterEqual => ">=",
                Self::Less => "<",
                Self::LessEqual => "<=",
                Self::Equal => "==",
                Self::NotEqual => "!=",
            }
        )
    }
}

#[derive(Debug, Clone)]
pub enum LiteralType {
    String { value: String },
    Number { value: f64 },
    Bool { value: bool },
    Nil,
}

impl Display for LiteralType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Nil => "nil".to_string(),
                Self::Bool { value } => value.to_string(),
                Self::String { value } => value.to_string(),
                Self::Number { value } => {
                    let mut stringified = value.to_string();
                    if stringified.contains('.') {
                        stringified
                    } else {
                        stringified.push_str(".0");
                        stringified
                    }
                }
            }
        )
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.literal_type.to_string())
    }
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Bang,
    Minus,
}

impl Display for UnaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Bang => '!',
                Self::Minus => '-',
            }
        )
    }
}
