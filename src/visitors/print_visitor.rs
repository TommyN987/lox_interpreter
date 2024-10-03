use crate::parser::{Binary, Expr, Grouping, Literal, Unary, Visitor};

pub struct PrintVisitor;

impl Visitor<String> for PrintVisitor {
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
        self.parenthesize("group", &expr.expressions.iter().collect::<Vec<_>>())
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
