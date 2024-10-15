use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::lexer::Token;

use super::{value::Value, RuntimeError, RuntimeResult};

#[derive(Default, Clone)]
pub struct Environment {
    pub enclosing: Option<Rc<RefCell<Environment>>>,
    values: HashMap<String, Value>,
}

impl Environment {
    pub fn new(enclosing: Option<Rc<RefCell<Environment>>>) -> Rc<RefCell<Environment>> {
        Rc::new(RefCell::new(Environment {
            values: HashMap::new(),
            enclosing,
        }))
    }

    pub fn define(&mut self, name: &str, value: Value) {
        self.values.insert(name.to_string(), value);
    }

    pub fn get(&self, token: &Token) -> RuntimeResult<Value> {
        let name = &token.token_type.lexeme();

        if let Some(value) = self.values.get(name) {
            return Ok(value.clone());
        }

        if let Some(ref enclosing) = self.enclosing {
            return enclosing.borrow().get(token);
        }

        Err(RuntimeError::new(
            token.line_number,
            format!("Undefined variable \"{}\".", name).as_str(),
        ))
    }

    pub fn assign(&mut self, name: &Token, value: Value) -> RuntimeResult<()> {
        if let Some(old_value) = self.values.get_mut(&name.token_type.lexeme()) {
            *old_value = value;
            return Ok(());
        }

        if let Some(ref enclosing) = self.enclosing {
            return enclosing.borrow_mut().assign(name, value);
        }

        Err(RuntimeError::new(
            name.line_number,
            format!("Undefined variable \"{}\".", name.token_type.lexeme()).as_str(),
        ))
    }
}
