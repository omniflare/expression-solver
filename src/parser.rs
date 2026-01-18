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
    Variable(String),
    Let {
        name: String,
        value: Box<Expr>,
        body: Box<Expr>,
    },
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

    //to make sure we return exactly one token from the entire collection;
    pub fn parse(&mut self) -> Result<Expr, String> {
        let expr = self.parse_expr()?;

        if self.peek().is_some() {
            return Err("Unexpected Tokens after Parsing".into());
        }

        Ok(expr)
    }

    pub fn parse_expr(&mut self) -> Result<Expr, String> {
        if let Some(Token::Define) = self.peek() {
            return self.parse_let();
        }
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

    pub fn parse_let(&mut self) -> Result<Expr, String> {
        // this consumes 'define'
        self.advance();

        match self.advance() {
            Some(Token::LPara) => {}
            _ => return Err("Expected '(' after 'define' ".into()),
        }
        let name = match self.advance() {
            Some(Token::Ident(n)) => n.clone(),
            _ => return Err("Expected variable name after 'define (' ".into()),
        };

        let value = self.parse_expr()?;
        let body = self.parse_expr()?;

        match self.advance() {
            Some(Token::RPara) => {}
            _ => return Err("Expected ')' to close define expression".into()),
        }

        Ok(Expr::Let {
            name,
            value: Box::new(value),
            body: Box::new(body),
        })
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
            Some(Token::Ident(name)) => Ok(Expr::Variable(name.clone())),
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
