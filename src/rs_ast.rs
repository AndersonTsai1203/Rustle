use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Number(i32),
    String(String),
    Variable(String),
    Boolean(bool),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Command {
    PenUp,
    PenDown,
    Forward(Expression),
    Back(Expression),
    Left(Expression),
    Right(Expression),
    SetPenColor(Expression),
    Turn(Expression),
    SetHeading(Expression),
    SetX(Expression),
    SetY(Expression),
    Make(Expression, Expression),
    AddAssign(String, Expression),
    If(Expression, Vec<Command>),
    While(Expression, Vec<Command>),
    Expression(Box<Expression>),
    ProcedureDefinition {
        name: String,
        parameters: Vec<String>,
        body: Vec<Command>,
    },
    ProcedureCall {
        name: String,
        arguments: Vec<Expression>,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    And,
    Or,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Value(Value),
    BinaryOp(Operator, Box<Expression>, Box<Expression>),
    Query(String),
}

#[derive(Debug, PartialEq)]
pub struct Program {
    pub commands: Vec<Command>,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Variable(v) => write!(f, ":{}", v),
            Value::Boolean(b) => write!(f, "{}", if *b { "TRUE" } else { "FALSE" }),
        }
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Command::PenUp => write!(f, "PENUP"),
            Command::PenDown => write!(f, "PENDOWN"),
            Command::Forward(v) => write!(f, "FORWARD {}", v),
            Command::Back(v) => write!(f, "BACK {}", v),
            Command::Left(v) => write!(f, "LEFT {}", v),
            Command::Right(v) => write!(f, "RIGHT {}", v),
            Command::SetPenColor(v) => write!(f, "SETPENCOLOR {}", v),
            Command::Turn(v) => write!(f, "TURN {}", v),
            Command::SetHeading(v) => write!(f, "SETHEADING {}", v),
            Command::SetX(v) => write!(f, "SETX {}", v),
            Command::SetY(v) => write!(f, "SETY {}", v),
            Command::Make(expr1, expr2) => write!(f, "MAKE {} {}", expr1, expr2),
            Command::AddAssign(name, v) => write!(f, "ADDASSIGN {} {}", name, v),
            Command::If(condition, body) => {
                write!(f, "IF {} [", condition)?;
                for (i, cmd) in body.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?
                    }
                    write!(f, "{}", cmd)?;
                }
                write!(f, "]")
            }
            Command::While(condition, body) => {
                write!(f, "WHILE {} [", condition)?;
                for (i, cmd) in body.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?
                    }
                    write!(f, "{}", cmd)?;
                }
                write!(f, "]")
            }
            Command::Expression(expr) => write!(f, "{}", expr),
            Command::ProcedureDefinition {
                name,
                parameters,
                body,
            } => {
                write!(f, "TO {} ", name)?;
                for param in parameters {
                    write!(f, "{} ", param)?;
                }
                write!(f, "[")?;
                for cmd in body {
                    write!(f, " {}", cmd)?;
                }
                write!(f, "] END")
            }
            Command::ProcedureCall { name, arguments } => {
                write!(f, "{}", name)?;
                for arg in arguments {
                    write!(f, " {}", arg)?;
                }
                Ok(())
            }
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Value(v) => write!(f, "{}", v),
            Expression::BinaryOp(left, op, right) => write!(f, "({} {} {})", left, op, right),
            Expression::Query(q) => write!(f, "{}", q),
        }
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operator::Add => write!(f, "+"),
            Operator::Subtract => write!(f, "-"),
            Operator::Multiply => write!(f, "*"),
            Operator::Divide => write!(f, "/"),
            Operator::Equal => write!(f, "EQ"),
            Operator::NotEqual => write!(f, "NE"),
            Operator::GreaterThan => write!(f, "GT"),
            Operator::LessThan => write!(f, "LT"),
            Operator::And => write!(f, "AND"),
            Operator::Or => write!(f, "OR"),
        }
    }
}
