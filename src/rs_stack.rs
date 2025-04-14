use crate::rs_ast::Value;
use crate::rs_error::RSLogoError;

#[derive(Debug)]
pub struct Stack {
    items: Vec<Value>,
}

impl Stack {
    pub fn new() -> Self {
        Stack { items: Vec::new() }
    }

    pub fn push(&mut self, value: Value) {
        self.items.push(value);
    }

    pub fn pop(&mut self) -> Result<Value, RSLogoError> {
        self.items.pop().ok_or(RSLogoError::StackUnderflow)
    }
}
