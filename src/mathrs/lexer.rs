use std::iter::Peekable;
use std::str::Chars;

pub enum Ops {
    Add,
    Sub,
    Mul,
    Div
}

pub enum Tokens {
    Number(i32),
    Identifier(String),

    OpenParen,
    CloseParen,

    UnaryOp(Ops),
    BinaryOp(Ops),
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

    pub fn read_number(&mut self) -> i32 {
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

    pub fn next_token(&mut self) -> Tokens {
        if let Some(c) = self.current_char() {
            match c {
                '0'..='9' => {
                    let number = self.read_number();
                    Tokens::Number(number) 
                }
                'A'..='Z' | 'a'..='z' => {
                    let identifier = self.read_identifier();
                    Tokens::Identifier(identifier)
                }

                '(' => Tokens::OpenParen,
                ')' => Tokens::CloseParen,

                '+' => Tokens::BinaryOp(Ops::Add),
                '-' => Tokens::BinaryOp(Ops::Sub),
                '*' => Tokens::BinaryOp(Ops::Mul),
                '/' => Tokens::BinaryOp(Ops::Div),
            
                _ => panic!("Unhandled char")
            }
        } else {
            Tokens::EOF
        }
    }
}