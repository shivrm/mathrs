use std::iter::Peekable;
use std::str::Chars;

use crate::mathrs::Error;

/// Enum that contains operators.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Ops {
    Add,
    Sub,
    Mul,
    Div,
    Pow
}

/// Enum used to represent a token
#[derive(Debug)]
pub enum Token {
    Number(i32),
    Identifier(String),

    OpenParen,
    CloseParen,

    Operator(Ops),
    EOF
}

/// Used for lexing math expressions
pub struct Lexer<'a> {
    pos: usize,
    _text: &'a str,
    iter: Peekable<Chars<'a>>,
    last_newline: usize,
    line: usize
}

impl<'a> Lexer<'a> {
    /// Used to create a new lexer:
    ///
    /// `text`: Text to lex
    pub fn new(text: &'a str) -> Self {
        return Lexer {
            _text: text,
            pos: 0,
            iter: text.chars().peekable(),
            last_newline: 0,
            line: 1
        }
    }

    /// Advances lexer to next position on the text
    /// and returns the new `current_char`
    fn advance(&mut self) -> Option<char> {
        let next_char = self.iter.next();

        if let Some(_) = next_char {
            self.pos += 1;
        }

        return next_char;
    }

    /// Inline function that returns the current character
    #[inline]
    fn current_char(&mut self) -> Option<&char> {
        self.iter.peek()
    }

    /// Skips consecutive whitespace characters
    fn skip_whitespace(&mut self) {
        while let c @ Some(' ' | '\n' | '\r' | '\t') = self.current_char() {
            if let Some('\n') = c {
                self.last_newline = self.pos;
                self.line += 1;
            }
            
            self.advance();
        }
    }

    /// Reads a number (consecutive digits)
    // TODO: Add support for reading floats
    fn read_number(&mut self) -> i32 {
        let mut num_string = String::new();
        
        while let c @ Some('0'..='9') = self.current_char() {
            num_string.push(*c.unwrap());
            self.advance();
        }

        return num_string.parse().unwrap();
    }

    /// Reads an identifier (consecutive letters)
    fn read_identifier(&mut self) -> String {
        let mut ident_string = String::new();
        
        while let c @ Some('A'..='Z' | 'a'..='z') = self.current_char() {
            ident_string.push(*c.unwrap());
            self.advance();
        }

        return ident_string;
    }

    /// Returns the next token, or `Token::EOF` if at end-of-string
    pub fn next_token(&mut self) -> Result<(Token, usize, usize), Error> {
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
                '^' => {
                    self.advance();
                    Token::Operator(Ops::Pow)
                }
            
                _ => return Err(Error {
                    title: "Unexpected character".to_owned(),
                    desc: "The lexer does not know how to handle this".to_owned(),
                    line: self.line, col: self.pos - self.last_newline
                })
            };
            let col = self.pos - self.last_newline;
            return Ok((token, self.line, col));
        }
        let col = self.pos - self.last_newline;
        return Ok((Token::EOF, self.line, col));
    }
}