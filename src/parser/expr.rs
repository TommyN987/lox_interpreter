use std::fmt::Display;

use crate::define_ast;

define_ast!(Expr,
    Binary(binary) { left: Box<Expr>, operator: BinaryOp, right: Box<Expr> },
    Grouping(grouping) { expression: Box<Expr> },
    Literal(literal) { literal_type: LiteralType },
    Unary(unary) { operator: UnaryOp, right: Box<Expr> },
);

#[derive(Debug, Clone)]
pub enum BinaryOp {
    Plus,
    Minus,
    Mul,
    Div,
}

impl Display for BinaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Plus => '+',
                Self::Minus => '-',
                Self::Mul => '*',
                Self::Div => '/',
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
                Self::Number { value } => value.to_string(),
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
