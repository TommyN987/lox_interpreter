use crate::define_ast;

use super::Expr;

define_ast!(Stmt,
    Print(print) { expression: Expr },
    Expression(expression) { expression: Expr },
    Var(var) { name: String, initializer: Option<Expr> },
    Block(block) { statements: Vec<Stmt> },
);
