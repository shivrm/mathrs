use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug)]
pub enum Ops {
    Add,
    Sub,
    Mul,
    Div
}

#[derive(Debug)]
pub enum Tokens {
    Number(i32),
    Identifier(String),

    OpenParen,
    CloseParen,

    UnaryOp(Ops),
    BinaryOp(Ops)
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

    pub fn next_token(&mut self) -> Option<Tokens> {
        if let Some(c) = self.current_char() {
            let token = match c {
                ' ' | '\n' | '\r' | '\t' => {
                    self.skip_whitespace();
                    return self.next_token()
                }

                '0'..='9' => {
                    let number = self.read_number();
                    Tokens::Number(number) 
                }
                'A'..='Z' | 'a'..='z' => {
                    let identifier = self.read_identifier();
                    Tokens::Identifier(identifier)
                }

                '(' => {
                    self.advance();
                    Tokens::OpenParen
                }
                ')' => {
                    self.advance();
                    Tokens::CloseParen
                }
                '+' => {
                    self.advance();
                    Tokens::BinaryOp(Ops::Add)
                }
                '-' => {
                    self.advance();
                    Tokens::BinaryOp(Ops::Sub)
                }
                '*' => {
                    self.advance();
                    Tokens::BinaryOp(Ops::Mul)
                }
                '/' => {
                    self.advance();
                    Tokens::BinaryOp(Ops::Div)
                },
            
                _ => panic!("Unhandled char")
            };
            return Some(token);
        }
        return None;
    }
}