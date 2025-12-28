//! Lambda calculus interpreter CLI

use clap::Parser;
use lambdust::eval::{bind_vars, trace_eval};
use lambdust::parser::parse;
use std::io::Write;

mod args;

fn main() {
    let mut args = args::Args::parse();

    println!("Lambdust");
    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let trimmed = input.trim();

        if trimmed.is_empty() {
            continue;
        }

        // Handle commands starting with ':'
        if trimmed.starts_with(':') {
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            let command = parts[0];

            match command {
                ":q" | ":quit" => break,
                ":h" | ":help" => {
                    println!("Available commands:");
                    println!("  :h, :help          - Show this help message");
                    println!("  :q, :quit          - Exit the program");
                    println!(
                        "  :p, :print         - Toggle print step (current: {})",
                        args.print_step
                    );
                    println!(
                        "  :u, :unique        - Toggle unique ID (current: {})",
                        args.unique_id
                    );
                    println!(
                        "  :t, :trace [num]   - Show or set trace limit (current: {})",
                        args.trace
                    );
                }
                ":p" | ":print" => {
                    args.print_step = !args.print_step;
                    println!("Print step: {}", args.print_step);
                }
                ":u" | ":unique" => {
                    args.unique_id = !args.unique_id;
                    println!("Unique ID: {}", args.unique_id);
                }
                ":t" | ":trace" => {
                    if parts.len() > 1 {
                        match parts[1].parse::<usize>() {
                            Ok(num) => {
                                args.trace = num;
                                println!("Trace limit: {}", args.trace);
                            }
                            Err(_) => println!("Error: Invalid number for trace limit"),
                        }
                    } else {
                        println!("Current trace limit: {}", args.trace);
                    }
                }
                _ => println!(
                    "Unknown command: {}. Type :help for available commands.",
                    command
                ),
            }
            continue;
        }

        let expr = parse(&input);
        expr.map(|e| {
            let result = trace_eval(bind_vars(*e), args.trace, args.unique_id, args.print_step);
            println!("{}", result.0.format(args.unique_id));
            if result.1 {
                println!("...");
            }
        })
        .unwrap_or_else(|err| println!("Error: {}", err));
    }
}
