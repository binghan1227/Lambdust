// This is a basic lambda calculus interpreter.

mod church;
mod tests;

#[derive(Clone, PartialEq)]
pub struct VarName {
    name: String,
    id: usize,
}

fn bind_var_free(name: String) -> VarName {
    VarName { name, id: 0 }
}

#[derive(Clone, PartialEq)]
pub enum Expr {
    Var(VarName),
    Fun(VarName, Box<Expr>),
    App(Box<Expr>, Box<Expr>),
}

pub fn var(name: String) -> Box<Expr> {
    Box::new(Expr::Var(bind_var_free(name)))
}

pub fn fun(name: String, body: Box<Expr>) -> Box<Expr> {
    Box::new(Expr::Fun(bind_var_free(name), body))
}

pub fn app(f: Box<Expr>, x: Box<Expr>) -> Box<Expr> {
    Box::new(Expr::App(f, x))
}

fn replace(arg: &VarName, body: Expr, val: &Expr) -> Expr {
    match body {
        Expr::Var(name) => {
            if name.name == arg.name && name.id == arg.id {
                val.clone()
            } else {
                Expr::Var(name)
            }
        }
        Expr::Fun(name, body) => Expr::Fun(name, Box::new(replace(arg, *body, val))),
        Expr::App(lhs, rhs) => Expr::App(
            Box::new(replace(arg, *lhs, val)),
            Box::new(replace(arg, *rhs, val)),
        ),
    }
}

fn eval(expr: Expr) -> Expr {
    match expr {
        Expr::Var(name) => Expr::Var(name),
        Expr::Fun(arg, body) => {
            let new_body = eval(*body);
            Expr::Fun(arg, Box::new(new_body))
        }
        Expr::App(lhs, rhs) => {
            let lhs_expr = *lhs;
            let new_lhs = eval(lhs_expr.clone());

            if new_lhs != lhs_expr {
                return Expr::App(Box::new(new_lhs), rhs);
            }

            match new_lhs {
                Expr::Fun(arg, body) => {
                    let evaled_rhs = eval(*rhs);
                    replace(&arg, *body, &evaled_rhs)
                }
                _ => {
                    let new_rhs = eval(*rhs);
                    Expr::App(Box::new(new_lhs), Box::new(new_rhs))
                }
            }
        }
    }
}

fn print_expr(expr: &Expr) {
    match expr {
        Expr::Var(name) => print!("{}{}", name.name, name.id),
        Expr::Fun(name, body) => {
            print!("(\\{}{}.", name.name, name.id);
            print_expr(body);
            print!(")");
        }
        Expr::App(lhs, rhs) => {
            print!("(");
            print_expr(lhs);
            print!(" ");
            print_expr(rhs);
            print!(")");
        }
    }
}

pub fn trace_expr(expr: Expr) -> Expr {
    let mut current = expr;
    let max_iterations = 10;

    println!("Step 0:");
    print_expr(&current);
    println!();

    for i in 1..=max_iterations {
        let next = eval(current.clone());

        if next == current {
            println!("\nReached normal form after {} step(s)", i - 1);
            return current;
        }

        println!("\nStep {}:", i);
        print_expr(&next);
        println!();

        current = next;
    }

    println!(
        "\nStopped after {} iterations (may not be in normal form)",
        max_iterations
    );
    current
}

fn bind_var(body: Expr, name: &VarName) -> Expr {
    match body {
        Expr::Var(mut body_name) => {
            if body_name.name == name.name {
                body_name.id = name.id;
            }
            Expr::Var(body_name)
        }
        Expr::Fun(body_name, fun_body) => Expr::Fun(body_name, Box::new(bind_var(*fun_body, name))),
        Expr::App(lhs, rhs) => Expr::App(
            Box::new(bind_var(*lhs, name)),
            Box::new(bind_var(*rhs, name)),
        ),
    }
}

pub fn bind_vars(expr: Expr) -> Expr {
    static mut VAR_ID: usize = 1;
    match expr {
        Expr::Var(name) => Expr::Var(name),
        Expr::Fun(mut arg, body) => {
            arg.id = unsafe { VAR_ID };
            unsafe { VAR_ID += 1 };
            let bound_body = bind_var(*body, &arg);
            let transformed_body = bind_vars(bound_body);
            Expr::Fun(arg, Box::new(transformed_body))
        }
        Expr::App(lhs, rhs) => {
            let transformed_lhs = bind_vars(*lhs);
            let transformed_rhs = bind_vars(*rhs);
            Expr::App(Box::new(transformed_lhs), Box::new(transformed_rhs))
        }
    }
}

fn main() {
    tests::run_all_tests();
}
