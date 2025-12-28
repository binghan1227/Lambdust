# Lambdust

A simple lambda calculus interpreter written in Rust.

## Installation

Make sure you have Rust installed. Then clone and build:

```bash
cargo build --release
```

## Usage

### Interactive REPL

Run the interactive lambda calculus interpreter:

```bash
cargo run
```

This starts an interactive session where you can enter lambda expressions and see them evaluated:

```
Lambdust
> (\x.x) y
y
> (\x.\y.x) a b
a
```

**Command-line options:**

```bash
cargo run -- [OPTIONS]

Options:
  -u, --unique-id     Show unique ID after variable names (default: off)
  -t, --trace <NUM>   Maximum number of evaluation steps (default: 10)
  -p, --print-step    Print each evaluation step (default: off)
  -h, --help          Print help information
  -V, --version       Print version information
```

**REPL commands:**

- `:h`, `:help` - Show help message
- `:q`, `:quit` - Exit the program
- `:p`, `:print` - Toggle step-by-step printing during evaluation
- `:u`, `:unique` - Toggle display of unique variable IDs
- `:t`, `:trace [num]` - Show or set the maximum trace limit

**Examples:**

```bash
# Run with step-by-step printing enabled
cargo run -- -p

# Set trace limit to 100 steps
cargo run -- -t 100

# Show unique IDs and print steps
cargo run -- -u -p
```

### Run Tests

```bash
cargo test
```

All 31 unit tests should pass, covering:
- Expression evaluation
- Church boolean operations
- Church numeral operations
- Parser functionality (syntax parsing and error handling)

### Use as a Library

Add to your `Cargo.toml`:

```toml
[dependencies]
lambdust = { path = "../lambdust" }
```

Then use in your code:

```rust
use lambdust::eval::{bind_vars, trace_eval};
use lambdust::expr::{app, fun, var};
use lambdust::parser;
use lambdust::church::*;

fn main() {
    // Method 1: Build expressions programmatically
    let identity = fun("x".to_string(), var("x".to_string()));
    let expr = app(identity, var("y".to_string()));

    // Method 2: Parse from string (supports both \ and λ)
    let expr = parser::parse("(\\x.x) y").unwrap();

    // Bind variables and evaluate
    let bound = bind_vars(*expr);
    let (result, exceeded_limit) = trace_eval(bound, 10, false, false);
    println!("{}", result.format(false));
    if exceeded_limit {
        println!("Evaluation exceeded step limit");
    }
}
```

### Parsing Lambda Expressions

The parser module allows you to write lambda expressions using a familiar syntax:

```rust
use lambdust::eval::{bind_vars, trace_eval};
use lambdust::parser;

// Parse expressions from strings
let identity = parser::parse("\\x.x").unwrap();              // Identity: λx.x
let const_fn = parser::parse("\\x.\\y.x").unwrap();          // Const: λx.λy.x
let apply = parser::parse("(\\x.x) y").unwrap();             // Application
let complex = parser::parse("(\\f.\\x.f (f x)) g z").unwrap(); // Complex expression

// You can also use the Unicode lambda symbol
let identity = parser::parse("λx.x").unwrap();

// Evaluate parsed expressions with options
let bound = bind_vars(*apply);
let (result, exceeded_limit) = trace_eval(
    bound,
    10,      // max iterations
    false,   // show unique IDs
    false    // print each step
);
println!("{}", result.format(false));
```

**Parser syntax:**
- Lambda abstraction: `\x.body` or `λx.body`
- Application: `f x` (left-associative, so `f x y` means `(f x) y`)
- Variables: any alphanumeric identifier (e.g., `x`, `y`, `foo`, `x_1`)
- Parentheses: use `(` `)` for grouping

## Project Structure

```
src/
├── lib.rs         # Library entry point with public API
├── expr.rs        # Expression types (Expr, VarName) and constructors
├── eval.rs        # Evaluation logic and variable binding
├── parser.rs      # Parser for lambda calculus syntax
├── church.rs      # Church encodings for booleans and numerals
├── examples.rs    # Example demonstrations
├── args.rs        # Command-line argument parsing
└── main.rs        # Interactive REPL entry point
```

## Features

- **Interactive REPL**: Command-line interface with:
  - Real-time lambda expression evaluation
  - Configurable display options (unique IDs, step-by-step tracing)
  - Adjustable evaluation limits
  - Built-in help system
- **Command-line Arguments**: Control behavior via flags (`-u`, `-t`, `-p`)
- **Expression Builder**: Programmatically construct lambda expressions using `var`, `fun`, and `app`
- **Parser**: Parse lambda expressions from string syntax (supports both `\` and `λ`)
- **Evaluator**: Beta-reduction with configurable step-by-step tracing
- **Church Encodings**: Built-in support for:
  - Booleans: TRUE, FALSE, AND, OR, NOT, IF
  - Numerals: ZERO, SUCC, PLUS, MULT, PRED, SUB
- **De Bruijn Indexing**: Automatic variable binding and renaming

## Lambda Calculus Syntax

Expressions are displayed using this notation:

- Variables: `x0`, `y0` (name + unique ID)
- Lambda abstraction: `(\x1.body)` - λx.body
- Application: `(f x)` - apply f to x

## Testing

The project includes comprehensive unit tests:

- 3 tests in `eval.rs` for core evaluation logic
- 15 tests in `church.rs` for Church encodings
- 13 tests in `parser.rs` for parsing and error handling

Tests were created with assistance from Claude (Anthropic's AI assistant).

Run tests with:

```bash
cargo test        # Run all tests
cargo test --lib  # Run library tests only
```

## Acknowledgments

- Inspired by [tsoding/lamb](https://github.com/tsoding/lamb)
- Tests created with assistance from Claude (Anthropic)
- Built with Rust 🦀
