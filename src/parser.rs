use crate::lexer::Token;

#[derive(Debug)]
pub enum UnaryOp {
    Neg,
}

#[derive(Debug)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
pub enum Expr {
    Number(i32),
    Unary {
        op: UnaryOp,
        expr: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) -> Option<&Token> {
        let tok = self.tokens.get(self.pos);
        self.pos += 1;
        tok
    }

    // TODO : need to change this error from string type to thiserror
    pub fn parse(&mut self) -> Result<Expr, String> {
        let expr = self.parse_expr()?;

        if self.peek().is_some() {
            return Err("Unexpected Tokens after Parsing".into());
        }

        Ok(expr)
    }

    // using a stack descent parser for now, will check my options later ;

    pub fn parse_expr(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_term()?;
        while let Some(tok) = self.peek() {
            match tok {
                Token::Minus | Token::Plus => {
                    let op = match self.advance().unwrap() {
                        Token::Plus => BinaryOp::Add,
                        Token::Minus => BinaryOp::Sub,
                        _ => unreachable!(),
                    };
                    let right = self.parse_term()?;
                    expr = Expr::Binary {
                        left: Box::new(expr),
                        op,
                        right: Box::new(right),
                    }
                }
                _ => break,
            }
        }
        Ok(expr)
    }

    fn parse_term(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_unary()?;
        while let Some(tok) = self.peek() {
            match tok {
                Token::Slash | Token::Star => {
                    let op = match self.advance().unwrap() {
                        Token::Star => BinaryOp::Mul,
                        Token::Slash => BinaryOp::Div,
                        _ => unreachable!(),
                    };

                    let right = self.parse_unary()?;
                    expr = Expr::Binary {
                        left: Box::new(expr),
                        op,
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }
        Ok(expr)
    }

    fn parse_unary(&mut self) -> Result<Expr, String> {
        if let Some(Token::Minus) = self.peek() {
            self.advance();
            let expr = self.parse_unary()?;
            Ok(Expr::Unary {
                op: UnaryOp::Neg,
                expr: Box::new(expr),
            })
        } else {
            self.parse_primary()
        }
    }

    fn parse_primary(&mut self) -> Result<Expr, String> {
        match self.advance() {
            Some(Token::Number(n)) => Ok(Expr::Number(*n)),
            Some(Token::LPara) => {
                let expr = self.parse_expr()?;
                match self.advance() {
                    Some(Token::RPara) => Ok(expr),
                    _ => Err("Expected ')'".into()),
                }
            }
            Some(tok) => Err(format!("Unexpected token: {:?}", tok)),
            None => Err("Unexpected end of input".into()),
        }
    }
}
