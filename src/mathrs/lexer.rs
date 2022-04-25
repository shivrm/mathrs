use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, Clone, Copy)]
pub enum Ops {
    Add,
    Sub,
    Mul,
    Div
}

#[derive(Debug)]
pub enum Token {
    Number(i32),
    Identifier(String),

    OpenParen,
    CloseParen,

    Operator(Ops),
    EOF
}

pub struct Lexer<'a> {
    pos: u32,
    text: &'a str,
    iter: Peekable<Chars<'a>>
}

impl<'a> Lexer<'a> {
    pub fn new(text: &'a str) -> Self {
        return Lexer {
            text,
            pos: 0,
            iter: text.chars().peekable()
        }
    }

    fn advance(&mut self) -> Option<char> {
        let next_char = self.iter.next();

        if let Some(_) = next_char {
            self.pos += 1;
        }

        return next_char;
    }

    #[inline]
    fn current_char(&mut self) -> Option<&char> {
        self.iter.peek()
    }

    fn skip_whitespace(&mut self) {
        while let Some(' ' | '\n' | '\r' | '\t') = self.current_char() {
            self.advance();
        }
    }

    fn read_number(&mut self) -> i32 {
        let mut num_string = String::new();
        
        while let op @ Some('0'..='9') = self.current_char() {
            match op {
                Some(&c) => num_string.push(c),
                None => break
            };
            self.advance();
        }

        return num_string.parse().unwrap();
    }

    fn read_identifier(&mut self) -> String {
        let mut ident_string = String::new();
        
        while let op @ Some('A'..='Z' | 'a'..='z') = self.current_char() {
            match op {
                Some(&c) => ident_string.push(c),
                None => break
            };
            self.advance();
        }

        return ident_string;
    }

    pub fn next_token(&mut self) -> Token {
        if let Some(c) = self.current_char() {
            let token = match c {
                ' ' | '\n' | '\r' | '\t' => {
                    self.skip_whitespace();
                    return self.next_token()
                }

                '0'..='9' => {
                    let number = self.read_number();
                    Token::Number(number) 
                }
                'A'..='Z' | 'a'..='z' => {
                    let identifier = self.read_identifier();
                    Token::Identifier(identifier)
                }

                '(' => {
                    self.advance();
                    Token::OpenParen
                }
                ')' => {
                    self.advance();
                    Token::CloseParen
                }
                '+' => {
                    self.advance();
                    Token::Operator(Ops::Add)
                }
                '-' => {
                    self.advance();
                    Token::Operator(Ops::Sub)
                }
                '*' => {
                    self.advance();
                    Token::Operator(Ops::Mul)
                }
                '/' => {
                    self.advance();
                    Token::Operator(Ops::Div)
                },
            
                _ => panic!("Unhandled char")
            };
            return token;
        }
        return Token::EOF;
    }
}