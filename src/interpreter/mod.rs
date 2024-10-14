pub mod environment;
pub mod error;
pub mod value;

use environment::Environment;
pub use error::*;
use value::{NumberPair, StringPair, Value};

use crate::parser::{
    stmt::{Expression, Print, Stmt, Var, Visitor as StmtVisitor},
    Binary, BinaryOp, Expr, Grouping, Literal, LiteralType, Unary, UnaryOp, Variable,
    Visitor as ExprVisitor,
};

#[derive(Default)]
pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn interpret(&mut self, stmts: &[Stmt]) -> RuntimeResult<()> {
        for stmt in stmts {
            self.execute(stmt)?;
        }
        Ok(())
    }

    pub fn evaluate(&mut self, expr: &Expr) -> RuntimeResult<Value> {
        expr.accept(self)
    }

    fn execute(&mut self, stmt: &Stmt) -> RuntimeResult<()> {
        stmt.accept(self)
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

impl StmtVisitor<RuntimeResult<()>> for Interpreter {
    fn expression(&mut self, stmt: &Expression) -> RuntimeResult<()> {
        self.evaluate(&stmt.expression)?;
        Ok(())
    }

    fn print(&mut self, stmt: &Print) -> RuntimeResult<()> {
        let value = self.evaluate(&stmt.expression)?;
        println!("{}", value);
        Ok(())
    }

    fn var(&mut self, stmt: &Var) -> RuntimeResult<()> {
        if let Some(init) = &stmt.initializer {
            let value = self.evaluate(init)?;
            self.environment.define(&stmt.name, value);
            Ok(())
        } else {
            self.environment.define(&stmt.name, Value::Nil);
            Ok(())
        }
    }
}

impl ExprVisitor<RuntimeResult<Value>> for Interpreter {
    fn literal(&mut self, expr: &Literal) -> RuntimeResult<Value> {
        match &expr.literal_type {
            LiteralType::Nil => Ok(Value::Nil),
            LiteralType::Bool { value } => Ok(Value::Boolean(*value)),
            LiteralType::String { value } => Ok(Value::String(value.clone())),
            LiteralType::Number { value } => Ok(Value::Number(*value)),
        }
    }

    fn grouping(&mut self, expr: &Grouping) -> RuntimeResult<Value> {
        self.evaluate(&expr.expression)
    }

    fn unary(&mut self, expr: &Unary) -> RuntimeResult<Value> {
        let right = self.evaluate(&expr.right)?;

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
        let left = self.evaluate(&expr.left)?;
        let right = self.evaluate(&expr.right)?;

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

    fn variable(&mut self, expr: &Variable) -> RuntimeResult<Value> {
        let result = self.environment.get(&expr.name)?;
        Ok(result.clone())
    }
}
