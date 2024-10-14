use std::collections::HashMap;

use crate::lexer::Token;

use super::{value::Value, RuntimeError, RuntimeResult};

#[derive(Default)]
pub struct Environment {
    values: HashMap<String, Value>,
}

impl Environment {
    pub fn define(&mut self, name: &str, value: Value) {
        self.values.insert(name.to_string(), value);
    }

    pub fn get(&self, token: &Token) -> RuntimeResult<&Value> {
        let name = &token.token_type.lexeme();
        match self.values.get(name) {
            Some(value) => Ok(value),
            None => Err(RuntimeError::new(
                token.line_number,
                format!("Undefined variable \"{}\".", name).as_str(),
            )),
        }
    }
}
