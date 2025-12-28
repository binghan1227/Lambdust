//! A lambda calculus interpreter with Church encodings
//!
//! This library provides a simple lambda calculus evaluator along with
//! implementations of Church encodings for booleans and numerals.

pub mod args;
pub mod church;
pub mod eval;
pub mod examples;
pub mod expr;
pub mod parser;

// Re-export commonly used items
pub use eval::{bind_vars, eval, trace_eval};
pub use expr::{app, fun, var, Expr, VarName};
