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
