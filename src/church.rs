use crate::{app, fun, var, Expr};

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
