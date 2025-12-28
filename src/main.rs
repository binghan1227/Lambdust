//! Lambda calculus interpreter CLI

use clap::Parser;
use lambdust::eval::{bind_vars, trace_eval};
use lambdust::parser::parse;

mod args;

fn main() {
    let args = args::Args::parse();

    println!("Lambdust");
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let expr = parse(&input);
        expr.map(|e| {
            trace_eval(bind_vars(*e), args.trace, args.unique_id);
        })
        .unwrap_or_else(|err| println!("Error: {}", err));
    }
}
