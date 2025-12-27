#[derive(Clone, PartialEq, Debug)]
pub struct VarName {
    pub(crate) name: String,
    pub(crate) id: usize,
}

impl VarName {
    pub fn new_free(name: String) -> Self {
        VarName { name, id: 0 }
    }

    pub(crate) fn new_bound(name: String, id: usize) -> Self {
        VarName { name, id }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Expr {
    Var(VarName),
    Fun(VarName, Box<Expr>),
    App(Box<Expr>, Box<Expr>),
}

pub fn var(name: String) -> Box<Expr> {
    Box::new(Expr::Var(VarName::new_free(name)))
}

pub fn fun(name: String, body: Box<Expr>) -> Box<Expr> {
    Box::new(Expr::Fun(VarName::new_free(name), body))
}

pub fn app(f: Box<Expr>, x: Box<Expr>) -> Box<Expr> {
    Box::new(Expr::App(f, x))
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Var(name) => write!(f, "{}{}", name.name, name.id),
            Expr::Fun(name, body) => write!(f, "(\\{}{}.{})", name.name, name.id, body),
            Expr::App(lhs, rhs) => write!(f, "({} {})", lhs, rhs),
        }
    }
}
