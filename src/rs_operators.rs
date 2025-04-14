use crate::rs_ast::{Operator, Value};
use crate::rs_error::RSLogoError;
use crate::rs_stack::Stack;

impl Operator {
    pub fn apply(&self, stack: &mut Stack) -> Result<Value, RSLogoError> {
        let right = stack.pop()?;
        let left = stack.pop()?;
        let result = match self {
            Operator::Add => add(&left, &right)?,
            Operator::Subtract => subtract(&left, &right)?,
            Operator::Multiply => multiply(&left, &right)?,
            Operator::Divide => divide(&left, &right)?,
            Operator::Equal => equal(&left, &right)?,
            Operator::NotEqual => not_equal(&left, &right)?,
            Operator::GreaterThan => greater_than(&left, &right)?,
            Operator::LessThan => less_than(&left, &right)?,
            Operator::And => and(&left, &right)?,
            Operator::Or => or(&left, &right)?,
        };
        Ok(result)
    }
}

fn value_to_number(value: &Value) -> Result<i32, RSLogoError> {
    match value {
        Value::Number(n) => Ok(*n),
        Value::String(s) => s.parse::<i32>().map_err(|_| RSLogoError::TypeMismatch),
        Value::Boolean(b) => Ok(if *b { 1 } else { 0 }),
        Value::Variable(_) => Err(RSLogoError::TypeMismatch), // Variables should be resolved before reaching here
    }
}

fn value_to_bool(value: &Value) -> Result<bool, RSLogoError> {
    match value {
        Value::Boolean(b) => Ok(*b),
        Value::Number(n) => Ok(*n != 0),
        Value::String(s) => Ok(s.to_uppercase() == "TRUE"),
        Value::Variable(_) => Err(RSLogoError::TypeMismatch), // Variables should be resolved before reaching here
    }
}

fn add(left: &Value, right: &Value) -> Result<Value, RSLogoError> {
    let left_num = value_to_number(left)?;
    let right_num = value_to_number(right)?;
    left_num
        .checked_add(right_num)
        .map(Value::Number)
        .ok_or(RSLogoError::Overflow)
}

fn subtract(left: &Value, right: &Value) -> Result<Value, RSLogoError> {
    let left_num = value_to_number(left)?;
    let right_num = value_to_number(right)?;
    left_num
        .checked_sub(right_num)
        .map(Value::Number)
        .ok_or(RSLogoError::Overflow)
}

fn multiply(left: &Value, right: &Value) -> Result<Value, RSLogoError> {
    let left_num = value_to_number(left)?;
    let right_num = value_to_number(right)?;
    left_num
        .checked_mul(right_num)
        .map(Value::Number)
        .ok_or(RSLogoError::Overflow)
}

fn divide(left: &Value, right: &Value) -> Result<Value, RSLogoError> {
    let left_num = value_to_number(left)?;
    let right_num = value_to_number(right)?;
    if right_num == 0 {
        Err(RSLogoError::DivisionByZero)
    } else {
        Ok(Value::Number(left_num / right_num))
    }
}

fn equal(left: &Value, right: &Value) -> Result<Value, RSLogoError> {
    match (left, right) {
        (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l == r)),
        (Value::String(l), Value::String(r)) => {
            Ok(Value::Boolean(l.to_uppercase() == r.to_uppercase()))
        }
        (Value::Boolean(l), Value::Boolean(r)) => Ok(Value::Boolean(l == r)),
        (Value::Number(l), Value::String(r)) | (Value::String(r), Value::Number(l)) => {
            if let Ok(r_num) = r.parse::<i32>() {
                Ok(Value::Boolean(l == &r_num))
            } else {
                Err(RSLogoError::TypeMismatch)
            }
        }
        _ => Err(RSLogoError::TypeMismatch),
    }
}

fn not_equal(left: &Value, right: &Value) -> Result<Value, RSLogoError> {
    let left_num = value_to_number(left)?;
    let right_num = value_to_number(right)?;
    Ok(Value::Boolean(left_num != right_num))
}

fn greater_than(left: &Value, right: &Value) -> Result<Value, RSLogoError> {
    let left_num = value_to_number(left)?;
    let right_num = value_to_number(right)?;
    Ok(Value::Boolean(left_num > right_num))
}

fn less_than(left: &Value, right: &Value) -> Result<Value, RSLogoError> {
    let left_num = value_to_number(left)?;
    let right_num = value_to_number(right)?;
    Ok(Value::Boolean(left_num < right_num))
}

fn and(left: &Value, right: &Value) -> Result<Value, RSLogoError> {
    let left_bool = value_to_bool(left)?;
    let right_bool = value_to_bool(right)?;
    Ok(Value::Boolean(left_bool && right_bool))
}

fn or(left: &Value, right: &Value) -> Result<Value, RSLogoError> {
    let left_bool = value_to_bool(left)?;
    let right_bool = value_to_bool(right)?;
    Ok(Value::Boolean(left_bool || right_bool))
}
