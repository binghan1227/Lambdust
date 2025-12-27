//! Lambda calculus interpreter CLI

use lambdust::eval::{bind_vars, trace_eval};
use lambdust::parser::parse;

fn main() {
    println!("Lambdust");
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let expr = parse(&input);
        match expr {
            Ok(expr) => {
                trace_eval(bind_vars(*expr), 10);
                ()
            }
            Err(err) => {
                println!("Error: {}", err);
            }
        }
    }
}
