// TODO: using a simpler repr now might add more later;

#[derive(Debug)]
pub enum Token {
    Number(i32),
    Plus,
    Minus,
    Star,
    Slash,
    LPara,
    RPara,
}

pub struct Lexer {
    input: Vec<char>,
    pos: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            pos: 0,
        }
    }

    fn peek (&self) -> Option<char> {
        self.input.get(self.pos).copied()
    }

    fn advance (&mut self) {
        self.pos += 1; 
    }

    fn skip_whitespace (&mut self,) {
        while self.peek().unwrap().is_whitespace() {
            self.advance();
        }
    }

    fn read_number(&mut self) -> i32 {
        let mut num = 0 ;
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                num = num * 10 + (c as i32 - '0' as i32);
                self.advance();
            }else {
                break;
            }
        }

        num
    }

    pub fn tokenize(&mut self, ) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new(); 

        while let Some(c) = self.peek() {
            match c {
                ' ' | '\t' | '\n' => {
                    self.skip_whitespace();
                }

                '0'..'9' => {
                    let num = self.read_number();
                    tokens.push(Token::Number(num));
                }

                '+' => {
                    self.advance();
                    tokens.push(Token::Plus);
                }
                '-' => {
                    self.advance();
                    tokens.push(Token::Minus);  
                }
                '*' => {
                    self.advance();
                    tokens.push(Token::Star);
                }
                '/' => {
                    self.advance();
                    tokens.push(Token::Slash);
                }
                '(' => {
                    self.advance();
                    tokens.push(Token::LPara);
                }
                ')' => {
                    self.advance();
                    tokens.push(Token::RPara);
                }
                _ => {
                    return Err(format!("Invalid character: '{}'", c));
                }
            }
        }
        Ok(tokens)
    }
}
