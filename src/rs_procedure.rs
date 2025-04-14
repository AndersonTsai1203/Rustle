use crate::rs_ast::{Command, Value};
use crate::rs_error::RSLogoError;
use crate::rs_variables::VariableManager;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Procedure {
    name: String,
    parameters: Vec<String>,
    body: Vec<Command>,
}

#[derive(Debug)]
pub struct ProcedureManager {
    procedures: HashMap<String, Procedure>,
    parameter_stack: Vec<HashMap<String, Value>>, // Stack of parameter bindings for each call
}

impl Procedure {
    pub fn new(name: String, parameters: Vec<String>, body: Vec<Command>) -> Self {
        Self {
            name,
            parameters,
            body,
        }
    }

    pub fn get_body(&self) -> &Vec<Command> {
        println!("get procedure \"{}\" body", self.name);
        &self.body
    }

    pub fn get_parameters(&self) -> &Vec<String> {
        println!("get procedure \"{}\" parameter", self.name);
        &self.parameters
    }
}

impl ProcedureManager {
    pub fn new() -> Self {
        Self {
            procedures: HashMap::new(),
            parameter_stack: Vec::new(),
        }
    }

    // Define a procedure, evaluating parameter names at definition time
    // Ensuring definition-time evaluation of procedure arguments
    // Definition-time binding: Capturing arguments during procedure definition
    pub fn define_procedure(
        &mut self,
        name: String,
        parameters: Vec<String>,
        body: Vec<Command>,
        variables: &mut VariableManager,
    ) -> Result<(), RSLogoError> {
        // First evaluate parameter names using current variable values
        let mut evaluated_params = Vec::new();
        for param in parameters {
            if let Some(stripped) = param.strip_prefix(':') {
                // If parameter starts with :, evaluate it now using current variables
                if let Some(var_value) = variables.get(stripped) {
                    match var_value {
                        Value::String(s) => evaluated_params.push(s.clone()),
                        _ => evaluated_params.push(param),
                    }
                } else {
                    evaluated_params.push(param);
                }
            } else {
                evaluated_params.push(param);
            }
        }

        // Store procedure with evaluated parameter names
        let procedure = Procedure::new(name.clone(), evaluated_params, body);
        self.procedures.insert(name, procedure);
        Ok(())
    }

    pub fn get_procedure(&self, name: &str) -> Option<&Procedure> {
        self.procedures.get(name)
    }

    // Push new parameter bindings for a procedure call
    pub fn push_parameters(
        &mut self,
        params: &[String],
        args: Vec<Value>,
    ) -> Result<(), RSLogoError> {
        if params.len() != args.len() {
            return Err(RSLogoError::InvalidArgument {
                command: "procedure call".to_string(),
                argument: format!("{} arguments", args.len()),
                expected: format!("{} arguments", params.len()),
            });
        }

        let mut param_bindings = HashMap::new();
        for (param, arg) in params.iter().zip(args) {
            param_bindings.insert(param.clone(), arg);
        }
        self.parameter_stack.push(param_bindings);
        Ok(())
    }

    pub fn pop_parameters(&mut self) {
        self.parameter_stack.pop();
    }

    // Get current value of a parameter
    pub fn get_parameter_value(&self, name: &str) -> Option<&Value> {
        // Search from top of stack down for most recent binding
        for bindings in self.parameter_stack.iter().rev() {
            if let Some(value) = bindings.get(name) {
                return Some(value);
            }
        }
        None
    }
}
