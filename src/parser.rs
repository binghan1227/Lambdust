use crate::expr::{app, fun, var, Expr};
use anyhow::Result;

#[derive(Debug, PartialEq)]
enum Token {
    Lambda, // \
    Dot,    // .
    OParen, // (
    CParen, // )
    Name(String),
}

struct Lexer {
    input: Vec<char>,
    pos: usize,
}

impl Lexer {
    fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            pos: 0,
        }
    }

    fn current(&self) -> Option<char> {
        if self.pos < self.input.len() {
            Some(self.input[self.pos])
        } else {
            None
        }
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn trim_space(&mut self) {
        while let Some(ch) = self.current() {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn read_name(&mut self) -> String {
        let mut ret = String::new();
        while let Some(ch) = self.current() {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                ret.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        ret
    }

    fn tokenize(&mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();

        loop {
            self.trim_space();

            match self.current() {
                None => break,
                Some('\\') | Some('λ') => {
                    tokens.push(Token::Lambda);
                    self.advance();
                }
                Some('.') => {
                    tokens.push(Token::Dot);
                    self.advance();
                }
                Some('(') => {
                    tokens.push(Token::OParen);
                    self.advance();
                }
                Some(')') => {
                    tokens.push(Token::CParen);
                    self.advance();
                }
                Some(ch) if ch.is_ascii_alphabetic() || ch == '_' => {
                    let name = self.read_name();
                    tokens.push(Token::Name(name));
                }
                _ => {
                    return Err(anyhow::anyhow!(
                        "unexpected character: {}",
                        self.current().unwrap()
                    ));
                }
            }
        }

        Ok(tokens)
    }
}

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn current(&self) -> Option<&Token> {
        if self.pos < self.tokens.len() {
            Some(&self.tokens[self.pos])
        } else {
            None
        }
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn expect(&mut self, expected: Token) -> Result<()> {
        match self.current() {
            Some(tok) if tok == &expected => {
                self.advance();
                Ok(())
            }
            Some(tok) => Err(anyhow::anyhow!("expected {:?}, got {:?}", expected, tok)),
            None => Err(anyhow::anyhow!("expected {:?}, got end of input", expected)),
        }
    }

    fn parse_expr(&mut self) -> Result<Box<Expr>> {
        match self.current() {
            Some(Token::Lambda) => self.parse_lambda(),
            _ => self.parse_application(),
        }
    }

    fn parse_lambda(&mut self) -> Result<Box<Expr>> {
        self.expect(Token::Lambda)?;

        let param = match self.current() {
            Some(Token::Name(name)) => {
                let n = name.clone();
                self.advance();
                n
            }
            Some(tok) => {
                return Err(anyhow::anyhow!("expected parameter name, got {:?}", tok));
            }
            None => {
                return Err(anyhow::anyhow!("expected parameter name, got end of input"));
            }
        };

        self.expect(Token::Dot)?;

        let body = self.parse_expr()?;

        Ok(fun(param, body))
    }

    fn parse_application(&mut self) -> Result<Box<Expr>> {
        let mut exprs = Vec::new();

        while let Some(Token::Name(_)) | Some(Token::OParen) = self.current() {
            exprs.push(self.parse_atom()?);
        }

        if exprs.is_empty() {
            return Err(anyhow::anyhow!("expected expression"));
        }

        let mut result = exprs[0].clone();
        for expr in exprs.into_iter().skip(1) {
            result = app(result, expr);
        }

        Ok(result)
    }

    fn parse_atom(&mut self) -> Result<Box<Expr>> {
        match self.current() {
            Some(Token::Name(name)) => {
                let n = name.clone();
                self.advance();
                Ok(var(n))
            }
            Some(Token::OParen) => {
                self.advance();
                let expr = self.parse_expr()?;
                self.expect(Token::CParen)?;
                Ok(expr)
            }
            Some(tok) => Err(anyhow::anyhow!("expected identifier or '(', got {:?}", tok)),
            None => Err(anyhow::anyhow!("unexpected end of input")),
        }
    }

    pub fn parse(&mut self) -> Result<Box<Expr>> {
        let expr = self.parse_expr()?;

        if self.current().is_some() {
            return Err(anyhow::anyhow!("unexpected tokens after expression"));
        }

        Ok(expr)
    }
}

pub fn parse(input: &str) -> Result<Box<Expr>> {
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize()?;
    let mut parser = Parser::new(tokens);
    parser.parse()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_variable() {
        let expr = parse("x").unwrap();
        assert_eq!(expr, var("x".to_string()));
    }

    #[test]
    fn test_parse_identity() {
        let expr = parse("\\x.x").unwrap();
        assert_eq!(expr, fun("x".to_string(), var("x".to_string())));
    }

    #[test]
    fn test_parse_const() {
        let expr = parse("\\x.\\y.x").unwrap();
        assert_eq!(
            expr,
            fun("x".to_string(), fun("y".to_string(), var("x".to_string())))
        );
    }

    #[test]
    fn test_parse_application() {
        let expr = parse("f x").unwrap();
        assert_eq!(expr, app(var("f".to_string()), var("x".to_string())));
    }

    #[test]
    fn test_parse_multiple_application() {
        let expr = parse("f x y").unwrap();
        assert_eq!(
            expr,
            app(
                app(var("f".to_string()), var("x".to_string())),
                var("y".to_string())
            )
        );
    }

    #[test]
    fn test_parse_parentheses() {
        let expr = parse("f (g x)").unwrap();
        assert_eq!(
            expr,
            app(
                var("f".to_string()),
                app(var("g".to_string()), var("x".to_string()))
            )
        );
    }

    #[test]
    fn test_parse_lambda_application() {
        let expr = parse("(\\x.x) y").unwrap();
        assert_eq!(
            expr,
            app(
                fun("x".to_string(), var("x".to_string())),
                var("y".to_string())
            )
        );
    }

    #[test]
    fn test_parse_complex() {
        let expr = parse("(\\f.\\x.f (f x)) (\\y.y) z").unwrap();
        let expected = app(
            app(
                fun(
                    "f".to_string(),
                    fun(
                        "x".to_string(),
                        app(
                            var("f".to_string()),
                            app(var("f".to_string()), var("x".to_string())),
                        ),
                    ),
                ),
                fun("y".to_string(), var("y".to_string())),
            ),
            var("z".to_string()),
        );
        assert_eq!(expr, expected);
    }

    #[test]
    fn test_parse_unicode_lambda() {
        let expr = parse("λx.x").unwrap();
        assert_eq!(expr, fun("x".to_string(), var("x".to_string())));
    }

    #[test]
    fn test_parse_whitespace() {
        let expr = parse("  \\x . x  ").unwrap();
        assert_eq!(expr, fun("x".to_string(), var("x".to_string())));
    }

    #[test]
    fn test_parse_error_unexpected_char() {
        let result = parse("x + y");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_error_missing_dot() {
        let result = parse("\\x x");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_error_unmatched_paren() {
        let result = parse("(x");
        assert!(result.is_err());
    }
}
