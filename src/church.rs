//! Church encodings for booleans and numerals in lambda calculus

use crate::expr::{app, fun, var, Expr};

// Church Booleans
pub fn church_true() -> Box<Expr> {
    // λx.λy.x
    fun("x".to_string(), fun("y".to_string(), var("x".to_string())))
}

pub fn church_false() -> Box<Expr> {
    // λx.λy.y
    fun("x".to_string(), fun("y".to_string(), var("y".to_string())))
}

pub fn church_if(cond: Box<Expr>, then_branch: Box<Expr>, else_branch: Box<Expr>) -> Box<Expr> {
    // cond then else
    app(app(cond, then_branch), else_branch)
}

pub fn church_not(p: Box<Expr>) -> Box<Expr> {
    // λp.λa.λb.p b a  =>  p FALSE TRUE
    app(app(p, church_false()), church_true())
}

pub fn church_and(p: Box<Expr>, q: Box<Expr>) -> Box<Expr> {
    // λp.λq.p q p  =>  p q FALSE
    app(app(p, q), church_false())
}

// Church Numerals
pub fn church_zero() -> Box<Expr> {
    // λf.λx.x
    fun("f".to_string(), fun("x".to_string(), var("x".to_string())))
}

pub fn church_one() -> Box<Expr> {
    // λf.λx.f x
    fun(
        "f".to_string(),
        fun(
            "x".to_string(),
            app(var("f".to_string()), var("x".to_string())),
        ),
    )
}

pub fn church_two() -> Box<Expr> {
    // λf.λx.f (f x)
    fun(
        "f".to_string(),
        fun(
            "x".to_string(),
            app(
                var("f".to_string()),
                app(var("f".to_string()), var("x".to_string())),
            ),
        ),
    )
}

pub fn church_three() -> Box<Expr> {
    // λf.λx.f (f (f x))
    fun(
        "f".to_string(),
        fun(
            "x".to_string(),
            app(
                var("f".to_string()),
                app(
                    var("f".to_string()),
                    app(var("f".to_string()), var("x".to_string())),
                ),
            ),
        ),
    )
}

pub fn church_succ() -> Box<Expr> {
    // λn.λf.λx.f (n f x)
    fun(
        "n".to_string(),
        fun(
            "f".to_string(),
            fun(
                "x".to_string(),
                app(
                    var("f".to_string()),
                    app(
                        app(var("n".to_string()), var("f".to_string())),
                        var("x".to_string()),
                    ),
                ),
            ),
        ),
    )
}

pub fn church_add() -> Box<Expr> {
    // λm.λn.λf.λx.m f (n f x)
    fun(
        "m".to_string(),
        fun(
            "n".to_string(),
            fun(
                "f".to_string(),
                fun(
                    "x".to_string(),
                    app(
                        app(var("m".to_string()), var("f".to_string())),
                        app(
                            app(var("n".to_string()), var("f".to_string())),
                            var("x".to_string()),
                        ),
                    ),
                ),
            ),
        ),
    )
}

pub fn church_mult() -> Box<Expr> {
    // λm.λn.λf.m (n f)
    fun(
        "m".to_string(),
        fun(
            "n".to_string(),
            fun(
                "f".to_string(),
                app(
                    var("m".to_string()),
                    app(var("n".to_string()), var("f".to_string())),
                ),
            ),
        ),
    )
}

pub fn church_is_zero() -> Box<Expr> {
    // λn.n (λx.FALSE) TRUE
    fun(
        "n".to_string(),
        app(
            app(var("n".to_string()), fun("x".to_string(), church_false())),
            church_true(),
        ),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eval::{bind_vars, eval};

    /// Helper function to evaluate an expression to normal form
    fn eval_to_normal(expr: Box<Expr>) -> Expr {
        let bound = bind_vars(*expr);
        let mut current = bound;
        for _ in 0..100 {
            let next = eval(current.clone());
            if next == current {
                return current;
            }
            current = next;
        }
        current
    }

    /// Helper to check if two church encodings are structurally equal (ignoring variable IDs)
    fn structural_eq(expr1: &Expr, expr2: &Expr) -> bool {
        match (expr1, expr2) {
            (Expr::Var(v1), Expr::Var(v2)) => v1.name == v2.name,
            (Expr::Fun(arg1, body1), Expr::Fun(arg2, body2)) => {
                arg1.name == arg2.name && structural_eq(body1, body2)
            }
            (Expr::App(f1, x1), Expr::App(f2, x2)) => {
                structural_eq(f1, f2) && structural_eq(x1, x2)
            }
            _ => false,
        }
    }

    #[test]
    fn test_if_true() {
        let result = eval_to_normal(church_if(
            church_true(),
            var("x".to_string()),
            var("y".to_string()),
        ));
        // Should evaluate to x
        match result {
            Expr::Var(name) => assert_eq!(name.name, "x"),
            _ => panic!("Expected variable x, got {:?}", result),
        }
    }

    #[test]
    fn test_if_false() {
        let result = eval_to_normal(church_if(
            church_false(),
            var("x".to_string()),
            var("y".to_string()),
        ));
        // Should evaluate to y
        match result {
            Expr::Var(name) => assert_eq!(name.name, "y"),
            _ => panic!("Expected variable y, got {:?}", result),
        }
    }

    #[test]
    fn test_not_true() {
        let result = eval_to_normal(church_not(church_true()));
        let expected = eval_to_normal(church_false());
        assert!(
            structural_eq(&result, &expected),
            "NOT TRUE should equal FALSE"
        );
    }

    #[test]
    fn test_not_false() {
        let result = eval_to_normal(church_not(church_false()));
        let expected = eval_to_normal(church_true());
        assert!(
            structural_eq(&result, &expected),
            "NOT FALSE should equal TRUE"
        );
    }

    #[test]
    fn test_and_true_true() {
        let result = eval_to_normal(church_and(church_true(), church_true()));
        let expected = eval_to_normal(church_true());
        assert!(
            structural_eq(&result, &expected),
            "AND TRUE TRUE should equal TRUE"
        );
    }

    #[test]
    fn test_and_true_false() {
        let result = eval_to_normal(church_and(church_true(), church_false()));
        let expected = eval_to_normal(church_false());
        assert!(
            structural_eq(&result, &expected),
            "AND TRUE FALSE should equal FALSE"
        );
    }

    #[test]
    fn test_succ_zero() {
        let result = eval_to_normal(app(church_succ(), church_zero()));
        let expected = eval_to_normal(church_one());
        assert!(structural_eq(&result, &expected), "SUCC 0 should equal 1");
    }

    #[test]
    fn test_succ_two() {
        let result = eval_to_normal(app(church_succ(), church_two()));
        let expected = eval_to_normal(church_three());
        assert!(structural_eq(&result, &expected), "SUCC 2 should equal 3");
    }

    #[test]
    fn test_add_one_two() {
        let result = eval_to_normal(app(app(church_add(), church_one()), church_two()));
        let expected = eval_to_normal(church_three());
        assert!(structural_eq(&result, &expected), "ADD 1 2 should equal 3");
    }

    #[test]
    fn test_add_zero_two() {
        let result = eval_to_normal(app(app(church_add(), church_zero()), church_two()));
        let expected = eval_to_normal(church_two());
        assert!(structural_eq(&result, &expected), "ADD 0 2 should equal 2");
    }

    #[test]
    fn test_mult_two_three() {
        let result = eval_to_normal(app(app(church_mult(), church_two()), church_three()));
        // 2 * 3 = 6, we need to construct 6
        let six = eval_to_normal(app(app(church_add(), church_three()), church_three()));
        assert!(structural_eq(&result, &six), "MULT 2 3 should equal 6");
    }

    #[test]
    fn test_mult_zero_three() {
        let result = eval_to_normal(app(app(church_mult(), church_zero()), church_three()));
        let expected = eval_to_normal(church_zero());
        assert!(structural_eq(&result, &expected), "MULT 0 3 should equal 0");
    }

    #[test]
    fn test_is_zero_zero() {
        let result = eval_to_normal(app(church_is_zero(), church_zero()));
        let expected = eval_to_normal(church_true());
        assert!(
            structural_eq(&result, &expected),
            "IS_ZERO 0 should equal TRUE"
        );
    }

    #[test]
    fn test_is_zero_one() {
        let result = eval_to_normal(app(church_is_zero(), church_one()));
        let expected = eval_to_normal(church_false());
        assert!(
            structural_eq(&result, &expected),
            "IS_ZERO 1 should equal FALSE"
        );
    }

    #[test]
    fn test_is_zero_two() {
        let result = eval_to_normal(app(church_is_zero(), church_two()));
        let expected = eval_to_normal(church_false());
        assert!(
            structural_eq(&result, &expected),
            "IS_ZERO 2 should equal FALSE"
        );
    }
}
