pub mod error;
pub mod value;

pub use error::*;
use value::{NumberPair, StringPair, Value};

use crate::parser::{
    Binary, BinaryOp, Expr, Grouping, Literal, LiteralType, Unary, UnaryOp, Visitor,
};

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
        let left = self.evaluate(*expr.left.clone())?;
        let right = self.evaluate(*expr.right.clone())?;

        match expr.operator {
            BinaryOp::Mul => match NumberPair::try_from((&left, &right)) {
                Ok(pair) => Ok(Value::Number(pair.0 * pair.1)),
                Err(_) => Err(RuntimeError::new(
                    expr.line_number,
                    "Operands must be numbers.",
                )),
            },
            BinaryOp::Div => match NumberPair::try_from((&left, &right)) {
                Ok(pair) => Ok(Value::Number(pair.0 / pair.1)),
                Err(_) => Err(RuntimeError::new(
                    expr.line_number,
                    "Operands must be numbers.",
                )),
            },
            BinaryOp::Minus => match NumberPair::try_from((&left, &right)) {
                Ok(pair) => Ok(Value::Number(pair.0 - pair.1)),
                Err(_) => Err(RuntimeError::new(
                    expr.line_number,
                    "Operands must be numbers.",
                )),
            },
            BinaryOp::Plus => {
                if let Ok(pair) = NumberPair::try_from((&left, &right)) {
                    return Ok(Value::Number(pair.0 + pair.1));
                }
                if let Ok(pair) = StringPair::try_from((&left, &right)) {
                    let concatenated = String::from_iter([pair.0, pair.1]);
                    return Ok(Value::String(concatenated));
                }
                Err(RuntimeError::new(expr.line_number, "blabla"))
            }
            BinaryOp::Greater => match NumberPair::try_from((&left, &right)) {
                Ok(pair) => Ok(Value::Boolean(pair.0 > pair.1)),
                Err(_) => Err(RuntimeError::new(
                    expr.line_number,
                    "Operands must be numbers.",
                )),
            },
            BinaryOp::Less => match NumberPair::try_from((&left, &right)) {
                Ok(pair) => Ok(Value::Boolean(pair.0 < pair.1)),
                Err(_) => Err(RuntimeError::new(
                    expr.line_number,
                    "Operands must be numbers.",
                )),
            },
            BinaryOp::GreaterEqual => match NumberPair::try_from((&left, &right)) {
                Ok(pair) => Ok(Value::Boolean(pair.0 >= pair.1)),
                Err(_) => Err(RuntimeError::new(
                    expr.line_number,
                    "Operands must be numbers.",
                )),
            },
            BinaryOp::LessEqual => match NumberPair::try_from((&left, &right)) {
                Ok(pair) => Ok(Value::Boolean(pair.0 <= pair.1)),
                Err(_) => Err(RuntimeError::new(
                    expr.line_number,
                    "Operands must be numbers.",
                )),
            },
            BinaryOp::Equal => Ok(Value::Boolean(left == right)),
            BinaryOp::NotEqual => Ok(Value::Boolean(left != right)),
        }
    }
}
