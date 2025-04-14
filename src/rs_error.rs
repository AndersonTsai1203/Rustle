use std::fmt;

#[derive(Debug)]
pub enum RSLogoError {
    #[allow(dead_code)]
    ParseError {
        input: String,
        span: (usize, usize),
        message: String,
    },
    IOError(std::io::Error),
    InvalidArgument {
        command: String,
        argument: String,
        expected: String,
    },
    DrawError(String),
    ImageSaveError(String),
    UndefinedVariable {
        variable_name: String,
        defined_variables: Vec<String>,
    },
    StackUnderflow,
    DivisionByZero,
    TypeMismatch,
    #[allow(dead_code)]
    InvalidExpression(String),
    #[allow(dead_code)]
    InvalidOperator(String),
    UnexpectedValue {
        expected: String,
        got: String,
    },
    Overflow,
}

impl fmt::Display for RSLogoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RSLogoError::ParseError {
                input,
                span: _,
                message,
            } => write!(f, "Parse error: '{}', {}", message, input),
            RSLogoError::IOError(err) => write!(f, "IO error: {}", err),
            RSLogoError::InvalidArgument {
                command,
                argument,
                expected,
            } => write!(
                f,
                "Invalid argument for command '{}': got '{}', expected {}",
                command, argument, expected
            ),
            RSLogoError::DrawError(message) => write!(f, "Draw error: {}", message),
            RSLogoError::ImageSaveError(message) => write!(f, "Image save error: {}", message),
            RSLogoError::UndefinedVariable {
                variable_name,
                defined_variables,
            } => write!(
                f,
                "Undefined variable '{}'. Defined variables are: {:?}",
                variable_name, defined_variables
            ),
            RSLogoError::StackUnderflow => {
                write!(f, "Stack underflow: Attempted to pop from an empty stack")
            }
            RSLogoError::DivisionByZero => write!(f, "Division by zero"),
            RSLogoError::TypeMismatch => {
                write!(f, "Type mismatch: Operation not supported for given types")
            }
            RSLogoError::InvalidExpression(msg) => write!(f, "Invalid expression: {}", msg),
            RSLogoError::InvalidOperator(op) => write!(f, "Invalid operator: {}", op),
            RSLogoError::UnexpectedValue { expected, got } => write!(
                f,
                "Unexpected value: expected {}, but got {}",
                expected, got
            ),
            RSLogoError::Overflow => write!(f, "Arithmetic overflow occurred"),
        }
    }
}

impl std::error::Error for RSLogoError {}

impl From<std::io::Error> for RSLogoError {
    fn from(err: std::io::Error) -> Self {
        RSLogoError::IOError(err)
    }
}
