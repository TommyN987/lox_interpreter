use crate::parser::{
    stmt::{Expression, Print, Var, Visitor as StmtVisitor},
    Binary, Expr, Grouping, Literal, Unary, Variable, Visitor as ExprVisitor,
};

pub struct PrintVisitor;

impl StmtVisitor<String> for PrintVisitor {
    fn print(&mut self, expr: &Print) -> String {
        expr.expression.accept(self)
    }

    fn expression(&mut self, expr: &Expression) -> String {
        expr.accept(self)
    }

    fn var(&mut self, expr: &Var) -> String {
        expr.accept(self)
    }
}

impl ExprVisitor<String> for PrintVisitor {
    fn unary(&mut self, expr: &Unary) -> String {
        self.parenthesize(&expr.operator.to_string(), &[&expr.right])
    }

    fn binary(&mut self, expr: &Binary) -> String {
        self.parenthesize(&expr.operator.to_string(), &[&expr.left, &expr.right])
    }

    fn literal(&mut self, expr: &Literal) -> String {
        expr.to_string()
    }

    fn grouping(&mut self, expr: &Grouping) -> String {
        self.parenthesize("group", &[&*expr.expression])
    }

    fn variable(&mut self, expr: &Variable) -> String {
        expr.name.token_type.lexeme()
    }
}

impl PrintVisitor {
    pub fn print(&mut self, expr: &Expr) -> String {
        expr.accept(self)
    }

    fn parenthesize(&mut self, name: &str, exprs: &[&Expr]) -> String {
        format!(
            "({} {})",
            name,
            exprs
                .iter()
                .map(|expr| expr.accept(self))
                .collect::<Vec<_>>()
                .join(" ")
        )
    }
}
