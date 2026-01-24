// TODO: using a simpler repr now might add more later;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(i32),
    Plus,
    Minus,
    Star,
    Slash,
    LPara,
    RPara,
    Define, 
    Ident(String), 
    If,
    Equal, 
    NotEqual, // != 
    Less,
    Greater,
    LessEq, // <= 
    GreaterEq, // >= 
    While,
    Percent,
    SlashSlash,
    StarStar,
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

    fn peek(&self) -> Option<char> {
        self.input.get(self.pos).copied()
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if !c.is_whitespace() {
                break;
            }
            self.advance();
        }
    }

    fn read_number(&mut self) -> i32 {
        let mut num = 0;
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                num = num * 10 + (c as i32 - '0' as i32);
                self.advance();
            } else {
                break;
            }
        }

        num
    }

    fn read_identifier(&mut self) -> String {
        let mut ident = String::new();
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '_' { // var can be "define my_age" or "var1"
                ident.push(c);
                self.advance();
            }else {
                break;
            }
        }
        ident
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
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
                'a'..='z' | 'A'..='Z' | '_' => {
                    let ident = self.read_identifier();
                    let token = match ident.as_str() {
                        "define" => Token::Define, 
                        "if" => Token::If,
                        "while" => Token::While,
                        _ => Token::Ident(ident),
                    };
                    tokens.push(token);
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
                    if let Some('*') = self.peek() {
                        self.advance();
                        tokens.push(Token::StarStar);
                    }else {
                        tokens.push(Token::Star);
                    }
                }
                '/' => {
                    self.advance();
                    if let Some('/') = self.peek() {
                        self.advance();
                        tokens.push(Token::SlashSlash);
                    }else {
                        tokens.push(Token::Slash);
                    }
                }
                '%' => {
                    self.advance();
                    tokens.push(Token::Percent);
                }
                '(' => {
                    self.advance();
                    tokens.push(Token::LPara);
                }
                ')' => {
                    self.advance();
                    tokens.push(Token::RPara);
                }
                '=' => {
                    self.advance();
                    if let Some('=') = self.peek(){
                        self.advance();
                        tokens.push(Token::Equal);  
                    }else {
                        return Err("Expected '=' after = ".into());
                    }
                }
                '!' => {
                    self.advance();
                    if let Some('=') = self.peek() {
                        self.advance();
                        tokens.push(Token::NotEqual);
                    }else {
                        return Err("Expected '=' after !".into());
                    }
                }
                '<' => {
                    self.advance();
                    if let Some('=') = self.peek() {
                        self.advance();
                        tokens.push(Token::LessEq);
                    }else {
                        tokens.push(Token::Less);
                    }
                }
                '>' => { 
                    self.advance();
                    if let Some('=') = self.peek() {
                        self.advance();
                        tokens.push(Token::GreaterEq);
                    }else {
                        tokens.push(Token::Greater);
                    }
                }
                _ => {
                    return Err(format!("Invalid character: '{}'", c));
                }
            }
        }
        Ok(tokens)
    }
}
