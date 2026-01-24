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
    Mod,
    Expn,
    FloorDiv,
    Equal,
    NotEqual,
    Less,
    Greater,
    LessEq,
    GreaterEq,
}

#[derive(Debug)]
pub enum Expr {
    Number(i32),
    Variable(String),
    Define {
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
    If {
        condition: Box<Expr>,
        then_branch: Box<Expr>,
        else_branch: Box<Expr>,
    },
    While {
        condition: Box<Expr>,
        body: Box<Expr>,
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
        if let Some(Token::If) = self.peek() {
            return self.parse_if();
        }

        if let Some(Token::While) = self.peek() {
            return self.parse_while();
        }
        self.parse_comparison()
    }

    fn parse_while(&mut self) -> Result<Expr, String> {
        self.advance();
        match self.advance() {
            Some(Token::LPara) => {}
            _ => return Err("Expected '(' after 'while' ".into()),
        }
        let condition = self.parse_expr()?;
        let body = self.parse_expr()?;

        match self.advance() {
            Some(Token::RPara) => {}
            _ => return Err("Expected ')' to end 'while' ".into()),
        }

        Ok(Expr::While {
            condition: Box::new(condition),
            body: Box::new(body),
        })
    }

    fn parse_if(&mut self) -> Result<Expr, String> {
        self.advance();
        match self.advance() {
            Some(Token::LPara) => {}
            _ => return Err("Expected '(' after 'if' ".into()),
        }
        let condition = self.parse_expr()?;
        let then_branch = self.parse_expr()?;
        let else_branch = self.parse_expr()?;
        match self.advance() {
            Some(Token::RPara) => {}
            _ => return Err("Expected ')' to close 'if' ".into()),
        }
        Ok(Expr::If {
            condition: Box::new(condition),
            then_branch: Box::new(then_branch),
            else_branch: Box::new(else_branch),
        })
    }

    fn parse_comparison(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_additive()?;
        while let Some(tok) = self.peek() {
            match tok {
                Token::Equal
                | Token::NotEqual
                | Token::Greater
                | Token::GreaterEq
                | Token::Less
                | Token::LessEq => {
                    let op = match self.advance().unwrap() {
                        Token::Equal => BinaryOp::Equal,
                        Token::NotEqual => BinaryOp::NotEqual,
                        Token::Less => BinaryOp::Less,
                        Token::Greater => BinaryOp::Greater,
                        Token::LessEq => BinaryOp::LessEq,
                        Token::GreaterEq => BinaryOp::GreaterEq,
                        _ => unreachable!(),
                    };
                    let right = self.parse_additive()?;
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

    fn parse_additive(&mut self) -> Result<Expr, String> {
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

        Ok(Expr::Define {
            name,
            value: Box::new(value),
            body: Box::new(body),
        })
    }

    fn parse_term(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_exponent()?;
        while let Some(tok) = self.peek() {
            match tok {
                Token::Slash | Token::Star | Token::Percent | Token::SlashSlash => {
                    let op = match self.advance().unwrap() {
                        Token::Star => BinaryOp::Mul,
                        Token::Slash => BinaryOp::Div,
                        Token::Percent => BinaryOp::Mod,
                        Token::SlashSlash => BinaryOp::FloorDiv,
                        _ => unreachable!(),
                    };

                    let right = self.parse_exponent()?;
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

    fn parse_exponent(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_unary()?;
        while let Some(tok) = self.peek() {
            match tok {
                Token::StarStar => {
                    self.advance();
                    let right = self.parse_unary()?;
                    expr = Expr::Binary {
                        left: Box::new(expr),
                        op: BinaryOp::Expn,
                        right: Box::new(right),
                    }
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
