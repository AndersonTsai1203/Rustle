# 🌀 Rustle
Play on "Rust" and "hustle" or "sketch". Short, snappy, memorable.

## 📘 Background & Motivation
**Rustle** is a lightweight, expressive drawing engine powered by an **embedded Logo-inspired DSL** (domain-specific language). Rooted in the legacy of educational programming and visual computing, Rustle reinterprets the iconic "turtle graphics" concept through a modern systems lens using **Rust**.

Rustle enables users to write structured Logo programs that control a virtual turtle—moving, rotating, and drawing with fine-grained control. These programs are parsed and executed into **SVG** or **PNG** outputs, bridging the gap between textual programming and visual feedback.

This project is an opportunity to build a **custom interpreter**, an **SVG rendering engine**, and a **domain-specific execution model** — all while applying clean Rust architecture and modular design principles.

## 💡 Why Rustle
Rustle merges **retro computing nostalgia** with modern systems design best practices. It offers a clean, deterministic environment to learn about **interpreters**, **state machines**, **graphics pipelines**, and **DSL design** — with real visual output to demonstrate what your code is doing.

This project also highlights your ability to:
- Design modular, testable Rust software
- Work with parsing, graphics, and system abstractions
- Deliver both creative and technical outcomes

## 🎯 Project Goals
- Build an interpreter that parses and executes a custom Logo dialect.
- Design an extensible, modular Rust application with well-separated components.
- Convert turtle motion commands into vector graphics.
- Offer a clean developer interface for generating programmatic art.

## 🚀 Project Structure
```text
rustle/
├── Cargo.toml                 # Project manifest
├── README.md                  # Project documentation and usage
├── assets/                    # Sample Logo programs, example outputs
│   ├── examples.logo
│   └── turtle_art.svg
├── src/
│   ├── main.rs                # Entry point: CLI setup and execution
│   ├── lib.rs                 # Library root: re-exports modules
│   ├── parser/                # Logo DSL parsing
│   │   ├── mod.rs
│   │   └── logo_parser.rs     # Tokenization and AST generation
│   ├── interpreter/           # Logo command interpreter
│   │   ├── mod.rs
│   │   └── executor.rs        # Executes parsed AST on turtle state
│   ├── turtle/                # Turtle state and command logic
│   │   ├── mod.rs
│   │   └── turtle.rs          # Position, heading, pen state, movement
│   ├── render/                # SVG and PNG output generation
│   │   ├── mod.rs
│   │   └── svg_renderer.rs
│   ├── repl/                  # Interactive shell (optional)
│   │   ├── mod.rs
│   │   └── repl.rs
│   └── utils.rs               # Shared utility functions
├── tests/                     # Integration tests
│   └── turtle_tests.rs
└── examples/                  # Runnable examples
    └── square.rs
```

### 🧩 Module Breakdown
| Module |	Responsibility |
| ------ | -------------- |
| ```parser``` | Converts Logo source code into AST |
| ```interpreter``` |	Walks the AST and updates turtle state |
| ```turtle``` |	Represents position, heading, pen state, pen color, etc. |
| ```render``` |	Converts turtle path into SVG or PNG |
| ```repl``` |	Provides optional interactive mode |
| ```examples/``` |	Shows usage patterns and sample art |
| ```assets/``` |	Stores .logo scripts and their visual outputs |

## 🧠 Core Features
#### Part 1: Turtle Control Command
- [ ] ```PENUP```: Set the turtle to "up" mode where it doesn't draw.
- [ ] ```PENDOWN```: Set the turtle to "down" mode where it draws while moving.
- [ ] ```FORWARD```: Move the turtle forward by a specified number of pixels.
- [ ] ```BACK```: Move the turtle backward by a specified number of pixels.
- [ ] ```LEFT```: Turn the turtle left by a specified number of degrees.
- [ ] ```RIGHT```: Turn the turtle right by a specified number of degrees.
- [ ] ```SETPENCOLOR```: Set the pen color to one of the 16 predefined colors.
- [ ] ```TURN```: Turn the turtle by a specified number of degrees (relative rotation).
- [ ] ```SETHEADING```: Set the turtle’s heading to a specified direction (absolute angle).
- [ ] ```SETX```: Set the turtle's X position to a specific coordinate.
- [ ] ```SETY```: Set the turtle's Y position to a specific coordinate.
- [ ] Handle Initial Turtle Position: Ensure the turtle starts at the center of the image (adjust for odd/even dimensions).

#### Part 2: Variables and Queries
- [ ] ```MAKE``` Command for Variables: Define and assign variables with ```MAKE <variable-name> <value>```.
- [ ] ```ADDASSIGN``` Command: Implement the ```+=``` operation for variables (e.g., ```ADDASSIGN :var 5```).
- [ ] Handle Variables Without Scope: Ensure that variables are globally available once created.
- [ ] Support Variable Referencing: Allow variable values to be used directly with :variable syntax.
- [ ] Implement Queries:
  - [ ] ```XCOR```: Return the turtle's current X position.
  - [ ] ```YCOR```: Return the turtle's current Y position.
  - [ ] ```HEADING```: Return the turtle's current heading.
  - [ ] ```COLOR```: Return the current pen color as a number.

#### Part 3: IFs, WHILE, [ ]
- [ ] ```IF EQ```: Conditional execution based on the equality of two values.
- [ ] ```WHILE EQ```: Loop execution as long as two values are equal.
- [ ] Conditional and Loop Parsing: Ensure that nested commands inside ```[ ]``` are handled properly.
- [ ] Test with Nested Conditions: Ensure nested ```IF EQ``` and ```WHILE EQ``` structures work as expected.

#### Part 4: Implementing Maths and Comparisons using a Stack
- [ ] Implement Math Operations:
  - [ ] ```EQ```: Check equality between two values.
  - [ ] ```NE```: Check inequality between two values.
  - [ ] ```GT```: Check if one value is greater than another.
  - [ ] ```LT```: Check if one value is less than another.
  - [ ] ```+```: Add two values.
  - [ ] ```-```: Subtract two values.
  - [ ] ```*```: Multiply two values.
  - [ ] ```/```: Divide two values (handle division by zero).
- [ ] Handle Polish Notation for Math: Ensure the prefix notation (e.g., ```+ "3 "4```) works for mathematical operations.
- [ ] Stack Management for Operations: Ensure stack behavior for processing expressions.
- [ ] Implement Boolean Operations:
  - [ ] ```AND```: Returns ```TRUE``` if both values are ```TRUE```.
  - [ ] ```OR```: Returns ```TRUE``` if at least one value is ```TRUE```.
  - [ ] Error Handling for Invalid Operations: Ensure errors are raised for invalid types or missing arguments.

#### Part 5: Logo Defined Procedures
- [ ] Implement the ```TO``` Command for Procedure Definition: Define named procedures with arguments.
- [ ] Implement the ```END``` Command for Procedure Termination: Mark the end of a procedure definition.
- [ ] Procedure Argument Evaluation: Ensure arguments are evaluated at definition-time, not call-time.
- [ ] Handle Procedure Calls: Ensure defined procedures can be called with specific arguments.
- [ ] Prevent Recursive Procedures: Ensure that procedures do not call themselves.
- [ ] Implement Procedure Argument Parsing: Handle multiple procedure arguments correctly.

#### General and Cross-cutting Tasks
- [ ] Turtle State Management: Track the turtle’s state (position, heading, pen state, color).
- [ ] Error Handling: Ensure all commands provide useful error messages for invalid input or wrong data types.
- [ ] Unit Tests for Turtle Commands: Implement tests to verify the functionality of turtle control commands.
- [ ] Unit Tests for Variables and Queries: Implement tests for variable assignment and querying.
- [ ] Unit Tests for Conditional and Looping: Implement tests for ```IF EQ``` and ```WHILE EQ``` structures.
- [ ] Unit Tests for Mathematical Operations: Implement tests for math operations and stack usage.
- [ ] Unit Tests for Procedures: Implement tests to verify the correct handling of procedures.

#### Extension Ideas (Optional)
- [ ] Implement Other Comparisons: Support additional comparison operations like ```GE``` (greater than or equal), ```LE``` (less than or equal).
- [ ] Dynamic Variable Types: Allow variables to store more complex types (e.g., lists, dictionaries) later on.
- [ ] Interactive Mode (REPL): Implement a ```REPL``` (Read-Eval-Print Loop) for users to interact with the language.
- [ ] Graphics Rendering: Enhance the pen color functionality to support full RGB values or gradients.
- [ ] File I/O: Add the ability to save the generated turtle images to files (e.g., SVG, PNG).
- [ ] Advanced Procedures: Support recursive procedures or support default argument values.

## 🔧 Extended Features (Optional - Todo)
#### 1. Live Web Preview (WASM or WebSocket Integration)
Build a frontend with real-time rendering using WebAssembly or a server-client bridge with live updates.
#### 2. Animation Mode
Instead of static SVG output, generate a frame-by-frame animation (e.g., GIF or HTML5 Canvas sequence).
#### 3. Macro System or Variables
Support simple macros, variables, and parameterized drawing functions.
#### 4. Error Handling and Debug Output
Provide line-level feedback, runtime error tracking, and visual trace paths.
#### 5. Interactive Mode
Implement a REPL-style interface to issue drawing commands live.
#### 6. Command Extensions
Add features like CIRCLE, ARC, COLOR, or even support for sound/scripting hooks.
#### 7. Export to Vector APIs
Optionally support exporting to formats like PDF, or integrate with plotting tools for pen plotter output.

