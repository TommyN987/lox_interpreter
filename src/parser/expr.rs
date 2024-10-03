use crate::define_ast;

define_ast!(Expr,
    Binary(binary) { left: Box<Expr>, operator: BinaryOp, right: Box<Expr> },
    Grouping(grouping) { expression: Vec<Expr> },
    Literal(literal) { literal_type: LiteralType },
    Unary(unary) { operator: UnaryOp, right: Box<Expr> },
);

pub enum BinaryOp {
    Plus,
    Minus,
    Mul,
    Div,
}

pub enum LiteralType {
    String,
    Number,
    Bool,
    Nil,
}

pub enum UnaryOp {
    Bang,
    Minus,
}
