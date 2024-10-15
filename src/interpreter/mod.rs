pub mod environment;
pub mod error;
pub mod value;

use std::{cell::RefCell, rc::Rc};

use environment::Environment;
pub use error::*;
use value::{NumberPair, StringPair, Value};

use crate::parser::{
    stmt::{Block, Expression, Print, Stmt, Var, Visitor as StmtVisitor},
    Assign, Binary, BinaryOp, Expr, Grouping, Literal, LiteralType, Unary, UnaryOp, Variable,
    Visitor as ExprVisitor,
};

#[derive(Default)]
pub struct Interpreter {
    environment: Rc<RefCell<Environment>>,
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

    fn execute_block(
        &mut self,
        statements: &[Stmt],
        new_environment: Rc<RefCell<Environment>>,
    ) -> RuntimeResult<()> {
        let previous_environment = self.environment.clone();
        self.environment = new_environment;

        for statement in statements {
            self.execute(statement)?;
        }

        self.environment = previous_environment;

        Ok(())
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
        let value = if let Some(init) = &stmt.initializer {
            self.evaluate(init)?
        } else {
            Value::Nil
        };

        let mut env = self.environment.borrow_mut();
        env.define(&stmt.name, value);

        Ok(())
    }

    fn block(&mut self, stmt: &Block) -> RuntimeResult<()> {
        let new_environment = Environment::new(Some(self.environment.clone())); // New block environment
        self.execute_block(&stmt.statements, new_environment)?;
        Ok(())
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
        let env = self.environment.borrow();
        let result = env.get(&expr.name)?;
        Ok(result.clone())
    }

    fn assign(&mut self, expr: &Assign) -> RuntimeResult<Value> {
        let value = self.evaluate(&expr.value)?;

        let mut env = self.environment.borrow_mut();
        env.assign(&expr.name, value.clone())?;

        Ok(value)
    }
}
