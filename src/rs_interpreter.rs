use crate::rs_ast::{Command, Expression, Program, Value};
use crate::rs_error::RSLogoError;
use crate::rs_procedure::ProcedureManager;
use crate::rs_stack::Stack;
use crate::rs_turtle::Turtle;
use crate::rs_variables::VariableManager;
use std::path::Path;

pub struct Interpreter {
    turtle: Turtle,
    variables: VariableManager,
    stack: Stack,
    procedures: ProcedureManager,
}

impl Interpreter {
    pub fn new(width: u32, height: u32) -> Self {
        println!(
            "Creating new interpreter with dimensions {}x{}",
            width, height
        );
        Self {
            turtle: Turtle::new(width, height),
            variables: VariableManager::new(),
            stack: Stack::new(),
            procedures: ProcedureManager::new(),
        }
    }

    pub fn execute(&mut self, program: &Program) -> Result<(), RSLogoError> {
        println!("Executing program with {} commands", program.commands.len());
        for (i, command) in program.commands.iter().enumerate() {
            println!("Executing command {}: {:?}", i + 1, command);
            self.execute_command(command)?;
        }
        println!("Program execution completed");
        Ok(())
    }

    pub fn save_image(&self, image_path: &Path) -> Result<(), RSLogoError> {
        println!("Saving image to {:?}", image_path);
        self.turtle.save_image(image_path)
    }

    fn execute_command(&mut self, command: &Command) -> Result<(), RSLogoError> {
        match command {
            Command::PenUp => {
                self.turtle.pen_up();
                Ok(())
            }
            Command::PenDown => {
                self.turtle.pen_down();
                Ok(())
            }
            Command::Forward(expr)
            | Command::Back(expr)
            | Command::Left(expr)
            | Command::Right(expr) => {
                let value = self.evaluate_expression(expr)?;
                let amount = self.value_to_int(&value)?;
                match command {
                    Command::Forward(_) => self.turtle.forward(amount),
                    Command::Back(_) => self.turtle.back(amount),
                    Command::Left(_) => self.turtle.left(amount),
                    Command::Right(_) => self.turtle.right(amount),
                    _ => unreachable!(),
                }
            }
            Command::SetPenColor(expr) => {
                let value = self.evaluate_expression(expr)?;
                let color = self.value_to_int(&value)?;
                if !(0..=15).contains(&color) {
                    return Err(RSLogoError::InvalidArgument {
                        command: "SETPENCOLOR".to_string(),
                        argument: color.to_string(),
                        expected: "an integer between 0 and 15".to_string(),
                    });
                }
                self.turtle.set_pen_color(color as u32)
            }
            Command::Turn(expr) | Command::SetHeading(expr) => {
                let value = self.evaluate_expression(expr)?;
                let degrees = self.value_to_int(&value)?;
                match command {
                    Command::Turn(_) => {
                        self.turtle.turn(degrees);
                        Ok(())
                    }
                    Command::SetHeading(_) => {
                        self.turtle.set_heading(degrees);
                        Ok(())
                    }
                    _ => unreachable!(),
                }
            }
            Command::SetX(expr) | Command::SetY(expr) => {
                let value = self.evaluate_expression(expr)?;
                let location = self.value_to_int(&value)?;
                match command {
                    Command::SetX(_) => {
                        self.turtle.set_x(location);
                        Ok(())
                    }
                    Command::SetY(_) => {
                        self.turtle.set_y(location);
                        Ok(())
                    }
                    _ => unreachable!(),
                }
            }
            Command::Make(name_expr, value_expr) => {
                let name = self.evaluate_expression(name_expr)?;
                let value = self.evaluate_expression(value_expr)?;

                let name_str = self.value_to_string(&name)?;

                // Convert to number if possible
                let stored_value = match value {
                    Value::String(s) => {
                        if let Ok(n) = s.parse::<i32>() {
                            Value::Number(n)
                        } else {
                            Value::String(s)
                        }
                    }
                    _ => value,
                };

                self.variables.set(&name_str, stored_value);
                Ok(())
            }
            Command::AddAssign(name, expr) => {
                let value = self.evaluate_expression(expr)?;
                let amount = self.value_to_int(&value)?;

                // Resolve the variable name stored in 'name'
                let var_name = if let Some(stripped) = name.strip_prefix(':') {
                    self.resolve_variable_value(stripped)?
                } else {
                    // If it doesn't start with ':', it could be a literal name (2_05) or a variable containing a name (2_07)
                    match self.variables.get(name) {
                        Some(Value::String(s)) => s.clone(), // For 2_07: variable containing a name
                        _ => name.to_string(),               // For 2_05: literal name
                    }
                };

                // Now get the current value of the resolved variable
                let current_value = self.variables.get(&var_name).ok_or_else(|| {
                    RSLogoError::UndefinedVariable {
                        variable_name: var_name.clone(),
                        defined_variables: self.variables.get_all_names(),
                    }
                })?;

                let current_amount = self.value_to_int(current_value)?;
                let new_value = current_amount + amount;
                self.variables.set(&var_name, Value::Number(new_value));
                Ok(())
            }
            Command::If(condition, body) => {
                let condition_value = self.evaluate_expression(condition)?;
                if self.value_to_bool(&condition_value)? {
                    for cmd in body {
                        self.execute_command(cmd)?;
                    }
                }
                Ok(())
            }
            Command::While(condition, body) => {
                loop {
                    let condition_value = self.evaluate_expression(condition)?;
                    if !self.value_to_bool(&condition_value)? {
                        break;
                    }
                    for cmd in body {
                        self.execute_command(cmd)?;
                    }
                }
                Ok(())
            }
            Command::Expression(expr) => {
                self.evaluate_expression(expr)?;
                Ok(())
            }
            Command::ProcedureDefinition {
                name,
                parameters,
                body,
            } => {
                // Just store the procedure definition without evaluating variables
                self.procedures.define_procedure(
                    name.clone(),
                    parameters.clone(),
                    body.clone(),
                    &mut self.variables,
                )
            }
            Command::ProcedureCall { name, arguments } => {
                // Get procedure first
                let (parameters, body) = {
                    let procedure = self.procedures.get_procedure(name).ok_or_else(|| {
                        RSLogoError::InvalidArgument {
                            command: "procedure call".to_string(),
                            argument: name.clone(),
                            expected: "a defined procedure name".to_string(),
                        }
                    })?;
                    (
                        procedure.get_parameters().clone(),
                        procedure.get_body().clone(),
                    )
                };

                // Evaluate arguments
                let mut evaluated_args = Vec::new();
                for arg in arguments {
                    let value = self.evaluate_expression(arg)?;
                    evaluated_args.push(value);
                }

                // Create parameter scope
                self.procedures
                    .push_parameters(&parameters, evaluated_args)?;

                // Execute body - variable resolution happens here
                for cmd in &body {
                    self.execute_command(cmd)?;
                }

                self.procedures.pop_parameters();
                Ok(())
            }
        }
    }

    fn evaluate_expression(&mut self, expr: &Expression) -> Result<Value, RSLogoError> {
        match expr {
            Expression::Value(v) => {
                let resolved = self.resolve_value(v)?;
                self.stack.push(resolved.clone());
                Ok(resolved)
            }
            Expression::BinaryOp(op, left, right) => {
                let left_value = self.evaluate_expression(left)?;
                let right_value = self.evaluate_expression(right)?;
                self.stack.push(left_value);
                self.stack.push(right_value);
                let result = op.apply(&mut self.stack)?;
                Ok(result)
            }
            Expression::Query(query) => {
                let result = self.resolve_query(query)?;
                self.stack.push(result.clone());
                Ok(result)
            }
        }
    }

    fn value_to_int(&self, value: &Value) -> Result<i32, RSLogoError> {
        match value {
            Value::Number(n) => Ok(*n),
            Value::String(s) => s.parse::<i32>().map_err(|_| RSLogoError::UnexpectedValue {
                expected: "a number".to_string(),
                got: s.clone(),
            }),
            Value::Variable(var_name) => {
                let var_value =
                    self.variables
                        .get(var_name)
                        .ok_or_else(|| RSLogoError::UndefinedVariable {
                            variable_name: var_name.clone(),
                            defined_variables: self.variables.get_all_names(),
                        })?;
                self.value_to_int(var_value)
            }
            Value::Boolean(b) => Ok(if *b { 1 } else { 0 }),
        }
    }

    fn value_to_bool(&self, value: &Value) -> Result<bool, RSLogoError> {
        match value {
            Value::Boolean(b) => Ok(*b),
            Value::String(s) => Ok(s.to_uppercase() == "TRUE"),
            Value::Number(n) => Ok(*n != 0),
            _ => Err(RSLogoError::TypeMismatch),
        }
    }

    fn value_to_string(&self, value: &Value) -> Result<String, RSLogoError> {
        match value {
            Value::String(s) => Ok(s.clone()),
            Value::Number(n) => Ok(n.to_string()),
            Value::Variable(var) => self.resolve_variable_value(var),
            Value::Boolean(b) => Ok(b.to_string()),
        }
    }

    fn resolve_variable_value(&self, name: &str) -> Result<String, RSLogoError> {
        match self.variables.get(name) {
            Some(Value::String(s)) => Ok(s.clone()),
            Some(Value::Number(n)) => Ok(n.to_string()),
            Some(_) => Err(RSLogoError::UnexpectedValue {
                expected: "a string or number".to_string(),
                got: format!("{:?}", self.variables.get(name)),
            }),
            None => {
                // Check procedure parameters
                if let Some(param_value) = self.procedures.get_parameter_value(name) {
                    match param_value {
                        Value::String(s) => return Ok(s.clone()),
                        Value::Number(n) => return Ok(n.to_string()),
                        _ => {}
                    }
                }
                Err(RSLogoError::UndefinedVariable {
                    variable_name: name.to_string(),
                    defined_variables: self.variables.get_all_names(),
                })
            }
        }
    }

    fn resolve_value(&self, value: &Value) -> Result<Value, RSLogoError> {
        match value {
            Value::Variable(name) => {
                // First check procedure parameters (current scope)
                if let Some(param_value) = self.procedures.get_parameter_value(name) {
                    return Ok(param_value.clone());
                }

                // Then check regular variables
                if let Some(var_value) = self.variables.get(name) {
                    // Handle possible variable reference values
                    match var_value {
                        Value::String(s) if s.starts_with(':') => {
                            let referenced_name = &s[1..];
                            if let Some(final_value) = self.variables.get(referenced_name) {
                                Ok(final_value.clone())
                            } else {
                                Err(RSLogoError::UndefinedVariable {
                                    variable_name: referenced_name.to_string(),
                                    defined_variables: self.variables.get_all_names(),
                                })
                            }
                        }
                        _ => Ok(var_value.clone()),
                    }
                } else {
                    Err(RSLogoError::UndefinedVariable {
                        variable_name: name.clone(),
                        defined_variables: self.variables.get_all_names(),
                    })
                }
            }
            Value::String(s) if s.to_uppercase() == "TRUE" => Ok(Value::Boolean(true)),
            Value::String(s) if s.to_uppercase() == "FALSE" => Ok(Value::Boolean(false)),
            Value::Number(_) | Value::String(_) | Value::Boolean(_) => Ok(value.clone()),
        }
    }

    fn resolve_query(&self, query: &str) -> Result<Value, RSLogoError> {
        match query {
            "XCOR" => Ok(Value::Number(self.turtle.get_x())),
            "YCOR" => Ok(Value::Number(self.turtle.get_y())),
            "HEADING" => Ok(Value::Number(self.turtle.get_heading())),
            "COLOR" => Ok(Value::Number(self.turtle.get_pen_color() as i32)),
            _ => Err(RSLogoError::InvalidArgument {
                command: "query".to_string(),
                argument: query.to_string(),
                expected: "XCOR, YCOR, HEADING, or COLOR".to_string(),
            }),
        }
    }
}
