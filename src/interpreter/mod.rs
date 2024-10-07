pub mod error;
pub mod value;

pub use error::*;
use value::Value;

use crate::parser::{Binary, Expr, Grouping, Literal, LiteralType, Unary, UnaryOp, Visitor};

pub struct Interpreter;

impl Interpreter {
    pub fn evaluate(&mut self, expr: Expr) -> RuntimeResult<Value> {
        Ok(expr.accept(self)?)
    }

    fn is_truthy(value: &Value) -> bool {
        match value {
            Value::Boolean(value) => *value,
            Value::Number(_) => true,
            Value::String(_) => true,
            Value::Nil => false,
        }
    }
}

impl Visitor<RuntimeResult<Value>> for Interpreter {
    fn literal(&mut self, expr: &Literal) -> RuntimeResult<Value> {
        match &expr.literal_type {
            LiteralType::Nil => Ok(Value::Nil),
            LiteralType::Bool { value } => Ok(Value::Boolean(*value)),
            LiteralType::String { value } => Ok(Value::String(value.clone())),
            LiteralType::Number { value } => Ok(Value::Number(*value)),
        }
    }

    fn grouping(&mut self, expr: &Grouping) -> RuntimeResult<Value> {
        let expr = *expr.expression.clone();
        Ok(self.evaluate(expr)?)
    }

    fn unary(&mut self, expr: &Unary) -> RuntimeResult<Value> {
        let right = self.evaluate(*expr.right.clone())?;

        match expr.operator {
            UnaryOp::Minus => match right {
                Value::Number(value) => Ok(Value::Number(value * -1.0)),
                _ => Err(RuntimeError::new(
                    expr.line_number,
                    "Operand must be a number",
                )),
            },
            UnaryOp::Bang => Ok(Value::Boolean(!Self::is_truthy(&right))),
        }
    }

    fn binary(&mut self, expr: &Binary) -> RuntimeResult<Value> {
        todo!()
    }
}
