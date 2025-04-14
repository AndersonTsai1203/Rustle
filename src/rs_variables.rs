use crate::rs_ast::Value;
use std::collections::HashMap;

#[derive(Clone)]
pub struct VariableScope {
    variables: HashMap<String, Value>,
}

pub struct VariableManager {
    current_scope: VariableScope,
}

impl VariableScope {
    fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }
}

impl VariableManager {
    pub fn new() -> Self {
        Self {
            current_scope: VariableScope::new(),
        }
    }

    pub fn set(&mut self, name: &str, value: Value) {
        let stored_value = match value {
            Value::String(ref s) if s.to_uppercase() == "TRUE" => Value::Boolean(true),
            Value::String(ref s) if s.to_uppercase() == "FALSE" => Value::Boolean(false),
            Value::String(ref s) => {
                if let Ok(n) = s.parse::<i32>() {
                    Value::Number(n)
                } else {
                    value.clone()
                }
            }
            _ => value.clone(),
        };
        println!("Setting variable: {} = {:?}", name, stored_value);
        self.current_scope
            .variables
            .insert(name.to_string(), stored_value);
    }

    pub fn get(&self, name: &str) -> Option<&Value> {
        let value = self.current_scope.variables.get(name);
        println!("Getting variable: {} = {:?}", name, value);
        value
    }

    pub fn get_all_names(&self) -> Vec<String> {
        self.current_scope.variables.keys().cloned().collect()
    }
}
