mod rs_ast;
mod rs_error;
mod rs_interpreter;
mod rs_operators;
mod rs_parser;
mod rs_procedure;
mod rs_stack;
mod rs_turtle;
mod rs_variables;

use clap::Parser;
use rs_error::RSLogoError;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
struct Args {
    /// Input file
    #[arg(value_name = "INPUT")]
    file_path: PathBuf,

    /// Output file
    #[arg(value_name = "OUTPUT")]
    image_path: PathBuf,

    /// Image height
    #[arg(value_name = "HEIGHT")]
    height: u32,

    /// Image width
    #[arg(value_name = "WIDTH")]
    width: u32,
}

fn main() {
    let args = Args::parse();

    if let Err(err) = run(args) {
        report_error(&err);
        std::process::exit(1);
    } else {
        println!("Program executed successfully.");
    }
}

fn run(args: Args) -> Result<(), RSLogoError> {
    println!("Reading input file...");
    let input = fs::read_to_string(&args.file_path)?;
    println!("Input file content: '{}'", input);

    println!("Parsing program...");
    let program = rs_parser::parse_program(&input)?;
    println!("Parsed program: {:?}", program);
    println!("Number of commands: {}", program.commands.len());

    println!("Creating interpreter...");
    let mut interpreter = rs_interpreter::Interpreter::new(args.width, args.height);

    println!("Executing program...");
    interpreter.execute(&program)?;

    println!("Saving image...");
    interpreter.save_image(&args.image_path)?;

    println!("Program execution completed.");
    Ok(())
}

fn report_error(err: &RSLogoError) {
    match err {
        RSLogoError::ParseError {
            input,
            span,
            message,
        } => {
            // Print the error message in red with bold
            println!("\x1b[1;31mError: {}\x1b[0m", message);

            // Print the relevant code snippet with line numbers
            if !input.is_empty() {
                println!("\nRelevant code:");
                let lines: Vec<&str> = input.lines().collect();
                let start_line = input[..span.0].matches('\n').count();

                // Print a few lines before and after the error for context
                let context_lines = 2;
                let start_idx = start_line.saturating_sub(context_lines);
                let end_idx = (start_line + context_lines + 1).min(lines.len());

                for (idx, line) in lines[start_idx..end_idx].iter().enumerate() {
                    let line_num = start_idx + idx + 1;
                    if line_num == start_line + 1 {
                        // Error line in red
                        println!("\x1b[31m{:4} | {}\x1b[0m", line_num, line);

                        // Print error pointer
                        let pointer_offset = if span.0 > line.len() {
                            line.len()
                        } else {
                            span.0 - input[..span.0].rfind('\n').map_or(0, |n| n + 1)
                        };
                        println!(
                            "     | {}\x1b[31m{}\x1b[0m",
                            " ".repeat(pointer_offset),
                            "^".repeat(span.1)
                        );

                        // Print suggestion
                        println!("     | ");
                        println!("\x1b[33mHint: 'END' commands must be paired with a 'TO' procedure definition:\x1b[0m");
                        println!("     | TO procedure_name");
                        println!("     |    commands...");
                        println!("     | END");
                    } else {
                        // Context lines in normal color
                        println!("{:4} | {}", line_num, line);
                    }
                }
            }
        }
        RSLogoError::InvalidArgument {
            command,
            argument,
            expected,
        } => {
            println!(
                "Invalid argument for command '{}': got '{}', expected {}",
                command, argument, expected
            );
        }
        RSLogoError::DrawError(message) => {
            println!("Draw error: {}", message);
        }
        RSLogoError::ImageSaveError(message) => {
            println!("Failed to save image: {}", message);
        }
        RSLogoError::IOError(err) => {
            println!("IO error: {}", err);
        }
        RSLogoError::UndefinedVariable {
            variable_name,
            defined_variables,
        } => {
            println!("Error: Undefined variable '{}'", variable_name);
            if defined_variables.is_empty() {
                println!("No variables have been defined yet.");
            } else {
                println!("Currently defined variables are:");
                for var in defined_variables {
                    println!("  - {}", var);
                }
            }
            println!("Make sure to define variables using the MAKE command before using them.");
        }
        RSLogoError::StackUnderflow => {
            println!("Error: Stack underflow - attempted to pop from an empty stack.");
        }
        RSLogoError::DivisionByZero => {
            println!("Error: Division by zero.");
        }
        RSLogoError::TypeMismatch => {
            println!("Error: Type mismatch - operation not supported for given types.");
        }
        RSLogoError::InvalidExpression(msg) => {
            println!("Error: Invalid expression - {}", msg);
        }
        RSLogoError::InvalidOperator(op) => {
            println!("Error: Invalid operator - {}", op);
        }
        RSLogoError::UnexpectedValue { expected, got } => {
            println!(
                "Error: Unexpected value - expected {}, got {}",
                expected, got
            );
        }
        RSLogoError::Overflow => {
            println!("Arithmetic overflow occurred");
        }
    }
}
