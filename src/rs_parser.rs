use crate::rs_ast::{Command, Expression, Operator, Program, Value};
use crate::rs_error::RSLogoError;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{char, digit1, line_ending, multispace0, multispace1, not_line_ending},
    combinator::{all_consuming, map, map_res, opt, recognize, value},
    error::Error,
    multi::many0,
    sequence::{delimited, preceded, terminated, tuple},
    Finish, IResult,
};

fn parse_value(input: &str) -> IResult<&str, Value> {
    alt((
        map(
            preceded(
                char('"'),
                take_while1(|c: char| c.is_alphanumeric() || c == '_' || c == '-'),
            ),
            |s: &str| Value::String(s.to_string()),
        ),
        map_res(recognize(tuple((opt(char('-')), digit1))), |s: &str| {
            s.parse::<i32>().map(Value::Number)
        }),
        map(
            preceded(
                char(':'),
                take_while1(|c: char| c.is_alphanumeric() || c == '_'),
            ),
            |s: &str| Value::Variable(s.to_string()),
        ),
        map(alt((tag("TRUE"), tag("FALSE"))), |s: &str| {
            Value::Boolean(s == "TRUE")
        }),
    ))(input)
}

fn parse_operator(input: &str) -> IResult<&str, Operator> {
    alt((
        value(Operator::Add, tag("+")),
        value(Operator::Subtract, tag("-")),
        value(Operator::Multiply, tag("*")),
        value(Operator::Divide, tag("/")),
        value(Operator::Equal, tag("EQ")),
        value(Operator::NotEqual, tag("NE")),
        value(Operator::GreaterThan, tag("GT")),
        value(Operator::LessThan, tag("LT")),
        value(Operator::And, tag("AND")),
        value(Operator::Or, tag("OR")),
    ))(input)
}

fn parse_expression(input: &str) -> IResult<&str, Expression> {
    alt((
        map(parse_value, Expression::Value),
        map(
            tuple((
                parse_operator,
                multispace1,
                parse_expression,
                multispace1,
                parse_expression,
            )),
            |(op, _, left, _, right)| Expression::BinaryOp(op, Box::new(left), Box::new(right)),
        ),
        map(
            alt((tag("XCOR"), tag("YCOR"), tag("HEADING"), tag("COLOR"))),
            |s: &str| Expression::Query(s.to_string()),
        ),
    ))(input)
}

fn parse_parameter(input: &str) -> IResult<&str, (String, bool)> {
    alt((
        // Variable parameter (starts with :)
        map(
            preceded(
                char(':'),
                take_while1(|c: char| c.is_alphanumeric() || c == '_'),
            ),
            |s: &str| (s.to_string(), true), // true indicates variable parameter
        ),
        // Literal parameter (starts with ")
        map(
            preceded(
                char('"'),
                take_while1(|c: char| c.is_alphanumeric() || c == '_'),
            ),
            |s: &str| (s.to_string(), false), // false indicates literal parameter
        ),
    ))(input)
}

fn parse_procedure_definition(input: &str) -> IResult<&str, Result<Command, RSLogoError>> {
    // Parse "TO" and procedure name
    let (remaining, _) = tag("TO")(input)?;
    let (remaining, _) = multispace1(remaining)?;
    let (remaining, name) = take_while1(|c: char| c.is_alphanumeric() || c == '_')(remaining)?;

    // Parse parameters - now handling both variable and literal parameters
    let mut parameters = Vec::new();
    let mut current = remaining;

    loop {
        // Skip whitespace
        let (next, _) = multispace0(current)?;

        // Try to parse a parameter
        match parse_parameter(next) {
            Ok((remaining, param)) => {
                parameters.push(param.0);
                current = remaining;
            }
            Err(_) => break,
        }
    }

    // Skip whitespace and comments after parameters
    let (current, _) = many0(alt((
        value((), multispace1),
        value((), line_ending::<&str, Error<&str>>),
        value(
            (),
            tuple((
                tag("//"),
                not_line_ending,
                opt(line_ending::<&str, Error<&str>>),
            )),
        ),
    )))(current)?;

    // Keep track of the start position for error reporting
    let start_pos = input.len() - current.len();
    let mut command_count = 0;

    // Parse commands until END
    let mut commands = Vec::new();
    let mut found_end = false;
    let mut current_pos = current;

    loop {
        // Skip whitespace, newlines and comments
        let (next, _) = many0(alt((
            value((), multispace1),
            value((), line_ending::<&str, Error<&str>>),
            value(
                (),
                tuple((
                    tag("//"),
                    not_line_ending,
                    opt(line_ending::<&str, Error<&str>>),
                )),
            ),
        )))(current_pos)?;

        // Check for END
        if let Ok((remaining, _)) = tag::<&str, &str, Error<&str>>("END")(next) {
            current_pos = remaining;
            found_end = true;
            break;
        }

        // Parse a command
        match parse_regular_command(next) {
            Ok((remaining, cmd)) => {
                if let Ok(cmd) = cmd {
                    commands.push(cmd);
                    command_count += 1;
                    current_pos = remaining;
                } else {
                    break;
                }
            }
            Err(nom::Err::Error(_)) => break,
            Err(e) => return Err(e),
        }

        // Check if we've reached the end of input
        if current_pos.trim().is_empty() {
            break;
        }
    }

    if !found_end {
        return Ok((
            current_pos,
            Err(RSLogoError::ParseError {
                input: input[start_pos..].to_string(),
                span: (start_pos, input.len() - start_pos),
                message: format!(
                    "Unterminated procedure definition '{}': Expected 'END' keyword after {} commands",
                    name, command_count
                ),
            }),
        ));
    }

    // Create the procedure definition command
    Ok((
        current_pos,
        Ok(Command::ProcedureDefinition {
            name: name.to_string(),
            parameters,
            body: commands,
        }),
    ))
}

fn parse_procedure_call(input: &str) -> IResult<&str, Result<Command, RSLogoError>> {
    let (remaining, name) = take_while1(|c: char| c.is_alphanumeric() || c == '_')(input)?;

    // Don't parse TO or END as procedure calls
    if name == "TO" || name == "END" {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    }

    let (remaining, arguments) = many0(preceded(multispace1, parse_expression))(remaining)?;

    Ok((
        remaining,
        Ok(Command::ProcedureCall {
            name: name.to_string(),
            arguments,
        }),
    ))
}

fn parse_make_command(input: &str) -> IResult<&str, Result<Command, RSLogoError>> {
    let (input, _) = tag("MAKE")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, name_expr) = parse_expression(input)?;
    let (input, _) = multispace1(input)?;
    let (input, value_expr) = parse_expression(input)?;

    Ok((input, Ok(Command::Make(name_expr, value_expr))))
}

fn parse_command_block(input: &str) -> IResult<&str, Result<Vec<Command>, RSLogoError>> {
    let (remaining, commands) = delimited(
        char('['),
        many0(delimited(multispace0, parse_command, multispace0)),
        char(']'),
    )(input)?;

    let result: Result<Vec<Command>, RSLogoError> = commands.into_iter().collect();
    Ok((remaining, result))
}

fn parse_if_command(input: &str) -> IResult<&str, Result<Command, RSLogoError>> {
    let (remaining, (_, _, condition, _, body)) = tuple((
        tag("IF"),
        multispace1,
        parse_expression,
        multispace0,
        parse_command_block,
    ))(input)?;

    let result = body.map(|b| Command::If(condition, b));
    Ok((remaining, result))
}

fn parse_while_command(input: &str) -> IResult<&str, Result<Command, RSLogoError>> {
    let (remaining, (_, _, condition, _, body)) = tuple((
        tag("WHILE"),
        multispace1,
        parse_expression,
        multispace0,
        parse_command_block,
    ))(input)?;

    let result = body.map(|b| Command::While(condition, b));
    Ok((remaining, result))
}

fn parse_regular_command(input: &str) -> IResult<&str, Result<Command, RSLogoError>> {
    alt((
        map(
            tuple((tag("PENUP"), opt(preceded(multispace1, parse_expression)))),
            |(cmd, expr)| match expr {
                Some(_) => Err(RSLogoError::InvalidArgument {
                    command: cmd.to_string(),
                    argument: "".to_string(),
                    expected: "no arguments".to_string(),
                }),
                None => Ok(Command::PenUp),
            },
        ),
        map(
            tuple((tag("PENDOWN"), opt(preceded(multispace1, parse_expression)))),
            |(cmd, expr)| match expr {
                Some(_) => Err(RSLogoError::InvalidArgument {
                    command: cmd.to_string(),
                    argument: "".to_string(),
                    expected: "no arguments".to_string(),
                }),
                None => Ok(Command::PenDown),
            },
        ),
        map(
            tuple((
                tag("FORWARD"),
                multispace1,
                parse_expression,
                opt(preceded(multispace1, parse_expression)),
            )),
            |(cmd, _, expr, extra)| match extra {
                Some(_) => Err(RSLogoError::InvalidArgument {
                    command: cmd.to_string(),
                    argument: "".to_string(),
                    expected: "only one argument".to_string(),
                }),
                None => Ok(Command::Forward(expr)),
            },
        ),
        map(
            tuple((
                tag("BACK"),
                multispace1,
                parse_expression,
                opt(preceded(multispace1, parse_expression)),
            )),
            |(cmd, _, expr, extra)| match extra {
                Some(_) => Err(RSLogoError::InvalidArgument {
                    command: cmd.to_string(),
                    argument: "".to_string(),
                    expected: "only one argument".to_string(),
                }),
                None => Ok(Command::Back(expr)),
            },
        ),
        map(
            tuple((
                tag("LEFT"),
                multispace1,
                parse_expression,
                opt(preceded(multispace1, parse_expression)),
            )),
            |(cmd, _, expr, extra)| match extra {
                Some(_) => Err(RSLogoError::InvalidArgument {
                    command: cmd.to_string(),
                    argument: "".to_string(),
                    expected: "only one argument".to_string(),
                }),
                None => Ok(Command::Left(expr)),
            },
        ),
        map(
            tuple((
                tag("RIGHT"),
                multispace1,
                parse_expression,
                opt(preceded(multispace1, parse_expression)),
            )),
            |(cmd, _, expr, extra)| match extra {
                Some(_) => Err(RSLogoError::InvalidArgument {
                    command: cmd.to_string(),
                    argument: "".to_string(),
                    expected: "only one argument".to_string(),
                }),
                None => Ok(Command::Right(expr)),
            },
        ),
        map(
            tuple((
                tag("SETPENCOLOR"),
                multispace1,
                parse_expression,
                opt(preceded(multispace1, parse_expression)),
            )),
            |(cmd, _, expr, extra)| match extra {
                Some(_) => Err(RSLogoError::InvalidArgument {
                    command: cmd.to_string(),
                    argument: "".to_string(),
                    expected: "only one argument".to_string(),
                }),
                None => Ok(Command::SetPenColor(expr)),
            },
        ),
        map(
            tuple((
                tag("TURN"),
                multispace1,
                parse_expression,
                opt(preceded(multispace1, parse_expression)),
            )),
            |(cmd, _, expr, extra)| match extra {
                Some(_) => Err(RSLogoError::InvalidArgument {
                    command: cmd.to_string(),
                    argument: "".to_string(),
                    expected: "only one argument".to_string(),
                }),
                None => Ok(Command::Turn(expr)),
            },
        ),
        map(
            tuple((
                tag("SETHEADING"),
                multispace1,
                parse_expression,
                opt(preceded(multispace1, parse_expression)),
            )),
            |(cmd, _, expr, extra)| match extra {
                Some(_) => Err(RSLogoError::InvalidArgument {
                    command: cmd.to_string(),
                    argument: "".to_string(),
                    expected: "only one argument".to_string(),
                }),
                None => Ok(Command::SetHeading(expr)),
            },
        ),
        map(
            tuple((
                tag("SETX"),
                multispace1,
                parse_expression,
                opt(preceded(multispace1, parse_expression)),
            )),
            |(cmd, _, expr, extra)| match extra {
                Some(_) => Err(RSLogoError::InvalidArgument {
                    command: cmd.to_string(),
                    argument: "".to_string(),
                    expected: "only one argument".to_string(),
                }),
                None => Ok(Command::SetX(expr)),
            },
        ),
        map(
            tuple((
                tag("SETY"),
                multispace1,
                parse_expression,
                opt(preceded(multispace1, parse_expression)),
            )),
            |(cmd, _, expr, extra)| match extra {
                Some(_) => Err(RSLogoError::InvalidArgument {
                    command: cmd.to_string(),
                    argument: "".to_string(),
                    expected: "only one argument".to_string(),
                }),
                None => Ok(Command::SetY(expr)),
            },
        ),
        parse_make_command,
        map(
            tuple((
                tag("ADDASSIGN"),
                multispace1,
                alt((
                    preceded(
                        char('"'),
                        take_while1(|c: char| c.is_alphanumeric() || c == '_'),
                    ),
                    preceded(
                        char(':'),
                        take_while1(|c: char| c.is_alphanumeric() || c == '_'),
                    ),
                )),
                multispace1,
                parse_expression,
                opt(preceded(multispace1, parse_expression)),
            )),
            |(cmd, _, var_name, _, expr, extra)| match extra {
                Some(_) => Err(RSLogoError::InvalidArgument {
                    command: cmd.to_string(),
                    argument: "".to_string(),
                    expected: "only two arguments".to_string(),
                }),
                None => Ok(Command::AddAssign(var_name.to_string(), expr)),
            },
        ),
        parse_if_command,
        parse_while_command,
        map(parse_expression, |expr| {
            Ok(Command::Expression(Box::new(expr)))
        }),
        parse_procedure_call,
    ))(input)
}

fn parse_command(input: &str) -> IResult<&str, Result<Command, RSLogoError>> {
    // First check if we have an END without a TO
    if let Ok((remaining, _)) = tag::<&str, &str, nom::error::Error<&str>>("END")(input) {
        // Find the line number
        let line_num = input[..input.len() - remaining.len()]
            .chars()
            .filter(|&c| c == '\n')
            .count()
            + 1;

        return Ok((remaining, Err(RSLogoError::ParseError {
            input: input.to_string(),
            span: (input.len() - remaining.len(), 3), // "END" is 3 characters
            message: format!(
                "Found 'END' command on line {} without matching 'TO' procedure definition. Each 'END' must be paired with a 'TO' procedure definition.",
                line_num
            ),
        })));
    }

    alt((parse_procedure_definition, parse_regular_command))(input)
}

fn parse_comment(input: &str) -> IResult<&str, ()> {
    value((), tuple((tag("//"), not_line_ending, opt(line_ending))))(input)
}

pub fn parse_program(input: &str) -> Result<Program, RSLogoError> {
    println!("Parsing input: '{}'", input);

    if input.trim().is_empty() {
        println!("Input is empty, returning empty program");
        return Ok(Program {
            commands: Vec::new(),
        });
    }

    let parse_result: IResult<&str, Vec<Option<Result<Command, RSLogoError>>>> =
        all_consuming(many0(terminated(
            alt((
                map(parse_comment, |_| None),
                map(delimited(multispace0, parse_command, multispace0), Some),
                map(line_ending, |_| None),
            )),
            many0(line_ending),
        )))(input);

    match parse_result.finish() {
        Ok((_, commands)) => {
            let filtered_commands: Result<Vec<Command>, RSLogoError> =
                commands.into_iter().flatten().collect();

            match filtered_commands {
                Ok(cmds) => {
                    println!("Successfully parsed {} commands", cmds.len());
                    println!("Commands: {:?}", cmds);
                    Ok(Program { commands: cmds })
                }
                Err(e) => {
                    println!("Error collecting commands: {:?}", e);
                    Err(e)
                }
            }
        }
        Err(e) => {
            println!("Parse error: {:?}", e);
            Err(RSLogoError::ParseError {
                input: input.to_string(),
                span: (
                    e.input.as_ptr() as usize - input.as_ptr() as usize,
                    e.input.len(),
                ),
                message: format!("Parse error: {}", e.code.description()),
            })
        }
    }
}
