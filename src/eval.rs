use crate::expr::{Expr, VarName};

/// Substitute a variable with a value in an expression
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

/// Evaluate a lambda calculus expression one step
pub fn eval(expr: Expr) -> Expr {
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

/// Trace evaluation steps
pub fn trace_eval(
    expr: Expr,
    max_iterations: usize,
    show_unique_id: bool,
    print_step: bool,
) -> (Expr, bool) {
    let mut current = expr;

    if print_step {
        println!("Step 0:");
        println!("{}", current.format(show_unique_id));
    }

    for i in 1..=max_iterations {
        let next = eval(current.clone());

        if next == current {
            // println!("\nReached normal form after {} step(s)", i - 1);
            return (current, false);
        }

        if print_step {
            println!("\nStep {}:", i);
            println!("{}", next.format(show_unique_id));
        }

        current = next;
    }

    // println!(
    //     "\nStopped after {} iterations (may not be in normal form)",
    //     max_iterations
    // );
    (current, true)
}

/// Bind a variable in an expression body
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

/// Assign unique IDs to all bound variables in an expression
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr::{app, fun, var};

    #[test]
    fn test_eval_variable() {
        let expr = *var("x".to_string());
        let result = eval(expr.clone());
        assert_eq!(result, expr);
    }

    #[test]
    fn test_eval_identity() {
        // (λx.x) y -> y
        let expr = *app(
            fun("x".to_string(), var("x".to_string())),
            var("y".to_string()),
        );
        let bound = bind_vars(expr);
        let result = eval(bound);

        // Result should be y (with id 0 since it's free)
        match result {
            Expr::Var(name) => {
                assert_eq!(name.name, "y");
            }
            _ => panic!("Expected a variable"),
        }
    }

    #[test]
    fn test_replace() {
        // Replace x with y in: x
        let arg = VarName {
            name: "x".to_string(),
            id: 1,
        };
        let body = Expr::Var(VarName {
            name: "x".to_string(),
            id: 1,
        });
        let val = Expr::Var(VarName {
            name: "y".to_string(),
            id: 0,
        });

        let result = replace(&arg, body, &val);
        assert_eq!(result, val);
    }
}
