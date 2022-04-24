use std::iter::Peekable

pub enum Operators {
    Add,
    Sub,
    Mul,
    Div
}

pub enum Tokens {
    Number
    Identifier

    OpenParen
    CloseParen

    UnaryOp(Operator)
    BinaryOp(Operator)
}

pub struct Lexer<'a> {
    pos: u32,
    text: &'a str,
    iter: Peekable<str::Chars<'a>>
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

    pub fn next_token(&mut self) -> Token {
        todo!();
    }
}