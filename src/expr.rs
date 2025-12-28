#[derive(Clone, PartialEq, Debug)]
pub struct VarName {
    pub(crate) name: String,
    pub(crate) id: usize,
}

impl VarName {
    pub fn new_free(name: String) -> Self {
        VarName { name, id: 0 }
    }

    #[allow(dead_code)]
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

impl Expr {
    /// Format the expression with optional unique IDs
    pub fn format(&self, show_id: bool) -> String {
        match self {
            Expr::Var(name) => {
                if show_id {
                    format!("{}{}", name.name, name.id)
                } else {
                    name.name.clone()
                }
            }
            Expr::Fun(name, body) => {
                let name_str = if show_id {
                    format!("{}{}", name.name, name.id)
                } else {
                    name.name.clone()
                };
                format!("(\\{}.{})", name_str, body.format(show_id))
            }
            Expr::App(lhs, rhs) => {
                format!("({} {})", lhs.format(show_id), rhs.format(show_id))
            }
        }
    }
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.format(true))
    }
}
